//! cargo'yu çalıştırır; çıktısındaki tanılamaları ve durum satırlarını
//! Türkçeleştirir.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;

use serde::Deserialize;

use crate::gorunum::Gorunum;
use crate::tanilama::{Gosterici, Tani};

/// cargo'nun `--message-format=json` çıktısındaki bir satır.
#[derive(Deserialize)]
struct CargoMesaji {
    #[serde(rename = "reason")]
    neden: String,
    #[serde(rename = "message")]
    mesaj: Option<Tani>,
    #[serde(rename = "executable")]
    calistirilabilir: Option<PathBuf>,
    #[serde(rename = "target")]
    hedef: Option<Hedef>,
    #[serde(rename = "success")]
    basari: Option<bool>,
}

#[derive(Deserialize)]
struct Hedef {
    #[serde(rename = "name")]
    ad: String,
    #[serde(rename = "kind")]
    tur: Vec<String>,
}

pub struct Program {
    pub ad: String,
    pub yol: PathBuf,
}

pub struct Sonuc {
    pub basari: bool,
    pub programlar: Vec<Program>,
}

/// `cargo <alt komut> --mesaj-format=json …` çalıştırır.
pub fn calistir(argumanlar: &[&str], gosterici: &mut Gosterici, gorunum: Gorunum) -> Result<Sonuc, String> {
    let cargo_yolu = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut surec = Command::new(cargo_yolu)
        .args(argumanlar)
        .arg("--message-format=json")
        .env("CARGO_TERM_PROGRESS_WHEN", "never")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cargo çalıştırılamadı: {e}"))?;

    // Durum satırları geldikçe yazılır; özet satırları tanılamalardan sonra.
    let hata_ciktisi = surec.stderr.take().expect("hata çıktısı yönlendirildi");
    let durum_satirlari = thread::spawn(move || {
        let mut ozet = Vec::new();
        for satir in BufReader::new(hata_ciktisi).lines().map_while(Result::ok) {
            match durumu_cevir(&satir, gorunum) {
                Durum::Simdi(metin) => eprintln!("{metin}"),
                Durum::Sonra(metin) => ozet.push(metin),
                Durum::Gizle => {}
            }
        }
        ozet
    });

    let mut sonuc = Sonuc { basari: false, programlar: Vec::new() };
    let standart_cikti = surec.stdout.take().expect("standart çıktı yönlendirildi");
    for satir in BufReader::new(standart_cikti).lines().map_while(Result::ok) {
        let Ok(mesaj) = serde_json::from_str::<CargoMesaji>(&satir) else {
            println!("{satir}");
            continue;
        };
        match mesaj.neden.as_str() {
            "compiler-message" => {
                if let Some(tani) = &mesaj.mesaj {
                    gosterici.goster(tani);
                }
            }
            "compiler-artifact" => {
                if let (Some(yol), Some(hedef)) = (mesaj.calistirilabilir, mesaj.hedef)
                    && hedef.tur.iter().any(|k| k == "bin")
                {
                    sonuc.programlar.push(Program { ad: hedef.ad, yol });
                }
            }
            "build-finished" => sonuc.basari = mesaj.basari.unwrap_or(false),
            _ => {}
        }
    }

    let durum = surec.wait().map_err(|e| format!("cargo beklenirken hata: {e}"))?;
    for satir in durum_satirlari.join().unwrap_or_default() {
        eprintln!("{satir}");
    }
    sonuc.basari &= durum.success();
    Ok(sonuc)
}

enum Durum {
    Simdi(String),
    Sonra(String),
    Gizle,
}

/// cargo'nun durum satırlarını Türkçeleştirir.
fn durumu_cevir(satir: &str, gorunum: Gorunum) -> Durum {
    let kirpilmis = satir.trim_start();
    let durum =
        |kelime: &str, kalan: &str| Durum::Simdi(format!("{} {kalan}", gorunum.basari(&format!("{kelime:>15}"))));
    let kelimeler: &[(&str, &str)] = &[
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
    for (ingilizce, turkce) in kelimeler {
        if let Some(kalan) = kirpilmis.strip_prefix(ingilizce) {
            return durum(turkce, yolu_at(kalan));
        }
    }
    if let Some(kalan) = kirpilmis.strip_prefix("Finished ") {
        let profil = if kalan.starts_with("`release`") { "hızlı (yayın) derleme, " } else { "" };
        let sure = kalan.rsplit(" in ").next().unwrap_or("").trim_end_matches('s');
        return durum("Tamamlandı", &format!("{profil}{sure} saniyede"));
    }
    if kirpilmis.starts_with("Blocking waiting for file lock") {
        return durum("Bekleniyor", "başka bir derlemenin bitmesi bekleniyor");
    }
    if kirpilmis.starts_with("Running ") {
        return Durum::Gizle;
    }
    if let Some(kalan) = satir.strip_prefix("error: could not compile `") {
        let ad = kalan.split('`').next().unwrap_or("");
        let hatalar = oncesindeki_sayi(kalan, "previous error");
        let uyarilar = oncesindeki_sayi(kalan, "warning");
        let mut metin = format!("{}: `{ad}` derlenemedi", gorunum.hata("hata"));
        if let Some(n) = hatalar {
            let _ = std::fmt::Write::write_fmt(&mut metin, format_args!(" ({n} hata"));
            match uyarilar {
                Some(w) => metin.push_str(&format!(", {w} uyarı)")),
                None => metin.push(')'),
            }
        }
        return Durum::Sonra(metin);
    }
    if let Some(kalan) = satir.strip_prefix("warning: `")
        && kalan.contains(" generated ")
    {
        let ad = kalan.split('`').next().unwrap_or("");
        let n = oncesindeki_sayi(kalan, "warning").unwrap_or(1);
        return Durum::Sonra(format!("{}: `{ad}` için {n} uyarı", gorunum.uyari("uyarı")));
    }
    if satir.starts_with("note: ") && satir.contains("cargo fix") {
        return Durum::Gizle;
    }
    Durum::Simdi(satir.to_owned())
}

/// `proje v0.1.0 (/uzun/yol)` → `proje v0.1.0`
fn yolu_at(kalan: &str) -> &str {
    match kalan.rfind(" (") {
        Some(i) if kalan[i..].contains('/') || kalan[i..].contains('\\') => &kalan[..i],
        _ => kalan,
    }
}

/// "… due to 2 previous errors" içinden `kelime`den önceki sayı.
fn oncesindeki_sayi(metin: &str, kelime: &str) -> Option<usize> {
    let oncesi = &metin[..metin.find(kelime)?];
    oncesi.split_whitespace().last()?.parse().ok()
}

#[cfg(test)]
mod testler {
    use super::*;

    fn metin(satir: &str) -> String {
        match durumu_cevir(satir, Gorunum::duz()) {
            Durum::Simdi(t) | Durum::Sonra(t) => t.trim().to_owned(),
            Durum::Gizle => String::new(),
        }
    }

    #[test]
    fn durum_satirlari() {
        assert_eq!(metin("   Compiling oyun v0.1.0 (/home/x/oyun/target/pas)"), "Derleniyor oyun v0.1.0");
        assert_eq!(
            metin("    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s"),
            "Tamamlandı 0.52 saniyede"
        );
        assert_eq!(
            metin("error: could not compile `oyun` (bin \"oyun\") due to 2 previous errors; 1 warning emitted"),
            "hata: `oyun` derlenemedi (2 hata, 1 uyarı)"
        );
        assert_eq!(
            metin(
                "warning: `oyun` (bin \"oyun\") generated 3 warnings (run `cargo fix --bin \"oyun\"` to apply 1 suggestion)"
            ),
            "uyarı: `oyun` için 3 uyarı"
        );
    }
}
