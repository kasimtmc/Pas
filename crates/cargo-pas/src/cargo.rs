//! cargo'yu çalıştırır; çıktısındaki tanılamaları ve durum satırlarını
//! Türkçeleştirir.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;

use serde::Deserialize;

use crate::diagnostics::{Diagnostic, Renderer};
use crate::style::Style;

#[derive(Deserialize)]
struct Message {
    reason: String,
    message: Option<Diagnostic>,
    executable: Option<PathBuf>,
    target: Option<Target>,
    success: Option<bool>,
}

#[derive(Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
}

pub struct Executable {
    pub name: String,
    pub path: PathBuf,
}

pub struct Outcome {
    pub success: bool,
    pub executables: Vec<Executable>,
}

/// `cargo <alt komut> --message-format=json …` çalıştırır.
pub fn run(args: &[&str], renderer: &mut Renderer, style: Style) -> Result<Outcome, String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut child = Command::new(cargo)
        .args(args)
        .arg("--message-format=json")
        .env("CARGO_TERM_PROGRESS_WHEN", "never")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cargo çalıştırılamadı: {e}"))?;

    // Durum satırları geldikçe yazılır; özet satırları tanılamalardan sonra.
    let stderr = child.stderr.take().expect("hata çıktısı yönlendirildi");
    let status_lines = thread::spawn(move || {
        let mut summary = Vec::new();
        for line in BufReader::new(stderr).lines().map_while(Result::ok) {
            match translate_status(&line, style) {
                Status::Now(text) => eprintln!("{text}"),
                Status::Later(text) => summary.push(text),
                Status::Hide => {}
            }
        }
        summary
    });

    let mut outcome = Outcome { success: false, executables: Vec::new() };
    let stdout = child.stdout.take().expect("standart çıktı yönlendirildi");
    for line in BufReader::new(stdout).lines().map_while(Result::ok) {
        let Ok(message) = serde_json::from_str::<Message>(&line) else {
            println!("{line}");
            continue;
        };
        match message.reason.as_str() {
            "compiler-message" => {
                if let Some(diag) = &message.message {
                    renderer.render(diag);
                }
            }
            "compiler-artifact" => {
                if let (Some(path), Some(target)) = (message.executable, message.target)
                    && target.kind.iter().any(|k| k == "bin")
                {
                    outcome.executables.push(Executable { name: target.name, path });
                }
            }
            "build-finished" => outcome.success = message.success.unwrap_or(false),
            _ => {}
        }
    }

    let status = child.wait().map_err(|e| format!("cargo beklenirken hata: {e}"))?;
    for line in status_lines.join().unwrap_or_default() {
        eprintln!("{line}");
    }
    outcome.success &= status.success();
    Ok(outcome)
}

enum Status {
    Now(String),
    Later(String),
    Hide,
}

/// cargo'nun durum satırlarını Türkçeleştirir.
fn translate_status(line: &str, style: Style) -> Status {
    let trimmed = line.trim_start();
    let status = |word: &str, rest: &str| Status::Now(format!("{} {rest}", style.success(&format!("{word:>15}"))));
    let words: &[(&str, &str)] = &[
        ("Compiling ", "Derleniyor"),
        ("Checking ", "Denetleniyor"),
        ("Updating ", "Güncelleniyor"),
        ("Locking ", "Kilitleniyor"),
        ("Adding ", "Ekleniyor"),
        ("Downloading ", "İndiriliyor"),
        ("Downloaded ", "İndirildi"),
        ("Documenting ", "Belgeleniyor"),
        ("Fresh ", "Güncel"),
    ];
    for (english, turkish) in words {
        if let Some(rest) = trimmed.strip_prefix(english) {
            return status(turkish, strip_path(rest));
        }
    }
    if let Some(rest) = trimmed.strip_prefix("Finished ") {
        let profile = if rest.starts_with("`release`") { "hızlı (yayın) derleme, " } else { "" };
        let time = rest.rsplit(" in ").next().unwrap_or("").trim_end_matches('s');
        return status("Tamamlandı", &format!("{profile}{time} saniyede"));
    }
    if trimmed.starts_with("Blocking waiting for file lock") {
        return status("Bekleniyor", "başka bir derlemenin bitmesi bekleniyor");
    }
    if trimmed.starts_with("Running ") {
        return Status::Hide;
    }
    if let Some(rest) = line.strip_prefix("error: could not compile `") {
        let name = rest.split('`').next().unwrap_or("");
        let errors = count_before(rest, "previous error");
        let warnings = count_before(rest, "warning");
        let mut text = format!("{}: `{name}` derlenemedi", style.error("hata"));
        if let Some(n) = errors {
            let _ = std::fmt::Write::write_fmt(&mut text, format_args!(" ({n} hata"));
            match warnings {
                Some(w) => text.push_str(&format!(", {w} uyarı)")),
                None => text.push(')'),
            }
        }
        return Status::Later(text);
    }
    if let Some(rest) = line.strip_prefix("warning: `")
        && rest.contains(" generated ")
    {
        let name = rest.split('`').next().unwrap_or("");
        let n = count_before(rest, "warning").unwrap_or(1);
        return Status::Later(format!("{}: `{name}` için {n} uyarı", style.warning("uyarı")));
    }
    if line.starts_with("note: ") && line.contains("cargo fix") {
        return Status::Hide;
    }
    Status::Now(line.to_owned())
}

/// `proje v0.1.0 (/uzun/yol)` → `proje v0.1.0`
fn strip_path(rest: &str) -> &str {
    match rest.rfind(" (") {
        Some(i) if rest[i..].contains('/') || rest[i..].contains('\\') => &rest[..i],
        _ => rest,
    }
}

/// "… due to 2 previous errors" içinden `word`den önceki sayı.
fn count_before(text: &str, word: &str) -> Option<usize> {
    let before = &text[..text.find(word)?];
    before.split_whitespace().last()?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(line: &str) -> String {
        match translate_status(line, Style::plain()) {
            Status::Now(t) | Status::Later(t) => t.trim().to_owned(),
            Status::Hide => String::new(),
        }
    }

    #[test]
    fn status_lines() {
        assert_eq!(text("   Compiling oyun v0.1.0 (/home/x/oyun/target/pas)"), "Derleniyor oyun v0.1.0");
        assert_eq!(
            text("    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s"),
            "Tamamlandı 0.52 saniyede"
        );
        assert_eq!(
            text("error: could not compile `oyun` (bin \"oyun\") due to 2 previous errors; 1 warning emitted"),
            "hata: `oyun` derlenemedi (2 hata, 1 uyarı)"
        );
        assert_eq!(
            text(
                "warning: `oyun` (bin \"oyun\") generated 3 warnings (run `cargo fix --bin \"oyun\"` to apply 1 suggestion)"
            ),
            "uyarı: `oyun` için 3 uyarı"
        );
    }
}
