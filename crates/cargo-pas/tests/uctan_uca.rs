//! `cargo pas` komutunu gerçek projeler üzerinde dener.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
    )
    .unwrap();
    for (path, contents) in files {
        fs::write(dir.join(path), contents).unwrap();
    }
    dir
}

fn cargo_pas(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
        .args(args)
        .current_dir(dir)
        .env("NO_COLOR", "1")
        .env_remove("CARGO_TERM_COLOR")
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn runs_a_program_with_turkish_modules() {
    let dir = project(
        "calisan",
        &[
            ("src/ana.pas", "modül hesap;\n\nişlev ana() {\n    yazdır!(\"{}\", hesap::topla(2, 3));\n}\n"),
            ("src/hesap.pas", "genel işlev topla(a: i32, b: i32) -> i32 {\n    a + b\n}\n"),
        ],
    );
    let output = cargo_pas(&dir, &["pas", "çalıştır"]);
    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "5\n");
}

#[test]
fn compile_errors_point_to_pas_source() {
    let dir =
        project("hatali", &[("src/ana.pas", "işlev ana() {\n    yap x = 5;\n    x = 6;\n    yazdır!(\"{x}\");\n}\n")]);
    let output = cargo_pas(&dir, &["derle"]);
    let stderr = text(&output.stderr);
    assert_eq!(output.status.code(), Some(101));
    assert!(stderr.contains("hata[E0384]"), "{stderr}");
    assert!(stderr.contains("--> src/ana.pas:3:5"), "{stderr}");
    assert!(stderr.contains("yap değişken x = 5;"), "{stderr}");
    assert!(!stderr.contains("main.rs"), "{stderr}");
}

#[test]
fn panics_are_translated() {
    let dir = project(
        "coken",
        &[(
            "src/ana.pas",
            "işlev ana() {\n    yap l = liste![1, 2];\n    yap i = l.uzunluk() + 1;\n    yazdır!(\"{}\", l[i]);\n}\n",
        )],
    );
    let output = cargo_pas(&dir, &["çalıştır"]);
    let stderr = text(&output.stderr);
    assert_eq!(output.status.code(), Some(101));
    assert!(stderr.contains("program çöktü"), "{stderr}");
    assert!(stderr.contains("--> src/ana.pas:4:"), "{stderr}");
    assert!(stderr.contains("uzunluk 2 ama istenen dizin 3"), "{stderr}");
}

#[test]
fn new_project_runs() {
    let parent = Path::new(env!("CARGO_TARGET_TMPDIR")).join("yeni");
    let _ = fs::remove_dir_all(&parent);
    fs::create_dir_all(&parent).unwrap();
    assert!(cargo_pas(&parent, &["yeni", "İlk Oyun"]).status.success());
    let manifest = fs::read_to_string(parent.join("İlk Oyun/Cargo.toml")).unwrap();
    assert!(manifest.contains("name = \"ilk-oyun\""), "{manifest}");
    // IDE'ler projeyi `cargo metadata` ile okur; `.rs` dosyası olmasa da çalışmalı.
    let metadata = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(parent.join("İlk Oyun"))
        .output()
        .unwrap();
    assert!(metadata.status.success(), "{}", text(&metadata.stderr));
    let output = cargo_pas(&parent.join("İlk Oyun"), &["çalıştır"]);
    assert_eq!(text(&output.stdout), "Merhaba dünya!\n", "{}", text(&output.stderr));
}
