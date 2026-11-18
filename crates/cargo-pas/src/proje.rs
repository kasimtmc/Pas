//! Bir Pas projesini, cargo'nun derleyebileceği bir "gölge" projeye çevirir.
//!
//! Gölge proje `target/pas/` altında durur ve kullanıcının projesinin
//! aynısıdır; tek farkı `.pas` dosyalarının yerinde çevrilmiş `.rs`
//! dosyalarının bulunmasıdır (`src/ana.pas` → `src/main.rs`). Dosyalar
//! yalnızca içerikleri değiştiğinde yeniden yazılır; böylece cargo'nun
//! artımlı derlemesi bozulmaz.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use pas_cekirdek::{Ceviri, KaynakHaritasi, Secenekler, UZANTI};

/// Gölge projenin, kullanıcının projesine göre yolu.
pub const GOLGE_ONEKI: &str = "target/pas/";

/// cargo'nun kendiliğinden kaynak aradığı dizinler.
const YANSITILAN_DIZINLER: &[&str] = &["src", "examples", "tests", "benches"];

pub struct Proje {
    pub kok: PathBuf,
    pub golge: PathBuf,
    pub hedef_dizini: PathBuf,
    pub paket_adi: Option<String>,
    /// Diskteki yerine kullanılacak dosya içeriği (göreli yol, içerik). IDE,
    /// kaydedilmemiş bir dosyayı denetletirken kullanır.
    pub kaydedilmemis: Option<(String, String)>,
}

/// Çevrilmiş bir Pas dosyası.
pub struct UretilenDosya {
    /// Gölge projeye göre Rust dosyasının yolu, örneğin `src/main.rs`.
    pub rust_yolu: String,
    /// Projeye göre Pas dosyasının yolu, örneğin `src/ana.pas`.
    pub pas_yolu: String,
    pub pas_kaynagi: String,
    pub rust_kaynagi: String,
    pub harita: KaynakHaritasi,
}

pub struct Hazirlik {
    pub dosyalar: Vec<UretilenDosya>,
}

impl Proje {
    /// Bulunulan dizinden yukarı doğru `Cargo.toml` arar.
    pub fn bul(baslangic: &Path) -> Result<Proje, String> {
        let kok = baslangic
            .ancestors()
            .find(|dizin| dizin.join("Cargo.toml").is_file())
            .ok_or(
                "burada bir Pas projesi yok (`Cargo.toml` bulunamadı).\n\
                    Yeni bir proje oluşturmak için: cargo pas yeni <ad>",
            )?
            .to_path_buf();
        let bildirim =
            fs::read_to_string(kok.join("Cargo.toml")).map_err(|e| format!("`Cargo.toml` okunamadı: {e}"))?;
        let paket_adi = bildirim
            .parse::<toml_edit::DocumentMut>()
            .ok()
            .and_then(|belge| belge.get("package")?.get("name")?.as_str().map(str::to_owned));
        let hedef_dizini = kok.join("target");
        Ok(Proje { golge: hedef_dizini.join("pas"), hedef_dizini, kok, paket_adi, kaydedilmemis: None })
    }

    pub fn golge_bildirimi(&self) -> PathBuf {
        self.golge.join("Cargo.toml")
    }

    /// Gölge projeyi oluşturur ya da günceller.
    pub fn hazirla(&self) -> Result<Hazirlik, String> {
        let mut yazilanlar = HashSet::new();
        let mut dosyalar = Vec::new();

        for dizin in YANSITILAN_DIZINLER {
            let kaynak_dizini = self.kok.join(dizin);
            if kaynak_dizini.is_dir() {
                self.dizini_yansit(&kaynak_dizini, &mut dosyalar, &mut yazilanlar)?;
            }
        }
        if self.kok.join("build.rs").is_file() {
            self.dosyayi_kopyala(Path::new("build.rs"), &mut yazilanlar)?;
        }
        self.bildirimi_yaz(&mut yazilanlar)?;

        for dizin in YANSITILAN_DIZINLER {
            eskileri_sil(&self.golge.join(dizin), &yazilanlar);
        }
        dosyalar.sort_by(|a, b| a.pas_yolu.cmp(&b.pas_yolu));
        Ok(Hazirlik { dosyalar })
    }

    fn dizini_yansit(
        &self,
        dizin: &Path,
        dosyalar: &mut Vec<UretilenDosya>,
        yazilanlar: &mut HashSet<PathBuf>,
    ) -> Result<(), String> {
        let girdiler = fs::read_dir(dizin).map_err(|e| format!("`{}` okunamadı: {e}", dizin.display()))?;
        for girdi in girdiler {
            let yol = girdi.map_err(|e| e.to_string())?.path();
            let goreli = yol.strip_prefix(&self.kok).expect("proje içinde").to_path_buf();
            if yol.is_dir() {
                self.dizini_yansit(&yol, dosyalar, yazilanlar)?;
            } else if yol.extension().is_some_and(|ext| ext == UZANTI) {
                dosyalar.push(self.dosyayi_cevir(&goreli, yazilanlar)?);
            } else {
                self.dosyayi_kopyala(&goreli, yazilanlar)?;
            }
        }
        Ok(())
    }

    fn dosyayi_cevir(&self, goreli: &Path, yazilanlar: &mut HashSet<PathBuf>) -> Result<UretilenDosya, String> {
        let pas_yolu = egik_cizgili_yol(goreli);
        let rust_goreli = rust_yolu_bul(goreli);
        if self.kok.join(&rust_goreli).exists() {
            return Err(format!(
                "`{pas_yolu}` ve `{}` aynı anda var; ikisinden birini sil",
                egik_cizgili_yol(&rust_goreli)
            ));
        }
        let pas_kaynagi = match &self.kaydedilmemis {
            Some((yol, icerik)) if *yol == pas_yolu => icerik.clone(),
            _ => fs::read_to_string(self.kok.join(goreli)).map_err(|e| format!("`{pas_yolu}` okunamadı: {e}"))?,
        };

        let ceviri_sonucu = self.cevir(goreli, &pas_kaynagi);

        degistiyse_yaz(&self.golge.join(&rust_goreli), ceviri_sonucu.rust.as_bytes())?;
        yazilanlar.insert(self.golge.join(&rust_goreli));
        Ok(UretilenDosya {
            rust_yolu: egik_cizgili_yol(&rust_goreli),
            pas_yolu,
            pas_kaynagi,
            rust_kaynagi: ceviri_sonucu.rust,
            harita: ceviri_sonucu.harita,
        })
    }

    /// Projedeki bir Pas dosyasının içeriğini, gölge projede derleneceği
    /// biçimde Rust'a çevirir (modül yolları ve destek kodu dahil).
    pub fn cevir(&self, goreli: &Path, pas_kaynagi: &str) -> Ceviri {
        let modul_yolu = |ad: &str| self.modul_yolu(goreli, ad);
        let secenekler = Secenekler { modul_yolu: Some(&modul_yolu), destek_ekle: sandik_koku(goreli) };
        pas_cekirdek::rusta_cevir_secenekle(pas_kaynagi, &secenekler)
    }

    /// `mod ad;` bildiriminin `#[path]` yolu. rustc bu yolu bildirimi
    /// yapan dosyanın dizinine göre çözer.
    fn modul_yolu(&self, bildiren_dosya: &Path, ad: &str) -> String {
        let dizin = bildiren_dosya.parent().unwrap_or(Path::new(""));
        let rust_dosyasi = rust_yolu_bul(bildiren_dosya);
        let govde = rust_dosyasi.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        // `main.rs`, `lib.rs`, `mod.rs` ve sandık kökleri alt modüllerini
        // kendi dizinlerinde arar; diğer dosyalar `dosya_adı/` altında.
        let kok_mu = matches!(govde, "main" | "lib" | "mod") || sandik_koku_mu(bildiren_dosya);
        let taban = if kok_mu { String::new() } else { format!("{govde}/") };
        let dizin_olarak = self.kok.join(dizin).join(&taban).join(ad);
        if dizin_olarak.join(format!("mod.{UZANTI}")).is_file() || dizin_olarak.join("mod.rs").is_file() {
            format!("{taban}{ad}/mod.rs")
        } else {
            format!("{taban}{ad}.rs")
        }
    }

    fn dosyayi_kopyala(&self, goreli: &Path, yazilanlar: &mut HashSet<PathBuf>) -> Result<(), String> {
        let icerik = fs::read(self.kok.join(goreli)).map_err(|e| format!("`{}` okunamadı: {e}", goreli.display()))?;
        let hedef = self.golge.join(goreli);
        degistiyse_yaz(&hedef, &icerik)?;
        yazilanlar.insert(hedef);
        Ok(())
    }

    /// Kullanıcının `Cargo.toml` dosyasını gölge projeye uyarlar.
    fn bildirimi_yaz(&self, yazilanlar: &mut HashSet<PathBuf>) -> Result<(), String> {
        let metin =
            fs::read_to_string(self.kok.join("Cargo.toml")).map_err(|e| format!("`Cargo.toml` okunamadı: {e}"))?;
        let mut belge: toml_edit::DocumentMut = metin.parse().map_err(|e| format!("`Cargo.toml` hatalı: {e}"))?;

        // Gölge proje kendi başına bir çalışma alanıdır; yoksa cargo üst
        // dizinlerdeki bir çalışma alanına ait olduğunu sanabilir.
        if !belge.contains_key("workspace") {
            belge["workspace"] = toml_edit::table();
        }
        // Göreli bağımlılık yolları artık bir başka dizinden okunuyor.
        for tablo in bagimlilik_tablolari(&mut belge) {
            for (_, bagimlilik) in tablo.iter_mut() {
                if let Some(bagimlilik) = bagimlilik.as_table_like_mut()
                    && let Some(yol) = bagimlilik.get_mut("path")
                {
                    mutlak_yap(yol, &self.kok);
                }
            }
        }
        // `[[bin]] path = "src/oyun.pas"` gibi hedef yolları
        // ve `path = "target/pas/src/main.rs"` gibi gölge projeyi gösteren yollar.
        // İkincisi, IDE'lerin (`cargo metadata`) `.rs` dosyası olmayan bir
        // projeyi okuyabilmesi için `cargo pas yeni` tarafından yazılır.
        for anahtar in ["lib", "bin", "example", "test", "bench"] {
            for hedef in hedefler(&mut belge, anahtar) {
                let Some(yol) = hedef.get_mut("path") else { continue };
                let Some(metin) = yol.as_str().map(|t| t.replace('\\', "/")) else { continue };
                if let Some(ic) = metin.strip_prefix(GOLGE_ONEKI) {
                    *yol = toml_edit::value(ic);
                } else if metin.ends_with(&format!(".{UZANTI}")) {
                    *yol = toml_edit::value(egik_cizgili_yol(&rust_yolu_bul(Path::new(&metin))));
                }
            }
        }

        let hedef = self.golge_bildirimi();
        degistiyse_yaz(&hedef, belge.to_string().as_bytes())?;
        yazilanlar.insert(hedef);
        Ok(())
    }
}

/// Bir Pas dosyasının gölge projedeki Rust karşılığının yolu.
pub fn rust_yolu_bul(pas: &Path) -> PathBuf {
    let govde = pas.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ust_dizin = pas.parent().unwrap_or(Path::new(""));
    let src_icinde = ust_dizin == Path::new("src");
    let bin_dizininde = ust_dizin.parent().is_some_and(|p| p == Path::new("src/bin"));
    let rust_govdesi = match govde {
        "ana" if src_icinde || bin_dizininde => "main",
        "kütüphane" if src_icinde => "lib",
        _ => govde,
    };
    ust_dizin.join(format!("{rust_govdesi}.rs"))
}

/// `ana.pas` ve `kütüphane.pas` gibi, kendi sandığının kökü olan bir dosya mı?
/// `oku!` gibi makroları sağlayan destek kodu bu dosyalara eklenir.
fn sandik_koku(pas: &Path) -> bool {
    let rust = rust_yolu_bul(pas);
    let ikili_dizininde = rust.parent().and_then(Path::parent).is_some_and(|p| p == Path::new("src/bin"));
    rust == Path::new("src/main.rs")
        || rust == Path::new("src/lib.rs")
        || (ikili_dizininde && rust.file_name().is_some_and(|ad| ad == "main.rs"))
        || sandik_koku_mu(pas)
}

/// `src/bin/x.pas`, `examples/x.pas` gibi dosyalar kendi sandıklarının köküdür.
fn sandik_koku_mu(pas: &Path) -> bool {
    let ust_dizin = pas.parent().unwrap_or(Path::new(""));
    ["src/bin", "examples", "tests", "benches"].iter().any(|dizin| ust_dizin == Path::new(dizin))
}

fn bagimlilik_tablolari(belge: &mut toml_edit::DocumentMut) -> Vec<&mut dyn toml_edit::TableLike> {
    const ANAHTARLAR: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];
    let mut tablolar: Vec<&mut dyn toml_edit::TableLike> = Vec::new();
    for (anahtar, oge) in belge.as_table_mut().iter_mut() {
        if ANAHTARLAR.contains(&anahtar.get()) {
            tablolar.extend(oge.as_table_like_mut());
        } else if anahtar.get() == "target"
            && let Some(hedefler) = oge.as_table_like_mut()
        {
            for (_, platform) in hedefler.iter_mut() {
                let Some(platform) = platform.as_table_like_mut() else { continue };
                for (anahtar, oge) in platform.iter_mut() {
                    if ANAHTARLAR.contains(&anahtar.get()) {
                        tablolar.extend(oge.as_table_like_mut());
                    }
                }
            }
        }
    }
    tablolar
}

fn hedefler<'d>(belge: &'d mut toml_edit::DocumentMut, anahtar: &str) -> Vec<&'d mut dyn toml_edit::TableLike> {
    match belge.get_mut(anahtar) {
        Some(toml_edit::Item::Table(tablo)) => vec![tablo as &mut dyn toml_edit::TableLike],
        Some(toml_edit::Item::ArrayOfTables(dizi)) => {
            dizi.iter_mut().map(|t| t as &mut dyn toml_edit::TableLike).collect()
        }
        _ => Vec::new(),
    }
}

fn mutlak_yap(oge: &mut toml_edit::Item, kok: &Path) {
    if let Some(yol) = oge.as_str()
        && Path::new(yol).is_relative()
    {
        *oge = toml_edit::value(kok.join(yol).to_string_lossy().into_owned());
    }
}

fn degistiyse_yaz(yol: &Path, icerik: &[u8]) -> Result<(), String> {
    if fs::read(yol).is_ok_and(|eski| eski == icerik) {
        return Ok(());
    }
    if let Some(ust_dizin) = yol.parent() {
        fs::create_dir_all(ust_dizin).map_err(|e| format!("`{}` oluşturulamadı: {e}", ust_dizin.display()))?;
    }
    fs::write(yol, icerik).map_err(|e| format!("`{}` yazılamadı: {e}", yol.display()))
}

/// Kaynağı silinmiş dosyaların gölgedeki kopyalarını temizler.
fn eskileri_sil(dizin: &Path, yazilanlar: &HashSet<PathBuf>) {
    let Ok(girdiler) = fs::read_dir(dizin) else { return };
    for girdi in girdiler.flatten() {
        let yol = girdi.path();
        if yol.is_dir() {
            eskileri_sil(&yol, yazilanlar);
            let _ = fs::remove_dir(&yol); // yalnızca boşsa silinir
        } else if !yazilanlar.contains(&yol) {
            let _ = fs::remove_file(&yol);
        }
    }
}

pub fn egik_cizgili_yol(yol: &Path) -> String {
    yol.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn sandik_kokleri() {
        let k = |s: &str| sandik_koku(Path::new(s));
        assert!(k("src/ana.pas"));
        assert!(k("src/kütüphane.pas"));
        assert!(k("src/bin/araç.pas"));
        assert!(k("src/bin/araç/ana.pas"));
        assert!(k("examples/deneme.pas"));
        assert!(!k("src/oyun.pas"));
        assert!(!k("src/oyun/ana.pas"));
    }

    #[test]
    fn rust_yollari() {
        let p = |s: &str| egik_cizgili_yol(&rust_yolu_bul(Path::new(s)));
        assert_eq!(p("src/ana.pas"), "src/main.rs");
        assert_eq!(p("src/kütüphane.pas"), "src/lib.rs");
        assert_eq!(p("src/oyun/ana.pas"), "src/oyun/ana.rs");
        assert_eq!(p("src/bin/araç/ana.pas"), "src/bin/araç/main.rs");
        assert_eq!(p("src/çalış.pas"), "src/çalış.rs");
        assert_eq!(p("examples/ana.pas"), "examples/ana.rs");
    }
}
