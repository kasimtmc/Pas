//! `dersler/` ve `ornekler/` altındaki bütün Pas projelerinin hatasız ve
//! uyarısız derlendiğini denetler. Sözlükte bir sözcük değişirse dersler de
//! güncellenmelidir; uyarılar ise yeni başlayanların kafasını karıştırır.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

fn kopyala(kaynak: &Path, hedef: &Path) {
    fs::create_dir_all(hedef).unwrap();
    for girdi in fs::read_dir(kaynak).unwrap() {
        let yol = girdi.unwrap().path();
        let ad = yol.file_name().unwrap();
        if ad == "target" {
            continue;
        }
        if yol.is_dir() { kopyala(&yol, &hedef.join(ad)) } else { fs::copy(&yol, hedef.join(ad)).map(|_| ()).unwrap() }
    }
}

fn derle(proje: &Path, kok: &Path) -> Option<String> {
    let ad = format!(
        "{}-{}",
        proje.parent().unwrap().file_name().unwrap().to_string_lossy(),
        proje.file_name().unwrap().to_string_lossy()
    );
    let kopya = kok.join(&ad);
    let _ = fs::remove_dir_all(&kopya);
    kopyala(proje, &kopya);
    let cikti = Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
        .arg("derle")
        .current_dir(&kopya)
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let hata_ciktisi = String::from_utf8_lossy(&cikti.stderr);
    let sorunlu =
        !cikti.status.success() || hata_ciktisi.lines().any(|s| s.starts_with("uyarı") || s.starts_with("hata"));
    sorunlu.then(|| format!("### {ad}\n{hata_ciktisi}"))
}

#[test]
fn dersler_ve_ornekler_uyarisiz_derlenir() {
    let depo = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut projeler: Vec<PathBuf> = ["dersler", "ornekler"]
        .iter()
        .flat_map(|dizin| fs::read_dir(depo.join(dizin)).unwrap())
        .map(|g| g.unwrap().path())
        .filter(|yol| yol.join("Cargo.toml").is_file())
        .collect();
    projeler.sort();
    assert!(projeler.len() >= 12, "yalnızca {} proje bulundu", projeler.len());

    let kok = Path::new(env!("CARGO_TARGET_TMPDIR")).join("dersler");
    let sorunlar: Vec<String> = thread::scope(|kapsam| {
        let isler: Vec<_> = projeler
            .iter()
            .map(|proje| {
                let kok = &kok;
                kapsam.spawn(move || derle(proje, kok))
            })
            .collect();
        isler.into_iter().filter_map(|is| is.join().unwrap()).collect()
    });
    assert!(sorunlar.is_empty(), "Sorunlu projeler:\n{}", sorunlar.join("\n"));
}
