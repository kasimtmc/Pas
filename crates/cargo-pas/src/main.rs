//! `cargo pas`: Pas (Türkçe Rust) projelerini derler ve çalıştırır.

mod cargo;
mod gorunum;
mod program;
mod proje;
mod tanilama;

use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::ExitCode;

use pas_cekirdek::sozluk::{self, GIRDILER};

use crate::gorunum::Gorunum;
use crate::proje::{GOLGE_ONEKI, Proje};
use crate::tanilama::{Gosterici, Kaynaklar};

const YARDIM: &str = "\
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
    --json           Hataları JSON satırları olarak yazar (IDE'ler için)
    --stdin <dosya>  Bu dosyanın içeriğini diskten değil standart girdiden okur

Komutların İngilizce adları da çalışır: new, run, build, check,
translate, dictionary, help, version.";

fn main() -> ExitCode {
    let mut argumanlar: Vec<String> = std::env::args().skip(1).collect();
    // `cargo pas …` olarak çağrılınca ilk argüman `pas` olur.
    if argumanlar.first().is_some_and(|a| a == "pas") {
        argumanlar.remove(0);
    }
    let gorunum = Gorunum::algila();
    match komutu_isle(&argumanlar, gorunum) {
        Ok(kod) => kod,
        Err(mesaj) => {
            eprintln!("{}: {mesaj}", gorunum.hata("hata"));
            ExitCode::FAILURE
        }
    }
}

fn komutu_isle(argumanlar: &[String], gorunum: Gorunum) -> Result<ExitCode, String> {
    let Some((komut, kalan)) = argumanlar.split_first() else {
        println!("{YARDIM}");
        return Ok(ExitCode::SUCCESS);
    };
    match komut.as_str() {
        "yeni" | "new" => yeni_proje(kalan, gorunum),
        "çalıştır" | "calistir" | "run" => derle(kalan, Kip::Calistir, gorunum),
        "derle" | "build" => derle(kalan, Kip::Derle, gorunum),
        "denetle" | "check" => derle(kalan, Kip::Denetle, gorunum),
        "çevir" | "cevir" | "translate" => cevir(kalan),
        "sözlük" | "sozluk" | "dictionary" => sozlugu_yazdir(kalan),
        "yardım" | "yardim" | "help" | "-h" | "--help" => {
            println!("{YARDIM}");
            Ok(ExitCode::SUCCESS)
        }
        "sürüm" | "surum" | "version" | "-V" | "--version" => {
            println!("cargo-pas {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        diger => Err(format!("bilinmeyen komut `{diger}`. Komutları görmek için: cargo pas yardım")),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kip {
    Calistir,
    Derle,
    Denetle,
}

#[derive(Default)]
struct DerlemeSecenekleri {
    yayin: bool,
    ikili: Option<String>,
    program_argumanlari: Vec<String>,
    json: bool,
    stdin_dosyasi: Option<String>,
}

impl DerlemeSecenekleri {
    fn ayristir(argumanlar: &[String]) -> Result<Self, String> {
        let mut secenekler = DerlemeSecenekleri::default();
        let mut argumanlar = argumanlar.iter();
        while let Some(arguman) = argumanlar.next() {
            match arguman.as_str() {
                "--yayın" | "--yayin" | "--release" => secenekler.yayin = true,
                "--ikili" | "--bin" => {
                    secenekler.ikili =
                        Some(argumanlar.next().ok_or("`--ikili` seçeneğinden sonra bir ad yaz")?.clone());
                }
                "--json" => secenekler.json = true,
                "--stdin" => {
                    secenekler.stdin_dosyasi =
                        Some(argumanlar.next().ok_or("`--stdin` seçeneğinden sonra bir dosya yolu yaz")?.clone());
                }
                "--" => {
                    secenekler.program_argumanlari = argumanlar.cloned().collect();
                    break;
                }
                diger => return Err(format!("bilinmeyen seçenek `{diger}`")),
            }
        }
        Ok(secenekler)
    }
}

fn derle(argumanlar: &[String], kip: Kip, gorunum: Gorunum) -> Result<ExitCode, String> {
    let secenekler = DerlemeSecenekleri::ayristir(argumanlar)?;
    if kip != Kip::Calistir && !secenekler.program_argumanlari.is_empty() {
        return Err("programa argüman yalnızca `çalıştır` ile verilebilir".into());
    }
    if kip == Kip::Calistir && secenekler.json {
        return Err("`--json` yalnızca `derle` ve `denetle` ile kullanılabilir".into());
    }
    let calisma_dizini = std::env::current_dir().map_err(|e| format!("bulunulan dizin okunamadı: {e}"))?;
    let mut proje = Proje::bul(&calisma_dizini)?;
    if let Some(yol) = &secenekler.stdin_dosyasi {
        let mut icerik = String::new();
        std::io::stdin().read_to_string(&mut icerik).map_err(|e| format!("girdi okunamadı: {e}"))?;
        proje.kaydedilmemis = Some((yol.replace('\\', "/"), icerik));
    }
    let hazirlik = proje.hazirla()?;
    if hazirlik.dosyalar.is_empty() {
        eprintln!(
            "{}: projede hiç `.pas` dosyası yok; `src/ana.pas` dosyasını oluşturarak başlayabilirsin",
            gorunum.uyari("uyarı")
        );
    } else {
        gorunum.durum("Çevrildi", &format!("{} Pas dosyası", hazirlik.dosyalar.len()));
    }

    let bildirim = proje.golge_bildirimi();
    let mut cargo_argumanlari = vec![
        if kip == Kip::Denetle { "check" } else { "build" },
        "--manifest-path",
        bildirim.to_str().ok_or("proje yolu geçersiz karakterler içeriyor")?,
        "--target-dir",
        proje.hedef_dizini.to_str().ok_or("proje yolu geçersiz karakterler içeriyor")?,
    ];
    if secenekler.yayin {
        cargo_argumanlari.push("--release");
    }
    if let Some(ikili) = &secenekler.ikili {
        cargo_argumanlari.extend(["--bin", ikili]);
    }

    let mut gosterici = Gosterici::yeni(gorunum, Kaynaklar::yeni(&proje.golge, &hazirlik), secenekler.json);
    let sonuc = cargo::calistir(&cargo_argumanlari, &mut gosterici, gorunum)?;
    if !sonuc.basari {
        return Ok(ExitCode::from(101));
    }
    if kip != Kip::Calistir {
        return Ok(ExitCode::SUCCESS);
    }

    let program_dosyasi = program_sec(&sonuc.programlar, &secenekler, &proje)?;
    let gosterilen_yol = program_dosyasi.yol.strip_prefix(&proje.kok).unwrap_or(&program_dosyasi.yol);
    gorunum.durum("Çalıştırılıyor", &format!("`{}`", gosterilen_yol.display()));
    program::calistir(&program_dosyasi.yol, &secenekler.program_argumanlari, &mut gosterici.kaynaklar, gorunum)
}

fn program_sec<'a>(
    programlar: &'a [cargo::Program],
    secenekler: &DerlemeSecenekleri,
    proje: &Proje,
) -> Result<&'a cargo::Program, String> {
    if let Some(ikili) = &secenekler.ikili {
        return programlar.iter().find(|e| &e.ad == ikili).ok_or_else(|| format!("`{ikili}` adında bir program yok"));
    }
    match programlar {
        [] => Err("çalıştırılacak bir program bulunamadı; `src/ana.pas` dosyası var mı?".into()),
        [tek] => Ok(tek),
        cok => cok.iter().find(|e| Some(&e.ad) == proje.paket_adi.as_ref()).ok_or_else(|| {
            let adlar: Vec<&str> = cok.iter().map(|e| e.ad.as_str()).collect();
            format!("birden fazla program var: {}. Hangisini çalıştıracağını `--ikili <ad>` ile seç", adlar.join(", "))
        }),
    }
}

fn yeni_proje(argumanlar: &[String], gorunum: Gorunum) -> Result<ExitCode, String> {
    let [ad] = argumanlar else {
        return Err("proje adını yaz: cargo pas yeni <ad>".into());
    };
    let dizin = Path::new(ad);
    if dizin.exists() {
        return Err(format!("`{ad}` zaten var"));
    }
    let dizin_adi = dizin.file_name().and_then(|n| n.to_str()).unwrap_or(ad);
    let paket = paket_adi(dizin_adi);
    if paket.is_empty() {
        return Err(format!("`{ad}` proje adı olarak kullanılamaz; harflerle başlayan bir ad seç"));
    }

    // `[[bin]]` bölümü programın Rust'a çevrilmiş halini gösterir; IDE'ler
    // (`cargo metadata`) `.rs` dosyası olmayan projeyi bu sayede tanır.
    let bildirim = format!(
        "[package]\nname = \"{paket}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
         # `cargo pas` bu dosyayı `src/ana.pas` dosyasından üretir.\n\
         [[bin]]\nname = \"{paket}\"\npath = \"{GOLGE_ONEKI}src/main.rs\"\n\n[dependencies]\n"
    );
    let ana_dosya = "işlev ana() {\n    yazdır!(\"Merhaba dünya!\");\n}\n";
    let yaz =
        |yol: &Path, icerik: &str| fs::write(yol, icerik).map_err(|e| format!("`{}` yazılamadı: {e}", yol.display()));
    fs::create_dir_all(dizin.join("src")).map_err(|e| format!("`{ad}` oluşturulamadı: {e}"))?;
    yaz(&dizin.join("Cargo.toml"), &bildirim)?;
    yaz(&dizin.join("src/ana.pas"), ana_dosya)?;
    yaz(&dizin.join(".gitignore"), "/target\n")?;

    gorunum.durum("Oluşturuldu", &format!("`{ad}` projesi"));
    eprintln!("\nBaşlamak için:\n    cd {ad}\n    cargo pas çalıştır");
    Ok(ExitCode::SUCCESS)
}

/// cargo paket adları ASCII olmalı: `Uzay Oyunu` → `uzay-oyunu`.
fn paket_adi(ad: &str) -> String {
    let mut cikti = String::new();
    for c in ad.chars() {
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
        if !(c == '-' && (cikti.is_empty() || cikti.ends_with('-'))) {
            cikti.push(c);
        }
    }
    let kirpilmis = cikti.trim_end_matches('-').trim_start_matches(|c: char| c.is_ascii_digit() || c == '-');
    kirpilmis.to_owned()
}

fn cevir(argumanlar: &[String]) -> Result<ExitCode, String> {
    let mut ters = false;
    let mut dosya = None;
    for arguman in argumanlar {
        match arguman.as_str() {
            "--ters" | "--reverse" => ters = true,
            diger if dosya.is_none() => dosya = Some(diger),
            diger => return Err(format!("fazladan argüman `{diger}`")),
        }
    }
    let dosya = dosya.ok_or("çevrilecek dosyayı yaz: cargo pas çevir <dosya.pas> (standart girdi için `-`)")?;
    let kaynak = if dosya == "-" {
        let mut metin = String::new();
        std::io::stdin().read_to_string(&mut metin).map_err(|e| format!("girdi okunamadı: {e}"))?;
        metin
    } else {
        fs::read_to_string(dosya).map_err(|e| format!("`{dosya}` okunamadı: {e}"))?
    };
    let cikti = if ters { pas_cekirdek::pasa_cevir(&kaynak) } else { pas_cekirdek::rusta_cevir(&kaynak).rust };
    print!("{cikti}");
    Ok(ExitCode::SUCCESS)
}

fn sozlugu_yazdir(argumanlar: &[String]) -> Result<ExitCode, String> {
    match argumanlar {
        [] => {
            let mut tur = None;
            for girdi in GIRDILER {
                if tur != Some(girdi.tur) {
                    tur = Some(girdi.tur);
                    println!("\n{}:", ilk_harfi_buyut(&cogul(girdi.tur)));
                }
                let pas = match girdi.tur {
                    sozluk::Sinif::Makro => format!("{}!", girdi.pas),
                    _ => girdi.pas.to_owned(),
                };
                let rust = match girdi.tur {
                    sozluk::Sinif::Makro => format!("{}!", girdi.rust),
                    _ => girdi.rust.to_owned(),
                };
                println!("    {pas:<18} {rust:<14} {}", girdi.aciklama);
            }
            Ok(ExitCode::SUCCESS)
        }
        [bayrak] if bayrak == "--json" => {
            let girdiler: Vec<serde_json::Value> = GIRDILER
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "pas": e.pas,
                        "rust": e.rust,
                        "tur": e.tur.kimlik(),
                        "aciklama": e.aciklama,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&girdiler).expect("JSON"));
            Ok(ExitCode::SUCCESS)
        }
        _ => Err("kullanım: cargo pas sözlük [--json]".into()),
    }
}

fn cogul(tur: sozluk::Sinif) -> String {
    match tur {
        sozluk::Sinif::AnahtarSozcuk => "anahtar sözcükler".into(),
        sozluk::Sinif::Makro => "makrolar".into(),
        sozluk::Sinif::Tur => "türler".into(),
        sozluk::Sinif::Deger => "değerler".into(),
        sozluk::Sinif::Islev => "işlevler".into(),
        sozluk::Sinif::Yontem => "yöntemler".into(),
    }
}

/// Türkçe kurallarıyla ilk harfi büyütür: `i` → `İ`, `ı` → `I`.
fn ilk_harfi_buyut(metin: &str) -> String {
    let mut karakterler = metin.chars();
    match karakterler.next() {
        Some('i') => std::iter::once('İ').chain(karakterler).collect(),
        Some('ı') => std::iter::once('I').chain(karakterler).collect(),
        Some(ilk) => ilk.to_uppercase().chain(karakterler).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn turkce_buyuk_harf() {
        assert_eq!(ilk_harfi_buyut("işlevler"), "İşlevler");
        assert_eq!(ilk_harfi_buyut("ılık"), "Ilık");
        assert_eq!(ilk_harfi_buyut("çeşit"), "Çeşit");
    }

    #[test]
    fn paket_adlari() {
        assert_eq!(paket_adi("Uzay Oyunu"), "uzay-oyunu");
        assert_eq!(paket_adi("çılgın_şövalye"), "cilgin_sovalye");
        assert_eq!(paket_adi("İlk Program!"), "ilk-program");
        assert_eq!(paket_adi("3 kedi"), "kedi");
    }
}
