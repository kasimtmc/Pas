//! Derlenen programı çalıştırır; çökerse (panik) mesajını Türkçeleştirip
//! Pas kaynağındaki yerini gösterir.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

use pas_cekirdek::messages::translate_panic_message;
use pas_cekirdek::{Bias, LineIndex};

use crate::diagnostics::{Sources, render_location};
use crate::style::Style;

pub fn run_program(exe: &Path, args: &[String], sources: &mut Sources, style: Style) -> Result<ExitCode, String> {
    let mut child = Command::new(exe)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("program çalıştırılamadı: {e}"))?;

    let stderr = child.stderr.take().expect("hata çıktısı yönlendirildi");
    let mut reader = BufReader::new(stderr);
    let mut buffer = Vec::new();
    let mut in_panic = false;
    let mut err = std::io::stderr();
    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let line = String::from_utf8_lossy(&buffer);
        let content = line.trim_end_matches(['\n', '\r']);

        if let Some(header) = panic_header(content, sources, style) {
            let _ = write!(err, "{header}");
            in_panic = true;
        } else if in_panic && content.starts_with("note: run with `RUST_BACKTRACE=1`") {
            let _ = writeln!(
                err,
                "{}: çökmenin hangi işlev çağrılarından geçtiğini görmek için programı \
                 `RUST_BACKTRACE=1` ortam değişkeniyle çalıştırabilirsin",
                style.note("not")
            );
            in_panic = false;
        } else if in_panic {
            let _ = writeln!(err, "  {}", style.bold(&translate_panic_message(content)));
        } else {
            let _ = err.write_all(&buffer);
        }
    }

    let status = child.wait().map_err(|e| format!("program beklenirken hata: {e}"))?;
    Ok(match status.code() {
        Some(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        None => ExitCode::from(1),
    })
}

/// `thread 'main' (1234) panicked at src/main.rs:5:34:`
fn panic_header(line: &str, sources: &mut Sources, style: Style) -> Option<String> {
    let rest = line.strip_prefix("thread '")?;
    let thread = &rest[..rest.find('\'')?];
    let location = rest.split(" panicked at ").nth(1)?.strip_suffix(':')?;
    let mut parts = location.rsplitn(3, ':');
    let col: usize = parts.next()?.parse().ok()?;
    let line_no: usize = parts.next()?.parse().ok()?;
    let file_name = parts.next()?;

    let mut out = String::new();
    let thread = if thread == "main" { "ana".to_owned() } else { thread.to_owned() };
    out.push_str(&format!(
        "\n{}{}\n",
        style.error("hata"),
        style.bold(&format!(": program çöktü (panik, `{thread}` iş parçacığında)"))
    ));
    match sources.get(file_name) {
        Some(file) => {
            let rust_offset = LineIndex::new(&file.rust_text).offset(line_no, col).unwrap_or(0);
            let offset = match &file.map {
                Some(map) => map.rust_to_pas(rust_offset, Bias::Start),
                None => rust_offset,
            };
            render_location(&mut out, &style, &file, offset);
        }
        None => out.push_str(&format!("  {} {}\n", style.gutter("-->"), sources.fix_paths(location))),
    }
    Some(out)
}
