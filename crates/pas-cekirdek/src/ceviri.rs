//! Pas ⇄ Rust çevirisi.
//!
//! Çeviri sözcük düzeyindedir: sözlükteki isimler karşılıklarıyla
//! değiştirilir, geri kalan her şey (boşluklar, yorumlar, dizgiler)
//! olduğu gibi kalır. Satır sayısı hiç değişmez. Bunun üç istisnası var:
//!
//! * Biçim dizgilerindeki isimler (`yazdır!("{uzunluk}")`) de çevrilir.
//! * `mod çalış;` gibi ASCII olmayan modül bildirimlerine rustc'nin
//!   istediği `#[path = "…"]` niteliği eklenir.
//! * `yaz!(…)` çıktıyı hemen ekrana basar (`print!` + `flush`); yoksa
//!   girdi okumadan önce yazılan soru ekranda görünmez.

use std::collections::HashMap;

use unicode_normalization::{UnicodeNormalization, is_nfc};

use crate::kaynak_haritasi::{KaynakHaritasi, Parca};
use crate::sozcukler::{Sozcuk, SozcukTuru, isim_mi, sozcuklere_ayir};
use crate::sozluk::{self, Sinif};

#[derive(Debug, Clone)]
pub struct Ceviri {
    pub rust: String,
    pub harita: KaynakHaritasi,
}

#[derive(Default)]
pub struct Secenekler<'a> {
    /// ASCII olmayan ya da sözlükte bulunan bir modül adı için (`mod çalış;`)
    /// `#[path]` niteliğine yazılacak yolu üretir. Verilmezse `ad.rs` kullanılır.
    pub modul_yolu: Option<&'a dyn Fn(&str) -> String>,
}

/// Pas kaynağını Rust'a çevirir.
pub fn rusta_cevir(kaynak: &str) -> Ceviri {
    rusta_cevir_secenekle(kaynak, &Secenekler::default())
}

pub fn rusta_cevir_secenekle(kaynak: &str, secenekler: &Secenekler) -> Ceviri {
    let (rust, harita) = Cevirmen::yeni(kaynak, Hedef::Rust).calistir(secenekler);
    Ceviri { rust, harita }
}

/// Rust kaynağını Pas'a çevirir. Hata mesajlarındaki kod parçaları ve
/// `cargo pas çevir --ters` için kullanılır.
pub fn pasa_cevir(kaynak: &str) -> String {
    Cevirmen::yeni(kaynak, Hedef::Pas).calistir(&Secenekler::default()).0
}

/// Biçim dizgisi alan makrolar ve biçim dizgisinden önce gelen virgül sayısı.
const BICIM_MAKROLARI: &[(&str, usize)] = &[
    ("println", 0),
    ("print", 0),
    ("eprintln", 0),
    ("eprint", 0),
    ("format", 0),
    ("format_args", 0),
    ("panic", 0),
    ("todo", 0),
    ("unimplemented", 0),
    ("unreachable", 0),
    ("write", 1),
    ("writeln", 1),
    ("assert", 1),
    ("debug_assert", 1),
    ("assert_eq", 2),
    ("assert_ne", 2),
    ("debug_assert_eq", 2),
    ("debug_assert_ne", 2),
];

const CIKTIYI_BOSALT: &str = ";::std::io::Write::flush(&mut ::std::io::stdout()).ok();}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hedef {
    Rust,
    Pas,
}

struct Cevirmen<'a> {
    kaynak: &'a str,
    hedef: Hedef,
    sozcukler: Vec<Sozcuk>,
    /// Anlamlı (boşluk ve yorum olmayan) sözcüklerin `sozcukler` içindeki sıraları.
    anlamlilar: Vec<usize>,
    cikti: String,
    harita: KaynakHaritasi,
    /// `kaynak` içinde çıktıya kopyalanmış son konum.
    kopyalanan: usize,
}

impl<'a> Cevirmen<'a> {
    fn yeni(kaynak: &'a str, hedef: Hedef) -> Self {
        let sozcukler = sozcuklere_ayir(kaynak);
        let anlamlilar = (0..sozcukler.len()).filter(|&i| !sozcukler[i].anlamsiz_mi()).collect();
        Cevirmen {
            kaynak,
            hedef,
            sozcukler,
            anlamlilar,
            cikti: String::with_capacity(kaynak.len() + kaynak.len() / 8),
            harita: KaynakHaritasi::default(),
            kopyalanan: 0,
        }
    }

    fn calistir(mut self, secenekler: &Secenekler) -> (String, KaynakHaritasi) {
        let mut oncesine: HashMap<usize, String> = HashMap::new();
        let mut sonrasina: HashMap<usize, String> = HashMap::new();
        if self.hedef == Hedef::Rust {
            self.modul_yollarini_planla(secenekler, &mut oncesine);
            self.bosaltan_yaziyi_planla(&mut oncesine, &mut sonrasina);
        }

        let mut bicim = BicimIzleyici::default();
        for k in 0..self.anlamlilar.len() {
            let ti = self.anlamlilar[k];
            let sozcuk = self.sozcukler[ti];
            if let Some(metin) = oncesine.get(&ti) {
                self.degistir(sozcuk.baslangic..sozcuk.baslangic, metin);
            }
            match sozcuk.tur {
                SozcukTuru::Isim => {
                    if let Some(kelime) = self.kelimeyi_cevir(sozcuk.metin(self.kaynak)) {
                        self.degistir(sozcuk.baslangic..sozcuk.bitis, &kelime);
                    }
                }
                SozcukTuru::YasamSuresi => self.yasam_suresini_cevir(sozcuk),
                SozcukTuru::Dizgi | SozcukTuru::HamDizgi if bicim.dizgi_bekliyor_mu() => {
                    self.bicim_dizgisini_cevir(sozcuk);
                    bicim.dizgi_goruldu();
                }
                SozcukTuru::Isaret => bicim.isareti_isle(&self, k),
                _ => {}
            }
            if let Some(metin) = sonrasina.get(&ti) {
                self.degistir(sozcuk.bitis..sozcuk.bitis, metin);
            }
        }
        self.kopyala(self.kaynak.len());
        (self.cikti, self.harita)
    }

    fn kopyala(&mut self, konum: usize) {
        self.cikti.push_str(&self.kaynak[self.kopyalanan..konum]);
        self.kopyalanan = konum;
    }

    fn degistir(&mut self, aralik: std::ops::Range<usize>, metin: &str) {
        self.kopyala(aralik.start);
        self.harita.ekle(Parca {
            pas_baslangic: aralik.start,
            pas_uzunluk: aralik.len(),
            rust_baslangic: self.cikti.len(),
            rust_uzunluk: metin.len(),
        });
        self.cikti.push_str(metin);
        self.kopyalanan = aralik.end;
    }

    fn anlamli_sozcuk(&self, k: usize) -> Option<Sozcuk> {
        self.anlamlilar.get(k).map(|&i| self.sozcukler[i])
    }

    fn anlamli_metin(&self, k: usize) -> &'a str {
        self.anlamli_sozcuk(k).map_or("", |t| t.metin(self.kaynak))
    }

    /// Bir ismin çeviri yönündeki karşılığı; değişmeyecekse `None`.
    fn kelimeyi_cevir(&self, kelime: &str) -> Option<String> {
        match self.hedef {
            Hedef::Rust => sozluk::pas_ile(&nfc(kelime)).map(|e| e.rust.to_owned()),
            Hedef::Pas => match sozluk::rust_ile(kelime) {
                Some(girdi) => Some(girdi.pas.to_owned()),
                // Rust'ta sıradan bir isim olan `her`, Pas'ta ayrılmış bir sözcük.
                None => sozluk::pas_ile(&nfc(kelime))
                    .filter(|e| e.tur == Sinif::AnahtarSozcuk)
                    .map(|_| format!("r#{kelime}")),
            },
        }
    }

    /// Bir ismin Rust karşılığı (çevrilmeyecekse kendisi).
    fn rust_adi(&self, kelime: &'a str) -> std::borrow::Cow<'a, str> {
        match self.hedef {
            Hedef::Rust => match sozluk::pas_ile(&nfc(kelime)) {
                Some(girdi) => girdi.rust.into(),
                None => kelime.into(),
            },
            Hedef::Pas => kelime.into(),
        }
    }

    fn yasam_suresini_cevir(&mut self, sozcuk: Sozcuk) {
        let ad = &sozcuk.metin(self.kaynak)[1..];
        let girdi = match self.hedef {
            Hedef::Rust => sozluk::pas_ile(&nfc(ad)).map(|e| (e, e.rust)),
            Hedef::Pas => sozluk::rust_ile(ad).map(|e| (e, e.pas)),
        };
        // Anahtar sözcüklerden yalnızca `'static` geçerli bir yaşam süresidir.
        if let Some((girdi, kelime)) = girdi
            && (girdi.tur != Sinif::AnahtarSozcuk || girdi.rust == "static")
        {
            self.degistir(sozcuk.baslangic..sozcuk.bitis, &format!("'{kelime}"));
        }
    }

    /// `"{uzunluk} {x:?}"` gibi biçim dizgilerindeki isimleri çevirir.
    fn bicim_dizgisini_cevir(&mut self, sozcuk: Sozcuk) {
        let metin = sozcuk.metin(self.kaynak);
        let (Some(acilis), Some(kapanis)) = (metin.find('"'), metin.rfind('"')) else { return };
        if kapanis <= acilis {
            return;
        }
        let icerik_baslangici = sozcuk.baslangic + acilis + 1;
        let icerik = &metin[acilis + 1..kapanis];
        let mut i = 0;
        while i < icerik.len() {
            let kalan = &icerik[i..];
            if kalan.starts_with("{{") || kalan.starts_with("}}") {
                i += 2;
            } else if let Some(ic) = kalan.strip_prefix('{') {
                let arguman_uzunlugu = ic.find([':', '}']).unwrap_or(ic.len());
                let arguman = &ic[..arguman_uzunlugu];
                if isim_mi(arguman)
                    && let Some(kelime) = self.kelimeyi_cevir(arguman)
                {
                    let baslangic = icerik_baslangici + i + 1;
                    self.degistir(baslangic..baslangic + arguman.len(), &kelime);
                }
                i += 1 + arguman_uzunlugu;
            } else {
                i += kalan.chars().next().map_or(1, char::len_utf8);
            }
        }
    }

    /// `mod çalış;` → `#[path = "çalış.rs"] mod çalış;`
    fn modul_yollarini_planla(&self, secenekler: &Secenekler, oncesine: &mut HashMap<usize, String>) {
        for k in 0..self.anlamlilar.len() {
            let Some(sozcuk) = self.anlamli_sozcuk(k) else { continue };
            if sozcuk.tur != SozcukTuru::Isim || self.rust_adi(sozcuk.metin(self.kaynak)) != "mod" {
                continue;
            }
            let Some(ad) = self.anlamli_sozcuk(k + 1).filter(|t| t.tur == SozcukTuru::Isim) else {
                continue;
            };
            if self.anlamli_metin(k + 2) != ";" {
                continue;
            }
            let ad = nfc(ad.metin(self.kaynak));
            if ad.is_ascii() && sozluk::pas_ile(&ad).is_none() {
                continue;
            }

            // Nitelik, görünürlük belirtecinden (`genel`, `genel(sandık)`) önce gelmeli.
            let mut j = k;
            if j > 0
                && self.anlamli_metin(j - 1) == ")"
                && let Some(acilis) = self.eslesen_acilis(j - 1)
                && acilis > 0
                && self.rust_adi(self.anlamli_metin(acilis - 1)) == "pub"
            {
                j = acilis - 1;
            } else if j > 0 && self.rust_adi(self.anlamli_metin(j - 1)) == "pub" {
                j -= 1;
            }
            // Zaten bir `#[path]` niteliği varsa dokunma.
            let mut yolu_var = false;
            let mut a = j;
            while a > 0 && self.anlamli_metin(a - 1) == "]" {
                let Some(acilis) = self.eslesen_acilis(a - 1) else { break };
                if acilis == 0 || self.anlamli_metin(acilis - 1) != "#" {
                    break;
                }
                yolu_var |= self.anlamli_metin(acilis + 1) == "path";
                a = acilis - 1;
            }
            if yolu_var {
                continue;
            }

            let yol = match secenekler.modul_yolu {
                Some(modul_yolu) => modul_yolu(&ad),
                None => format!("{ad}.rs"),
            };
            oncesine.insert(self.anlamlilar[j], format!("#[path = {yol:?}] "));
        }
    }

    /// `yaz!(…)` → `{print!(…);::std::io::Write::flush(…).ok();}`
    fn bosaltan_yaziyi_planla(&self, oncesine: &mut HashMap<usize, String>, sonrasina: &mut HashMap<usize, String>) {
        for k in 0..self.anlamlilar.len() {
            let Some(sozcuk) = self.anlamli_sozcuk(k) else { continue };
            let kelime = sozcuk.metin(self.kaynak);
            if sozcuk.tur != SozcukTuru::Isim || kelime == "print" || self.rust_adi(kelime) != "print" {
                continue;
            }
            if self.anlamli_metin(k + 1) != "!" || !matches!(self.anlamli_metin(k + 2), "(" | "[" | "{") {
                continue;
            }
            if let Some(kapanis) = self.eslesen_kapanis(k + 2) {
                oncesine.insert(self.anlamlilar[k], "{".to_owned());
                sonrasina.insert(self.anlamlilar[kapanis], CIKTIYI_BOSALT.to_owned());
            }
        }
    }

    fn eslesen_kapanis(&self, acilis: usize) -> Option<usize> {
        let mut derinlik = 0usize;
        for k in acilis..self.anlamlilar.len() {
            match self.anlamli_metin(k) {
                "(" | "[" | "{" => derinlik += 1,
                ")" | "]" | "}" => {
                    derinlik -= 1;
                    if derinlik == 0 {
                        return Some(k);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn eslesen_acilis(&self, kapanis: usize) -> Option<usize> {
        let mut derinlik = 0usize;
        for k in (0..=kapanis).rev() {
            match self.anlamli_metin(k) {
                ")" | "]" | "}" => derinlik += 1,
                "(" | "[" | "{" => {
                    derinlik -= 1;
                    if derinlik == 0 {
                        return Some(k);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

/// Hangi dizginin bir biçim dizgisi olduğunu izler.
#[derive(Default)]
struct BicimIzleyici {
    derinlik: usize,
    yigin: Vec<BicimCagrisi>,
}

struct BicimCagrisi {
    /// Makro çağrısının parantezlerinin içindeki derinlik.
    derinlik: usize,
    /// Biçim dizgisinden önce görülmesi gereken virgül sayısı.
    kalan_virgul: usize,
    bitti: bool,
}

impl BicimIzleyici {
    fn ust(&mut self) -> Option<&mut BicimCagrisi> {
        let derinlik = self.derinlik;
        self.yigin.last_mut().filter(|cagri| cagri.derinlik == derinlik)
    }

    fn dizgi_bekliyor_mu(&mut self) -> bool {
        self.ust().is_some_and(|cagri| cagri.kalan_virgul == 0 && !cagri.bitti)
    }

    fn dizgi_goruldu(&mut self) {
        if let Some(cagri) = self.ust() {
            cagri.bitti = true;
        }
    }

    fn isareti_isle(&mut self, t: &Cevirmen, k: usize) {
        match t.anlamli_metin(k) {
            "(" | "[" | "{" => {
                self.derinlik += 1;
                if k >= 2
                    && t.anlamli_metin(k - 1) == "!"
                    && t.anlamli_sozcuk(k - 2).is_some_and(|s| s.tur == SozcukTuru::Isim)
                {
                    let ad = t.rust_adi(t.anlamli_metin(k - 2));
                    if let Some(&(_, virgul)) = BICIM_MAKROLARI.iter().find(|(m, _)| *m == ad) {
                        self.yigin.push(BicimCagrisi { derinlik: self.derinlik, kalan_virgul: virgul, bitti: false });
                    }
                }
            }
            ")" | "]" | "}" => {
                if self.ust().is_some() {
                    self.yigin.pop();
                }
                self.derinlik = self.derinlik.saturating_sub(1);
            }
            "," => {
                if let Some(cagri) = self.ust() {
                    cagri.kalan_virgul = cagri.kalan_virgul.saturating_sub(1);
                }
            }
            _ => {}
        }
    }
}

fn nfc(kelime: &str) -> std::borrow::Cow<'_, str> {
    if is_nfc(kelime) { kelime.into() } else { kelime.nfc().collect::<String>().into() }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::kaynak_haritasi::Yon;

    fn rusta(kaynak: &str) -> String {
        rusta_cevir(kaynak).rust
    }

    #[test]
    fn merhaba_dunya() {
        assert_eq!(
            rusta("işlev ana() {\n    yazdır!(\"Merhaba dünya!\");\n}\n"),
            "fn main() {\n    println!(\"Merhaba dünya!\");\n}\n"
        );
    }

    #[test]
    fn anahtar_sozcukler_ve_hazir_isimler() {
        assert_eq!(
            rusta("yap değişken l: Liste<Metin> = liste![]; eğer l.boş_mu() { döndür Yok; } değilse { kır; }"),
            "let mut l: Vec<String> = vec![]; if l.is_empty() { return None; } else { break; }"
        );
        assert_eq!(rusta("her i içinde 0..10 {}"), "for i in 0..10 {}");
    }

    #[test]
    fn ingilizce_rust_da_calisir() {
        let kaynak = "fn main() { let mut x = 5; if x > 2 { println!(\"{x}\"); } }";
        assert_eq!(rusta(kaynak), kaynak);
    }

    #[test]
    fn dizgilere_ve_yorumlara_dokunulmaz() {
        assert_eq!(
            rusta("yap s = \"yap eğer\"; // eğer yap\n/* işlev */ 'ı'"),
            "let s = \"yap eğer\"; // eğer yap\n/* işlev */ 'ı'"
        );
    }

    #[test]
    fn ham_isimler_anahtar_sozcukleri_kacirir() {
        assert_eq!(rusta("yap r#tür = 1;"), "let r#tür = 1;");
    }

    #[test]
    fn bicim_dizgisi_argumanlari() {
        assert_eq!(
            rusta("yazdır!(\"{uzunluk} {{uzunluk}} {x:?} {ana:>5}\", x = 1);"),
            "println!(\"{len} {{uzunluk}} {x:?} {main:>5}\", x = 1);"
        );
        // Biçim dizgisi olmayan dizgilere dokunulmaz.
        assert_eq!(
            rusta("yazdır!(\"{}\", \"{uzunluk}\"); yap s = \"{uzunluk}\";"),
            "println!(\"{}\", \"{uzunluk}\"); let s = \"{uzunluk}\";"
        );
        assert_eq!(rusta("doğrula_eşit!(a, \"{ana}\", \"{ana}\");"), "assert_eq!(a, \"{ana}\", \"{main}\");");
    }

    #[test]
    fn yasam_sureleri() {
        assert_eq!(rusta("&'durağan str"), "&'static str");
        assert_eq!(rusta("<'a>"), "<'a>");
    }

    #[test]
    fn yaz_ciktiyi_hemen_basar() {
        assert_eq!(rusta("yaz!(\"Adın: \");"), format!("{{print!(\"Adın: \"){CIKTIYI_BOSALT};"));
        assert_eq!(rusta("print!(\"a\");"), "print!(\"a\");");
        assert_eq!(rusta("makro_kuralları! yaz { () => {} }"), "macro_rules! print { () => {} }");
    }

    #[test]
    fn ascii_olmayan_modullere_yol_eklenir() {
        assert_eq!(rusta("mod çalış;"), "#[path = \"çalış.rs\"] mod çalış;");
        assert_eq!(rusta("genel modül çalış;"), "#[path = \"çalış.rs\"] pub mod çalış;");
        assert_eq!(rusta("genel(sandık) modül çalış;"), "#[path = \"çalış.rs\"] pub(crate) mod çalış;");
        assert_eq!(rusta("#[path = \"x.rs\"] mod çalış;"), "#[path = \"x.rs\"] mod çalış;");
        assert_eq!(rusta("modül oyun;"), "mod oyun;");
        assert_eq!(rusta("modül çalış { }"), "mod çalış { }");
        // Sözlükteki bir isim çevrilir ama dosyası yine Türkçe adıyla aranır.
        assert_eq!(rusta("modül liste;"), "#[path = \"liste.rs\"] mod vec;");

        let modul_yolu = |ad: &str| format!("oyun/{ad}.rs");
        let secenekler = Secenekler { modul_yolu: Some(&modul_yolu) };
        assert_eq!(rusta_cevir_secenekle("mod şekil;", &secenekler).rust, "#[path = \"oyun/şekil.rs\"] mod şekil;");
    }

    #[test]
    fn ayrik_yazilmis_girdi_birlestirilir() {
        // "değilse" with "g" + U+0306 (birleşik breve)
        assert_eq!(rusta("de\u{67}\u{306}ilse"), "else");
    }

    #[test]
    fn ters_ceviri() {
        assert_eq!(
            pasa_cevir("fn main() { let mut v: Vec<String> = vec![]; println!(\"{}\", v.len()); }"),
            "işlev ana() { yap değişken v: Liste<Metin> = liste![]; yazdır!(\"{}\", v.uzunluk()); }"
        );
        assert_eq!(pasa_cevir("let her = 1;"), "yap r#her = 1;");
        assert_eq!(pasa_cevir("`Option<i32>`"), "`Belki<i32>`");
        assert_eq!(pasa_cevir("&'static str"), "&'durağan str");
    }

    #[test]
    fn gidip_donme() {
        let kaynak = "işlev ana() {\n    yap değişken sayılar = liste![1, 2];\n    sayılar.ekle(3);\n    yazdır!(\"{}\", sayılar.uzunluk());\n}\n";
        assert_eq!(pasa_cevir(&rusta(kaynak)), kaynak);
    }

    #[test]
    fn kaynak_haritasi_pasi_gosterir() {
        let kaynak = "işlev ana() { yap x = yok_böyle; }";
        let t = rusta_cevir(kaynak);
        let rust_baslangic = t.rust.find("yok_böyle").unwrap();
        let rust_bitis = rust_baslangic + "yok_böyle".len();
        let (baslangic, bitis) = t.harita.rust_araligini_pasa(rust_baslangic, rust_bitis);
        assert_eq!(&kaynak[baslangic..bitis], "yok_böyle");

        let islev_sonu = t.rust.find("fn").unwrap() + 2;
        assert_eq!(t.harita.rust_araligini_pasa(0, islev_sonu), (0, "işlev".len()));
        assert_eq!(t.harita.pastan_rusta(kaynak.find("x").unwrap(), Yon::Baslangic), t.rust.find("x").unwrap());
    }

    #[test]
    fn satir_numaralari_korunur() {
        let kaynak = "işlev ana() {\n  yaz!(\"a\");\n  modül çalış;\n}\n";
        assert_eq!(rusta(kaynak).lines().count(), kaynak.lines().count());
    }
}
