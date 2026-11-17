//! rustc tanılamalarını (JSON) Pas kaynağına eşleyip Türkçe gösterir.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use pas_cekirdek::messages::{explanation, is_noise, translate_compiler_message};
use pas_cekirdek::{LineIndex, SourceMap, to_pas};
use serde::Deserialize;

use crate::project::{Prepared, slash_path};
use crate::style::Style;

#[derive(Debug, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub code: Option<Code>,
    pub level: String,
    #[serde(default)]
    pub spans: Vec<Span>,
    #[serde(default)]
    pub children: Vec<Diagnostic>,
}

#[derive(Debug, Deserialize)]
pub struct Code {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Span {
    pub file_name: String,
    pub byte_start: usize,
    pub byte_end: usize,
    pub is_primary: bool,
    pub label: Option<String>,
    pub suggested_replacement: Option<String>,
    pub expansion: Option<Box<Expansion>>,
}

#[derive(Debug, Deserialize)]
pub struct Expansion {
    pub span: Span,
}

/// Kullanıcıya gösterilecek bir kaynak dosya.
pub struct SourceFile {
    /// Kullanıcının gördüğü yol, örneğin `src/ana.pas`.
    pub display: String,
    /// Kullanıcının gördüğü metin (Pas kaynağı ya da kopyalanmış `.rs`).
    pub text: String,
    /// Derlenen Rust metni.
    pub rust_text: String,
    pub map: Option<SourceMap>,
}

impl SourceFile {
    /// Derlenen koddaki bir aralığın kullanıcının metnindeki karşılığı.
    pub fn to_display(&self, start: usize, end: usize) -> (usize, usize) {
        match &self.map {
            Some(map) => map.rust_range_to_pas(start, end),
            None => (start, end),
        }
    }
}

/// Derleyicinin andığı dosyaları bulur.
pub struct Sources {
    shadow: std::path::PathBuf,
    files: HashMap<String, Option<Rc<SourceFile>>>,
    /// Mesaj metinlerindeki `src/main.rs` → `src/ana.pas` düzeltmeleri.
    renames: Vec<(String, String)>,
}

impl Sources {
    pub fn new(shadow: &Path, prepared: &Prepared) -> Self {
        let mut files = HashMap::new();
        let mut renames = Vec::new();
        for file in &prepared.files {
            files.insert(
                file.rust_path.clone(),
                Some(Rc::new(SourceFile {
                    display: file.pas_path.clone(),
                    text: file.pas_source.clone(),
                    rust_text: file.rust_source.clone(),
                    map: Some(file.map.clone()),
                })),
            );
            renames.push((file.rust_path.clone(), file.pas_path.clone()));
        }
        Sources { shadow: shadow.to_path_buf(), files, renames }
    }

    pub fn get(&mut self, file_name: &str) -> Option<Rc<SourceFile>> {
        let key = match Path::new(file_name).strip_prefix(&self.shadow) {
            Ok(relative) => slash_path(relative),
            Err(_) => file_name.replace('\\', "/"),
        };
        if let Some(file) = self.files.get(&key) {
            return file.clone();
        }
        // Gölge projeye olduğu gibi kopyalanmış bir `.rs` dosyası olabilir.
        let path = self.shadow.join(&key);
        let file = (Path::new(&key).is_relative() && path.is_file())
            .then(|| fs::read_to_string(&path).ok())
            .flatten()
            .map(|text| Rc::new(SourceFile { display: key.clone(), rust_text: text.clone(), text, map: None }));
        self.files.insert(key, file.clone());
        file
    }

    pub fn fix_paths(&self, message: &str) -> String {
        let mut message = message.to_owned();
        for (rust, pas) in &self.renames {
            message = message.replace(rust.as_str(), pas);
        }
        message
    }
}

/// Bir kaynak dosyada işaretlenecek bir aralık.
struct Mark {
    file: Rc<SourceFile>,
    start: usize,
    end: usize,
    primary: bool,
    label: Option<String>,
}

pub struct Renderer {
    style: Style,
    pub sources: Sources,
    explained: HashSet<String>,
}

impl Renderer {
    pub fn new(style: Style, sources: Sources) -> Self {
        Renderer { style, sources, explained: HashSet::new() }
    }

    fn translate(&self, message: &str) -> String {
        self.sources.fix_paths(&translate_compiler_message(message))
    }

    pub fn render(&mut self, diag: &Diagnostic) {
        if diag.level == "failure-note" || is_noise(&diag.message) {
            return;
        }
        let (word, paint): (&str, fn(&Style, &str) -> String) = match diag.level.as_str() {
            "error" => ("hata", Style::error),
            "warning" => ("uyarı", Style::warning),
            "error: internal compiler error" => ("derleyici iç hatası", Style::error),
            other => (level_word(other), Style::note),
        };

        let mut out = String::new();
        // Lint adları (`dead_code`) çocuklar için anlamsız; yalnızca hata kodlarını göster.
        let code = diag.code.as_ref().map(|c| c.code.as_str()).filter(|c| c.starts_with('E'));
        let header = match code {
            Some(code) => format!("{word}[{code}]"),
            None => word.to_owned(),
        };
        let message = self.translate(&diag.message);
        let _ = writeln!(out, "{}{}", paint(&self.style, &header), self.style.bold(&format!(": {message}")));

        let marks: Vec<Mark> = diag.spans.iter().filter_map(|span| self.mark(span)).collect();
        let width = self.gutter_width(diag, &marks);
        let pad = " ".repeat(width);
        self.snippet(&mut out, &marks, width, paint);

        for child in &diag.children {
            if is_noise(&child.message) {
                continue;
            }
            let word = level_word(&child.level);
            let message = self.translate(&child.message);
            let suggestion = self.suggestion(child);
            let child_marks: Vec<Mark> = if suggestion.is_none() {
                child.spans.iter().filter_map(|span| self.mark(span)).collect()
            } else {
                Vec::new()
            };
            if child_marks.is_empty() {
                let _ = writeln!(out, "{pad} {} {}: {message}", self.style.gutter("="), self.style.bold(word));
            } else {
                let _ = writeln!(out, "{}: {message}", self.style.bold(word));
                self.snippet(&mut out, &child_marks, width, Style::note);
            }
            if let Some((line, text)) = suggestion {
                let _ = writeln!(out, "{pad} {}", self.style.gutter("|"));
                let _ =
                    writeln!(out, "{} {}", self.style.gutter(&format!("{line:>width$} |")), self.style.success(&text));
            }
        }

        if let Some(code) = code
            && let Some(text) = explanation(code)
            && self.explained.insert(code.to_owned())
        {
            let _ = writeln!(out, "{pad} {} {}: {text}", self.style.gutter("="), self.style.note("açıklama"));
        }
        eprintln!("{out}");
    }

    /// Bir aralığı kullanıcının dosyasında bulur. Makro içindeyse makronun
    /// çağrıldığı yere çıkar.
    fn mark(&mut self, span: &Span) -> Option<Mark> {
        match self.sources.get(&span.file_name) {
            Some(file) => {
                let (start, end) = file.to_display(span.byte_start, span.byte_end);
                Some(Mark {
                    file,
                    start,
                    end,
                    primary: span.is_primary,
                    label: span.label.as_deref().map(|l| self.translate(l)),
                })
            }
            None => {
                let mut mark = self.mark(&span.expansion.as_ref()?.span)?;
                mark.primary = span.is_primary;
                mark.label = span.label.as_deref().map(|l| self.translate(l));
                Some(mark)
            }
        }
    }

    fn gutter_width(&mut self, diag: &Diagnostic, marks: &[Mark]) -> usize {
        let mut max_line = marks.iter().map(|m| LineIndex::new(&m.file.text).line_col(m.end).0).max().unwrap_or(1);
        for child in &diag.children {
            for span in &child.spans {
                if let Some(m) = self.mark(span) {
                    max_line = max_line.max(LineIndex::new(&m.file.text).line_col(m.end).0);
                }
            }
        }
        max_line.to_string().len()
    }

    fn snippet(&self, out: &mut String, marks: &[Mark], width: usize, paint: fn(&Style, &str) -> String) {
        if marks.is_empty() {
            return;
        }
        let pad = " ".repeat(width);
        let style = &self.style;
        let primary = marks.iter().find(|m| m.primary).unwrap_or(&marks[0]);

        // Dosyalara göre grupla; birincil işaretin dosyası önce.
        let mut files: Vec<&Rc<SourceFile>> = vec![&primary.file];
        for mark in marks {
            if !files.iter().any(|f| Rc::ptr_eq(f, &mark.file)) {
                files.push(&mark.file);
            }
        }
        for (i, file) in files.into_iter().enumerate() {
            let index = LineIndex::new(&file.text);
            let first = marks.iter().filter(|m| Rc::ptr_eq(&m.file, file));
            let anchor = if i == 0 { primary } else { first.clone().next().expect("en az bir işaret") };
            let (line, col) = index.line_col(anchor.start);
            let arrow = if i == 0 { "-->" } else { ":::" };
            let _ = writeln!(out, "{pad}{} {}:{line}:{col}", style.gutter(arrow), file.display);
            let _ = writeln!(out, "{pad} {}", style.gutter("|"));

            let mut lines: BTreeMap<usize, Vec<&Mark>> = BTreeMap::new();
            for mark in first {
                lines.entry(index.line_col(mark.start).0).or_default().push(mark);
            }
            let mut previous: Option<usize> = None;
            for (line, mut line_marks) in lines {
                line_marks.sort_by_key(|m| m.start);
                if previous.is_some_and(|p| line > p + 1) {
                    let _ = writeln!(out, "{}", style.gutter("..."));
                }
                previous = Some(line);
                let text = index.line(line).unwrap_or("").replace('\t', " ");
                let _ = writeln!(out, "{} {text}", style.gutter(&format!("{line:>width$} |")));
                for mark in line_marks {
                    let (_, start_col) = index.line_col(mark.start);
                    let (end_line, end_col) = index.line_col(mark.end);
                    let end_col = if end_line == line { end_col } else { text.chars().count() + 1 };
                    let length = end_col.saturating_sub(start_col).max(1);
                    let (symbol, paint) =
                        if mark.primary { ("^", paint) } else { ("-", Style::gutter as fn(&Style, &str) -> String) };
                    let underline = format!(
                        "{}{}",
                        symbol.repeat(length),
                        mark.label.as_deref().map(|l| format!(" {l}")).unwrap_or_default()
                    );
                    let _ = writeln!(
                        out,
                        "{pad} {} {}{}",
                        style.gutter("|"),
                        " ".repeat(start_col - 1),
                        paint(style, &underline)
                    );
                }
            }
            let _ = writeln!(out, "{pad} {}", style.gutter("|"));
        }
    }

    /// Bir yardım mesajındaki kod önerisini Pas satırına uygular.
    fn suggestion(&mut self, child: &Diagnostic) -> Option<(usize, String)> {
        let mut edits = Vec::new();
        for span in &child.spans {
            let replacement = span.suggested_replacement.as_ref()?;
            if replacement.contains('\n') {
                return None;
            }
            let mark = self.mark(span)?;
            edits.push((mark, to_pas(replacement)));
        }
        let (first, _) = edits.first()?;
        let file = first.file.clone();
        let index = LineIndex::new(&file.text);
        let line = index.line_col(first.start).0;
        let line_start = index.line_start(line)?;
        let mut text = index.line(line)?.to_owned();
        edits.sort_by_key(|(mark, _)| std::cmp::Reverse(mark.start));
        for (mark, replacement) in &edits {
            if !Rc::ptr_eq(&mark.file, &file)
                || index.line_col(mark.start).0 != line
                || index.line_col(mark.end).0 != line
            {
                return None;
            }
            text.replace_range(mark.start - line_start..mark.end - line_start, replacement);
        }
        Some((line, text.replace('\t', " ")))
    }
}

fn level_word(level: &str) -> &str {
    match level {
        "error" => "hata",
        "warning" => "uyarı",
        "note" => "not",
        "help" => "yardım",
        other => other,
    }
}

/// Program çökünce (panik) gösterilecek konum ve kaynak satırı.
pub fn render_location(out: &mut String, style: &Style, file: &SourceFile, offset: usize) {
    let index = LineIndex::new(&file.text);
    let (line, col) = index.line_col(offset);
    let width = line.to_string().len();
    let pad = " ".repeat(width);
    let _ = writeln!(out, "{pad}{} {}:{line}:{col}", style.gutter("-->"), file.display);
    let _ = writeln!(out, "{pad} {}", style.gutter("|"));
    let text = index.line(line).unwrap_or("").replace('\t', " ");
    let _ = writeln!(out, "{} {text}", style.gutter(&format!("{line} |")));
    let _ = writeln!(out, "{pad} {} {}{}", style.gutter("|"), " ".repeat(col - 1), style.error("^"));
}
