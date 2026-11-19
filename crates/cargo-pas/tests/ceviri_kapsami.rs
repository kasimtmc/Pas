//! Çocukların sık yapacağı hataların (`tests/hata_derlemi/*.pas`) derleyici
//! mesajlarının Türkçeye çevrildiğini denetler. rustc güncellenip bir mesajın
//! yazımı değişirse bu test hangi mesajın İngilizce kaldığını gösterir.
//!
//! Bazı hatalar (örneğin dizi sınırı dışına erişim) yalnızca gerçek derlemede
//! çıktığı için `denetle` yerine `derle` kullanılır.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

/// Ters tırnak dışında görülmemesi gereken İngilizce sözcükler.
const INGILIZCE: &[&str] = &[
    "the",
    "this",
    "that",
    "expected",
    "found",
    "consider",
    "cannot",
    "can't",
    "value",
    "type",
    "types",
    "function",
    "method",
    "here",
    "is",
    "are",
    "was",
    "to",
    "of",
    "for",
    "with",
    "be",
    "use",
    "used",
    "help",
    "missing",
    "argument",
    "arguments",
    "implement",
    "trait",
    "borrow",
    "mutable",
    "variable",
    "field",
    "pattern",
    "unknown",
    "unused",
    "similar",
    "defined",
    "declared",
    "required",
    "because",
    "which",
    "but",
    "should",
    "has",
    "have",
    "you",
    "your",
    "add",
    "remove",
    "instead",
    "an",
    "or",
    "and",
    "if",
];

fn ingilizce_sozcuk(metin: &str) -> Option<String> {
    // "not: …", "yardım: …" önekleri Türkçedir; `kod` parçaları sayılmaz.
    let metin = ["not: ", "yardım: ", "uyarı: ", "hata: "].iter().find_map(|o| metin.strip_prefix(o)).unwrap_or(metin);
    let duz: String = metin.split('`').step_by(2).collect::<Vec<_>>().join(" ");
    duz.split(|c: char| !c.is_alphanumeric() && c != '\'').find(|sozcuk| INGILIZCE.contains(sozcuk)).map(str::to_owned)
}

fn ornegi_denetle(pas_dosyasi: &Path, kok: &Path) -> Vec<String> {
    let ad = pas_dosyasi.file_stem().unwrap().to_string_lossy().into_owned();
    let dizin = kok.join(&ad);
    let _ = fs::remove_dir_all(&dizin);
    fs::create_dir_all(dizin.join("src")).unwrap();
    fs::write(
        dizin.join("Cargo.toml"),
        format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n", ad.replace('_', "-")),
    )
    .unwrap();
    fs::copy(pas_dosyasi, dizin.join("src/ana.pas")).unwrap();
    let cikti =
        Command::new(env!("CARGO_BIN_EXE_cargo-pas")).args(["derle", "--json"]).current_dir(&dizin).output().unwrap();

    let mut sorunlar = Vec::new();
    let mut tani_sayisi = 0;
    // Her hata kodunun çocuklara yönelik bir açıklaması olmalı (aynı kod bir
    // derlemede yalnızca ilk kez açıklanır).
    let mut kodlar = std::collections::BTreeMap::<String, bool>::new();
    for satir in String::from_utf8_lossy(&cikti.stdout).lines() {
        let Ok(tani) = serde_json::from_str::<serde_json::Value>(satir) else { continue };
        tani_sayisi += 1;
        if let Some(kod) = tani["kod"].as_str() {
            *kodlar.entry(kod.to_owned()).or_default() |= tani["aciklama"].is_string();
        }
        let mut metinler = vec![tani["mesaj"].as_str().unwrap_or_default().to_owned()];
        metinler.extend(
            tani["etiketler"].as_array().into_iter().flatten().filter_map(|e| e["mesaj"].as_str().map(str::to_owned)),
        );
        // Notların ikinci satırı önerilen Pas kodudur.
        metinler.extend(
            tani["notlar"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|n| n.as_str()?.lines().next().map(str::to_owned)),
        );
        for metin in metinler {
            if let Some(sozcuk) = ingilizce_sozcuk(&metin) {
                sorunlar.push(format!("{ad}: \"{sozcuk}\" → {metin}"));
            }
        }
    }
    for (kod, _) in kodlar.iter().filter(|(_, aciklandi)| !**aciklandi) {
        sorunlar.push(format!("{ad}: {kod} hata kodunun açıklaması yok (mesajlar.rs içindeki ACIKLAMALAR)"));
    }
    if tani_sayisi == 0 {
        sorunlar.push(format!("{ad}: hiç tanılama üretilmedi (örnek artık hata vermiyor mu?)"));
    }
    sorunlar
}

#[test]
fn hata_derlemindeki_mesajlar_turkce() {
    let derlem = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/hata_derlemi");
    let kok = Path::new(env!("CARGO_TARGET_TMPDIR")).join("hata_derlemi");
    let mut ornekler: Vec<PathBuf> = fs::read_dir(&derlem).unwrap().map(|g| g.unwrap().path()).collect();
    ornekler.sort();
    assert!(ornekler.len() >= 30, "derlemde yalnızca {} örnek var", ornekler.len());

    // Her örnek kendi projesinde derlenir; birkaçı aynı anda çalışabilir.
    let sorunlar: Vec<String> = thread::scope(|kapsam| {
        let isler: Vec<_> = ornekler
            .chunks(ornekler.len().div_ceil(8))
            .map(|parca| {
                let kok = &kok;
                kapsam.spawn(move || parca.iter().flat_map(|ornek| ornegi_denetle(ornek, kok)).collect::<Vec<_>>())
            })
            .collect();
        isler.into_iter().flat_map(|is| is.join().unwrap()).collect()
    });
    assert!(sorunlar.is_empty(), "Türkçeye çevrilmemiş mesajlar:\n{}", sorunlar.join("\n"));
}
