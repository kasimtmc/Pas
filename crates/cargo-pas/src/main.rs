//! `cargo pas`: Pas (Türkçe Rust) projelerini derler ve çalıştırır.

mod cargo;
mod diagnostics;
mod project;
mod run;
mod style;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

use pas_cekirdek::dictionary::{self, ENTRIES};

use crate::diagnostics::{Renderer, Sources};
use crate::project::Project;
use crate::style::Style;

const HELP: &str = "\
Pas: Türkçe Rust

Kullanım: cargo pas <komut> [seçenekler]

Komutlar:
    yeni <ad>        Yeni bir Pas projesi oluşturur
    çalıştır         Projeyi derler ve çalıştırır
    derle            Projeyi derler
    denetle          Projeyi derlemeden hatalara bakar (daha hızlı)
    çevir <dosya>    Bir Pas dosyasının Rust karşılığını gösterir
                     (--ters: Rust dosyasını Pas'a çevirir)
    sözlük           Türkçe sözcükleri ve Rust karşılıklarını listeler
                     (--json: makinenin okuyacağı biçimde)
    yardım           Bu yardımı gösterir
    sürüm            Pas sürümünü gösterir

Derleme seçenekleri (çalıştır, derle, denetle):
    --yayın          Hızlı çalışan (yayın) sürümü derler
    --ikili <ad>     Birden fazla program varsa hangisinin çalışacağı
    -- <argümanlar>  Bundan sonrası programa verilir

Komutların İngilizce adları da çalışır: new, run, build, check,
translate, dictionary, help, version.";

fn main() -> ExitCode {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // `cargo pas …` olarak çağrılınca ilk argüman `pas` olur.
    if args.first().is_some_and(|a| a == "pas") {
        args.remove(0);
    }
    let style = Style::detect();
    match run(&args, style) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{}: {message}", style.error("hata"));
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String], style: Style) -> Result<ExitCode, String> {
    let Some((command, rest)) = args.split_first() else {
        println!("{HELP}");
        return Ok(ExitCode::SUCCESS);
    };
    match command.as_str() {
        "yeni" | "new" => new_project(rest, style),
        "çalıştır" | "calistir" | "run" => build(rest, Mode::Run, style),
        "derle" | "build" => build(rest, Mode::Build, style),
        "denetle" | "check" => build(rest, Mode::Check, style),
        "çevir" | "cevir" | "translate" => translate(rest),
        "sözlük" | "sozluk" | "dictionary" => print_dictionary(rest),
        "yardım" | "yardim" | "help" | "-h" | "--help" => {
            println!("{HELP}");
            Ok(ExitCode::SUCCESS)
        }
        "sürüm" | "surum" | "version" | "-V" | "--version" => {
            println!("cargo-pas {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        other => Err(format!("bilinmeyen komut `{other}`. Komutları görmek için: cargo pas yardım")),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Run,
    Build,
    Check,
}

#[derive(Default)]
struct BuildOptions {
    release: bool,
    bin: Option<String>,
    program_args: Vec<String>,
}

impl BuildOptions {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut options = BuildOptions::default();
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--yayın" | "--yayin" | "--release" => options.release = true,
                "--ikili" | "--bin" => {
                    options.bin = Some(args.next().ok_or("`--ikili` seçeneğinden sonra bir ad yaz")?.clone());
                }
                "--" => {
                    options.program_args = args.cloned().collect();
                    break;
                }
                other => return Err(format!("bilinmeyen seçenek `{other}`")),
            }
        }
        Ok(options)
    }
}

fn build(args: &[String], mode: Mode, style: Style) -> Result<ExitCode, String> {
    let options = BuildOptions::parse(args)?;
    if mode != Mode::Run && !options.program_args.is_empty() {
        return Err("programa argüman yalnızca `çalıştır` ile verilebilir".into());
    }
    let cwd = std::env::current_dir().map_err(|e| format!("bulunulan dizin okunamadı: {e}"))?;
    let project = Project::find(&cwd)?;
    let prepared = project.prepare()?;
    if prepared.files.is_empty() {
        eprintln!(
            "{}: projede hiç `.pas` dosyası yok; `src/ana.pas` dosyasını oluşturarak başlayabilirsin",
            style.warning("uyarı")
        );
    } else {
        style.status("Çevrildi", &format!("{} Pas dosyası", prepared.files.len()));
    }

    let manifest = project.shadow_manifest();
    let mut cargo_args = vec![
        if mode == Mode::Check { "check" } else { "build" },
        "--manifest-path",
        manifest.to_str().ok_or("proje yolu geçersiz karakterler içeriyor")?,
        "--target-dir",
        project.target_dir.to_str().ok_or("proje yolu geçersiz karakterler içeriyor")?,
    ];
    if options.release {
        cargo_args.push("--release");
    }
    if let Some(bin) = &options.bin {
        cargo_args.extend(["--bin", bin]);
    }

    let mut renderer = Renderer::new(style, Sources::new(&project.shadow, &prepared));
    let outcome = cargo::run(&cargo_args, &mut renderer, style)?;
    if !outcome.success {
        return Ok(ExitCode::from(101));
    }
    if mode != Mode::Run {
        return Ok(ExitCode::SUCCESS);
    }

    let exe = choose_executable(&outcome.executables, &options, &project)?;
    let shown = exe.path.strip_prefix(&project.root).unwrap_or(&exe.path);
    style.status("Çalıştırılıyor", &format!("`{}`", shown.display()));
    run::run_program(&exe.path, &options.program_args, &mut renderer.sources, style)
}

fn choose_executable<'a>(
    executables: &'a [cargo::Executable],
    options: &BuildOptions,
    project: &Project,
) -> Result<&'a cargo::Executable, String> {
    if let Some(bin) = &options.bin {
        return executables.iter().find(|e| &e.name == bin).ok_or_else(|| format!("`{bin}` adında bir program yok"));
    }
    match executables {
        [] => Err("çalıştırılacak bir program bulunamadı; `src/ana.pas` dosyası var mı?".into()),
        [only] => Ok(only),
        many => many.iter().find(|e| Some(&e.name) == project.package_name.as_ref()).ok_or_else(|| {
            let names: Vec<&str> = many.iter().map(|e| e.name.as_str()).collect();
            format!("birden fazla program var: {}. Hangisini çalıştıracağını `--ikili <ad>` ile seç", names.join(", "))
        }),
    }
}

fn new_project(args: &[String], style: Style) -> Result<ExitCode, String> {
    let [name] = args else {
        return Err("proje adını yaz: cargo pas yeni <ad>".into());
    };
    let dir = Path::new(name);
    if dir.exists() {
        return Err(format!("`{name}` zaten var"));
    }
    let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or(name);
    let package = package_name(dir_name);
    if package.is_empty() {
        return Err(format!("`{name}` proje adı olarak kullanılamaz; harflerle başlayan bir ad seç"));
    }

    let manifest =
        format!("[package]\nname = \"{package}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n");
    let main = "işlev ana() {\n    yazdır!(\"Merhaba dünya!\");\n}\n";
    let write = |path: &Path, contents: &str| {
        fs::write(path, contents).map_err(|e| format!("`{}` yazılamadı: {e}", path.display()))
    };
    fs::create_dir_all(dir.join("src")).map_err(|e| format!("`{name}` oluşturulamadı: {e}"))?;
    write(&dir.join("Cargo.toml"), &manifest)?;
    write(&dir.join("src/ana.pas"), main)?;
    write(&dir.join(".gitignore"), "/target\n")?;

    style.status("Oluşturuldu", &format!("`{name}` projesi"));
    eprintln!("\nBaşlamak için:\n    cd {name}\n    cargo pas çalıştır");
    Ok(ExitCode::SUCCESS)
}

/// cargo paket adları ASCII olmalı: `Uzay Oyunu` → `uzay-oyunu`.
fn package_name(name: &str) -> String {
    let mut out = String::new();
    for c in name.chars() {
        let c = match c {
            'ç' | 'Ç' => 'c',
            'ğ' | 'Ğ' => 'g',
            'ı' | 'I' | 'İ' | 'i' => 'i',
            'ö' | 'Ö' => 'o',
            'ş' | 'Ş' => 's',
            'ü' | 'Ü' => 'u',
            'â' | 'Â' => 'a',
            'î' | 'Î' => 'i',
            'û' | 'Û' => 'u',
            c if c.is_ascii_alphanumeric() || c == '_' => c.to_ascii_lowercase(),
            _ => '-',
        };
        if !(c == '-' && (out.is_empty() || out.ends_with('-'))) {
            out.push(c);
        }
    }
    let trimmed = out.trim_end_matches('-').trim_start_matches(|c: char| c.is_ascii_digit() || c == '-');
    trimmed.to_owned()
}

fn translate(args: &[String]) -> Result<ExitCode, String> {
    let mut reverse = false;
    let mut file = None;
    for arg in args {
        match arg.as_str() {
            "--ters" | "--reverse" => reverse = true,
            other if file.is_none() => file = Some(other),
            other => return Err(format!("fazladan argüman `{other}`")),
        }
    }
    let file = file.ok_or("çevrilecek dosyayı yaz: cargo pas çevir <dosya.pas> (standart girdi için `-`)")?;
    let source = if file == "-" {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).map_err(|e| format!("girdi okunamadı: {e}"))?;
        text
    } else {
        fs::read_to_string(file).map_err(|e| format!("`{file}` okunamadı: {e}"))?
    };
    let output = if reverse { pas_cekirdek::to_pas(&source) } else { pas_cekirdek::to_rust(&source).rust };
    print!("{output}");
    Ok(ExitCode::SUCCESS)
}

fn print_dictionary(args: &[String]) -> Result<ExitCode, String> {
    match args {
        [] => {
            let mut kind = None;
            for entry in ENTRIES {
                if kind != Some(entry.kind) {
                    kind = Some(entry.kind);
                    println!("\n{}:", capitalize(&plural(entry.kind)));
                }
                let pas = match entry.kind {
                    dictionary::Kind::Macro => format!("{}!", entry.pas),
                    _ => entry.pas.to_owned(),
                };
                let rust = match entry.kind {
                    dictionary::Kind::Macro => format!("{}!", entry.rust),
                    _ => entry.rust.to_owned(),
                };
                println!("    {pas:<18} {rust:<14} {}", entry.description);
            }
            Ok(ExitCode::SUCCESS)
        }
        [flag] if flag == "--json" => {
            let entries: Vec<serde_json::Value> = ENTRIES
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "pas": e.pas,
                        "rust": e.rust,
                        "kind": e.kind.id(),
                        "description": e.description,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&entries).expect("JSON"));
            Ok(ExitCode::SUCCESS)
        }
        _ => Err("kullanım: cargo pas sözlük [--json]".into()),
    }
}

fn plural(kind: dictionary::Kind) -> String {
    match kind {
        dictionary::Kind::Keyword => "anahtar sözcükler".into(),
        dictionary::Kind::Macro => "makrolar".into(),
        dictionary::Kind::Type => "türler".into(),
        dictionary::Kind::Value => "değerler".into(),
        dictionary::Kind::Function => "işlevler".into(),
        dictionary::Kind::Method => "yöntemler".into(),
    }
}

fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_names() {
        assert_eq!(package_name("Uzay Oyunu"), "uzay-oyunu");
        assert_eq!(package_name("çılgın_şövalye"), "cilgin_sovalye");
        assert_eq!(package_name("İlk Program!"), "ilk-program");
        assert_eq!(package_name("3 kedi"), "kedi");
    }
}
