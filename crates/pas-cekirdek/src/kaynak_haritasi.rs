//! Pas kaynağındaki konumlar ile üretilen Rust kodundaki konumlar
//! arasındaki eşleme.
//!
//! Çevirici yalnızca tek tek sözcükleri değiştirir ve satır eklemez; bu
//! yüzden eşleme, değişen her parça için bir [`Parca`] tutmaktan ibarettir.
//! İki parça arasındaki her şey birebir aynıdır.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parca {
    pub pas_baslangic: usize,
    pub pas_uzunluk: usize,
    pub rust_baslangic: usize,
    pub rust_uzunluk: usize,
}

/// Bir konum değişen bir parçanın ortasına düşerse nereye yuvarlanacağı.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yon {
    /// Bir aralığın başlangıcı: parçanın başına.
    Baslangic,
    /// Bir aralığın sonu: parçanın sonuna.
    Bitis,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KaynakHaritasi {
    parcalar: Vec<Parca>,
}

impl KaynakHaritasi {
    /// Parçalar kaynakta sıralı ve çakışmasız eklenmelidir.
    pub(crate) fn ekle(&mut self, parca: Parca) {
        debug_assert!(self.parcalar.last().is_none_or(|last| {
            last.pas_baslangic + last.pas_uzunluk <= parca.pas_baslangic
                && last.rust_baslangic + last.rust_uzunluk <= parca.rust_baslangic
        }));
        self.parcalar.push(parca);
    }

    pub fn parcalar(&self) -> &[Parca] {
        &self.parcalar
    }

    pub fn rusttan_pasa(&self, ofset: usize, yon: Yon) -> usize {
        esle(&self.parcalar, ofset, yon, |s| (s.rust_baslangic, s.rust_uzunluk), |s| (s.pas_baslangic, s.pas_uzunluk))
    }

    pub fn pastan_rusta(&self, ofset: usize, yon: Yon) -> usize {
        esle(&self.parcalar, ofset, yon, |s| (s.pas_baslangic, s.pas_uzunluk), |s| (s.rust_baslangic, s.rust_uzunluk))
    }

    /// Rust kodundaki bir aralığı Pas kaynağındaki karşılığına çevirir.
    pub fn rust_araligini_pasa(&self, baslangic: usize, bitis: usize) -> (usize, usize) {
        let baslangic = self.rusttan_pasa(baslangic, Yon::Baslangic);
        (baslangic, self.rusttan_pasa(bitis, Yon::Bitis).max(baslangic))
    }
}

fn esle(
    parcalar: &[Parca],
    ofset: usize,
    yon: Yon,
    nereden: impl Fn(&Parca) -> (usize, usize),
    nereye: impl Fn(&Parca) -> (usize, usize),
) -> usize {
    let sira = parcalar.partition_point(|s| nereden(s).0 <= ofset);
    let Some(parca) = sira.checked_sub(1).map(|i| &parcalar[i]) else {
        return ofset;
    };
    let (nereden_baslangic, nereden_uzunluk) = nereden(parca);
    let (nereye_baslangic, nereye_uzunluk) = nereye(parca);
    if ofset == nereden_baslangic {
        nereye_baslangic
    } else if ofset < nereden_baslangic + nereden_uzunluk {
        match yon {
            Yon::Baslangic => nereye_baslangic,
            Yon::Bitis => nereye_baslangic + nereye_uzunluk,
        }
    } else {
        nereye_baslangic + nereye_uzunluk + (ofset - nereden_baslangic - nereden_uzunluk)
    }
}

/// Bayt konumları ile (satır, sütun) çiftleri arasında dönüşüm yapar.
/// Satır ve sütunlar 1'den başlar; sütunlar karakter sayısıdır (rustc gibi).
pub struct SatirDizini<'a> {
    metin: &'a str,
    satir_baslari: Vec<usize>,
}

impl<'a> SatirDizini<'a> {
    pub fn yeni(metin: &'a str) -> Self {
        let satir_baslari = std::iter::once(0).chain(metin.match_indices('\n').map(|(i, _)| i + 1)).collect();
        SatirDizini { metin, satir_baslari }
    }

    pub fn satir_sayisi(&self) -> usize {
        self.satir_baslari.len()
    }

    pub fn satir_sutun(&self, ofset: usize) -> (usize, usize) {
        let ofset = karakter_sinirina_indir(self.metin, ofset.min(self.metin.len()));
        let satir = self.satir_baslari.partition_point(|&s| s <= ofset) - 1;
        let sutun = self.metin[self.satir_baslari[satir]..ofset].chars().count();
        (satir + 1, sutun + 1)
    }

    pub fn ofset(&self, satir: usize, sutun: usize) -> Option<usize> {
        let baslangic = *self.satir_baslari.get(satir.checked_sub(1)?)?;
        let metin = self.satir(satir)?;
        let atla = sutun.checked_sub(1)?;
        match metin.char_indices().nth(atla) {
            Some((i, _)) => Some(baslangic + i),
            None => Some(baslangic + metin.len()),
        }
    }

    /// Satır sonu karakterleri olmadan bir satırın metni.
    pub fn satir(&self, satir: usize) -> Option<&'a str> {
        let baslangic = *self.satir_baslari.get(satir.checked_sub(1)?)?;
        let bitis = self.satir_baslari.get(satir).map_or(self.metin.len(), |&next| next - 1);
        Some(self.metin[baslangic..bitis].trim_end_matches('\r'))
    }

    pub fn satir_basi(&self, satir: usize) -> Option<usize> {
        self.satir_baslari.get(satir.checked_sub(1)?).copied()
    }
}

fn karakter_sinirina_indir(metin: &str, mut ofset: usize) -> usize {
    while !metin.is_char_boundary(ofset) {
        ofset -= 1;
    }
    ofset
}

#[cfg(test)]
mod testler {
    use super::*;

    fn ornek() -> KaynakHaritasi {
        // "yap x" (pas) -> "let x" (rust); "#[p] " eklemesi ve ardından "genel" -> "pub"
        let mut esle = KaynakHaritasi::default();
        esle.ekle(Parca { pas_baslangic: 0, pas_uzunluk: 6, rust_baslangic: 0, rust_uzunluk: 3 });
        esle.ekle(Parca { pas_baslangic: 10, pas_uzunluk: 0, rust_baslangic: 7, rust_uzunluk: 5 });
        esle.ekle(Parca { pas_baslangic: 10, pas_uzunluk: 5, rust_baslangic: 12, rust_uzunluk: 3 });
        esle
    }

    #[test]
    fn degismeyen_metni_esler() {
        let esle = ornek();
        assert_eq!(esle.rusttan_pasa(4, Yon::Baslangic), 7);
        assert_eq!(esle.pastan_rusta(7, Yon::Baslangic), 4);
        assert_eq!(esle.rusttan_pasa(16, Yon::Baslangic), 16);
    }

    #[test]
    fn degisen_sozcukleri_esler() {
        let esle = ornek();
        assert_eq!(esle.rust_araligini_pasa(0, 3), (0, 6));
        assert_eq!(esle.rusttan_pasa(1, Yon::Baslangic), 0);
        assert_eq!(esle.rusttan_pasa(1, Yon::Bitis), 6);
        // Eklenen metin, kaynağında sıfır uzunluklu bir noktaya düşer.
        assert_eq!(esle.rust_araligini_pasa(7, 12), (10, 10));
        assert_eq!(esle.rust_araligini_pasa(12, 15), (10, 15));
    }

    #[test]
    fn satir_dizini() {
        let metin = "işlev ana() {\r\n    yazdır!(\"ş\");\n}";
        let sira = SatirDizini::yeni(metin);
        assert_eq!(sira.satir_sayisi(), 3);
        assert_eq!(sira.satir(1), Some("işlev ana() {"));
        let ofs = metin.find("yazdır").unwrap();
        assert_eq!(sira.satir_sutun(ofs), (2, 5));
        assert_eq!(sira.ofset(2, 5), Some(ofs));
        assert_eq!(sira.satir_sutun(metin.len()), (3, 2));
    }
}
