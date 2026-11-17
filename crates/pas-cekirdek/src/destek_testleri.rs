//! `destek.rs` içindeki, Pas programlarına eklenen kodun testleri.

use crate::pas_destek::{deger_oku_kaynaktan, satir_oku_kaynaktan};

fn oku<T: std::str::FromStr>(girdi: &str) -> (Option<T>, String) {
    let mut cikti = Vec::new();
    let deger = deger_oku_kaynaktan("Sayı: ", &mut girdi.as_bytes(), &mut cikti);
    (deger, String::from_utf8(cikti).unwrap())
}

#[test]
fn satir_okur_ve_bosluklari_atar() {
    let mut cikti = Vec::new();
    let satir = satir_oku_kaynaktan("Adın ne? ", &mut "  Ayşe  \r\nfazlası".as_bytes(), &mut cikti);
    assert_eq!(satir.as_deref(), Some("Ayşe"));
    assert_eq!(String::from_utf8(cikti).unwrap(), "Adın ne? ");
}

#[test]
fn girdi_bitince_none_doner() {
    assert_eq!(satir_oku_kaynaktan("", &mut "".as_bytes(), &mut Vec::new()), None);
    assert_eq!(oku::<u32>("abc\n").0, None);
}

#[test]
fn gecersiz_sayida_yeniden_sorar() {
    let (deger, cikti) = oku::<u32>("abc\n-3\n42\n");
    assert_eq!(deger, Some(42));
    assert_eq!(
        cikti,
        "Sayı: `abc` geçerli bir tam sayı değil, tekrar dene.\n\
         Sayı: `-3` geçerli bir tam sayı değil, tekrar dene.\n\
         Sayı: "
    );
}

#[test]
fn ondalik_virgul_kabul_edilir() {
    assert_eq!(oku::<f64>("3,5\n").0, Some(3.5));
    assert_eq!(oku::<f64>("2.25\n").0, Some(2.25));
}

#[test]
fn turkce_evet_hayir() {
    assert_eq!(oku::<bool>("Evet\n").0, Some(true));
    assert_eq!(oku::<bool>("yanlış\n").0, Some(false));
    assert_eq!(oku::<bool>("belki\nh\n").1, "Sayı: `belki` `doğru` ya da `yanlış` değil, tekrar dene.\nSayı: ");
}

#[test]
fn makro_derlenir() {
    // Makro gerçekten çağrılmaz (standart girdiyi bekler); yalnızca türlerin
    // doğru çıktığı denetlenir.
    if false {
        let _: String = pas_oku!();
        let _: String = pas_oku!("Adın ne? ");
        let soru = String::from("Yaşın? ");
        let _: u32 = pas_oku!(soru, u32);
    }
}
