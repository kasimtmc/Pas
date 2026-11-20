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
use std::sync::OnceLock;

use unicode_normalization::{UnicodeNormalization, is_nfc};

use crate::kaynak_haritasi::{KaynakHaritasi, Parca, Yonlendirme};
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
    /// `oku!` gibi Pas'a özgü makroları sağlayan destek kodunu dosyanın başına
    /// ekler. Yalnızca sandık köklerinde (`ana.pas`, `kütüphane.pas`) kullanılır.
    pub destek_ekle: bool,
    /// `yazdır!` gibi makroların değerleri Türkçe yazdırması için argümanlarını
    /// destek kodundaki sarmalayıcıya alır (`doğru`, `Var(…)`, `Yok`). Derlenen
    /// projelerin her dosyasında açıktır; `cargo pas çevir` sade çeviriyi gösterir.
    pub turkce_cikti: bool,
    /// Programın okuyacağı girdi önceden biliniyorsa (oyun alanı), `oku!`
    /// klavye yerine bu metinden okur. Yalnızca `destek_ekle` ile anlamlıdır.
    pub hazir_girdi: Option<&'a str>,
    /// `rastgele!` için sabit bir tohum (oyun alanı). Verilmezse saatten alınır.
    pub tohum: Option<u64>,
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

/// `destek.rs` dosyasının, satır sayısını değiştirmeden eklenebilmesi için
/// yorumları atılıp tek satıra indirilmiş hali.
fn destek_satiri() -> &'static str {
    static SATIR: OnceLock<String> = OnceLock::new();
    SATIR.get_or_init(|| {
        include_str!("destek.rs")
            .lines()
            .map(str::trim)
            .filter(|satir| !satir.is_empty() && !satir.starts_with("//"))
            .collect::<Vec<_>>()
            .join(" ")
    })
}

/// Destek kodu; hazır girdi ya da tohum verildiyse onlar yerleştirilir.
fn destek_metni(secenekler: &Secenekler) -> String {
    let mut metin = destek_satiri().to_owned();
    if let Some(girdi) = secenekler.hazir_girdi {
        let bos = "const HAZIR_GIRDI: Option<&str> = None;";
        debug_assert!(metin.contains(bos));
        metin = metin.replace(bos, &format!("const HAZIR_GIRDI: Option<&str> = Some({girdi:?});"));
    }
    if let Some(tohum) = secenekler.tohum {
        let bos = "const HAZIR_TOHUM: Option<u64> = None;";
        debug_assert!(metin.contains(bos));
        metin = metin.replace(bos, &format!("const HAZIR_TOHUM: Option<u64> = Some({tohum});"));
    }
    metin
}

/// Pas çıktı makroları; argümanları Türkçe yazdırılsın diye sarmalanır
/// (`doğru`, `Var(…)`, `Yok`, …). İngilizce yazılan `println!` dokunulmaz.
const TURKCE_CIKTI_MAKROLARI: &[&str] = &["println", "print", "eprintln", "format", "panic"];

/// Bir argümanı Türkçe yazdıran sarmalayıcı (bkz. `destek.rs`).
const SARMALA_BAS: &str = "(&crate::pas_destek::Sar(&(";
const SARMALA_SON: &str = "))).pas_turkce()";
/// Sarmalayıcının yöntemini sağlayan nitelikler makronun çevresindeki blokta kullanıma alınır.
const SARMALAYICI_BLOGU: &str = "{#[allow(unused_imports)] use crate::pas_destek::{DuzSecim as _, TurkceSecimi as _}; ";

/// Bir sözcüğün önüne ya da arkasına eklenecek metin.
#[derive(Default)]
struct Ekleme {
    metin: String,
    /// (metindeki bayt aralığı, Pas kaynağındaki karşılığı)
    yonlendirmeler: Vec<(std::ops::Range<usize>, std::ops::Range<usize>)>,
}

impl Ekleme {
    fn yeni(metin: impl Into<String>) -> Self {
        Ekleme { metin: metin.into(), yonlendirmeler: Vec::new() }
    }

    /// `diger` bu eklemenin sonuna gelir.
    fn sonuna(mut self, diger: Ekleme) -> Self {
        let kayma = self.metin.len();
        self.metin.push_str(&diger.metin);
        self.yonlendirmeler.extend(diger.yonlendirmeler.into_iter().map(|(r, p)| (r.start + kayma..r.end + kayma, p)));
        self
    }
}

type Eklemeler = HashMap<usize, Ekleme>;

/// `yeni`, var olan eklemenin dışında kalır: önüne gelir.
fn onune_ekle(eklemeler: &mut Eklemeler, sozcuk: usize, yeni: Ekleme) {
    let eski = eklemeler.remove(&sozcuk).unwrap_or_default();
    eklemeler.insert(sozcuk, yeni.sonuna(eski));
}

/// `yeni`, var olan eklemenin dışında kalır: arkasına gelir.
fn arkasina_ekle(eklemeler: &mut Eklemeler, sozcuk: usize, yeni: Ekleme) {
    let eski = eklemeler.remove(&sozcuk).unwrap_or_default();
    eklemeler.insert(sozcuk, eski.sonuna(yeni));
}

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
        let mut oncesine = Eklemeler::new();
        let mut sonrasina = Eklemeler::new();
        if self.hedef == Hedef::Rust {
            self.modul_yollarini_planla(secenekler, &mut oncesine);
            self.bosaltan_yaziyi_planla(&mut oncesine, &mut sonrasina);
            if secenekler.turkce_cikti {
                self.turkce_ciktiyi_planla(&mut oncesine, &mut sonrasina);
            }
        }
        let mut destek_eklenecek = self.hedef == Hedef::Rust && secenekler.destek_ekle;
        if destek_eklenecek && let Some(k) = self.ilk_oge() {
            onune_ekle(&mut oncesine, self.anlamlilar[k], Ekleme::yeni(format!("{} ", destek_metni(secenekler))));
            destek_eklenecek = false;
        }

        let mut bicim = BicimIzleyici::default();
        // `uygula Nitelik için Tür {`: başlıktaki `için` Rust'ta `for` olur.
        let mut uygula_basliginda = false;
        for k in 0..self.anlamlilar.len() {
            let ti = self.anlamlilar[k];
            let sozcuk = self.sozcukler[ti];
            if let Some(ekleme) = oncesine.remove(&ti) {
                self.ekle(sozcuk.baslangic, ekleme);
            }
            match sozcuk.tur {
                SozcukTuru::Isim => {
                    let metin = sozcuk.metin(self.kaynak);
                    let kelime = match (uygula_basliginda, self.hedef) {
                        (true, Hedef::Rust) if nfc(metin) == "için" => Some("for".to_owned()),
                        (true, Hedef::Pas) if metin == "for" => Some("için".to_owned()),
                        _ => self.kelimeyi_cevir(metin),
                    };
                    if self.rust_adi(metin) == "impl" {
                        uygula_basliginda = true;
                    }
                    if let Some(kelime) = kelime {
                        self.degistir(sozcuk.baslangic..sozcuk.bitis, &kelime);
                    }
                }
                SozcukTuru::Isaret if matches!(sozcuk.metin(self.kaynak), "{" | ";") => {
                    uygula_basliginda = false;
                    bicim.isareti_isle(&self, k);
                }
                SozcukTuru::YasamSuresi => self.yasam_suresini_cevir(sozcuk),
                SozcukTuru::Dizgi | SozcukTuru::HamDizgi if bicim.dizgi_bekliyor_mu() => {
                    self.bicim_dizgisini_cevir(sozcuk);
                    bicim.dizgi_goruldu();
                }
                SozcukTuru::Isaret => bicim.isareti_isle(&self, k),
                _ => {}
            }
            if let Some(ekleme) = sonrasina.remove(&ti) {
                self.ekle(sozcuk.bitis, ekleme);
            }
        }
        if destek_eklenecek {
            // Dosyada hiç öğe yok; destek kodu sona eklenir.
            self.degistir(self.kaynak.len()..self.kaynak.len(), &destek_metni(secenekler));
        }
        self.kopyala(self.kaynak.len());
        (self.cikti, self.harita)
    }

    /// Dosyanın başındaki iç niteliklerden (`#![…]`) sonraki ilk anlamlı sözcük.
    /// Öğeler iç niteliklerden önce gelemez.
    fn ilk_oge(&self) -> Option<usize> {
        let mut k = 0;
        while self.anlamli_metin(k) == "#" && self.anlamli_metin(k + 1) == "!" && self.anlamli_metin(k + 2) == "[" {
            k = self.eslesen_kapanis(k + 2)? + 1;
        }
        (k < self.anlamlilar.len()).then_some(k)
    }

    fn kopyala(&mut self, konum: usize) {
        self.cikti.push_str(&self.kaynak[self.kopyalanan..konum]);
        self.kopyalanan = konum;
    }

    /// Pas kaynağının `konum` noktasına karşılığı olmayan bir metin ekler.
    fn ekle(&mut self, konum: usize, ekleme: Ekleme) {
        let rust_baslangic = self.degistir(konum..konum, &ekleme.metin);
        for (rust, pas) in ekleme.yonlendirmeler {
            self.harita.yonlendir(Yonlendirme {
                rust_baslangic: rust_baslangic + rust.start,
                rust_uzunluk: rust.len(),
                pas_baslangic: pas.start,
                pas_uzunluk: pas.len(),
            });
        }
    }

    /// Değiştirilen metnin çıktıdaki başlangıcını döndürür.
    fn degistir(&mut self, aralik: std::ops::Range<usize>, metin: &str) -> usize {
        self.kopyala(aralik.start);
        self.harita.ekle(Parca {
            pas_baslangic: aralik.start,
            pas_uzunluk: aralik.len(),
            rust_baslangic: self.cikti.len(),
            rust_uzunluk: metin.len(),
        });
        let rust_baslangic = self.cikti.len();
        self.cikti.push_str(metin);
        self.kopyalanan = aralik.end;
        rust_baslangic
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
    fn modul_yollarini_planla(&self, secenekler: &Secenekler, oncesine: &mut Eklemeler) {
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
            onune_ekle(oncesine, self.anlamlilar[j], Ekleme::yeni(format!("#[path = {yol:?}] ")));
        }
    }

    /// `yaz!(…)` → `{print!(…);::std::io::Write::flush(…).ok();}`
    fn bosaltan_yaziyi_planla(&self, oncesine: &mut Eklemeler, sonrasina: &mut Eklemeler) {
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
                onune_ekle(oncesine, self.anlamlilar[k], Ekleme::yeni("{"));
                arkasina_ekle(sonrasina, self.anlamlilar[kapanis], Ekleme::yeni(CIKTIYI_BOSALT));
            }
        }
    }

    /// `yazdır!("{} {ad:?}", x)` →
    /// `{use …; println!("{} {ad:?}", Sar(&(x)).pas_turkce(), ad = Sar(&ad).pas_turkce())}`
    ///
    /// Argümanlar, `doğru`/`yanlış`, `Var(…)`/`Yok` gibi Türkçe yazdıran bir
    /// sarmalayıcıya alınır. Dizgi içine yazılan `{ad}` değişkenleri adlandırılmış
    /// argümana dönüşür; adları hatalar için dizgideki yerlerine yönlendirilir.
    fn turkce_ciktiyi_planla(&self, oncesine: &mut Eklemeler, sonrasina: &mut Eklemeler) {
        for k in 0..self.anlamlilar.len() {
            let Some(sozcuk) = self.anlamli_sozcuk(k) else { continue };
            let kelime = sozcuk.metin(self.kaynak);
            let rust = self.rust_adi(kelime);
            if sozcuk.tur != SozcukTuru::Isim || rust == kelime || !TURKCE_CIKTI_MAKROLARI.contains(&&*rust) {
                continue;
            }
            if self.anlamli_metin(k + 1) != "!" || !matches!(self.anlamli_metin(k + 2), "(" | "[" | "{") {
                continue;
            }
            let Some(kapanis) = self.eslesen_kapanis(k + 2) else { continue };
            let argumanlar = self.argumanlar(k + 3, kapanis);
            let Some(&(dizgi, dizgi_sonu)) = argumanlar.first() else { continue };
            let dizgi_sozcugu = self.anlamli_sozcuk(dizgi).expect("argüman var");
            if dizgi != dizgi_sonu || !matches!(dizgi_sozcugu.tur, SozcukTuru::Dizgi | SozcukTuru::HamDizgi) {
                continue;
            }
            // Genişlik/duyarlık argümanları (`{:1$}`, `{:.*}`) sarmalanamaz; dokunulmaz.
            let dizgi_metni = dizgi_sozcugu.metin(self.kaynak);
            if dizgi_metni.contains('$') || dizgi_metni.contains(".*") {
                continue;
            }

            let mut adlandirilmis = Vec::new();
            for &(bas, son) in &argumanlar[1..] {
                if bas > son {
                    continue; // sondaki virgül
                }
                let mut ifade = bas;
                if self.anlamli_sozcuk(bas).is_some_and(|s| s.tur == SozcukTuru::Isim)
                    && self.anlamli_metin(bas + 1) == "="
                    && self.anlamli_metin(bas + 2) != "="
                {
                    adlandirilmis.push(self.rust_adi(self.anlamli_metin(bas)).into_owned());
                    ifade = bas + 2;
                }
                if ifade > son {
                    continue;
                }
                onune_ekle(oncesine, self.anlamlilar[ifade], Ekleme::yeni(SARMALA_BAS));
                arkasina_ekle(sonrasina, self.anlamlilar[son], Ekleme::yeni(SARMALA_SON));
            }

            // Dizgideki `{ad}` değişkenleri
            let mut eklenecek = Ekleme::default();
            let mut virgul_gerek = self.anlamli_metin(kapanis - 1) != ",";
            for (ad, pas_araligi) in bicim_degiskenleri(dizgi_metni, dizgi_sozcugu.baslangic) {
                let rust_adi = self.rust_adi(ad).into_owned();
                if adlandirilmis.contains(&rust_adi) {
                    continue;
                }
                let on = if virgul_gerek { ", " } else { " " };
                virgul_gerek = true;
                let ilk = on.len();
                let ikinci = ilk + rust_adi.len() + " = ".len() + SARMALA_BAS.len();
                eklenecek = eklenecek.sonuna(Ekleme {
                    metin: format!("{on}{rust_adi} = {SARMALA_BAS}{rust_adi}{SARMALA_SON}"),
                    yonlendirmeler: vec![
                        (ilk..ilk + rust_adi.len(), pas_araligi.clone()),
                        (ikinci..ikinci + rust_adi.len(), pas_araligi),
                    ],
                });
                adlandirilmis.push(rust_adi);
            }
            if argumanlar.len() == 1 && eklenecek.metin.is_empty() {
                continue; // yazdırılacak değer yok
            }
            arkasina_ekle(oncesine, self.anlamlilar[kapanis], eklenecek);
            onune_ekle(oncesine, self.anlamlilar[k], Ekleme::yeni(SARMALAYICI_BLOGU));
            arkasina_ekle(sonrasina, self.anlamlilar[kapanis], Ekleme::yeni("}"));
        }
    }

    /// `bas..kapanis` arasındaki, en üst düzey virgüllerle ayrılmış argümanların
    /// (ilk, son) anlamlı sözcük sıraları. `::<…>` içindeki virgüller ayırmaz.
    fn argumanlar(&self, bas: usize, kapanis: usize) -> Vec<(usize, usize)> {
        let mut argumanlar = Vec::new();
        let mut derinlik = 0usize;
        let mut baslangic = bas;
        for k in bas..kapanis {
            match self.anlamli_metin(k) {
                "(" | "[" | "{" => derinlik += 1,
                ")" | "]" | "}" => derinlik = derinlik.saturating_sub(1),
                "<" if self.anlamli_metin(k.wrapping_sub(1)) == ":" => derinlik += 1,
                ">" if derinlik > 0 && self.anlamli_metin(k.wrapping_sub(1)) != "-" => derinlik -= 1,
                "," if derinlik == 0 => {
                    argumanlar.push((baslangic, k.wrapping_sub(1)));
                    baslangic = k + 1;
                }
                _ => {}
            }
        }
        if baslangic < kapanis {
            argumanlar.push((baslangic, kapanis - 1));
        } else if !argumanlar.is_empty() {
            argumanlar.push((baslangic, baslangic.wrapping_sub(1)));
        }
        argumanlar
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

/// Bir biçim dizgisindeki `{ad}`, `{ad:?}` değişkenleri ve adlarının kaynaktaki yerleri.
fn bicim_degiskenleri(dizgi: &str, dizgi_baslangici: usize) -> Vec<(&str, std::ops::Range<usize>)> {
    let (Some(acilis), Some(kapanis)) = (dizgi.find('"'), dizgi.rfind('"')) else { return Vec::new() };
    if kapanis <= acilis {
        return Vec::new();
    }
    let icerik = &dizgi[acilis + 1..kapanis];
    let mut sonuc = Vec::new();
    let mut i = 0;
    while i < icerik.len() {
        let kalan = &icerik[i..];
        if kalan.starts_with("{{") || kalan.starts_with("}}") {
            i += 2;
        } else if let Some(ic) = kalan.strip_prefix('{') {
            let uzunluk = ic.find([':', '}']).unwrap_or(ic.len());
            let ad = &ic[..uzunluk];
            if isim_mi(ad) {
                let baslangic = dizgi_baslangici + acilis + 1 + i + 1;
                sonuc.push((ad, baslangic..baslangic + ad.len()));
            }
            i += 1 + uzunluk;
        } else {
            i += kalan.chars().next().map_or(1, char::len_utf8);
        }
    }
    sonuc
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
            rusta("tanımla değişken l: Liste<Metin> = liste![]; eğer l.boş_mu() { döndür Yok; } değilse { kır; }"),
            "let mut l: Vec<String> = vec![]; if l.is_empty() { return None; } else { break; }"
        );
        assert_eq!(rusta("her i için 0..10 {}"), "for i in 0..10 {}");
    }

    #[test]
    fn ingilizce_rust_da_calisir() {
        let kaynak = "fn main() { let mut x = 5; if x > 2 { println!(\"{x}\"); } }";
        assert_eq!(rusta(kaynak), kaynak);
    }

    #[test]
    fn dizgilere_ve_yorumlara_dokunulmaz() {
        assert_eq!(
            rusta("tanımla s = \"tanımla eğer\"; // eğer yap\n/* işlev */ 'ı'"),
            "let s = \"tanımla eğer\"; // eğer yap\n/* işlev */ 'ı'"
        );
    }

    #[test]
    fn ham_isimler_anahtar_sozcukleri_kacirir() {
        assert_eq!(rusta("tanımla r#tür = 1;"), "let r#tür = 1;");
    }

    #[test]
    fn bicim_dizgisi_argumanlari() {
        assert_eq!(
            rusta("yazdır!(\"{uzunluk} {{uzunluk}} {x:?} {ana:>5}\", x = 1);"),
            "println!(\"{len} {{uzunluk}} {x:?} {main:>5}\", x = 1);"
        );
        // Biçim dizgisi olmayan dizgilere dokunulmaz.
        assert_eq!(
            rusta("yazdır!(\"{}\", \"{uzunluk}\"); tanımla s = \"{uzunluk}\";"),
            "println!(\"{}\", \"{uzunluk}\"); let s = \"{uzunluk}\";"
        );
        assert_eq!(rusta("doğrula_eşit!(a, \"{ana}\", \"{ana}\");"), "assert_eq!(a, \"{ana}\", \"{main}\");");
    }

    #[test]
    fn yasam_sureleri() {
        assert_eq!(rusta("&'kalıcı str"), "&'static str");
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
        let secenekler = Secenekler { modul_yolu: Some(&modul_yolu), ..Default::default() };
        assert_eq!(rusta_cevir_secenekle("mod şekil;", &secenekler).rust, "#[path = \"oyun/şekil.rs\"] mod şekil;");
    }

    #[test]
    fn ayrik_yazilmis_girdi_birlestirilir() {
        // "değilse" with "g" + U+0306 (birleşik breve)
        assert_eq!(rusta("de\u{67}\u{306}ilse"), "else");
    }

    #[test]
    fn destek_kodu_satir_eklemeden_eklenir() {
        let secenekler = Secenekler { destek_ekle: true, ..Default::default() };
        let kaynak = "// yorum\nişlev ana() {\n    tanımla ad = oku!(\"Adın? \");\n}\n";
        let ceviri = rusta_cevir_secenekle(kaynak, &secenekler);
        assert_eq!(ceviri.rust.lines().count(), kaynak.lines().count());
        assert!(
            ceviri.rust.starts_with("// yorum\n#[allow(unused_imports)] use crate::pas_destek::hazir::*; #[macro_use]"),
            "{}",
            ceviri.rust
        );
        assert!(ceviri.rust.contains("mod pas_destek {"));
        assert!(ceviri.rust.contains("let ad = pas_oku!(\"Adın? \");"));
        // `oku!` çağrısı Pas kaynağındaki yerine eşlenir.
        let rust_konumu = ceviri.rust.find("pas_oku!(").unwrap();
        let (baslangic, _) = ceviri.harita.rust_araligini_pasa(rust_konumu, rust_konumu + 7);
        assert!(kaynak[baslangic..].starts_with("oku!("));
    }

    #[test]
    fn destek_kodu_ic_niteliklerden_sonra_gelir() {
        let secenekler = Secenekler { destek_ekle: true, ..Default::default() };
        let ceviri = rusta_cevir_secenekle("#![allow(unused)]\nmodül şekil;\n", &secenekler);
        assert!(
            ceviri.rust.starts_with("#![allow(unused)]\n#[allow(unused_imports)] use crate::pas_destek"),
            "{}",
            ceviri.rust
        );
        assert!(ceviri.rust.ends_with("#[path = \"şekil.rs\"] mod şekil;\n"), "{}", ceviri.rust);
        // Boş dosyaya da eklenebilmeli.
        assert!(rusta_cevir_secenekle("", &secenekler).rust.contains("mod pas_destek"));
        // Varsayılan olarak eklenmez.
        assert!(!rusta("işlev ana() {}").contains("pas_destek"));
    }

    fn turkce(kaynak: &str) -> Ceviri {
        rusta_cevir_secenekle(kaynak, &Secenekler { turkce_cikti: true, ..Default::default() })
    }

    #[test]
    fn yazdir_turkce_cikti_icin_sarmalanir() {
        let rusta = |kaynak: &str| turkce(kaynak).rust;
        let b = SARMALAYICI_BLOGU;
        let (bas, son) = (SARMALA_BAS, SARMALA_SON);
        assert_eq!(
            rusta("yazdır!(\"{} {x:?}\", a + 1);"),
            format!("{b}println!(\"{{}} {{x:?}}\", {bas}a + 1{son}, x = {bas}x{son})}};")
        );
        // Adlandırılmış argümanlar ve sözlük sözcükleri
        assert_eq!(
            rusta("yazdır!(\"{uzunluk} {ad}\", ad = 5,);"),
            format!("{b}println!(\"{{len}} {{ad}}\", ad = {bas}5{son}, len = {bas}len{son})}};")
        );
        // Değer yoksa ya da dizgi genişlik argümanı kullanıyorsa dokunulmaz.
        assert_eq!(rusta("yazdır!(\"merhaba\");"), "println!(\"merhaba\");");
        assert_eq!(rusta("yazdır!(\"{:1$}\", a, 5);"), "println!(\"{:1$}\", a, 5);");
        // İngilizce makrolar Rust gibi davranır.
        assert_eq!(rusta("println!(\"{x}\");"), "println!(\"{x}\");");
        // `::<…>` içindeki virgül argümanları ayırmaz.
        assert!(
            rusta("yazdır!(\"{}\", Sözlük::<i32, i32>::new().uzunluk());")
                .contains(&format!("{bas}HashMap::<i32, i32>::new().len(){son}"))
        );
        // `yaz!` hem sarmalanır hem çıktıyı hemen basar.
        assert_eq!(rusta("yaz!(\"{x}\");"), format!("{b}{{print!(\"{{x}}\", x = {bas}x{son}){CIKTIYI_BOSALT}}};"));
    }

    #[test]
    fn dizgideki_degisken_dizgiye_yonlendirilir() {
        let kaynak = "yazdır!(\"{sayi}\");";
        let ceviri = turkce(kaynak);
        // Eklenen `sayi = …` argümanındaki ad, dizgideki `{sayi}` yerini gösterir.
        let eklenen = ceviri.rust.find("sayi = ").unwrap();
        let (bas, son) = ceviri.harita.rust_araligini_pasa(eklenen, eklenen + 4);
        assert_eq!(&kaynak[bas..son], "sayi");
        assert_eq!(bas, kaynak.find("sayi").unwrap());
        // Sarmalayıcıyla başlayan bir aralık da (`&(sayi)`) aynı yeri gösterir.
        let sarmalanan = ceviri.rust.find("&(sayi)").unwrap();
        let (bas, son) = ceviri.harita.rust_araligini_pasa(sarmalanan, sarmalanan + "&(sayi)".len());
        assert_eq!(&kaynak[bas..son], "sayi");
    }

    #[test]
    fn uygula_basligindaki_icin() {
        assert_eq!(
            rusta("uygula Hayvan için Kedi { işlev f() { her x için 0..3 {} } }"),
            "impl Hayvan for Kedi { fn f() { for x in 0..3 {} } }"
        );
        // Eski yazım da geçerli.
        assert_eq!(rusta("uygula Hayvan her Kedi {}"), "impl Hayvan for Kedi {}");
        assert_eq!(
            rusta("işlev f(h: &uygula Hayvan) { her x için 0..3 {} }"),
            "fn f(h: &impl Hayvan) { for x in 0..3 {} }"
        );
        assert_eq!(
            pasa_cevir("impl<T> Hayvan for Kutu<T> { fn f() { for x in y {} } }"),
            "uygula<T> Hayvan için Kutu<T> { işlev f() { her x için y {} } }"
        );
    }

    #[test]
    fn ters_ceviri() {
        assert_eq!(
            pasa_cevir("fn main() { let mut v: Vec<String> = vec![]; println!(\"{}\", v.len()); }"),
            "işlev ana() { tanımla değişken v: Liste<Metin> = liste![]; yazdır!(\"{}\", v.uzunluk()); }"
        );
        assert_eq!(pasa_cevir("let her = 1;"), "tanımla r#her = 1;");
        assert_eq!(pasa_cevir("`Option<i32>`"), "`Belki<i32>`");
        assert_eq!(pasa_cevir("&'static str"), "&'kalıcı str");
        assert_eq!(pasa_cevir("pas_oku!()"), "oku!()");
    }

    #[test]
    fn gidip_donme() {
        let kaynak = "işlev ana() {\n    tanımla değişken sayılar = liste![1, 2];\n    sayılar.ekle(3);\n    yazdır!(\"{}\", sayılar.uzunluk());\n}\n";
        assert_eq!(pasa_cevir(&rusta(kaynak)), kaynak);
    }

    #[test]
    fn kaynak_haritasi_pasi_gosterir() {
        let kaynak = "işlev ana() { tanımla x = yok_böyle; }";
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
