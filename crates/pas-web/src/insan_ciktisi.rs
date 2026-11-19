//! rustc'nin insanlar için yazdığı tanılama metnini ayrıştırır.
//!
//! Rust oyun alanı (play.rust-lang.org) tanılamaları JSON olarak vermez;
//! terminalde görünen metni verir:
//!
//! ```text
//! error[E0308]: mismatched types
//!  --> src/main.rs:2:18
//!   |
//! 2 |     let x: i32 = "a";
//!   |            ---   ^^^ expected `i32`, found `&str`
//!   |            |
//!   |            expected due to this
//! ```
//!
//! Konumlar, gösterilen satırın derlenen Rust kaynağındaki karşılığı bulunarak
//! hesaplanır. rustc uzun satırları `...` ile kırptığı ve sekmeleri dört
//! boşluk gösterdiği için ekrandaki sütun ile kaynaktaki sütun farklı olabilir.

use pas_cekirdek::SatirDizini;

/// Ana kaynak dosyasının oyun alanındaki adı.
pub const ANA_DOSYA: &str = "src/main.rs";

/// Bir tanılama. Konumlar derlenen Rust kaynağındadır; satırlar ve
/// (karakter olarak) sütunlar 1'den başlar.
#[derive(Debug, Default, PartialEq)]
pub struct Tani {
    /// `error` ya da `warning`
    pub seviye: String,
    pub kod: Option<String>,
    pub mesaj: String,
    /// `-->` satırındaki konum: (satır, sütun)
    pub konum: Option<(usize, usize)>,
    pub etiketler: Vec<Etiket>,
    /// `= note: …` satırları ve `help: …` gibi alt tanılamalar
    pub notlar: Vec<Not>,
}

/// Kaynakta işaretlenen bir aralık; `bitis` dahil değildir.
#[derive(Debug, PartialEq)]
pub struct Etiket {
    pub satir: usize,
    pub baslangic: usize,
    pub bitis: usize,
    pub birincil: bool,
    pub mesaj: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Not {
    /// `note` ya da `help`
    pub seviye: String,
    pub mesaj: String,
    /// Önerilen değişiklik: satır numarası ve satırın önerilen yeni hali
    /// (sekmeler dört boşluk olarak).
    pub oneri: Option<(usize, String)>,
    /// Öneri var olan bir satırı değiştirmiyor, o satırın önüne yeni bir satır ekliyor.
    pub yeni_satir: bool,
}

/// Derleyicinin çıktısındaki tanılamaları ayrıştırır. `rust`, derlenen
/// `src/main.rs` dosyasının içeriğidir.
pub fn ayristir(metin: &str, rust: &str) -> Vec<Tani> {
    let mut ayristirici = Ayristirici {
        rust: SatirDizini::yeni(rust),
        tanilar: Vec::new(),
        tani: None,
        alt: None,
        ana_dosyada: false,
        kod_satiri: None,
        yeni_satirlar: Vec::new(),
        eski_satirlar: Vec::new(),
        oneri_var: false,
        not_devam_ediyor: false,
    };
    for satir in metin.lines() {
        ayristirici.satir(satir.trim_end_matches('\r'));
    }
    ayristirici.tani_bitir();
    ayristirici.tanilar
}

struct Ayristirici<'r> {
    rust: SatirDizini<'r>,
    tanilar: Vec<Tani>,
    /// Şu an okunan tanılama
    tani: Option<Tani>,
    /// Bir alt tanılamanın (`help: …`) içindeysek onun `notlar` dizini
    alt: Option<usize>,
    /// Gösterilen kesit ana dosyadan mı?
    ana_dosyada: bool,
    kod_satiri: Option<KodSatiri>,
    /// Alt tanılamadaki kod satırları; öneri varsa önerilen yeni satırlardır.
    /// Üçüncü alan: satır `N + …` biçiminde mi gösterildi?
    yeni_satirlar: Vec<(usize, String, bool)>,
    /// Alt tanılamada `N - …` biçiminde gösterilen (silinen ya da değişen) satırlar
    eski_satirlar: Vec<usize>,
    oneri_var: bool,
    /// Son satır bir nottu; girintili devam satırları ona eklenir.
    not_devam_ediyor: bool,
}

impl Ayristirici<'_> {
    fn satir(&mut self, satir: &str) {
        if satir.is_empty() {
            self.tani_bitir();
            return;
        }
        if let Some(tani) = baslik(satir) {
            self.tani_bitir();
            self.tani = Some(tani);
            return;
        }
        if self.tani.is_none() {
            return;
        }
        if let Some((seviye, mesaj)) = alt_baslik(satir) {
            self.alt_bitir();
            let tani = self.tani.as_mut().expect("tanılama var");
            tani.notlar.push(Not {
                seviye: seviye.to_owned(),
                mesaj: mesaj.to_owned(),
                oneri: None,
                yeni_satir: false,
            });
            self.alt = Some(tani.notlar.len() - 1);
            // Kendi `-->` satırı yoksa alt tanılama ana dosyayı gösterir.
            self.ana_dosyada = true;
            self.not_devam_ediyor = true;
            return;
        }
        if let Some((isaret, yol)) = ok_satiri(satir) {
            self.ok_isle(isaret, yol);
            self.not_devam_ediyor = false;
            return;
        }
        match oluk(satir) {
            Some((Some(no), '|', icerik)) => {
                self.kod_satiri = self.ana_dosyada.then(|| KodSatiri::yeni(no, icerik, self.rust.satir(no)));
                if self.alt.is_some() && self.ana_dosyada {
                    self.yeni_satirlar.push((no, icerik.to_owned(), false));
                }
                self.not_devam_ediyor = false;
            }
            Some((None, '|', icerik)) => {
                if self.alt.is_some() {
                    let isaretler = icerik.trim();
                    if !isaretler.is_empty() && isaretler.chars().all(|c| matches!(c, '+' | '~' | '-' | ' ')) {
                        self.oneri_var = true;
                    }
                } else {
                    self.etiketleri_oku(icerik);
                }
                self.not_devam_ediyor = false;
            }
            Some((None, '=', icerik)) => {
                let (seviye, mesaj) = icerik.split_once(": ").unwrap_or(("note", icerik));
                let tani = self.tani.as_mut().expect("tanılama var");
                tani.notlar.push(Not {
                    seviye: seviye.to_owned(),
                    mesaj: mesaj.to_owned(),
                    oneri: None,
                    yeni_satir: false,
                });
                self.not_devam_ediyor = true;
            }
            Some((Some(no), isaret @ ('+' | '~'), icerik)) => {
                // Boş eklenen satırlar (bir `kullan` satırından sonraki boşluk) önemsizdir.
                if self.alt.is_some() && self.ana_dosyada && !(isaret == '+' && icerik.is_empty()) {
                    self.yeni_satirlar.push((no, icerik.to_owned(), isaret == '+'));
                    if isaret == '~' {
                        self.eski_satirlar.push(no);
                    }
                    self.oneri_var = true;
                }
                self.not_devam_ediyor = false;
            }
            Some((Some(no), '-', _)) => {
                self.eski_satirlar.push(no);
                self.not_devam_ediyor = false;
            }
            Some(_) => self.not_devam_ediyor = false,
            None => {
                // Uzun bir notun girintili devam satırı
                if self.not_devam_ediyor
                    && satir.starts_with(' ')
                    && let Some(not) = self.tani.as_mut().and_then(|t| t.notlar.last_mut())
                {
                    not.mesaj.push('\n');
                    not.mesaj.push_str(satir.trim());
                }
            }
        }
    }

    fn ok_isle(&mut self, isaret: &str, yol: &str) {
        let mut parcalar = yol.rsplitn(3, ':');
        let sutun = parcalar.next().and_then(|s| s.parse::<usize>().ok());
        let satir = parcalar.next().and_then(|s| s.parse::<usize>().ok());
        let dosya = parcalar.next();
        self.ana_dosyada = dosya == Some(ANA_DOSYA);
        self.kod_satiri = None;
        if let (Some(satir), Some(sutun)) = (satir, sutun)
            && isaret == "-->"
            && self.ana_dosyada
            && self.alt.is_none()
            && let Some(tani) = self.tani.as_mut()
            && tani.konum.is_none()
        {
            tani.konum = Some((satir, sutun));
        }
    }

    /// Bir kod satırının altındaki işaretleri (`^^^`, `---`) ve etiketleri okur.
    fn etiketleri_oku(&mut self, icerik: &str) {
        let (Some(kod_satiri), Some(tani)) = (self.kod_satiri.as_mut(), self.tani.as_mut()) else { return };
        let karakterler: Vec<char> = icerik.chars().collect();
        let mut bu_satirdakiler = Vec::new();
        let mut i = 0;
        while i < karakterler.len() {
            let c = karakterler[i];
            match c {
                // Boşluklar ve çok satırlı aralıkların çizgileri
                ' ' | '_' | '|' | '/' | '\\' => i += 1,
                '^' | '-' => {
                    let mut j = i;
                    while karakterler.get(j) == Some(&c) {
                        j += 1;
                    }
                    tani.etiketler.push(Etiket {
                        satir: kod_satiri.no,
                        baslangic: kod_satiri.sutun(i),
                        bitis: kod_satiri.sutun(j - 1) + 1,
                        birincil: c == '^',
                        mesaj: None,
                    });
                    let dizin = tani.etiketler.len() - 1;
                    kod_satiri.isaretler.push((i, dizin));
                    bu_satirdakiler.push(dizin);
                    i = j;
                }
                _ => {
                    // Etiket, sağında başka bir işaret yoksa satırın sonuna kadar sürer.
                    let son = etiket_sonu(&karakterler, i);
                    let mesaj: String = karakterler[i..son].iter().collect::<String>().trim_end().to_owned();
                    // Etiket ya işaretin hemen sağında ya da altında, işaretle aynı sütundadır.
                    let hedef = bu_satirdakiler.last().copied().or_else(|| {
                        kod_satiri
                            .isaretler
                            .iter()
                            .find(|(sutun, dizin)| *sutun == i && tani.etiketler[*dizin].mesaj.is_none())
                            .map(|(_, dizin)| *dizin)
                    });
                    // Kısa öneriler etiket olarak yazılır: `help: remove this `mut``
                    if let Some(oneri) = mesaj.strip_prefix("help: ") {
                        tani.notlar.push(Not {
                            seviye: "help".into(),
                            mesaj: oneri.to_owned(),
                            oneri: None,
                            yeni_satir: false,
                        });
                    } else if let Some(dizin) = hedef {
                        tani.etiketler[dizin].mesaj = Some(mesaj);
                    }
                    bu_satirdakiler.clear();
                    i = son;
                }
            }
        }
    }

    fn alt_bitir(&mut self) {
        if let (Some(dizin), Some(tani)) = (self.alt.take(), self.tani.as_mut())
            && self.oneri_var
            && let [(no, satir, arti)] = self.yeni_satirlar.as_slice()
            && !satir.starts_with("...")
        {
            let not = &mut tani.notlar[dizin];
            not.oneri = Some((*no, satir.clone()));
            // `N - …` karşılığı olmadan `N + …` ile gösterilen satır yeni eklenen bir satırdır.
            not.yeni_satir = *arti && !self.eski_satirlar.contains(no);
        }
        self.yeni_satirlar.clear();
        self.eski_satirlar.clear();
        self.oneri_var = false;
        self.kod_satiri = None;
        self.ana_dosyada = false;
    }

    fn tani_bitir(&mut self) {
        self.alt_bitir();
        self.not_devam_ediyor = false;
        if let Some(mut tani) = self.tani.take() {
            // `-->` satırındaki konum birincil işaretin başıdır; işaret
            // okunamadıysa (çok satırlı aralıklar) tek karakterlik bir işaret konur.
            if let Some((satir, sutun)) = tani.konum {
                let birincil =
                    tani.etiketler.iter_mut().filter(|e| e.birincil && e.satir == satir).min_by_key(|e| e.baslangic);
                match birincil {
                    // İkincil bir işaret birincilin başını örtmüş olabilir: `----^`
                    Some(etiket) if etiket.baslangic > sutun => etiket.baslangic = sutun,
                    Some(_) => {}
                    None => tani.etiketler.push(Etiket {
                        satir,
                        baslangic: sutun,
                        bitis: sutun + 1,
                        birincil: true,
                        mesaj: None,
                    }),
                }
            }
            self.tanilar.push(tani);
        }
    }
}

/// `i`de başlayan etiketin bittiği yer: iki ya da daha çok boşluktan sonra
/// bir işaret (`^^^`, `---`) geliyorsa orası, yoksa satırın sonu.
fn etiket_sonu(karakterler: &[char], i: usize) -> usize {
    let mut j = i;
    while j < karakterler.len() {
        if karakterler[j] == ' ' && karakterler.get(j + 1) == Some(&' ') {
            let mut k = j;
            while karakterler.get(k) == Some(&' ') {
                k += 1;
            }
            if let Some(&c) = karakterler.get(k)
                && matches!(c, '^' | '-')
            {
                let mut m = k;
                while karakterler.get(m) == Some(&c) {
                    m += 1;
                }
                if karakterler.get(m).is_none_or(|&d| d == ' ') {
                    return j;
                }
            }
            j = k;
        } else {
            j += 1;
        }
    }
    karakterler.len()
}

/// Gösterilen bir kod satırı ve ekrandaki sütunların kaynaktaki karşılıkları.
struct KodSatiri {
    no: usize,
    /// Satırın sekmeleri açılmış hali; her ekran sütunu için kaynaktaki
    /// karakter dizini (0'dan başlar).
    acik: Vec<usize>,
    /// Gösterilen metnin ilk karakterinin `acik` içindeki yeri, eksi soldaki
    /// kırpma (`...`) ve çok satırlı aralık çizgileri için bırakılan boşluk.
    kayma: isize,
    /// Bu satırın altındaki işaretler: (ekran sütunu, etiket dizini)
    isaretler: Vec<(usize, usize)>,
}

impl KodSatiri {
    fn yeni(no: usize, gosterilen: &str, kaynak: Option<&str>) -> Self {
        let kaynak = kaynak.unwrap_or("");
        let mut acik = Vec::new();
        let mut acik_metin = String::new();
        for (i, c) in kaynak.chars().enumerate() {
            let tekrar = if c == '\t' { 4 } else { 1 };
            for _ in 0..tekrar {
                acik.push(i);
                acik_metin.push(if c == '\t' { ' ' } else { c });
            }
        }
        KodSatiri { no, kayma: kaymayi_bul(gosterilen, &acik_metin).unwrap_or(0), acik, isaretler: Vec::new() }
    }

    /// Ekran sütununun (0'dan başlar) kaynaktaki karakter sütunu (1'den başlar).
    fn sutun(&self, ekran: usize) -> usize {
        let acik_dizin = (ekran as isize + self.kayma).max(0) as usize;
        match self.acik.get(acik_dizin) {
            Some(&dizin) => dizin + 1,
            None => self.acik.last().map_or(1, |&son| son + 2),
        }
    }
}

/// Gösterilen metnin, sekmeleri açılmış kaynak satırına göre kayması.
fn kaymayi_bul(gosterilen: &str, acik_metin: &str) -> Option<isize> {
    let karakterler: Vec<char> = gosterilen.chars().collect();
    // Çok satırlı bir aralık varsa satırların solunda `| ` gibi çizgiler olur.
    for kenar in [0, 2, 4, 6] {
        let Some(kalan) = karakterler.get(kenar..) else { break };
        let kalan: String = kalan.iter().collect();
        let (kirpma, parca) = match kalan.strip_prefix("...") {
            Some(parca) => (3, parca),
            None => (0, kalan.as_str()),
        };
        let parca = parca.strip_suffix("...").unwrap_or(parca);
        let bulunan = if kirpma == 0 {
            acik_metin.starts_with(parca).then_some(0)
        } else {
            acik_metin.find(parca).map(|bayt| acik_metin[..bayt].chars().count())
        };
        if let Some(yer) = bulunan {
            return Some(yer as isize - (kenar + kirpma) as isize);
        }
    }
    None
}

/// `error[E0308]: mismatched types` → tanılama
fn baslik(satir: &str) -> Option<Tani> {
    let (seviye, kalan) = if let Some(kalan) = satir.strip_prefix("error") {
        ("error", kalan)
    } else {
        ("warning", satir.strip_prefix("warning")?)
    };
    let (kod, mesaj) = match kalan.strip_prefix('[') {
        Some(kalan) => {
            let (kod, mesaj) = kalan.split_once("]: ")?;
            (Some(kod.to_owned()), mesaj)
        }
        None => (None, kalan.strip_prefix(": ")?),
    };
    // Cargo'nun özet satırları
    if mesaj.starts_with("could not compile")
        || mesaj.starts_with("aborting due to")
        || mesaj.starts_with("`playground`")
        || mesaj.starts_with("build failed")
    {
        return None;
    }
    Some(Tani { seviye: seviye.to_owned(), kod, mesaj: mesaj.to_owned(), ..Tani::default() })
}

/// `help: consider borrowing here` → ("help", "consider borrowing here")
fn alt_baslik(satir: &str) -> Option<(&str, &str)> {
    ["help", "note", "suggestion"]
        .into_iter()
        .find_map(|seviye| Some((seviye, satir.strip_prefix(seviye)?.strip_prefix(": ")?)))
}

/// ` --> src/main.rs:2:18` → ("-->", "src/main.rs:2:18")
fn ok_satiri(satir: &str) -> Option<(&str, &str)> {
    let kalan = satir.trim_start();
    ["-->", ":::"].into_iter().find_map(|isaret| Some((isaret, kalan.strip_prefix(isaret)?.strip_prefix(' ')?)))
}

/// Soldaki oluğu ayırır: `2 |     let x` → (Some(2), '|', "    let x"),
/// `  = note: …` → (None, '=', "note: …").
fn oluk(satir: &str) -> Option<(Option<usize>, char, &str)> {
    let onek_uzunlugu = satir.find(|c: char| !(c == ' ' || c.is_ascii_digit()))?;
    let onek = &satir[..onek_uzunlugu];
    if !onek.ends_with(' ') {
        return None;
    }
    let kalan = &satir[onek_uzunlugu..];
    let isaret = kalan.chars().next()?;
    if !matches!(isaret, '|' | '=' | '+' | '-' | '~') {
        return None;
    }
    let icerik = &kalan[1..];
    let icerik = if icerik.is_empty() { "" } else { icerik.strip_prefix(' ')? };
    let no = match onek.trim() {
        "" => None,
        sayi => Some(sayi.parse().ok()?),
    };
    Some((no, isaret, icerik))
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn etiketler_ve_asili_etiketler() {
        let rust = "fn main() {\n    let x: i32 = \"a\";\n}\n";
        let cikti = "   Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let x: i32 = \"a\";
  |            ---   ^^^ expected `i32`, found `&str`
  |            |
  |            expected due to this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin \"playground\") due to 1 previous error
";
        let tanilar = ayristir(cikti, rust);
        assert_eq!(tanilar.len(), 1);
        let tani = &tanilar[0];
        assert_eq!(tani.kod.as_deref(), Some("E0308"));
        assert_eq!(tani.mesaj, "mismatched types");
        assert_eq!(tani.konum, Some((2, 18)));
        assert_eq!(
            tani.etiketler,
            vec![
                Etiket {
                    satir: 2,
                    baslangic: 12,
                    bitis: 15,
                    birincil: false,
                    mesaj: Some("expected due to this".into())
                },
                Etiket {
                    satir: 2,
                    baslangic: 18,
                    bitis: 21,
                    birincil: true,
                    mesaj: Some("expected `i32`, found `&str`".into())
                },
            ]
        );
    }

    #[test]
    fn kirpilmis_satir_ve_sekmeler() {
        let uzun = format!("    let a{} = 1; let b: i32 = \"a\"; let c = 2;", "a".repeat(150));
        let rust = format!("fn main() {{\n{uzun}\n\tlet ğ: u8 = sayı + \"x\";\n}}\n");
        let sutun = uzun.chars().count() - "\"a\"; let c = 2;".chars().count() + 1;
        let cikti = format!(
            "error[E0308]: mismatched types
 --> src/main.rs:2:{sutun}
  |
2 | ...a = 1; let b: i32 = \"a\"; let c = 2;
  | {} ^^^ expected `i32`, found `&str`

error[E0277]: cannot add `&str` to `{{integer}}`
 --> src/main.rs:3:19
  |
3 |     let ğ: u8 = sayı + \"x\";
  | {} ^ no implementation for `{{integer}} + &str`
  |
  = help: the trait `Add<&str>` is not implemented for `{{integer}}`
  = help: the following other types implement trait `Add<Rhs>`:
            `&f128` implements `Add<f128>`
          and 56 others
",
            " ".repeat(22),
            " ".repeat(20),
        );
        let tanilar = ayristir(&cikti, &rust);
        assert_eq!(tanilar.len(), 2);
        let birincil = &tanilar[0].etiketler[0];
        assert_eq!((birincil.baslangic, birincil.bitis), (sutun, sutun + 3));
        let birincil = &tanilar[1].etiketler[0];
        assert_eq!((birincil.satir, birincil.baslangic, birincil.bitis), (3, 19, 20));
        assert_eq!(tanilar[1].notlar.len(), 2);
        assert_eq!(
            tanilar[1].notlar[1].mesaj,
            "the following other types implement trait `Add<Rhs>`:\n`&f128` implements `Add<f128>`\nand 56 others"
        );
    }

    #[test]
    fn oneriler() {
        let rust = "fn main() {\n    let v = 1;\n    println!(\"{}\", z);\n}\n";
        let cikti = "error[E0425]: cannot find value `z` in this scope
 --> src/main.rs:3:20
  |
3 |     println!(\"{}\", z);
  |                    ^ not found in this scope
  |
help: a local variable with a similar name exists
  |
3 -     println!(\"{}\", z);
3 +     println!(\"{}\", v);
  |

error[E0308]: mismatched types
 --> src/main.rs:2:13
  |
2 |     let v = 1;
  |             ^ expected `&i32`
  |
help: consider borrowing here
  |
2 |     let v = &1;
  |             +
";
        let tanilar = ayristir(cikti, rust);
        assert_eq!(tanilar.len(), 2);
        assert_eq!(
            tanilar[0].notlar,
            vec![Not {
                seviye: "help".into(),
                mesaj: "a local variable with a similar name exists".into(),
                oneri: Some((3, "    println!(\"{}\", v);".into())),
                yeni_satir: false,
            }]
        );
        assert_eq!(tanilar[1].notlar[0].oneri, Some((2, "    let v = &1;".into())));
        assert!(!tanilar[1].notlar[0].yeni_satir);
    }

    #[test]
    fn satir_ekleme_onerisi() {
        let rust = "fn main() {\n    let m = HashMap::new();\n}\n";
        let cikti = "error[E0433]: cannot find type `HashMap` in this scope
 --> src/main.rs:2:13
  |
2 |     let m = HashMap::new();
  |             ^^^^^^^ use of undeclared type `HashMap`
  |
help: consider importing this struct
  |
1 + use std::collections::HashMap;
  |
";
        let not = &ayristir(cikti, rust)[0].notlar[0];
        assert_eq!(not.oneri, Some((1, "use std::collections::HashMap;".into())));
        assert!(not.yeni_satir);
    }

    #[test]
    fn cok_satirli_aralik() {
        let rust = "fn f() -> i32 {\n    5;\n}\n";
        let cikti = "error[E0308]: mismatched types
 --> src/main.rs:1:15
  |
1 |   fn f() -> i32 {
  |      -          ^ expected `i32`, found `()`
  |  ____|
  | |
2 | |     5;
3 | | }
  | |_^
";
        let tanilar = ayristir(cikti, rust);
        let birincil = tanilar[0].etiketler.iter().find(|e| e.birincil).unwrap();
        assert_eq!((birincil.satir, birincil.baslangic), (1, 15));
        assert_eq!(birincil.mesaj.as_deref(), Some("expected `i32`, found `()`"));
    }
}
