//! Oyun alanının yerine yerel `rustc` kullanılarak yapılan uçtan uca testler.
//! rustc'nin insanlar için yazdığı çıktı oyun alanınınkiyle aynıdır.

use std::process::Command;

use serde_json::Value;

use super::*;

/// Pas kaynağını oyun alanındaki gibi derleyip çalıştırır; (standart çıktı, çözülen sonuç).
fn calistir(kaynak: &str, girdi: &str) -> (String, Value) {
    calistir_tohumla(kaynak, girdi, 7)
}

fn calistir_tohumla(kaynak: &str, girdi: &str, tohum: u64) -> (String, Value) {
    static SAYAC: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let sira = SAYAC.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dizin = std::env::temp_dir().join(format!("pas-web-{}-{sira}", std::process::id()));
    std::fs::create_dir_all(dizin.join("src")).unwrap();
    std::fs::write(dizin.join(ANA_DOSYA), derlenecek_kod(kaynak, girdi, tohum)).unwrap();
    let program = format!("program{}", std::env::consts::EXE_SUFFIX);
    let derleme = Command::new("rustc")
        .args(["--edition", "2024", ANA_DOSYA, "-o", &program])
        .current_dir(&dizin)
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .unwrap();
    let mut hata_ciktisi =
        format!("   Compiling playground v0.0.1 (/playground)\n{}", String::from_utf8_lossy(&derleme.stderr));
    let mut cikti = String::new();
    if derleme.status.success() {
        let program = Command::new(dizin.join(&program)).output().unwrap();
        hata_ciktisi.push_str("     Running `target/debug/playground`\n");
        hata_ciktisi.push_str(&String::from_utf8_lossy(&program.stderr));
        cikti = String::from_utf8(program.stdout).unwrap();
    }
    if std::env::var_os("PAS_WEB_SAKLA").is_none() {
        let _ = std::fs::remove_dir_all(&dizin);
    }
    (cikti, sonucu_coz(kaynak, girdi, tohum, &hata_ciktisi))
}

#[test]
fn tur_hatasi_pas_kaynagina_eslenir() {
    let (_, sonuc) = calistir("işlev ana() {\n    yap sayı: i32 = \"beş\";\n}\n", "");
    assert_eq!(sonuc["derlendi"], false);
    let tani = &sonuc["tanilamalar"][0];
    assert_eq!(tani["seviye"], "hata");
    assert_eq!(tani["kod"], "E0308");
    assert_eq!(tani["mesaj"], "türler uyuşmuyor");
    assert_eq!(tani["aralik"]["satir"], 2);
    assert_eq!(tani["aralik"]["sutun"], 21);
    assert_eq!(tani["aralik"]["bitis_sutunu"], 26);
    assert!(tani["aciklama"].is_string());
    let etiketler = tani["etiketler"].as_array().unwrap();
    assert_eq!(etiketler.len(), 2, "{etiketler:?}");
    assert_eq!(etiketler[0]["sutun"], 15);
}

#[test]
fn yazdir_icindeki_hata_ve_oneri() {
    let kaynak = "işlev ana() {\n    yap sayı = 5;\n    yazdır!(\"{} {}\", sayı, sayi);\n}\n";
    let (_, sonuc) = calistir(kaynak, "");
    let tani = &sonuc["tanilamalar"][0];
    assert_eq!(tani["kod"], "E0425");
    let aralik = &tani["aralik"];
    assert_eq!((aralik["satir"].as_u64(), aralik["sutun"].as_u64()), (Some(3), Some(28)));
    assert_eq!(aralik["bitis_sutunu"], 32);
    let oneri = tani["notlar"].as_array().unwrap().iter().find_map(|n| n["oneri"].as_object()).expect("öneri");
    assert_eq!(oneri["satir"], 3);
    assert_eq!(oneri["metin"], "    yazdır!(\"{} {}\", sayı, sayı);");
}

#[test]
fn panik_yeri_ve_mesaji() {
    let kaynak = "işlev ana() {\n    yap l = liste![1, 2];\n    yazdır!(\"{}\", l[5]);\n}\n";
    let (_, sonuc) = calistir(kaynak, "");
    assert_eq!(sonuc["derlendi"], true);
    let panik = &sonuc["panik"];
    assert_eq!(panik["is_parcacigi"], "ana");
    assert!(panik["mesaj"].as_str().unwrap().contains("dizin sınırların dışında"), "{panik}");
    assert_eq!(panik["aralik"]["satir"], 3);
}

#[test]
fn hazir_girdi_okunur_ve_yazilir() {
    let kaynak = "işlev ana() {\n    yap ad = oku!(\"Adın ne? \");\n    yap yaş = oku!(\"Yaşın? \", u32);\n    yazdır!(\"{ad} {}\", yaş + 1);\n}\n";
    let (cikti, sonuc) = calistir(kaynak, "Ayşe\non\n10\n");
    assert_eq!(sonuc["tanilamalar"], Value::Array(Vec::new()));
    assert_eq!(cikti, "Adın ne? Ayşe\nYaşın? on\n`on` geçerli bir tam sayı değil, tekrar dene.\nYaşın? 10\nAyşe 11\n");
}

#[test]
fn hazir_girdi_bitince_program_durur() {
    let kaynak = "işlev ana() {\n    döngü {\n        yap sayı = oku!(\"Sayı: \", i32);\n        yazdır!(\"{}\", sayı * 2);\n    }\n}\n";
    let (cikti, sonuc) = calistir(kaynak, "4\n");
    assert_eq!(cikti, "Sayı: 4\n8\nSayı: ");
    assert_eq!(sonuc["girdi_bitti"], true);
    assert_eq!(sonuc["panik"], Value::Null);
    assert_eq!(sonuc["hata_ciktisi"], "");
}

#[test]
fn kullan_onerisi() {
    let (_, sonuc) = calistir("işlev ana() {\n    yap m: Sözlük<i32, i32> = Sözlük::yeni();\n}\n", "");
    let oneri = sonuc["tanilamalar"][0]["notlar"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|n| n["oneri"].as_object())
        .expect("öneri");
    assert_eq!(oneri["metin"], "kullan std::collections::Sözlük;");
    assert_eq!(oneri["satir"], 1);
    assert_eq!(oneri["ekle"], true);
}

#[test]
fn ayni_tohumla_ayni_rastgele_sayilar() {
    let kaynak = "işlev ana() {\n    yazdır!(\"{} {}\", rastgele!(1, 1000000), rastgele!(1, 1000000));\n}\n";
    let (ilk, _) = calistir_tohumla(kaynak, "", 42);
    let (ikinci, _) = calistir_tohumla(kaynak, "", 42);
    let (baska, _) = calistir_tohumla(kaynak, "", 43);
    assert_eq!(ilk, ikinci);
    assert_ne!(ilk, baska);
}

#[test]
fn uyarilar_turkce() {
    let (cikti, sonuc) = calistir("işlev ana() {\n    yap kullanılmayan = 1;\n    yazdır!(\"{}\", doğru);\n}\n", "");
    assert_eq!(cikti, "doğru\n");
    let tani = &sonuc["tanilamalar"][0];
    assert_eq!(tani["seviye"], "uyarı");
    assert_eq!(tani["aralik"]["satir"], 2);
    assert!(tani["kod"].is_null());
}

#[test]
fn renklendirme() {
    let sozcukler = renklendir("işlev ana() { yazdır!(\"ğ\"); // not\n}");
    let siniflar: Vec<(u64, u64, &str)> = sozcukler
        .as_array()
        .unwrap()
        .iter()
        .map(|s| (s[0].as_u64().unwrap(), s[1].as_u64().unwrap(), s[2].as_str().unwrap()))
        .collect();
    assert_eq!(
        siniflar,
        vec![(0, 5, "anahtar_sozcuk"), (6, 9, "islev"), (14, 20, "makro"), (22, 25, "dizgi"), (28, 34, "yorum")]
    );
}
