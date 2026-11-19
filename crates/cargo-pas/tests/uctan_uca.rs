//! `cargo pas` komutunu gerçek projeler üzerinde dener.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn proje(ad: &str, dosyalar: &[(&str, &str)]) -> PathBuf {
    let dizin = Path::new(env!("CARGO_TARGET_TMPDIR")).join(ad);
    let _ = fs::remove_dir_all(&dizin);
    fs::create_dir_all(dizin.join("src")).unwrap();
    fs::write(
        dizin.join("Cargo.toml"),
        format!("[package]\nname = \"{ad}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
    )
    .unwrap();
    for (yol, icerik) in dosyalar {
        fs::write(dizin.join(yol), icerik).unwrap();
    }
    dizin
}

fn cargo_pas(dizin: &Path, argumanlar: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
        .args(argumanlar)
        .current_dir(dizin)
        .env("NO_COLOR", "1")
        .env_remove("CARGO_TERM_COLOR")
        .output()
        .unwrap()
}

fn metin(baytlar: &[u8]) -> String {
    String::from_utf8_lossy(baytlar).into_owned()
}

#[test]
fn turkce_modullu_program_calisir() {
    let dizin = proje(
        "calisan",
        &[
            ("src/ana.pas", "modül hesap;\n\nişlev ana() {\n    yazdır!(\"{}\", hesap::topla(2, 3));\n}\n"),
            ("src/hesap.pas", "genel işlev topla(a: i32, b: i32) -> i32 {\n    a + b\n}\n"),
        ],
    );
    let cikti = cargo_pas(&dizin, &["pas", "çalıştır"]);
    assert!(cikti.status.success(), "{}", metin(&cikti.stderr));
    assert_eq!(metin(&cikti.stdout), "5\n");
}

#[test]
fn derleme_hatalari_pas_kaynagini_gosterir() {
    let dizin =
        proje("hatali", &[("src/ana.pas", "işlev ana() {\n    yap x = 5;\n    x = 6;\n    yazdır!(\"{x}\");\n}\n")]);
    let cikti = cargo_pas(&dizin, &["derle"]);
    let hata_ciktisi = metin(&cikti.stderr);
    assert_eq!(cikti.status.code(), Some(101));
    assert!(hata_ciktisi.contains("hata[E0384]"), "{hata_ciktisi}");
    assert!(hata_ciktisi.contains("--> src/ana.pas:3:5"), "{hata_ciktisi}");
    assert!(hata_ciktisi.contains("yap değişken x = 5;"), "{hata_ciktisi}");
    assert!(!hata_ciktisi.contains("main.rs"), "{hata_ciktisi}");
}

#[test]
fn panikler_cevrilir() {
    let dizin = proje(
        "coken",
        &[(
            "src/ana.pas",
            "işlev ana() {\n    yap l = liste![1, 2];\n    yap i = l.uzunluk() + 1;\n    yazdır!(\"{}\", l[i]);\n}\n",
        )],
    );
    let cikti = cargo_pas(&dizin, &["çalıştır"]);
    let hata_ciktisi = metin(&cikti.stderr);
    assert_eq!(cikti.status.code(), Some(101));
    assert!(hata_ciktisi.contains("program çöktü"), "{hata_ciktisi}");
    assert!(hata_ciktisi.contains("--> src/ana.pas:4:"), "{hata_ciktisi}");
    assert!(hata_ciktisi.contains("uzunluk 2 ama istenen dizin 3"), "{hata_ciktisi}");
}

#[test]
fn yeni_proje_calisir() {
    let ust_dizin = Path::new(env!("CARGO_TARGET_TMPDIR")).join("yeni");
    let _ = fs::remove_dir_all(&ust_dizin);
    fs::create_dir_all(&ust_dizin).unwrap();
    assert!(cargo_pas(&ust_dizin, &["yeni", "İlk Oyun"]).status.success());
    let bildirim = fs::read_to_string(ust_dizin.join("İlk Oyun/Cargo.toml")).unwrap();
    assert!(bildirim.contains("name = \"ilk-oyun\""), "{bildirim}");
    // IDE'ler projeyi `cargo metadata` ile okur; `.rs` dosyası olmasa da çalışmalı.
    let ust_veri = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(ust_dizin.join("İlk Oyun"))
        .output()
        .unwrap();
    assert!(ust_veri.status.success(), "{}", metin(&ust_veri.stderr));
    let cikti = cargo_pas(&ust_dizin.join("İlk Oyun"), &["çalıştır"]);
    assert_eq!(metin(&cikti.stdout), "Merhaba dünya!\n", "{}", metin(&cikti.stderr));
}

#[test]
fn json_tanilamalari_ide_icin_okunabilir() {
    let dizin = proje("jsonlu", &[("src/ana.pas", "işlev ana() {\n    yap x = 5;\n    x = 6;\n}\n")]);
    // Kaydedilmemiş içerik standart girdiden verilir; diskteki dosya farklıdır.
    let mut surec = Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
        .args(["denetle", "--json", "--stdin", "src/ana.pas"])
        .current_dir(&dizin)
        .env("NO_COLOR", "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    use std::io::Write;
    surec.stdin.take().unwrap().write_all("işlev ana() {\n    yap ş = 5;\n    ş = 6;\n}\n".as_bytes()).unwrap();
    let cikti = surec.wait_with_output().unwrap();
    let satirlar = metin(&cikti.stdout);
    let hata: serde_json::Value = satirlar
        .lines()
        .map(|satir| serde_json::from_str::<serde_json::Value>(satir).unwrap())
        .find(|tani| tani["kod"] == "E0384")
        .unwrap_or_else(|| panic!("E0384 bulunamadı: {satirlar}"));
    assert_eq!(hata["seviye"], "hata");
    assert_eq!(hata["dosya"], "src/ana.pas");
    assert_eq!(hata["aralik"]["satir"], 3);
    assert_eq!(hata["aralik"]["sutun"], 5);
    assert!(hata["mesaj"].as_str().unwrap().contains("`ş`"), "{hata}");
    assert!(hata["aciklama"].is_string());
    assert!(hata["notlar"].as_array().unwrap().iter().any(|not| not.as_str().unwrap().contains("yap değişken ş")));
    // IDE'nin Alt+Enter ile uygulayacağı öneri: `ş`nin önüne `değişken ` ekle.
    let oneri = &hata["oneriler"][0];
    assert_eq!(oneri["kesin"], true);
    assert_eq!(oneri["degisiklikler"][0]["metin"], "değişken ");
    assert_eq!(oneri["degisiklikler"][0]["aralik"]["satir"], 2);
    assert_eq!(oneri["degisiklikler"][0]["aralik"]["sutun"], 9);
    assert_eq!(oneri["degisiklikler"][0]["aralik"]["bitis_sutunu"], 9);
}

/// `cargo pas çalıştır` komutunu verilen standart girdiyle çalıştırır.
fn girdiyle_calistir(dizin: &Path, girdi: &str) -> Output {
    let mut surec = Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
        .arg("çalıştır")
        .current_dir(dizin)
        .env("NO_COLOR", "1")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    std::io::Write::write_all(&mut surec.stdin.take().unwrap(), girdi.as_bytes()).unwrap();
    surec.wait_with_output().unwrap()
}

#[test]
fn oku_makrosu_girdi_okur() {
    let dizin = proje(
        "okuyan",
        &[
            (
                "src/ana.pas",
                "modül selam;\n\nişlev ana() {\n    yap ad = oku!(\"Adın ne? \");\n    \
                 yap yaş = oku!(\"Kaç yaşındasın? \", u32);\n    yap boy = oku!(\"Boyun? \", f64);\n    \
                 yazdır!(\"{} {yaş} yaşında, boyu {boy}.\", selam::selamla(&ad));\n}\n",
            ),
            ("src/selam.pas", "genel işlev selamla(ad: &str) -> Metin {\n    biçimle!(\"Merhaba {ad}!\")\n}\n"),
        ],
    );
    let cikti = girdiyle_calistir(&dizin, "  Ayşe \non iki\n12\n1,45\n");
    assert!(cikti.status.success(), "{}", metin(&cikti.stderr));
    assert_eq!(
        metin(&cikti.stdout),
        "Adın ne? Kaç yaşındasın? `on iki` geçerli bir tam sayı değil, tekrar dene.\n\
         Kaç yaşındasın? Boyun? Merhaba Ayşe! 12 yaşında, boyu 1.45.\n"
    );
}

#[test]
fn girdi_bitince_oku_turkce_panik_verir() {
    let dizin = proje(
        "girdisiz",
        &[("src/ana.pas", "işlev ana() {\n    yap sayı = oku!(\"Sayı: \", i32);\n    yazdır!(\"{sayı}\");\n}\n")],
    );
    let cikti = girdiyle_calistir(&dizin, "");
    let hata_ciktisi = metin(&cikti.stderr);
    assert_eq!(cikti.status.code(), Some(101));
    assert!(hata_ciktisi.contains("girdi bitti"), "{hata_ciktisi}");
    // Çökme, destek kodunu değil `oku!` satırını göstermeli.
    assert!(hata_ciktisi.contains("--> src/ana.pas:2:"), "{hata_ciktisi}");
}

#[test]
fn yeni_sozluk_sozcukleri_calisir() {
    let ana = r#"kullan std::collections::HashMap;

işlev ana() {
    yap değişken sayılar: Liste<i32> = Liste::yeni();
    her s içinde [5, 3, 8, 1] {
        sayılar.ekle(s);
    }
    sayılar.sırala();
    yazdır!("{:?} {:?}", sayılar, sayılar.al(10).yoksa(&0));
    sayılar.ters_çevir();
    yap toplam: i32 = sayılar.yinele().toplam();
    yazdır!("{toplam} {:?} {:?}", sayılar.yinele().en_büyük(), sayılar.yinele().en_küçük());
    yap çiftler: Liste<i32> = sayılar.yinele().süz(|s| *s % 2 == 0).dönüştür(|s| s * 10).biriktir();
    yazdır!("{:?} {}", çiftler, sayılar.yinele().bul(|s| **s > 4).var_mı());
    her (i, s) içinde sayılar.yinele().numaralandır() {
        eğer i == 0 {
            yazdır!("ilk: {s}");
        }
    }
    yap değişken yaşlar: HashMap<Metin, u32> = HashMap::yeni();
    yaşlar.yerleştir("Ali".metne_çevir(), 10);
    yazdır!("{} {:?}", yaşlar.anahtar_var_mı("Ali"), yaşlar.sil("Ali"));
    yap kelimeler: Liste<&str> = "elma armut muz".böl(" ").biriktir();
    yazdır!("{} {}", kelimeler.birleştir(", "), "elma".değiştir("e", "E"));
    yazdır!("{} {}", "Pas".ile_başlar_mı("P"), "a\nb".satırlar().count() + "çiğ".karakterler().count());
    yazdır!("{} {} {}", (-3i32).mutlak(), 2i32.kuvvet(3), 2.25f64.karekök().yuvarla());
}
"#;
    let dizin = proje("sozcukler", &[("src/ana.pas", ana)]);
    let cikti = cargo_pas(&dizin, &["çalıştır"]);
    assert!(cikti.status.success(), "{}", metin(&cikti.stderr));
    assert_eq!(
        metin(&cikti.stdout),
        "[1, 3, 5, 8] 0\n17 Var(8) Var(1)\n[80] doğru\nilk: 8\ndoğru Var(10)\nelma, armut, muz Elma\ndoğru 5\n3 8 2\n"
    );
}
