//! `destek.rs` içindeki, Pas programlarına eklenen kodun testleri.

use crate::pas_destek::{RastgeleSayi, deger_oku_kaynaktan, rastgele, satir_oku_kaynaktan};

fn oku<T: std::str::FromStr>(girdi: &str) -> (Option<T>, String) {
    let mut cikti = Vec::new();
    let deger = deger_oku_kaynaktan("Sayı: ", &mut girdi.as_bytes(), &mut cikti, false);
    (deger, String::from_utf8(cikti).unwrap())
}

#[test]
fn satir_okur_ve_bosluklari_atar() {
    let mut cikti = Vec::new();
    let satir = satir_oku_kaynaktan("Adın ne? ", &mut "  Ayşe  \r\nfazlası".as_bytes(), &mut cikti, false);
    assert_eq!(satir.as_deref(), Some("Ayşe"));
    assert_eq!(String::from_utf8(cikti).unwrap(), "Adın ne? ");
}

#[test]
fn hazir_girdide_okunan_satir_yazilir() {
    let mut cikti = Vec::new();
    let mut girdi = "abc\n42".as_bytes();
    let deger: Option<u32> = deger_oku_kaynaktan("Sayı: ", &mut girdi, &mut cikti, true);
    assert_eq!(deger, Some(42));
    assert_eq!(
        String::from_utf8(cikti).unwrap(),
        "Sayı: abc\n`abc` geçerli bir tam sayı değil, tekrar dene.\nSayı: 42\n"
    );
}

#[test]
fn girdi_bitince_none_doner() {
    assert_eq!(satir_oku_kaynaktan("", &mut "".as_bytes(), &mut Vec::new(), false), None);
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

#[test]
fn hata_ayiklama_ciktisi_turkcelesir() {
    use crate::pas_destek::hata_ayiklamayi_turkcelestir;
    assert_eq!(
        hata_ayiklamayi_turkcelestir(r#"[Some("Some true"), None, Ok('\''), Err(false), Nonesuch]"#),
        r#"[Var("Some true"), Yok, Tamam('\''), Hata(yanlış), Nonesuch]"#
    );
    assert_eq!(
        hata_ayiklamayi_turkcelestir(r#"Kedi { ad: "None \"Ok\"", tok: true }"#),
        r#"Kedi { ad: "None \"Ok\"", tok: doğru }"#
    );
}

#[test]
// `(&Sar(&x))` içindeki `&` gereksiz değil: hangi niteliğin seçileceğini o belirler.
#[allow(clippy::needless_borrow)]
fn sarmalayici_turkce_yazar() {
    #[allow(unused_imports)]
    use crate::pas_destek::{DuzSecim as _, Sar, TurkceSecimi as _};
    let secenek: Option<Vec<bool>> = Some(vec![true]);
    assert_eq!(
        format!(
            "{} {:?} {} {:?}",
            (&Sar(&true)).pas_turkce(),
            (&Sar(&secenek)).pas_turkce(),
            (&Sar(&5)).pas_turkce(),
            (&Sar(&"metin")).pas_turkce()
        ),
        "doğru Var([doğru]) 5 \"metin\""
    );
    // Biçim seçenekleri korunur.
    assert_eq!(
        format!("{:>8}|{:x}|{:.2}", (&Sar(&false)).pas_turkce(), (&Sar(&255)).pas_turkce(), (&Sar(&1.5)).pas_turkce()),
        "  yanlış|ff|1.50"
    );

    // `Debug` uygulamayan türler olduğu gibi geçer.
    struct Kedi;
    impl std::fmt::Display for Kedi {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Pamuk")
        }
    }
    let duz: &Kedi = (&Sar(&Kedi)).pas_turkce();
    assert_eq!(format!("{duz}"), "Pamuk");
}

#[test]
fn rastgele_sayilar_aralikta() {
    for _ in 0..1000 {
        let zar = rastgele(1, 6);
        assert!((1..=6).contains(&zar));
        assert!((-3..=3).contains(&rastgele(3, -3)));
        let oran = rastgele(0.0, 1.0);
        assert!((0.0..1.0).contains(&oran));
    }
    assert_eq!(u8::aralikta(0, 255, 256 + 7), 7);
    assert_eq!(u64::aralikta(0, u64::MAX, u64::MAX), u64::MAX);
    assert_eq!(i8::aralikta(-128, 127, 0), -128);
}
