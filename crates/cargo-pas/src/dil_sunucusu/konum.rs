//! Konum dönüşümleri ve dosya URI'leri.
//!
//! Editör (LSP istemcisi) sütunları UTF-16 birimiyle sayar; rust-analyzer ile
//! bayt (UTF-8) sütunlarında anlaşılır. Kaynak haritası da bayt ofsetleriyle
//! çalıştığı için dönüşümler hep bayt ofseti üzerinden yapılır.

use std::path::{Path, PathBuf};

/// Bir metindeki satır başları.
pub struct Metin<'a> {
    metin: &'a str,
    satir_baslari: Vec<usize>,
}

impl<'a> Metin<'a> {
    pub fn yeni(metin: &'a str) -> Self {
        let satir_baslari = std::iter::once(0).chain(metin.match_indices('\n').map(|(i, _)| i + 1)).collect();
        Metin { metin, satir_baslari }
    }

    fn satir(&self, satir: usize) -> (usize, &'a str) {
        let satir = satir.min(self.satir_baslari.len() - 1);
        let baslangic = self.satir_baslari[satir];
        let bitis = self.satir_baslari.get(satir + 1).map_or(self.metin.len(), |&s| s - 1);
        (baslangic, &self.metin[baslangic..bitis])
    }

    /// UTF-16 (satır, sütun) → bayt ofseti
    pub fn utf16_ofseti(&self, satir: u64, sutun: u64) -> usize {
        let (baslangic, metin) = self.satir(satir as usize);
        let mut sayac = 0;
        for (i, c) in metin.char_indices() {
            if sayac >= sutun {
                return baslangic + i;
            }
            sayac += c.len_utf16() as u64;
        }
        baslangic + metin.len()
    }

    /// Bayt ofseti → UTF-16 (satır, sütun)
    pub fn utf16_konumu(&self, ofset: usize) -> (u64, u64) {
        let (satir, baslangic) = self.satir_bul(ofset);
        let sutun = self.metin[baslangic..ofset].chars().map(|c| c.len_utf16() as u64).sum();
        (satir as u64, sutun)
    }

    /// UTF-8 (satır, bayt sütunu) → bayt ofseti
    pub fn utf8_ofseti(&self, satir: u64, sutun: u64) -> usize {
        let (baslangic, metin) = self.satir(satir as usize);
        let mut ofset = baslangic + (sutun as usize).min(metin.len());
        while !self.metin.is_char_boundary(ofset) {
            ofset -= 1;
        }
        ofset
    }

    /// Bayt ofseti → UTF-8 (satır, bayt sütunu)
    pub fn utf8_konumu(&self, ofset: usize) -> (u64, u64) {
        let (satir, baslangic) = self.satir_bul(ofset);
        (satir as u64, (ofset - baslangic) as u64)
    }

    fn satir_bul(&self, ofset: usize) -> (usize, usize) {
        let mut ofset = ofset.min(self.metin.len());
        while !self.metin.is_char_boundary(ofset) {
            ofset -= 1;
        }
        let satir = self.satir_baslari.partition_point(|&s| s <= ofset) - 1;
        (satir, self.satir_baslari[satir])
    }
}

/// `file://` URI'sini dosya yoluna çevirir.
pub fn uri_yolu(uri: &str) -> Option<PathBuf> {
    let yol = yuzde_coz(uri.strip_prefix("file://")?);
    // Windows: `file:///C:/…`; rust-analyzer sürücü harfini küçük gönderir
    // (`file:///c%3A/…`), editör büyük. Yollar karşılaştırılabilsin diye büyütülür.
    let yol = match yol.as_bytes() {
        [b'/', harf, b':', ..] if harf.is_ascii_alphabetic() => {
            format!("{}{}", (*harf as char).to_ascii_uppercase(), &yol[2..])
        }
        _ => yol,
    };
    Some(PathBuf::from(yol))
}

/// Dosya yolunu `file://` URI'sine çevirir.
pub fn yol_uri(yol: &Path) -> String {
    let yol = yol.to_string_lossy().replace('\\', "/");
    let on = if yol.starts_with('/') { "file://" } else { "file:///" };
    format!("{on}{}", yuzde_kodla(&yol))
}

fn yuzde_coz(metin: &str) -> String {
    let baytlar = metin.as_bytes();
    let mut sonuc = Vec::with_capacity(baytlar.len());
    let mut i = 0;
    while i < baytlar.len() {
        if baytlar[i] == b'%'
            && let Some(deger) = metin.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(h, 16).ok())
        {
            sonuc.push(deger);
            i += 3;
        } else {
            sonuc.push(baytlar[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&sonuc).into_owned()
}

fn yuzde_kodla(metin: &str) -> String {
    let mut sonuc = String::with_capacity(metin.len());
    for &b in metin.as_bytes() {
        if b.is_ascii_alphanumeric() || b"-._~/:".contains(&b) {
            sonuc.push(b as char);
        } else {
            sonuc.push_str(&format!("%{b:02X}"));
        }
    }
    sonuc
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn utf16_ve_utf8() {
        // "ş" 2 bayt, 1 UTF-16 birimi; "😀" 4 bayt, 2 UTF-16 birimi.
        let metin = Metin::yeni("yap ş = 1;\nyap 😀x = 2;");
        let x = "yap ş = 1;\nyap 😀".len();
        assert_eq!(metin.utf16_ofseti(1, 6), x);
        assert_eq!(metin.utf16_konumu(x), (1, 6));
        assert_eq!(metin.utf8_konumu(x), (1, 8));
        assert_eq!(metin.utf8_ofseti(1, 8), x);
        assert_eq!(metin.utf16_ofseti(0, 99), "yap ş = 1;".len());
        // Karakterin ortasına düşen bayt sütunu karakter başına çekilir.
        assert_eq!(metin.utf8_ofseti(0, 5), 4);
    }

    #[test]
    fn uriler() {
        let yol = Path::new("/ev/kasım/şekiller.rs");
        let uri = yol_uri(yol);
        assert_eq!(uri, "file:///ev/kas%C4%B1m/%C5%9Fekiller.rs");
        assert_eq!(uri_yolu(&uri).unwrap(), yol);
        assert_eq!(uri_yolu("file:///ev/kasım/a.pas").unwrap(), Path::new("/ev/kasım/a.pas"));
        assert_eq!(uri_yolu("file:///C:/Kullanıcılar/a.pas").unwrap(), Path::new("C:/Kullanıcılar/a.pas"));
        assert_eq!(uri_yolu("file:///c%3A/Kullan%C4%B1c%C4%B1lar/a.pas").unwrap(), Path::new("C:/Kullanıcılar/a.pas"));
        assert_eq!(uri_yolu("https://ornek.com"), None);
    }
}
