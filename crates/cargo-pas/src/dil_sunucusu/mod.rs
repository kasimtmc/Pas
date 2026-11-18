//! `cargo pas lsp`: Pas dosyaları için dil sunucusu (LSP).
//!
//! Kendisi bir köprüdür: editörden gelen istekleri gölge projedeki Rust
//! dosyalarına çevirip rust-analyzer'a iletir, cevapları da Pas'a geri
//! çevirir. Konumlar kaynak haritasıyla `.pas` dosyalarına eşlenir; isimler
//! ve kod parçaları (`push` → `ekle`) sözlükle çevrilir.
//!
//! Tanılamalar (hatalar) buradan değil, `cargo pas denetle --json` ile gelir;
//! rust-analyzer'ın kendi tanılamaları kapatılır.

mod iletisim;
mod konum;
mod tanilama;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use pas_cekirdek::{Ceviri, KaynakHaritasi, UZANTI, Yon, pasa_cevir};
use serde_json::{Value, json};

use crate::proje::{Hazirlik, Proje, egik_cizgili_yol, rust_yolu_bul};
use konum::{Metin, uri_yolu, yol_uri};

/// rust-analyzer bulunamadığında kullanıcıya gösterilen mesaj.
const RA_YOK: &str = "Pas dil sunucusu rust-analyzer'ı bulamadı; tanıma gitme ve akıllı tamamlama çalışmayacak. \
                      Kurmak için: rustup component add rust-analyzer";

/// Konum içeren ve rust-analyzer'a iletilen istekler.
const KONUMLU_ISTEKLER: &[&str] = &[
    "textDocument/definition",
    "textDocument/typeDefinition",
    "textDocument/implementation",
    "textDocument/references",
    "textDocument/hover",
    "textDocument/documentHighlight",
    "textDocument/completion",
];

enum Olay {
    Istemci(Value),
    IstemciBitti,
    Ra(Value),
    RaBitti,
    Denetim(tanilama::DenetimSonucu),
}

/// rust-analyzer'a gönderilmiş ve cevabı beklenen bir istek.
enum Bekleyen {
    Baslatma,
    /// `ra_istegi`: yeniden denenebilmesi için rust-analyzer'a gönderilen istek.
    Istek {
        istemci_id: Value,
        yontem: String,
        belge: PathBuf,
        ra_istegi: Value,
        deneme: u32,
    },
    Kapatma {
        istemci_id: Value,
    },
}

struct ProjeDurumu {
    proje: Proje,
    hazirlik: Hazirlik,
}

/// Editörde açık bir Pas dosyası.
struct AcikBelge {
    kok: PathBuf,
    rust_yolu: PathBuf,
    metin: String,
    ceviri: Ceviri,
}

/// Bir Rust dosyasındaki konumların kullanıcıya hangi dosyada ve nasıl
/// gösterileceği.
struct Eslem {
    hedef: PathBuf,
    rust_metni: String,
    gosterilen_metin: String,
    harita: Option<KaynakHaritasi>,
}

impl Eslem {
    /// rust-analyzer'ın (UTF-8) aralığını kullanıcının dosyasındaki (UTF-16) aralığa çevirir.
    fn aralik(&self, aralik: &Value) -> Value {
        let rust = Metin::yeni(&self.rust_metni);
        let ofset = |konum: &Value| {
            rust.utf8_ofseti(konum["line"].as_u64().unwrap_or(0), konum["character"].as_u64().unwrap_or(0))
        };
        let (baslangic, bitis) = (ofset(&aralik["start"]), ofset(&aralik["end"]));
        let (baslangic, bitis) = match &self.harita {
            Some(harita) => harita.rust_araligini_pasa(baslangic, bitis),
            None => (baslangic, bitis),
        };
        let gosterilen = Metin::yeni(&self.gosterilen_metin);
        let konum = |ofset| {
            let (satir, sutun) = gosterilen.utf16_konumu(ofset);
            json!({ "line": satir, "character": sutun })
        };
        json!({ "start": konum(baslangic), "end": konum(bitis) })
    }
}

struct Sunucu {
    istemci: Box<dyn Write>,
    gonderici: mpsc::Sender<Olay>,
    /// İstemci Türkçe tanılamaları sunucudan istiyor mu (`tanilamalar: true`)?
    tanilamalar_acik: bool,
    /// Denetlenecek dosyalar ve en erken ne zaman denetlenecekleri
    denetlenecek: HashMap<PathBuf, Instant>,
    /// Denetimi süren projelerin kökleri
    denetlenen: HashSet<PathBuf>,
    /// rust-analyzer'ın "içerik değişti" diye geri çevirdiği, yeniden gönderilecek istekler
    yeniden_denenecek: Vec<(Instant, Bekleyen)>,
    ra: Option<(Child, ChildStdin)>,
    ra_hazir: bool,
    bekletilenler: Vec<Value>,
    sonraki_id: i64,
    bekleyenler: HashMap<i64, Bekleyen>,
    kok_uri: Option<String>,
    projeler: BTreeMap<PathBuf, ProjeDurumu>,
    acik: HashMap<PathBuf, AcikBelge>,
}

pub fn calistir() -> Result<ExitCode, String> {
    let (gonderici, alici) = mpsc::channel::<Olay>();

    let istemci_gondericisi = gonderici.clone();
    thread::spawn(move || {
        let mut okuyucu = BufReader::new(std::io::stdin().lock());
        while let Some(mesaj) = iletisim::oku(&mut okuyucu) {
            if istemci_gondericisi.send(Olay::Istemci(mesaj)).is_err() {
                return;
            }
        }
        let _ = istemci_gondericisi.send(Olay::IstemciBitti);
    });

    let mut sunucu = Sunucu {
        istemci: Box::new(std::io::stdout()),
        gonderici,
        tanilamalar_acik: false,
        denetlenecek: HashMap::new(),
        denetlenen: HashSet::new(),
        yeniden_denenecek: Vec::new(),
        ra: None,
        ra_hazir: false,
        bekletilenler: Vec::new(),
        sonraki_id: 1,
        bekleyenler: HashMap::new(),
        kok_uri: None,
        projeler: BTreeMap::new(),
        acik: HashMap::new(),
    };

    loop {
        let olay = match sunucu.siradaki_denetime_kalan() {
            Some(sure) => match alici.recv_timeout(sure) {
                Ok(olay) => Some(olay),
                Err(mpsc::RecvTimeoutError::Timeout) => None,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            },
            None => match alici.recv() {
                Ok(olay) => Some(olay),
                Err(_) => break,
            },
        };
        match olay {
            Some(Olay::Istemci(mesaj)) => {
                if mesaj["method"] == "exit" {
                    break;
                }
                sunucu.istemciden(mesaj);
            }
            Some(Olay::Ra(mesaj)) => sunucu.ra_dan(mesaj),
            Some(Olay::RaBitti) => {
                sunucu.ra = None;
                sunucu.ra_hazir = false;
            }
            Some(Olay::Denetim(sonuc)) => sunucu.denetim_bitti(sonuc),
            Some(Olay::IstemciBitti) => break,
            None => {}
        }
        sunucu.zamani_gelenleri_denetle();
        sunucu.zamani_gelenleri_yeniden_dene();
    }
    if let Some((mut surec, _)) = sunucu.ra.take() {
        let _ = surec.kill();
    }
    Ok(ExitCode::SUCCESS)
}

impl Sunucu {
    fn istemciye(&mut self, mesaj: Value) {
        let _ = iletisim::yaz(&mut self.istemci, &mesaj);
    }

    fn ra_ya(&mut self, mesaj: Value) {
        if let Some((_, stdin)) = &mut self.ra {
            let _ = iletisim::yaz(stdin, &mesaj);
        }
    }

    fn cevap(&mut self, id: Value, sonuc: Value) {
        self.istemciye(json!({ "jsonrpc": "2.0", "id": id, "result": sonuc }));
    }

    fn uyari_goster(&mut self, mesaj: &str) {
        self.istemciye(json!({
            "jsonrpc": "2.0",
            "method": "window/showMessage",
            "params": { "type": 2, "message": mesaj },
        }));
    }

    // ---- Editörden gelenler ----

    fn istemciden(&mut self, mesaj: Value) {
        let yontem = mesaj["method"].as_str().unwrap_or_default().to_owned();
        match yontem.as_str() {
            "initialize" => self.baslat(mesaj),
            "textDocument/codeAction" => {
                let parametreler = &mesaj["params"];
                let eylemler = match (self.tanilamalar_acik, parametreler["textDocument"]["uri"].as_str()) {
                    (true, Some(uri)) => tanilama::duzeltmeler(
                        uri,
                        parametreler["context"]["diagnostics"].as_array().map_or(&[][..], Vec::as_slice),
                    ),
                    _ => Vec::new(),
                };
                self.cevap(mesaj["id"].clone(), json!(eylemler));
            }
            "initialized" | "$/cancelRequest" | "$/setTrace" => {}
            "shutdown" => {
                if self.ra.is_some() {
                    let id = self.yeni_id(Bekleyen::Kapatma { istemci_id: mesaj["id"].clone() });
                    self.ra_ya(json!({ "jsonrpc": "2.0", "id": id, "method": "shutdown" }));
                } else {
                    self.cevap(mesaj["id"].clone(), Value::Null);
                }
            }
            _ if !self.ra_hazir && self.ra.is_some() => self.bekletilenler.push(mesaj),
            "textDocument/didOpen" => self.belge_acildi(&mesaj["params"]["textDocument"]),
            "textDocument/didChange" => self.belge_degisti(&mesaj["params"]),
            "textDocument/didClose" => self.belge_kapandi(&mesaj["params"]["textDocument"]),
            "textDocument/didSave" => self.belge_kaydedildi(&mesaj["params"]["textDocument"]),
            y if KONUMLU_ISTEKLER.contains(&y) => self.konumlu_istek(mesaj),
            _ => {
                // Desteklenmeyen istekler boş cevap alır; bildirimler yok sayılır.
                if let Some(id) = mesaj.get("id").cloned() {
                    self.cevap(id, Value::Null);
                }
            }
        }
    }

    fn yeni_id(&mut self, bekleyen: Bekleyen) -> i64 {
        let id = self.sonraki_id;
        self.sonraki_id += 1;
        self.bekleyenler.insert(id, bekleyen);
        id
    }

    fn baslat(&mut self, mesaj: Value) {
        let parametreler = &mesaj["params"];
        self.tanilamalar_acik = parametreler["initializationOptions"]["tanilamalar"] == true;
        let kok_uri = parametreler["rootUri"]
            .as_str()
            .or_else(|| parametreler["workspaceFolders"][0]["uri"].as_str())
            .map(str::to_owned);
        if let Some(kok) = kok_uri.as_deref().and_then(uri_yolu) {
            for dizin in pas_projelerini_bul(&kok) {
                self.proje_ekle(&dizin);
            }
        }
        self.kok_uri = kok_uri;

        self.cevap(
            mesaj["id"].clone(),
            json!({
                "capabilities": {
                    "positionEncoding": "utf-16",
                    "textDocumentSync": { "openClose": true, "change": 1, "save": { "includeText": false } },
                    "definitionProvider": true,
                    "typeDefinitionProvider": true,
                    "implementationProvider": true,
                    "referencesProvider": true,
                    "hoverProvider": true,
                    "documentHighlightProvider": true,
                    "completionProvider": { "triggerCharacters": [".", ":"] },
                    "codeActionProvider": if self.tanilamalar_acik { json!({ "codeActionKinds": ["quickfix"] }) } else { json!(false) },
                },
                "serverInfo": { "name": "pas-lsp", "version": env!("CARGO_PKG_VERSION") },
            }),
        );

        match ra_baslat(&self.gonderici) {
            Some(ra) => {
                self.ra = Some(ra);
                let id = self.yeni_id(Bekleyen::Baslatma);
                let ayarlar = self.ra_ayarlari();
                self.ra_ya(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": "initialize",
                    "params": {
                        "processId": std::process::id(),
                        "rootUri": self.kok_uri,
                        "capabilities": {
                            "general": { "positionEncodings": ["utf-8"] },
                            "workspace": { "configuration": true },
                            "textDocument": {
                                "hover": { "contentFormat": ["markdown", "plaintext"] },
                                "completion": { "completionItem": { "snippetSupport": false, "documentationFormat": ["markdown"] } },
                            },
                        },
                        "initializationOptions": ayarlar,
                    },
                }));
            }
            None => self.uyari_goster(RA_YOK),
        }
    }

    /// rust-analyzer ayarları: yalnızca Pas projelerinin gölge projeleri yüklenir.
    fn ra_ayarlari(&self) -> Value {
        let projeler: Vec<String> =
            self.projeler.values().map(|p| p.proje.golge_bildirimi().to_string_lossy().into_owned()).collect();
        json!({
            "linkedProjects": projeler,
            "checkOnSave": false,
            "diagnostics": { "enable": false },
            "files": { "watcher": "server" },
            "cargo": { "buildScripts": { "enable": false } },
            "procMacro": { "enable": false },
            "lens": { "enable": false },
            "completion": { "autoimport": { "enable": false }, "callable": { "snippets": "none" } },
        })
    }

    fn proje_ekle(&mut self, dizin: &Path) -> Option<PathBuf> {
        let proje = Proje::bul(dizin).ok()?;
        if self.projeler.contains_key(&proje.kok) {
            return Some(proje.kok);
        }
        let hazirlik = proje.hazirla().ok()?;
        let kok = proje.kok.clone();
        self.projeler.insert(kok.clone(), ProjeDurumu { proje, hazirlik });
        if self.ra_hazir {
            // rust-analyzer yeni gölge projeyi de yüklesin.
            let ayarlar = self.ra_ayarlari();
            self.ra_ya(json!({
                "jsonrpc": "2.0",
                "method": "workspace/didChangeConfiguration",
                "params": { "settings": { "rust-analyzer": ayarlar } },
            }));
        }
        Some(kok)
    }

    fn belge_acildi(&mut self, belge: &Value) {
        let Some(pas_yolu) = belge["uri"].as_str().and_then(uri_yolu) else { return };
        if pas_yolu.extension().is_none_or(|u| u != UZANTI) {
            return;
        }
        let Some(kok) = pas_yolu.parent().and_then(|dizin| self.proje_ekle(dizin)) else { return };
        let metin = belge["text"].as_str().unwrap_or_default().to_owned();
        let Some(acik) = self.belge_cevir(&kok, &pas_yolu, metin) else { return };
        let mesaj = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": { "textDocument": {
                "uri": yol_uri(&acik.rust_yolu),
                "languageId": "rust",
                "version": belge["version"],
                "text": acik.ceviri.rust,
            }},
        });
        self.acik.insert(pas_yolu.clone(), acik);
        self.ra_ya(mesaj);
        self.denetim_planla(pas_yolu, Duration::ZERO);
    }

    fn belge_cevir(&self, kok: &Path, pas_yolu: &Path, metin: String) -> Option<AcikBelge> {
        let durum = self.projeler.get(kok)?;
        let goreli = pas_yolu.strip_prefix(kok).ok()?;
        let ceviri = durum.proje.cevir(goreli, &metin);
        Some(AcikBelge {
            kok: kok.to_path_buf(),
            rust_yolu: durum.proje.golge.join(rust_yolu_bul(goreli)),
            metin,
            ceviri,
        })
    }

    fn belge_degisti(&mut self, parametreler: &Value) {
        let Some(pas_yolu) = parametreler["textDocument"]["uri"].as_str().and_then(uri_yolu) else { return };
        let Some(metin) =
            parametreler["contentChanges"].as_array().and_then(|d| d.last()).and_then(|d| d["text"].as_str())
        else {
            return;
        };
        let Some(kok) = self.acik.get(&pas_yolu).map(|a| a.kok.clone()) else { return };
        let Some(acik) = self.belge_cevir(&kok, &pas_yolu, metin.to_owned()) else { return };
        let mesaj = json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didChange",
            "params": {
                "textDocument": { "uri": yol_uri(&acik.rust_yolu), "version": parametreler["textDocument"]["version"] },
                "contentChanges": [{ "text": acik.ceviri.rust }],
            },
        });
        self.acik.insert(pas_yolu.clone(), acik);
        self.ra_ya(mesaj);
        // Yazarken her harfte değil, yazmaya ara verilince denetlenir.
        self.denetim_planla(pas_yolu, Duration::from_millis(600));
    }

    fn belge_kapandi(&mut self, belge: &Value) {
        let Some(pas_yolu) = belge["uri"].as_str().and_then(uri_yolu) else { return };
        self.denetlenecek.remove(&pas_yolu);
        if self.tanilamalar_acik {
            self.tanilamalari_yayinla(&pas_yolu, Vec::new());
        }
        if let Some(acik) = self.acik.remove(&pas_yolu) {
            self.ra_ya(json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didClose",
                "params": { "textDocument": { "uri": yol_uri(&acik.rust_yolu) } },
            }));
        }
    }

    /// Kaydedilen dosyanın gölge projedeki karşılığı da güncellenir; açık
    /// olmayan dosyaların eşlemeleri buradan gelir.
    fn belge_kaydedildi(&mut self, belge: &Value) {
        let Some(pas_yolu) = belge["uri"].as_str().and_then(uri_yolu) else { return };
        let Some(kok) = self.acik.get(&pas_yolu).map(|a| a.kok.clone()) else { return };
        if let Some(durum) = self.projeler.get_mut(&kok)
            && let Ok(hazirlik) = durum.proje.hazirla()
        {
            durum.hazirlik = hazirlik;
        }
        self.denetim_planla(pas_yolu, Duration::ZERO);
    }

    // ---- Türkçe tanılamalar ----

    fn denetim_planla(&mut self, pas_yolu: PathBuf, gecikme: Duration) {
        if self.tanilamalar_acik {
            self.denetlenecek.insert(pas_yolu, Instant::now() + gecikme);
        }
    }

    /// Denetimi süren projelerin dosyaları beklemez; o denetim bitince sıraları gelir.
    /// Bir sonraki zamanlanmış işe (denetim ya da yeniden deneme) kalan süre.
    fn siradaki_denetime_kalan(&self) -> Option<Duration> {
        let denetimler = self
            .denetlenecek
            .iter()
            .filter(|(yol, _)| self.acik.get(*yol).is_some_and(|a| !self.denetlenen.contains(&a.kok)))
            .map(|(_, zaman)| *zaman);
        let denemeler = self.yeniden_denenecek.iter().map(|(zaman, _)| *zaman);
        denetimler.chain(denemeler).min().map(|zaman| zaman.saturating_duration_since(Instant::now()))
    }

    fn zamani_gelenleri_denetle(&mut self) {
        let simdi = Instant::now();
        let hazirlar: Vec<PathBuf> =
            self.denetlenecek.iter().filter(|(_, zaman)| **zaman <= simdi).map(|(yol, _)| yol.clone()).collect();
        for pas_yolu in hazirlar {
            let Some(acik) = self.acik.get(&pas_yolu) else {
                self.denetlenecek.remove(&pas_yolu);
                continue;
            };
            if self.denetlenen.contains(&acik.kok) {
                continue;
            }
            self.denetlenecek.remove(&pas_yolu);
            self.denetlenen.insert(acik.kok.clone());
            tanilama::denetle(acik.kok.clone(), pas_yolu, acik.metin.clone(), self.gonderici.clone());
        }
    }

    fn denetim_bitti(&mut self, sonuc: tanilama::DenetimSonucu) {
        self.denetlenen.remove(&sonuc.kok);
        let mut yayinlanacak = Vec::new();
        for (pas_yolu, acik) in &self.acik {
            if acik.kok != sonuc.kok {
                continue;
            }
            // Denetim sürerken değişen dosya için yeni bir denetim zaten sırada.
            if *pas_yolu == sonuc.pas_yolu && acik.metin != sonuc.metin {
                continue;
            }
            let goreli = egik_cizgili_yol(pas_yolu.strip_prefix(&acik.kok).unwrap_or(pas_yolu));
            yayinlanacak.push((pas_yolu.clone(), tanilama::lsp_tanilamalari(&sonuc.tanilar, &goreli, &acik.metin)));
        }
        for (pas_yolu, tanilamalar) in yayinlanacak {
            self.tanilamalari_yayinla(&pas_yolu, tanilamalar);
        }
    }

    fn tanilamalari_yayinla(&mut self, pas_yolu: &Path, tanilamalar: Vec<Value>) {
        self.istemciye(json!({
            "jsonrpc": "2.0",
            "method": "textDocument/publishDiagnostics",
            "params": { "uri": yol_uri(pas_yolu), "diagnostics": tanilamalar },
        }));
    }

    fn konumlu_istek(&mut self, mut mesaj: Value) {
        let istemci_id = mesaj["id"].clone();
        let yontem = mesaj["method"].as_str().unwrap_or_default().to_owned();
        let parametreler = &mut mesaj["params"];
        let pas_yolu = parametreler["textDocument"]["uri"].as_str().and_then(uri_yolu);
        let Some((pas_yolu, acik)) = pas_yolu.and_then(|yol| self.acik.get(&yol).map(|a| (yol, a))) else {
            self.cevap(istemci_id, Value::Null);
            return;
        };
        if self.ra.is_none() {
            self.cevap(istemci_id, Value::Null);
            return;
        }

        let pas = Metin::yeni(&acik.metin);
        let ofset = pas.utf16_ofseti(
            parametreler["position"]["line"].as_u64().unwrap_or(0),
            parametreler["position"]["character"].as_u64().unwrap_or(0),
        );
        let rust_ofseti = acik.ceviri.harita.pastan_rusta(ofset, Yon::Baslangic);
        let (satir, sutun) = Metin::yeni(&acik.ceviri.rust).utf8_konumu(rust_ofseti);
        parametreler["textDocument"]["uri"] = json!(yol_uri(&acik.rust_yolu));
        parametreler["position"] = json!({ "line": satir, "character": sutun });

        self.ra_istegi_gonder(Bekleyen::Istek { istemci_id, yontem, belge: pas_yolu, ra_istegi: mesaj, deneme: 0 });
    }

    // ---- rust-analyzer'dan gelenler ----

    fn ra_dan(&mut self, mesaj: Value) {
        match (mesaj.get("id").cloned(), mesaj["method"].as_str()) {
            // rust-analyzer'ın bize sorduğu: ayarlar ve benzeri
            (Some(id), Some(yontem)) => {
                let sonuc = if yontem == "workspace/configuration" {
                    let adet = mesaj["params"]["items"].as_array().map_or(1, Vec::len);
                    Value::Array(vec![self.ra_ayarlari(); adet])
                } else {
                    Value::Null
                };
                self.ra_ya(json!({ "jsonrpc": "2.0", "id": id, "result": sonuc }));
            }
            (Some(id), None) => self.ra_cevabi(id, mesaj),
            // Bildirimler (ilerleme, rust-analyzer'ın tanılamaları) iletilmez.
            (None, _) => {}
        }
    }

    fn ra_istegi_gonder(&mut self, bekleyen: Bekleyen) {
        let Bekleyen::Istek { ra_istegi, .. } = &bekleyen else { return };
        let mut mesaj = ra_istegi.clone();
        let id = self.yeni_id(bekleyen);
        mesaj["id"] = json!(id);
        self.ra_ya(mesaj);
    }

    fn zamani_gelenleri_yeniden_dene(&mut self) {
        let simdi = Instant::now();
        let (hazirlar, bekleyenler): (Vec<_>, Vec<_>) =
            std::mem::take(&mut self.yeniden_denenecek).into_iter().partition(|(zaman, _)| *zaman <= simdi);
        self.yeniden_denenecek = bekleyenler;
        for (_, bekleyen) in hazirlar {
            self.ra_istegi_gonder(bekleyen);
        }
    }

    fn ra_cevabi(&mut self, id: Value, mut mesaj: Value) {
        let Some(bekleyen) = id.as_i64().and_then(|id| self.bekleyenler.remove(&id)) else { return };
        match bekleyen {
            Bekleyen::Baslatma => {
                self.ra_hazir = true;
                self.ra_ya(json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }));
                for mesaj in std::mem::take(&mut self.bekletilenler) {
                    self.istemciden(mesaj);
                }
            }
            Bekleyen::Kapatma { istemci_id } => self.cevap(istemci_id, Value::Null),
            Bekleyen::Istek { istemci_id, yontem, belge, ra_istegi, deneme } => {
                // rust-analyzer projeyi yüklerken ya da belge değişirken isteği
                // "içerik değişti" (-32801) diye geri çevirir; biraz sonra yeniden denenir.
                if mesaj["error"]["code"] == -32801 && deneme < 10 {
                    let bekleyen = Bekleyen::Istek { istemci_id, yontem, belge, ra_istegi, deneme: deneme + 1 };
                    self.yeniden_denenecek.push((Instant::now() + Duration::from_millis(300), bekleyen));
                    return;
                }
                if let Some(hata) = mesaj.get("error").cloned() {
                    self.istemciye(json!({ "jsonrpc": "2.0", "id": istemci_id, "error": hata }));
                    return;
                }
                let mut sonuc = mesaj["result"].take();
                self.sonucu_cevir(&yontem, &belge, &mut sonuc);
                self.cevap(istemci_id, sonuc);
            }
        }
    }

    /// rust-analyzer'ın cevabındaki konumları ve kod parçalarını Pas'a çevirir.
    fn sonucu_cevir(&mut self, yontem: &str, belge: &Path, sonuc: &mut Value) {
        let Some(istek_eslemi) = self.acik.get(belge).map(|acik| Eslem {
            hedef: belge.to_path_buf(),
            rust_metni: acik.ceviri.rust.clone(),
            gosterilen_metin: acik.metin.clone(),
            harita: Some(acik.ceviri.harita.clone()),
        }) else {
            *sonuc = Value::Null;
            return;
        };
        match yontem {
            "textDocument/hover" => {
                if let Some(aralik) = sonuc.get("range").cloned() {
                    sonuc["range"] = istek_eslemi.aralik(&aralik);
                }
                if let Some(icerik) = sonuc.get_mut("contents") {
                    icerigi_cevir(icerik);
                }
            }
            "textDocument/documentHighlight" => {
                for vurgu in sonuc.as_array_mut().into_iter().flatten() {
                    vurgu["range"] = istek_eslemi.aralik(&vurgu["range"]);
                }
            }
            "textDocument/completion" => {
                let ogeler = match sonuc {
                    Value::Array(ogeler) => Some(ogeler),
                    Value::Object(liste) => {
                        // rust-analyzer önerileri süzmez; süzmeyi editör Pas
                        // adlarıyla (`ek` → `ekle`) yapsın, her harfte yeniden sormasın.
                        liste.insert("isIncomplete".into(), json!(false));
                        liste.get_mut("items").and_then(Value::as_array_mut)
                    }
                    _ => None,
                };
                for oge in ogeler.into_iter().flatten() {
                    tamamlama_ogesini_cevir(oge, &istek_eslemi);
                }
            }
            // tanım, tür tanımı, uygulama, kullanımlar: konum ya da konum listesi
            _ => self.konumlari_cevir(sonuc, &istek_eslemi),
        }
    }

    fn konumlari_cevir(&mut self, deger: &mut Value, istek_eslemi: &Eslem) {
        match deger {
            Value::Array(ogeler) => {
                for oge in ogeler {
                    self.konumlari_cevir(oge, istek_eslemi);
                }
            }
            Value::Object(nesne) => {
                if let Some(uri) = nesne.get("uri").and_then(Value::as_str).map(str::to_owned)
                    && let Some(eslem) = self.eslem(&uri)
                {
                    nesne.insert("uri".into(), json!(yol_uri(&eslem.hedef)));
                    if let Some(aralik) = nesne.get("range").cloned() {
                        nesne.insert("range".into(), eslem.aralik(&aralik));
                    }
                }
                if let Some(uri) = nesne.get("targetUri").and_then(Value::as_str).map(str::to_owned)
                    && let Some(eslem) = self.eslem(&uri)
                {
                    nesne.insert("targetUri".into(), json!(yol_uri(&eslem.hedef)));
                    for anahtar in ["targetRange", "targetSelectionRange"] {
                        if let Some(aralik) = nesne.get(anahtar).cloned() {
                            nesne.insert(anahtar.into(), eslem.aralik(&aralik));
                        }
                    }
                }
                if let Some(aralik) = nesne.get("originSelectionRange").cloned() {
                    nesne.insert("originSelectionRange".into(), istek_eslemi.aralik(&aralik));
                }
            }
            _ => {}
        }
    }

    /// Bir Rust dosyasının kullanıcıya nasıl gösterileceği: çevrilmiş bir Pas
    /// dosyasıysa `.pas` dosyası ve haritası, gölgeye kopyalanmış bir `.rs`
    /// dosyasıysa projedeki asıl dosya, diğerleri (standart kütüphane) olduğu gibi.
    fn eslem(&self, uri: &str) -> Option<Eslem> {
        let rust_yolu = uri_yolu(uri)?;
        if let Some((pas_yolu, acik)) = self.acik.iter().find(|(_, a)| a.rust_yolu == rust_yolu) {
            return Some(Eslem {
                hedef: pas_yolu.clone(),
                rust_metni: acik.ceviri.rust.clone(),
                gosterilen_metin: acik.metin.clone(),
                harita: Some(acik.ceviri.harita.clone()),
            });
        }
        for durum in self.projeler.values() {
            let Ok(goreli) = rust_yolu.strip_prefix(&durum.proje.golge) else { continue };
            let goreli = egik_cizgili_yol(goreli);
            if let Some(dosya) = durum.hazirlik.dosyalar.iter().find(|d| d.rust_yolu == goreli) {
                return Some(Eslem {
                    hedef: durum.proje.kok.join(&dosya.pas_yolu),
                    rust_metni: dosya.rust_kaynagi.clone(),
                    gosterilen_metin: dosya.pas_kaynagi.clone(),
                    harita: Some(dosya.harita.clone()),
                });
            }
            let asil = durum.proje.kok.join(&goreli);
            let metin = fs::read_to_string(&asil).ok()?;
            return Some(Eslem { hedef: asil, rust_metni: metin.clone(), gosterilen_metin: metin, harita: None });
        }
        let metin = fs::read_to_string(&rust_yolu).ok()?;
        Some(Eslem { hedef: rust_yolu, rust_metni: metin.clone(), gosterilen_metin: metin, harita: None })
    }
}

/// Tamamlama önerisindeki adları ve kodu Pas'a çevirir.
fn tamamlama_ogesini_cevir(oge: &mut Value, istek_eslemi: &Eslem) {
    for anahtar in ["label", "filterText", "insertText", "detail"] {
        if let Some(metin) = oge[anahtar].as_str() {
            oge[anahtar] = json!(pasa_cevir(metin));
        }
    }
    for anahtar in ["detail", "description"] {
        if let Some(metin) = oge["labelDetails"][anahtar].as_str() {
            oge["labelDetails"][anahtar] = json!(pasa_cevir(metin));
        }
    }
    if let Some(belgeleme) = oge.get_mut("documentation") {
        icerigi_cevir(belgeleme);
    }
    if let Some(duzenleme) = oge.get_mut("textEdit") {
        duzenlemeyi_cevir(duzenleme, istek_eslemi);
    }
    for duzenleme in oge["additionalTextEdits"].as_array_mut().into_iter().flatten() {
        duzenlemeyi_cevir(duzenleme, istek_eslemi);
    }
    if let Some(metin) = oge["textEdit"]["newText"].as_str()
        && oge.get("filterText").is_none()
    {
        oge["filterText"] = json!(metin);
    }
}

fn duzenlemeyi_cevir(duzenleme: &mut Value, istek_eslemi: &Eslem) {
    for anahtar in ["range", "insert", "replace"] {
        if let Some(aralik) = duzenleme.get(anahtar).cloned() {
            duzenleme[anahtar] = istek_eslemi.aralik(&aralik);
        }
    }
    if let Some(metin) = duzenleme["newText"].as_str() {
        duzenleme["newText"] = json!(pasa_cevir(metin));
    }
}

/// Hover ve belgeleme içeriği: kod blokları ve `ters tırnaklı` parçalar Pas'a
/// çevrilir, düz metin (çoğu standart kütüphane belgesi) olduğu gibi kalır.
fn icerigi_cevir(icerik: &mut Value) {
    match icerik {
        Value::String(metin) => *metin = markdown_cevir(metin),
        Value::Object(nesne) => {
            let duz_metin = nesne.get("kind").and_then(Value::as_str) == Some("plaintext");
            if let Some(Value::String(metin)) = nesne.get_mut("value") {
                *metin = if duz_metin { pasa_cevir(metin) } else { markdown_cevir(metin) };
            }
        }
        Value::Array(ogeler) => ogeler.iter_mut().for_each(icerigi_cevir),
        _ => {}
    }
}

fn markdown_cevir(metin: &str) -> String {
    let mut sonuc = String::with_capacity(metin.len());
    let mut kod_blogunda = false;
    for satir in metin.split_inclusive('\n') {
        if satir.trim_start().starts_with("```") {
            kod_blogunda = !kod_blogunda;
            sonuc.push_str(satir);
        } else if kod_blogunda {
            sonuc.push_str(&pasa_cevir(satir));
        } else {
            for (i, parca) in satir.split('`').enumerate() {
                if i > 0 {
                    sonuc.push('`');
                }
                sonuc.push_str(&if i % 2 == 1 { pasa_cevir(parca) } else { parca.to_owned() });
            }
        }
    }
    sonuc
}

/// rust-analyzer'ı başlatır ve çıktısını okuyan iş parçacığını kurar.
fn ra_baslat(gonderici: &mpsc::Sender<Olay>) -> Option<(Child, ChildStdin)> {
    let program = std::env::var_os("PAS_RUST_ANALYZER").unwrap_or_else(|| "rust-analyzer".into());
    // rustup'ın yer tutucusu, bileşen kurulu değilse hata verir.
    let calisiyor = Command::new(&program)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|durum| durum.success());
    if !calisiyor {
        return None;
    }
    let mut surec =
        Command::new(&program).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok()?;
    let stdin = surec.stdin.take()?;
    let stdout = surec.stdout.take()?;
    let gonderici = gonderici.clone();
    thread::spawn(move || {
        let mut okuyucu = BufReader::new(stdout);
        while let Some(mesaj) = iletisim::oku(&mut okuyucu) {
            if gonderici.send(Olay::Ra(mesaj)).is_err() {
                return;
            }
        }
        let _ = gonderici.send(Olay::RaBitti);
    });
    Some((surec, stdin))
}

/// Bir dizinin altındaki Pas projeleri (`src/` altında `.pas` dosyası olan
/// `Cargo.toml` dizinleri).
fn pas_projelerini_bul(kok: &Path) -> Vec<PathBuf> {
    fn pas_var_mi(dizin: &Path) -> bool {
        fs::read_dir(dizin).into_iter().flatten().flatten().any(|g| {
            let yol = g.path();
            if yol.is_dir() { pas_var_mi(&yol) } else { yol.extension().is_some_and(|u| u == UZANTI) }
        })
    }
    fn gez(dizin: &Path, derinlik: usize, sonuc: &mut Vec<PathBuf>) {
        if dizin.join("Cargo.toml").is_file() && pas_var_mi(&dizin.join("src")) {
            sonuc.push(dizin.to_path_buf());
        }
        if derinlik == 0 {
            return;
        }
        for girdi in fs::read_dir(dizin).into_iter().flatten().flatten() {
            let yol = girdi.path();
            let ad = girdi.file_name();
            let atla = ad.to_string_lossy().starts_with('.')
                || ["target", "node_modules", "build"].contains(&&*ad.to_string_lossy());
            if yol.is_dir() && !atla {
                gez(&yol, derinlik - 1, sonuc);
            }
        }
    }
    let mut sonuc = Vec::new();
    gez(kok, 4, &mut sonuc);
    sonuc
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn markdown_kodu_cevrilir_duz_metin_kalir() {
        let md = "```rust\npub fn push(&mut self, value: T)\n```\n\nAppends an element to the back. See `Vec::push` for more.\n";
        assert_eq!(
            markdown_cevir(md),
            "```rust\ngenel işlev ekle(&değişken kendi, value: T)\n```\n\nAppends an element to the back. See `Liste::ekle` for more.\n"
        );
    }
}
