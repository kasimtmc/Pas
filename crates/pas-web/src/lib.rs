//! Pas oyun alanının tarayıcıda (WebAssembly) çalışan parçası.
//!
//! Tarayıcı Pas kaynağını buraya verir, çıkan Rust kodunu Rust oyun alanına
//! (play.rust-lang.org) gönderir ve gelen derleyici/program çıktısını yine
//! buraya verip Pas kaynağına eşlenmiş, Türkçe tanılamalar alır.
//!
//! Dışarıya açılan işlevler JSON alır ve JSON döndürür; böylece JavaScript
//! tarafında ek bir araca (wasm-bindgen) gerek kalmaz. Bkz. [`wasm`].

pub mod insan_ciktisi;
mod wasm;

use pas_cekirdek::mesajlar::{
    aciklama, calisma_mesajini_cevir, derleyici_mesajini_cevir, gereksiz_mi, oku_ipucu, panik_mesajini_cevir,
};
use pas_cekirdek::sozcukler::{SozcukTuru, sozcuklere_ayir};
use pas_cekirdek::sozluk::{self, GIRDILER, Sinif};
use pas_cekirdek::{Ceviri, SatirDizini, Secenekler, Yon, pasa_cevir, rusta_cevir_secenekle};
use serde_json::{Value, json};

use insan_ciktisi::{ANA_DOSYA, Tani};

/// Kullanıcıya gösterilen dosya adı.
const PAS_DOSYASI: &str = "ana.pas";

/// Hazır girdi bitince destek kodunun hata çıktısına yazdığı satır (bkz. `destek.rs`).
const GIRDI_BITTI: &str = "[pas: girdi bitti]";

/// Kaplumbağanın çizimi (SVG) hata çıktısına bu önekle tek satır olarak yazılır.
const CIZIM: &str = "[pas: çizim] ";

/// Oyun alanına gönderilecek Rust kodu. `girdi`, `oku!` ile okunacak metin;
/// `tohum`, `rastgele!` için. Program cevaplarla yeniden çalıştırılırken
/// aynı tohum verilirse rastgele sayılar değişmez.
pub fn derlenecek_kod(kaynak: &str, girdi: &str, tohum: u64) -> String {
    cevir(kaynak, girdi, tohum).rust
}

/// Öğrenmek için gösterilen sade Rust çevirisi (destek kodu olmadan).
pub fn sade_rust(kaynak: &str) -> String {
    pas_cekirdek::rusta_cevir(kaynak).rust
}

fn cevir(kaynak: &str, girdi: &str, tohum: u64) -> Ceviri {
    let secenekler = Secenekler {
        destek_ekle: true,
        turkce_cikti: true,
        hazir_girdi: Some(girdi),
        tohum: Some(tohum),
        ..Secenekler::default()
    };
    rusta_cevir_secenekle(kaynak, &secenekler)
}

/// Oyun alanından gelen sonucu (`stderr`) çözer: derleyici tanılamaları,
/// programın çöktüğü yer ve programın kendi hata çıktısı. `girdi` ve `tohum`
/// derlenen kodunkilerle aynı olmalı; konumlar ona göre eşlenir.
pub fn sonucu_coz(kaynak: &str, girdi: &str, tohum: u64, hata_ciktisi: &str) -> Value {
    let ceviri = cevir(kaynak, girdi, tohum);
    let esleyici = Esleyici { pas: kaynak, ceviri: &ceviri };

    // Cargo programı çalıştırmadan önce `Running` satırını yazar; öncesi derleme çıktısıdır.
    let (derleme, program) = match hata_ciktisi.find("     Running `") {
        Some(yer) => {
            let satir_sonu = hata_ciktisi[yer..].find('\n').map_or(hata_ciktisi.len(), |s| yer + s + 1);
            (&hata_ciktisi[..yer], Some(&hata_ciktisi[satir_sonu..]))
        }
        None => (hata_ciktisi, None),
    };

    let mut aciklananlar = Vec::new();
    let tanilamalar: Vec<Value> = insan_ciktisi::ayristir(derleme, &ceviri.rust)
        .iter()
        .filter(|tani| !gereksiz_mi(&tani.mesaj))
        .map(|tani| esleyici.tanilama(tani, &mut aciklananlar))
        .collect();

    let (panik, mut program_ciktisi, cizimler) = match program {
        Some(metin) => esleyici.program_ciktisi(metin),
        None => (Value::Null, String::new(), Vec::new()),
    };
    let girdi_bitti = program_ciktisi.contains(GIRDI_BITTI);
    if girdi_bitti {
        program_ciktisi = program_ciktisi.lines().filter(|s| *s != GIRDI_BITTI).map(|s| format!("{s}\n")).collect();
    }
    json!({
        "derlendi": program.is_some(),
        "girdi_bitti": girdi_bitti,
        "tanilamalar": tanilamalar,
        "panik": panik,
        "hata_ciktisi": program_ciktisi,
        "cizimler": cizimler,
    })
}

/// Derlenen Rust kaynağındaki konumları Pas kaynağına eşler.
struct Esleyici<'a> {
    pas: &'a str,
    ceviri: &'a Ceviri,
}

impl Esleyici<'_> {
    /// Rust'taki (satır, karakter sütunu) aralığının Pas'taki bayt aralığı.
    /// Aralık çevirmenin eklediği kodun içindeyse (destek kodu) `None`.
    fn pas_araligi(&self, satir: usize, baslangic: usize, bitis: usize) -> Option<(usize, usize)> {
        let dizin = SatirDizini::yeni(&self.ceviri.rust);
        let rust_baslangic = dizin.ofset(satir, baslangic)?;
        let rust_bitis = dizin.ofset(satir, bitis)?.max(rust_baslangic);
        let harita = &self.ceviri.harita;
        let (pas_baslangic, pas_bitis) = harita.rust_araligini_pasa(rust_baslangic, rust_bitis);
        if harita.eklenen_metinde_mi(rust_baslangic) && pas_baslangic == pas_bitis {
            return None;
        }
        Some((pas_baslangic, pas_bitis))
    }

    /// Pas'taki bayt aralığının JSON karşılığı: satır/sütun (1'den, karakter
    /// olarak) ve düzenleyici için UTF-16 konumları.
    fn aralik_json(&self, baslangic: usize, bitis: usize) -> Value {
        let dizin = SatirDizini::yeni(self.pas);
        let (satir, sutun) = dizin.satir_sutun(baslangic);
        let (bitis_satiri, bitis_sutunu) = dizin.satir_sutun(bitis);
        json!({
            "satir": satir,
            "sutun": sutun,
            "bitis_satiri": bitis_satiri,
            "bitis_sutunu": bitis_sutunu,
            "bas": utf16_konumu(self.pas, baslangic),
            "son": utf16_konumu(self.pas, bitis),
        })
    }

    fn cevir(&self, mesaj: &str) -> String {
        // Oyun alanındaki sandığın adı kullanıcının bildiği bir şey değil.
        derleyici_mesajini_cevir(mesaj).replace(ANA_DOSYA, PAS_DOSYASI).replace("`playground` sandığında ", "")
    }

    fn tanilama(&self, tani: &Tani, aciklananlar: &mut Vec<String>) -> Value {
        // Lint adları çocuklar için anlamsız; yalnızca hata kodları gösterilir.
        let kod = tani.kod.as_deref().filter(|kod| kod.starts_with('E'));
        let mut etiketler = Vec::new();
        let mut birincil = None;
        let mut ilk = None;
        for etiket in &tani.etiketler {
            let Some((baslangic, bitis)) = self.pas_araligi(etiket.satir, etiket.baslangic, etiket.bitis) else {
                continue;
            };
            let mesaj = etiket.mesaj.as_deref().map(|m| self.cevir(m));
            if etiket.birincil && birincil.is_none() {
                birincil = Some((baslangic, bitis, mesaj.clone()));
            }
            if ilk.is_none() {
                ilk = Some((baslangic, bitis, mesaj.clone()));
            }
            let mut deger = self.aralik_json(baslangic, bitis);
            deger["birincil"] = etiket.birincil.into();
            deger["mesaj"] = mesaj.into();
            etiketler.push(deger);
        }
        // Birincil işaret destek kodundaysa ilk işaret gösterilir.
        let birincil = birincil.or(ilk);

        let mut notlar = Vec::new();
        for not in &tani.notlar {
            if gereksiz_mi(&not.mesaj) {
                continue;
            }
            let oneri = not.oneri.as_ref().and_then(|(satir, metin)| match not.yeni_satir {
                true => self.yeni_satir(*satir, metin),
                false => self.oneri(*satir, metin),
            });
            notlar.push(json!({
                "seviye": seviye_sozcugu(&not.seviye),
                "mesaj": self.cevir(&not.mesaj),
                "oneri": oneri.map(|(satir, metin)| json!({ "satir": satir, "metin": metin, "ekle": not.yeni_satir })),
            }));
        }
        if let (Some(kod), Some((baslangic, _, mesaj))) = (kod, &birincil) {
            let dizin = SatirDizini::yeni(self.pas);
            let satir = dizin.satir(dizin.satir_sutun(*baslangic).0).unwrap_or("");
            if let Some(ipucu) = oku_ipucu(kod, satir, mesaj.as_deref().unwrap_or("")) {
                notlar.push(json!({ "seviye": "yardım", "mesaj": ipucu, "oneri": null }));
            }
        }

        // Her hata kodunun açıklaması bir kez gösterilir.
        let aciklama = kod.filter(|kod| !aciklananlar.iter().any(|a| a == kod)).and_then(aciklama);
        if let (Some(kod), Some(_)) = (kod, aciklama) {
            aciklananlar.push(kod.to_owned());
        }
        json!({
            "seviye": seviye_sozcugu(&tani.seviye),
            "kod": kod,
            "mesaj": self.cevir(&tani.mesaj),
            "aralik": birincil.map(|(baslangic, bitis, _)| self.aralik_json(baslangic, bitis)),
            "etiketler": etiketler,
            "notlar": notlar,
            "aciklama": aciklama,
        })
    }

    /// Rust'ta `satir`ın önüne eklenmesi önerilen satırın Pas'taki yeri ve hali.
    fn yeni_satir(&self, satir: usize, metin: &str) -> Option<(usize, String)> {
        let ofset = SatirDizini::yeni(&self.ceviri.rust).satir_basi(satir)?;
        let pas = self.ceviri.harita.rusttan_pasa(ofset, Yon::Baslangic);
        Some((SatirDizini::yeni(self.pas).satir_sutun(pas).0, pasa_cevir(metin.trim())))
    }

    /// Rust satırı için önerilen değişikliği Pas satırına uygular.
    fn oneri(&self, satir: usize, yeni: &str) -> Option<(usize, String)> {
        let eski = SatirDizini::yeni(&self.ceviri.rust).satir(satir)?.replace('\t', "    ");
        let eski: Vec<char> = eski.chars().collect();
        let yeni: Vec<char> = yeni.chars().collect();
        let onek = eski.iter().zip(&yeni).take_while(|(a, b)| a == b).count();
        let sonek = eski[onek..].iter().rev().zip(yeni[onek..].iter().rev()).take_while(|(a, b)| a == b).count();
        let eklenen: String = yeni[onek..yeni.len() - sonek].iter().collect();
        if eklenen.contains("pas_destek") {
            return None;
        }

        // Sekmeler açıldığı için sütunlar kaynaktaki karakterlere geri sayılır.
        let kaynak_satiri = SatirDizini::yeni(&self.ceviri.rust).satir(satir)?;
        let sutun = |acik: usize| {
            let mut ekran = 0;
            for (i, c) in kaynak_satiri.chars().enumerate() {
                if ekran >= acik {
                    return i + 1;
                }
                ekran += if c == '\t' { 4 } else { 1 };
            }
            kaynak_satiri.chars().count() + 1
        };
        let (baslangic, bitis) = self.pas_araligi(satir, sutun(onek), sutun(eski.len() - sonek))?;
        let dizin = SatirDizini::yeni(self.pas);
        let pas_satiri = dizin.satir_sutun(baslangic).0;
        if dizin.satir_sutun(bitis).0 != pas_satiri {
            return None;
        }
        let satir_basi = dizin.satir_basi(pas_satiri)?;
        let mut metin = dizin.satir(pas_satiri)?.to_owned();
        metin.replace_range(baslangic - satir_basi..bitis - satir_basi, &pasa_cevir(&eklenen));
        Some((pas_satiri, metin.replace('\t', "    ")))
    }

    /// Programın hata çıktısındaki panik mesajını Türkçeleştirip konumunu
    /// Pas kaynağına eşler. Kalan satırlar (programın kendi yazdıkları)
    /// olduğu gibi döner.
    fn program_ciktisi(&self, metin: &str) -> (Value, String, Vec<String>) {
        let mut panik = Value::Null;
        let mut diger = String::new();
        let mut cizimler = Vec::new();
        let mut satirlar = metin.lines();
        while let Some(satir) = satirlar.next() {
            if let Some(svg) = satir.strip_prefix(CIZIM) {
                cizimler.push(svg.to_owned());
                continue;
            }
            if let Some(ceviri) = calisma_mesajini_cevir(satir) {
                // Yığın taşması gibi panik olmayan çökmeler
                if !ceviri.is_empty() {
                    diger.push_str(&format!("hata: {ceviri}\n"));
                }
                continue;
            }
            let Some((is_parcacigi, satir_no, sutun)) = panik_basligi(satir) else {
                diger.push_str(satir);
                diger.push('\n');
                continue;
            };
            let mut mesaj = Vec::new();
            for satir in satirlar.by_ref() {
                if satir.starts_with("note: run with `RUST_BACKTRACE=1`") {
                    break;
                }
                match satir.strip_prefix(CIZIM) {
                    Some(svg) => cizimler.push(svg.to_owned()),
                    None => mesaj.push(panik_mesajini_cevir(satir)),
                }
            }
            let aralik = SatirDizini::yeni(&self.ceviri.rust).ofset(satir_no, sutun).map(|ofset| {
                let pas = self.ceviri.harita.rusttan_pasa(ofset, Yon::Baslangic);
                self.aralik_json(pas, pas)
            });
            panik = json!({
                "is_parcacigi": if is_parcacigi == "main" { "ana" } else { is_parcacigi },
                "mesaj": mesaj.join("\n"),
                "aralik": aralik,
            });
        }
        // Panik mesajının çevresindeki boş satırlar
        let diger = diger.trim_matches('\n');
        (panik, if diger.is_empty() { String::new() } else { format!("{diger}\n") }, cizimler)
    }
}

/// `thread 'main' (12) panicked at src/main.rs:5:34:` → ("main", 5, 34)
fn panik_basligi(satir: &str) -> Option<(&str, usize, usize)> {
    let kalan = satir.strip_prefix("thread '")?;
    let is_parcacigi = &kalan[..kalan.find('\'')?];
    let konum = kalan.split(" panicked at ").nth(1)?.strip_suffix(':')?;
    let mut parcalar = konum.rsplitn(3, ':');
    let sutun = parcalar.next()?.parse().ok()?;
    let satir_no = parcalar.next()?.parse().ok()?;
    (parcalar.next()? == ANA_DOSYA).then_some((is_parcacigi, satir_no, sutun))
}

fn seviye_sozcugu(seviye: &str) -> &str {
    match seviye {
        "error" => "hata",
        "warning" => "uyarı",
        "note" => "not",
        "help" => "yardım",
        diger => diger,
    }
}

fn utf16_konumu(metin: &str, bayt: usize) -> usize {
    metin[..bayt.min(metin.len())].encode_utf16().count()
}

/// Renklendirme için sözcükler: `[başlangıç, bitiş, sınıf]` (UTF-16 konumları).
pub fn renklendir(kaynak: &str) -> Value {
    let sozcukler = sozcuklere_ayir(kaynak);
    let anlamlilar: Vec<usize> = (0..sozcukler.len()).filter(|&i| !sozcukler[i].anlamsiz_mi()).collect();
    let anlamli_metin = |k: usize| anlamlilar.get(k).map(|&i| sozcukler[i].metin(kaynak));
    let mut sonuc = Vec::new();
    let mut utf16 = 0;
    let mut sira: usize = 0;
    for sozcuk in &sozcukler {
        let metin = sozcuk.metin(kaynak);
        let baslangic = utf16;
        utf16 += metin.encode_utf16().count();
        let sinif = match sozcuk.tur {
            SozcukTuru::Bosluk => continue,
            SozcukTuru::SatirYorumu | SozcukTuru::BlokYorumu => Some("yorum"),
            tur => {
                sira += 1;
                match tur {
                    SozcukTuru::Isim => {
                        isim_sinifi(metin, sira.checked_sub(2).and_then(anlamli_metin), anlamli_metin(sira))
                    }
                    SozcukTuru::Dizgi | SozcukTuru::HamDizgi | SozcukTuru::Karakter => Some("dizgi"),
                    SozcukTuru::Sayi => Some("sayi"),
                    SozcukTuru::YasamSuresi => Some("omur"),
                    _ => None,
                }
            }
        };
        if let Some(sinif) = sinif {
            sonuc.push(json!([baslangic, utf16, sinif]));
        }
    }
    Value::Array(sonuc)
}

fn isim_sinifi(metin: &str, onceki: Option<&str>, sonraki: Option<&str>) -> Option<&'static str> {
    let girdi = sozluk::pas_ile(metin).or_else(|| sozluk::rust_ile(metin));
    let sinif = girdi.map(|g| g.tur);
    match sinif {
        Some(Sinif::AnahtarSozcuk) => Some("anahtar_sozcuk"),
        Some(Sinif::Tur) => Some("tur"),
        Some(Sinif::Deger) => Some("deger"),
        Some(Sinif::Makro) if sonraki == Some("!") => Some("makro"),
        Some(Sinif::Islev) if onceki == Some(":") => Some("islev"),
        Some(Sinif::Yontem) if onceki == Some(".") => Some("yontem"),
        _ if sonraki == Some("!") => Some("makro"),
        _ if metin.starts_with(|c: char| c.is_uppercase()) => Some("tur"),
        _ if sonraki == Some("(") => Some("islev"),
        _ => None,
    }
}

/// Sözlüğün tamamı (yardım paneli için).
pub fn sozluk_json() -> Value {
    GIRDILER
        .iter()
        .map(|g| {
            let (pas, rust) = match g.tur {
                Sinif::Makro => (format!("{}!", g.pas), format!("{}!", g.rust)),
                _ => (g.pas.to_owned(), g.rust.to_owned()),
            };
            json!({ "pas": pas, "rust": rust, "tur": g.tur.kimlik(), "tur_adi": g.tur.ad(), "aciklama": g.aciklama })
        })
        .collect()
}

#[cfg(test)]
mod gorev_testleri;
#[cfg(test)]
mod testler;
