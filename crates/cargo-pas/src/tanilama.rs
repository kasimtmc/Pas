//! rustc tanılamalarını (JSON) Pas kaynağına eşleyip Türkçe gösterir.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use pas_cekirdek::mesajlar::{aciklama, derleyici_mesajini_cevir, gereksiz_mi};
use pas_cekirdek::{KaynakHaritasi, SatirDizini, pasa_cevir};
use serde::Deserialize;

use crate::gorunum::Gorunum;
use crate::proje::{Hazirlik, egik_cizgili_yol};

/// rustc'nin JSON tanılaması; alan adları rustc'ninkilerdir.
#[derive(Debug, Deserialize)]
pub struct Tani {
    #[serde(rename = "message")]
    pub mesaj: String,
    #[serde(rename = "code")]
    pub kod: Option<Kod>,
    #[serde(rename = "level")]
    pub seviye: String,
    #[serde(default, rename = "spans")]
    pub araliklar: Vec<Aralik>,
    #[serde(default, rename = "children")]
    pub alt_tanilar: Vec<Tani>,
}

#[derive(Debug, Deserialize)]
pub struct Kod {
    #[serde(rename = "code")]
    pub kod: String,
}

#[derive(Debug, Deserialize)]
pub struct Aralik {
    #[serde(rename = "file_name")]
    pub dosya_adi: String,
    #[serde(rename = "byte_start")]
    pub bayt_baslangici: usize,
    #[serde(rename = "byte_end")]
    pub bayt_bitisi: usize,
    #[serde(rename = "is_primary")]
    pub birincil: bool,
    #[serde(rename = "label")]
    pub etiket: Option<String>,
    #[serde(rename = "suggested_replacement")]
    pub onerilen: Option<String>,
    #[serde(rename = "expansion")]
    pub genisleme: Option<Box<Genisleme>>,
}

#[derive(Debug, Deserialize)]
pub struct Genisleme {
    #[serde(rename = "span")]
    pub aralik: Aralik,
}

/// Kullanıcıya gösterilecek bir kaynak dosya.
pub struct KaynakDosya {
    /// Kullanıcının gördüğü yol, örneğin `kaynak/ana.pas`.
    pub gosterilen: String,
    /// Kullanıcının gördüğü metin (Pas kaynağı ya da kopyalanmış `.rs`).
    pub metin: String,
    /// Derlenen Rust metni.
    pub rust_metni: String,
    pub harita: Option<KaynakHaritasi>,
}

impl KaynakDosya {
    /// Derlenen koddaki bir aralığın kullanıcının metnindeki karşılığı.
    pub fn gosterilene(&self, baslangic: usize, bitis: usize) -> (usize, usize) {
        match &self.harita {
            Some(harita) => harita.rust_araligini_pasa(baslangic, bitis),
            None => (baslangic, bitis),
        }
    }
}

/// Derleyicinin andığı dosyaları bulur.
pub struct Kaynaklar {
    golge: std::path::PathBuf,
    dosyalar: HashMap<String, Option<Rc<KaynakDosya>>>,
    /// Mesaj metinlerindeki `kaynak/main.rs` → `kaynak/ana.pas` düzeltmeleri.
    yeniden_adlar: Vec<(String, String)>,
}

impl Kaynaklar {
    pub fn yeni(golge: &Path, hazirlik: &Hazirlik) -> Self {
        let mut dosyalar = HashMap::new();
        let mut yeniden_adlar = Vec::new();
        for dosya in &hazirlik.dosyalar {
            dosyalar.insert(
                dosya.rust_yolu.clone(),
                Some(Rc::new(KaynakDosya {
                    gosterilen: dosya.pas_yolu.clone(),
                    metin: dosya.pas_kaynagi.clone(),
                    rust_metni: dosya.rust_kaynagi.clone(),
                    harita: Some(dosya.harita.clone()),
                })),
            );
            yeniden_adlar.push((dosya.rust_yolu.clone(), dosya.pas_yolu.clone()));
        }
        Kaynaklar { golge: golge.to_path_buf(), dosyalar, yeniden_adlar }
    }

    pub fn al(&mut self, dosya_adi: &str) -> Option<Rc<KaynakDosya>> {
        let anahtar = match Path::new(dosya_adi).strip_prefix(&self.golge) {
            Ok(goreli) => egik_cizgili_yol(goreli),
            Err(_) => dosya_adi.replace('\\', "/"),
        };
        if let Some(dosya) = self.dosyalar.get(&anahtar) {
            return dosya.clone();
        }
        // Gölge projeye olduğu gibi kopyalanmış bir `.rs` dosyası olabilir.
        let yol = self.golge.join(&anahtar);
        let dosya = (Path::new(&anahtar).is_relative() && yol.is_file())
            .then(|| fs::read_to_string(&yol).ok())
            .flatten()
            .map(|metin| {
                Rc::new(KaynakDosya { gosterilen: anahtar.clone(), rust_metni: metin.clone(), metin, harita: None })
            });
        self.dosyalar.insert(anahtar, dosya.clone());
        dosya
    }

    pub fn yollari_duzelt(&self, mesaj: &str) -> String {
        let mut mesaj = mesaj.to_owned();
        for (rust, pas) in &self.yeniden_adlar {
            mesaj = mesaj.replace(rust.as_str(), pas);
        }
        mesaj
    }
}

/// Bir kaynak dosyada işaretlenecek bir aralık.
struct Vurgu {
    dosya: Rc<KaynakDosya>,
    baslangic: usize,
    bitis: usize,
    birincil: bool,
    etiket: Option<String>,
}

pub struct Gosterici {
    gorunum: Gorunum,
    pub kaynaklar: Kaynaklar,
    aciklananlar: HashSet<String>,
    /// Tanılamaları IDE'ler için JSON satırları olarak standart çıktıya yaz.
    json: bool,
}

impl Gosterici {
    pub fn yeni(gorunum: Gorunum, kaynaklar: Kaynaklar, json: bool) -> Self {
        Gosterici { gorunum, kaynaklar, aciklananlar: HashSet::new(), json }
    }

    fn cevir(&self, mesaj: &str) -> String {
        self.kaynaklar.yollari_duzelt(&derleyici_mesajini_cevir(mesaj))
    }

    pub fn goster(&mut self, tani: &Tani) {
        if tani.seviye == "failure-note" || gereksiz_mi(&tani.mesaj) {
            return;
        }
        let (kelime, boya): (&str, fn(&Gorunum, &str) -> String) = match tani.seviye.as_str() {
            "error" => ("hata", Gorunum::hata),
            "warning" => ("uyarı", Gorunum::uyari),
            "error: internal compiler error" => ("derleyici iç hatası", Gorunum::hata),
            diger => (seviye_sozcugu(diger), Gorunum::not),
        };

        let mut cikti = String::new();
        // Lint adları (`dead_code`) çocuklar için anlamsız; yalnızca hata kodlarını göster.
        let kod = tani.kod.as_ref().map(|c| c.kod.as_str()).filter(|c| c.starts_with('E'));
        let baslik = match kod {
            Some(kod) => format!("{kelime}[{kod}]"),
            None => kelime.to_owned(),
        };
        let mesaj = self.cevir(&tani.mesaj);
        let _ = writeln!(cikti, "{}{}", boya(&self.gorunum, &baslik), self.gorunum.kalin(&format!(": {mesaj}")));

        let vurgular: Vec<Vurgu> = tani.araliklar.iter().filter_map(|aralik| self.vurgula(aralik)).collect();
        if self.json {
            self.json_goster(tani, kod, &mesaj, &vurgular);
            return;
        }
        let genislik = self.kenar_genisligi(tani, &vurgular);
        let dolgu = " ".repeat(genislik);
        self.kesit(&mut cikti, &vurgular, genislik, boya);

        for alt_tani in &tani.alt_tanilar {
            if gereksiz_mi(&alt_tani.mesaj) {
                continue;
            }
            let kelime = seviye_sozcugu(&alt_tani.seviye);
            let mesaj = self.cevir(&alt_tani.mesaj);
            let oneri = self.oneri(alt_tani);
            let alt_vurgular: Vec<Vurgu> = if oneri.is_none() {
                alt_tani.araliklar.iter().filter_map(|aralik| self.vurgula(aralik)).collect()
            } else {
                Vec::new()
            };
            if alt_vurgular.is_empty() {
                let _ = writeln!(cikti, "{dolgu} {} {}: {mesaj}", self.gorunum.kenar("="), self.gorunum.kalin(kelime));
            } else {
                let _ = writeln!(cikti, "{}: {mesaj}", self.gorunum.kalin(kelime));
                self.kesit(&mut cikti, &alt_vurgular, genislik, Gorunum::not);
            }
            if let Some((satir, metin)) = oneri {
                let _ = writeln!(cikti, "{dolgu} {}", self.gorunum.kenar("|"));
                let _ = writeln!(
                    cikti,
                    "{} {}",
                    self.gorunum.kenar(&format!("{satir:>genislik$} |")),
                    self.gorunum.basari(&metin)
                );
            }
        }

        if let Some(kod) = kod
            && let Some(metin) = aciklama(kod)
            && self.aciklananlar.insert(kod.to_owned())
        {
            let _ = writeln!(cikti, "{dolgu} {} {}: {metin}", self.gorunum.kenar("="), self.gorunum.not("açıklama"));
        }
        eprintln!("{cikti}");
    }

    /// Bir tanılamayı tek satırlık JSON olarak yazar. Konumlar 1'den başlayan
    /// satır ve (karakter olarak) sütun numaralarıdır.
    fn json_goster(&mut self, tani: &Tani, kod: Option<&str>, mesaj: &str, vurgular: &[Vurgu]) {
        let Some(birincil) = vurgular.iter().find(|m| m.birincil).or(vurgular.first()) else { return };
        let aralik = |vurgu: &Vurgu| {
            let dizin = SatirDizini::yeni(&vurgu.dosya.metin);
            let (satir, sutun) = dizin.satir_sutun(vurgu.baslangic);
            let (bitis_satiri, bitis_sutunu) = dizin.satir_sutun(vurgu.bitis);
            serde_json::json!({ "satir": satir, "sutun": sutun, "bitis_satiri": bitis_satiri, "bitis_sutunu": bitis_sutunu })
        };
        let etiketler: Vec<serde_json::Value> = vurgular
            .iter()
            .filter(|m| Rc::ptr_eq(&m.dosya, &birincil.dosya))
            .filter_map(|m| {
                let mut deger = aralik(m);
                deger["mesaj"] = m.etiket.clone()?.into();
                deger["birincil"] = m.birincil.into();
                Some(deger)
            })
            .collect();
        let mut notlar = Vec::new();
        for alt_tani in &tani.alt_tanilar {
            if gereksiz_mi(&alt_tani.mesaj) {
                continue;
            }
            let mut not = format!("{}: {}", seviye_sozcugu(&alt_tani.seviye), self.cevir(&alt_tani.mesaj));
            if let Some((_, metin)) = self.oneri(alt_tani) {
                not.push_str(&format!("\n    {}", metin.trim()));
            }
            notlar.push(not);
        }
        let deger = serde_json::json!({
            "seviye": seviye_sozcugu(&tani.seviye),
            "kod": kod,
            "mesaj": mesaj,
            "dosya": birincil.dosya.gosterilen,
            "aralik": aralik(birincil),
            "etiketler": etiketler,
            "notlar": notlar,
            "aciklama": kod.and_then(aciklama),
        });
        println!("{deger}");
    }

    /// Bir aralığı kullanıcının dosyasında bulur. Makro içindeyse makronun
    /// çağrıldığı yere çıkar.
    fn vurgula(&mut self, aralik: &Aralik) -> Option<Vurgu> {
        match self.kaynaklar.al(&aralik.dosya_adi) {
            Some(dosya) => {
                let (baslangic, bitis) = dosya.gosterilene(aralik.bayt_baslangici, aralik.bayt_bitisi);
                Some(Vurgu {
                    dosya,
                    baslangic,
                    bitis,
                    birincil: aralik.birincil,
                    etiket: aralik.etiket.as_deref().map(|l| self.cevir(l)),
                })
            }
            None => {
                let mut vurgu = self.vurgula(&aralik.genisleme.as_ref()?.aralik)?;
                vurgu.birincil = aralik.birincil;
                vurgu.etiket = aralik.etiket.as_deref().map(|l| self.cevir(l));
                Some(vurgu)
            }
        }
    }

    fn kenar_genisligi(&mut self, tani: &Tani, vurgular: &[Vurgu]) -> usize {
        let mut en_buyuk_satir =
            vurgular.iter().map(|m| SatirDizini::yeni(&m.dosya.metin).satir_sutun(m.bitis).0).max().unwrap_or(1);
        for alt_tani in &tani.alt_tanilar {
            for aralik in &alt_tani.araliklar {
                if let Some(m) = self.vurgula(aralik) {
                    en_buyuk_satir = en_buyuk_satir.max(SatirDizini::yeni(&m.dosya.metin).satir_sutun(m.bitis).0);
                }
            }
        }
        en_buyuk_satir.to_string().len()
    }

    fn kesit(&self, cikti: &mut String, vurgular: &[Vurgu], genislik: usize, boya: fn(&Gorunum, &str) -> String) {
        if vurgular.is_empty() {
            return;
        }
        let dolgu = " ".repeat(genislik);
        let gorunum = &self.gorunum;
        let birincil = vurgular.iter().find(|m| m.birincil).unwrap_or(&vurgular[0]);

        // Dosyalara göre grupla; birincil işaretin dosyası önce.
        let mut dosyalar: Vec<&Rc<KaynakDosya>> = vec![&birincil.dosya];
        for vurgu in vurgular {
            if !dosyalar.iter().any(|f| Rc::ptr_eq(f, &vurgu.dosya)) {
                dosyalar.push(&vurgu.dosya);
            }
        }
        for (i, dosya) in dosyalar.into_iter().enumerate() {
            let dizin = SatirDizini::yeni(&dosya.metin);
            let bu_dosyadakiler = vurgular.iter().filter(|m| Rc::ptr_eq(&m.dosya, dosya));
            let capa = if i == 0 { birincil } else { bu_dosyadakiler.clone().next().expect("en az bir işaret") };
            let (satir, sutun) = dizin.satir_sutun(capa.baslangic);
            let ok_isareti = if i == 0 { "-->" } else { ":::" };
            let _ = writeln!(cikti, "{dolgu}{} {}:{satir}:{sutun}", gorunum.kenar(ok_isareti), dosya.gosterilen);
            let _ = writeln!(cikti, "{dolgu} {}", gorunum.kenar("|"));

            let mut satirlar: BTreeMap<usize, Vec<&Vurgu>> = BTreeMap::new();
            for vurgu in bu_dosyadakiler {
                satirlar.entry(dizin.satir_sutun(vurgu.baslangic).0).or_default().push(vurgu);
            }
            let mut onceki: Option<usize> = None;
            for (satir, mut satir_vurgulari) in satirlar {
                satir_vurgulari.sort_by_key(|m| m.baslangic);
                if onceki.is_some_and(|p| satir > p + 1) {
                    let _ = writeln!(cikti, "{}", gorunum.kenar("..."));
                }
                onceki = Some(satir);
                let metin = dizin.satir(satir).unwrap_or("").replace('\t', " ");
                let _ = writeln!(cikti, "{} {metin}", gorunum.kenar(&format!("{satir:>genislik$} |")));
                for vurgu in satir_vurgulari {
                    let (_, baslangic_sutunu) = dizin.satir_sutun(vurgu.baslangic);
                    let (bitis_satiri, bitis_sutunu) = dizin.satir_sutun(vurgu.bitis);
                    let bitis_sutunu = if bitis_satiri == satir { bitis_sutunu } else { metin.chars().count() + 1 };
                    let uzunluk = bitis_sutunu.saturating_sub(baslangic_sutunu).max(1);
                    let (simge, boya) = if vurgu.birincil {
                        ("^", boya)
                    } else {
                        ("-", Gorunum::kenar as fn(&Gorunum, &str) -> String)
                    };
                    let alt_cizgi = format!(
                        "{}{}",
                        simge.repeat(uzunluk),
                        vurgu.etiket.as_deref().map(|l| format!(" {l}")).unwrap_or_default()
                    );
                    let _ = writeln!(
                        cikti,
                        "{dolgu} {} {}{}",
                        gorunum.kenar("|"),
                        " ".repeat(baslangic_sutunu - 1),
                        boya(gorunum, &alt_cizgi)
                    );
                }
            }
            let _ = writeln!(cikti, "{dolgu} {}", gorunum.kenar("|"));
        }
    }

    /// Bir yardım mesajındaki kod önerisini Pas satırına uygular.
    fn oneri(&mut self, alt_tani: &Tani) -> Option<(usize, String)> {
        let mut degisiklikler = Vec::new();
        for aralik in &alt_tani.araliklar {
            let yerine = aralik.onerilen.as_ref()?;
            if yerine.contains('\n') {
                return None;
            }
            let vurgu = self.vurgula(aralik)?;
            degisiklikler.push((vurgu, pasa_cevir(yerine)));
        }
        let (bu_dosyadakiler, _) = degisiklikler.first()?;
        let dosya = bu_dosyadakiler.dosya.clone();
        let dizin = SatirDizini::yeni(&dosya.metin);
        let satir = dizin.satir_sutun(bu_dosyadakiler.baslangic).0;
        let satir_basi = dizin.satir_basi(satir)?;
        let mut metin = dizin.satir(satir)?.to_owned();
        degisiklikler.sort_by_key(|(vurgu, _)| std::cmp::Reverse(vurgu.baslangic));
        for (vurgu, yerine) in &degisiklikler {
            if !Rc::ptr_eq(&vurgu.dosya, &dosya)
                || dizin.satir_sutun(vurgu.baslangic).0 != satir
                || dizin.satir_sutun(vurgu.bitis).0 != satir
            {
                return None;
            }
            metin.replace_range(vurgu.baslangic - satir_basi..vurgu.bitis - satir_basi, yerine);
        }
        Some((satir, metin.replace('\t', " ")))
    }
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

/// Program çökünce (panik) gösterilecek konum ve kaynak satırı.
pub fn konumu_goster(cikti: &mut String, gorunum: &Gorunum, dosya: &KaynakDosya, ofset: usize) {
    let dizin = SatirDizini::yeni(&dosya.metin);
    let (satir, sutun) = dizin.satir_sutun(ofset);
    let genislik = satir.to_string().len();
    let dolgu = " ".repeat(genislik);
    let _ = writeln!(cikti, "{dolgu}{} {}:{satir}:{sutun}", gorunum.kenar("-->"), dosya.gosterilen);
    let _ = writeln!(cikti, "{dolgu} {}", gorunum.kenar("|"));
    let metin = dizin.satir(satir).unwrap_or("").replace('\t', " ");
    let _ = writeln!(cikti, "{} {metin}", gorunum.kenar(&format!("{satir} |")));
    let _ = writeln!(cikti, "{dolgu} {} {}{}", gorunum.kenar("|"), " ".repeat(sutun - 1), gorunum.hata("^"));
}
