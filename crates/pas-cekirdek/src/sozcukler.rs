//! Kaynak kodu sözcüklere (token) ayıran basit bir sözcük çözümleyici.
//!
//! Rust sözdizimini tam olarak ayrıştırmaz; isimleri dizgilerden,
//! yorumlardan ve karakter sabitlerinden ayırt edecek kadarını bilir.
//! Kaynağın her baytı tam olarak bir sözcüğe aittir; böylece çevirici
//! değiştirmediği her şeyi olduğu gibi bırakabilir.

use unicode_ident::{is_xid_continue, is_xid_start};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SozcukTuru {
    Bosluk,
    SatirYorumu,
    BlokYorumu,
    Isim,
    /// `r#isim`
    HamIsim,
    /// `'a`, `'static`
    YasamSuresi,
    /// `"…"`, `b"…"`, `c"…"`
    Dizgi,
    /// `r"…"`, `r#"…"#`, `br"…"`, `cr"…"`
    HamDizgi,
    /// `'a'`, `b'a'`
    Karakter,
    Sayi,
    /// Tek karakterlik noktalama; `::` gibi birleşik işaretler ayrı sözcüklerdir.
    Isaret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sozcuk {
    pub tur: SozcukTuru,
    pub baslangic: usize,
    pub bitis: usize,
}

impl Sozcuk {
    pub fn metin<'a>(&self, kaynak: &'a str) -> &'a str {
        &kaynak[self.baslangic..self.bitis]
    }

    /// Boşluk ve yorumlar anlam taşımaz.
    pub fn anlamsiz_mi(&self) -> bool {
        matches!(self.tur, SozcukTuru::Bosluk | SozcukTuru::SatirYorumu | SozcukTuru::BlokYorumu)
    }
}

pub fn isim_baslangici_mi(c: char) -> bool {
    c == '_' || is_xid_start(c)
}

pub fn isim_devami_mi(c: char) -> bool {
    is_xid_continue(c)
}

pub fn isim_mi(metin: &str) -> bool {
    let mut chars = metin.chars();
    chars.next().is_some_and(isim_baslangici_mi) && chars.all(isim_devami_mi)
}

pub fn sozcuklere_ayir(kaynak: &str) -> Vec<Sozcuk> {
    let mut cozumleyici = Cozumleyici { kaynak, konum: 0 };
    let mut sozcukler = Vec::new();
    while cozumleyici.konum < kaynak.len() {
        let baslangic = cozumleyici.konum;
        let tur = cozumleyici.sonraki_tur();
        sozcukler.push(Sozcuk { tur, baslangic, bitis: cozumleyici.konum });
    }
    sozcukler
}

struct Cozumleyici<'a> {
    kaynak: &'a str,
    konum: usize,
}

impl Cozumleyici<'_> {
    fn kalan(&self) -> &str {
        &self.kaynak[self.konum..]
    }

    fn bak(&self) -> Option<char> {
        self.kalan().chars().next()
    }

    fn bak_n(&self, n: usize) -> Option<char> {
        self.kalan().chars().nth(n)
    }

    fn ilerle(&mut self) -> Option<char> {
        let c = self.bak()?;
        self.konum += c.len_utf8();
        Some(c)
    }

    fn surece_ilerle(&mut self, f: impl Fn(char) -> bool) {
        while self.bak().is_some_and(&f) {
            self.ilerle();
        }
    }

    fn sonraki_tur(&mut self) -> SozcukTuru {
        let kalan = self.kalan();
        let c = self.bak().expect("kaynağın sonuna gelinmedi");
        if c.is_whitespace() {
            self.surece_ilerle(char::is_whitespace);
            return SozcukTuru::Bosluk;
        }
        if kalan.starts_with("//") {
            self.surece_ilerle(|c| c != '\n');
            return SozcukTuru::SatirYorumu;
        }
        if kalan.starts_with("/*") {
            self.blok_yorumu();
            return SozcukTuru::BlokYorumu;
        }
        if let Some(tur) = self.onekli() {
            return tur;
        }
        if isim_baslangici_mi(c) {
            self.surece_ilerle(isim_devami_mi);
            return SozcukTuru::Isim;
        }
        match c {
            '"' => {
                self.ilerle();
                self.tirnakli('"');
                SozcukTuru::Dizgi
            }
            '\'' => self.karakter_ya_da_yasam_suresi(),
            '0'..='9' => {
                self.sayi();
                SozcukTuru::Sayi
            }
            _ => {
                self.ilerle();
                SozcukTuru::Isaret
            }
        }
    }

    /// Önekli sabitler (`r"…"`, `b'…'`, …) ve `r#isim`.
    fn onekli(&mut self) -> Option<SozcukTuru> {
        let kalan = self.kalan();
        for onek in ["br", "cr", "r"] {
            let Some(sonrasi) = kalan.strip_prefix(onek) else { continue };
            let diyez = sonrasi.len() - sonrasi.trim_start_matches('#').len();
            if sonrasi[diyez..].starts_with('"') {
                self.konum += onek.len() + diyez + 1;
                self.ham_dizgi_govdesi(diyez);
                return Some(SozcukTuru::HamDizgi);
            }
            if onek == "r" && diyez == 1 && sonrasi[1..].starts_with(isim_baslangici_mi) {
                self.konum += 2;
                self.surece_ilerle(isim_devami_mi);
                return Some(SozcukTuru::HamIsim);
            }
        }
        for onek in ["b", "c"] {
            let Some(sonrasi) = kalan.strip_prefix(onek) else { continue };
            if sonrasi.starts_with('"') {
                self.konum += 2;
                self.tirnakli('"');
                return Some(SozcukTuru::Dizgi);
            }
            if onek == "b" && sonrasi.starts_with('\'') {
                self.konum += 2;
                self.tirnakli('\'');
                return Some(SozcukTuru::Karakter);
            }
        }
        None
    }

    /// Açılış tırnağı tüketilmiş olmalı; kapanış tırnağını da tüketir.
    fn tirnakli(&mut self, tirnak: char) {
        while let Some(c) = self.ilerle() {
            if c == '\\' {
                self.ilerle();
            } else if c == tirnak {
                return;
            }
        }
    }

    fn ham_dizgi_govdesi(&mut self, diyez: usize) {
        let kapanis = format!("\"{}", "#".repeat(diyez));
        match self.kalan().find(&kapanis) {
            Some(i) => self.konum += i + kapanis.len(),
            None => self.konum = self.kaynak.len(),
        }
    }

    fn blok_yorumu(&mut self) {
        self.konum += 2;
        let mut derinlik = 1;
        while derinlik > 0 && self.konum < self.kaynak.len() {
            if self.kalan().starts_with("/*") {
                derinlik += 1;
                self.konum += 2;
            } else if self.kalan().starts_with("*/") {
                derinlik -= 1;
                self.konum += 2;
            } else {
                self.ilerle();
            }
        }
    }

    /// `'a'` bir karakter, `'a` (ardından `'` gelmeyen) bir yaşam süresidir.
    fn karakter_ya_da_yasam_suresi(&mut self) -> SozcukTuru {
        match (self.bak_n(1), self.bak_n(2)) {
            (Some('\\'), _) => {
                self.ilerle();
                self.tirnakli('\'');
                SozcukTuru::Karakter
            }
            (Some(c), Some('\'')) => {
                self.konum += 1 + c.len_utf8() + 1;
                SozcukTuru::Karakter
            }
            (Some(c), _) if isim_baslangici_mi(c) => {
                self.ilerle();
                self.surece_ilerle(isim_devami_mi);
                SozcukTuru::YasamSuresi
            }
            _ => {
                self.ilerle();
                SozcukTuru::Isaret
            }
        }
    }

    fn sayi(&mut self) {
        let onaltilik = self.kalan().starts_with("0x") || self.kalan().starts_with("0X");
        let mut onceki = self.ilerle().unwrap_or('0');
        while let Some(c) = self.bak() {
            let kabul = c.is_ascii_alphanumeric()
                || c == '_'
                || (c == '.' && self.bak_n(1).is_some_and(|n| n.is_ascii_digit()))
                || (matches!(c, '+' | '-') && !onaltilik && matches!(onceki, 'e' | 'E'));
            if !kabul {
                break;
            }
            onceki = c;
            self.ilerle();
        }
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use SozcukTuru::*;

    fn turler(kaynak: &str) -> Vec<(SozcukTuru, &str)> {
        sozcuklere_ayir(kaynak).into_iter().filter(|t| t.tur != Bosluk).map(|t| (t.tur, t.metin(kaynak))).collect()
    }

    #[test]
    fn her_bayti_kapsar() {
        let kaynak = "işlev ana() { yap ş = r#\"a\"#; } // yorum\n/* iç /* içe */ */ 'a' 'b";
        let sozcukler = sozcuklere_ayir(kaynak);
        let mut konum = 0;
        for t in &sozcukler {
            assert_eq!(t.baslangic, konum);
            konum = t.bitis;
        }
        assert_eq!(konum, kaynak.len());
    }

    #[test]
    fn turkce_isimler() {
        assert_eq!(turler("yap sayılar"), [(Isim, "yap"), (Isim, "sayılar")]);
        assert_eq!(turler("ğüşıöç_İ"), [(Isim, "ğüşıöç_İ")]);
    }

    #[test]
    fn dizgiler_sozcukleri_gizler() {
        assert_eq!(turler(r#""yap \" eğer""#), [(Dizgi, r#""yap \" eğer""#)]);
        assert_eq!(turler(r###"r##"a "# b"##"###), [(HamDizgi, r###"r##"a "# b"##"###)]);
        assert_eq!(turler(r#"b"x" br"y" c"z""#), [(Dizgi, r#"b"x""#), (HamDizgi, r#"br"y""#), (Dizgi, r#"c"z""#)]);
    }

    #[test]
    fn ham_isimler() {
        assert_eq!(turler("r#tür"), [(HamIsim, "r#tür")]);
        assert_eq!(turler("break"), [(Isim, "break")]);
    }

    #[test]
    fn karakterler_ve_yasam_sureleri() {
        assert_eq!(turler("'ş'"), [(Karakter, "'ş'")]);
        assert_eq!(turler(r"'\''"), [(Karakter, r"'\''")]);
        assert_eq!(turler("&'durağan"), [(Isaret, "&"), (YasamSuresi, "'durağan")]);
        assert_eq!(turler("b'a'"), [(Karakter, "b'a'")]);
    }

    #[test]
    fn yorumlar() {
        assert_eq!(turler("// yap\nx"), [(SatirYorumu, "// yap"), (Isim, "x")]);
        assert_eq!(turler("/* a /* b */ c */x"), [(BlokYorumu, "/* a /* b */ c */"), (Isim, "x")]);
    }

    #[test]
    fn sayilar() {
        assert_eq!(turler("1.5e-3"), [(Sayi, "1.5e-3")]);
        assert_eq!(turler("0..10"), [(Sayi, "0"), (Isaret, "."), (Isaret, "."), (Sayi, "10")]);
        assert_eq!(turler("0xe-1"), [(Sayi, "0xe"), (Isaret, "-"), (Sayi, "1")]);
        assert_eq!(turler("2.uzunluk"), [(Sayi, "2"), (Isaret, "."), (Isim, "uzunluk")]);
    }
}
