//! `cargo pas lsp` dil sunucusunu gerçek rust-analyzer ile dener.
//! rust-analyzer kurulu değilse test atlanır.

use std::fs;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

struct Istemci {
    surec: Child,
    stdin: ChildStdin,
    gelenler: mpsc::Receiver<Value>,
    sonraki_id: i64,
}

impl Istemci {
    fn baslat() -> Istemci {
        let mut surec = Command::new(env!("CARGO_BIN_EXE_cargo-pas"))
            .arg("lsp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let stdin = surec.stdin.take().unwrap();
        let mut okuyucu = BufReader::new(surec.stdout.take().unwrap());
        let (gonderici, gelenler) = mpsc::channel();
        thread::spawn(move || {
            loop {
                let mut uzunluk = 0;
                loop {
                    let mut satir = String::new();
                    if okuyucu.read_line(&mut satir).unwrap_or(0) == 0 {
                        return;
                    }
                    let satir = satir.trim_end();
                    if satir.is_empty() {
                        break;
                    }
                    if let Some(d) = satir.strip_prefix("Content-Length:") {
                        uzunluk = d.trim().parse().unwrap();
                    }
                }
                let mut govde = vec![0; uzunluk];
                okuyucu.read_exact(&mut govde).unwrap();
                if gonderici.send(serde_json::from_slice(&govde).unwrap()).is_err() {
                    return;
                }
            }
        });
        Istemci { surec, stdin, gelenler, sonraki_id: 1 }
    }

    fn gonder(&mut self, mesaj: Value) {
        let govde = serde_json::to_vec(&mesaj).unwrap();
        write!(self.stdin, "Content-Length: {}\r\n\r\n", govde.len()).unwrap();
        self.stdin.write_all(&govde).unwrap();
        self.stdin.flush().unwrap();
    }

    fn bildir(&mut self, yontem: &str, parametreler: Value) {
        self.gonder(json!({ "jsonrpc": "2.0", "method": yontem, "params": parametreler }));
    }

    fn iste(&mut self, yontem: &str, parametreler: Value) -> Value {
        let id = self.sonraki_id;
        self.sonraki_id += 1;
        self.gonder(json!({ "jsonrpc": "2.0", "id": id, "method": yontem, "params": parametreler }));
        loop {
            let mesaj = self.gelenler.recv_timeout(Duration::from_secs(120)).expect("dil sunucusu cevap vermedi");
            if mesaj["id"] == id {
                assert!(mesaj.get("error").is_none(), "{mesaj}");
                return mesaj["result"].clone();
            }
        }
    }

    /// Verilen koşulu sağlayan bir bildirim gelene kadar bekler.
    fn bildirim_bekle(&mut self, yontem: &str, kosul: impl Fn(&Value) -> bool) -> Value {
        loop {
            let mesaj = self.gelenler.recv_timeout(Duration::from_secs(120)).expect("beklenen bildirim gelmedi");
            if mesaj["method"] == yontem && kosul(&mesaj["params"]) {
                return mesaj["params"].clone();
            }
        }
    }

    /// rust-analyzer projeyi dizinlerken boş cevap verebilir; dolu cevap gelene kadar sorar.
    fn dolu_cevap_bekle(&mut self, yontem: &str, parametreler: Value) -> Value {
        let baslangic = Instant::now();
        loop {
            let sonuc = self.iste(yontem, parametreler.clone());
            let ogeler = sonuc.get("items").unwrap_or(&sonuc);
            let bos = sonuc.is_null() || ogeler.as_array().is_some_and(Vec::is_empty);
            if !bos || baslangic.elapsed() > Duration::from_secs(120) {
                return sonuc;
            }
            thread::sleep(Duration::from_millis(300));
        }
    }
}

impl Drop for Istemci {
    fn drop(&mut self) {
        let _ = self.surec.kill();
    }
}

fn rust_analyzer_var() -> bool {
    Command::new("rust-analyzer").arg("--version").output().is_ok_and(|c| c.status.success())
}

fn uri(yol: &Path) -> String {
    let yol = yol.display().to_string().replace('\\', "/");
    if yol.starts_with('/') { format!("file://{yol}") } else { format!("file:///{yol}") }
}

/// LSP konumu (UTF-16) → metindeki (satır, sütun) olarak beklenen değer.
fn konum(metin: &str, aranan: &str) -> Value {
    let ofset = metin.find(aranan).unwrap();
    let satir = metin[..ofset].matches('\n').count();
    let satir_basi = metin[..ofset].rfind('\n').map_or(0, |i| i + 1);
    json!({ "line": satir, "character": metin[satir_basi..ofset].encode_utf16().count() })
}

#[test]
fn tanima_gitme_hover_ve_tamamlama() {
    if !rust_analyzer_var() {
        eprintln!("rust-analyzer kurulu değil; dil sunucusu testi atlandı");
        return;
    }
    let dizin = Path::new(env!("CARGO_TARGET_TMPDIR")).join("dil-sunucusu");
    let _ = fs::remove_dir_all(&dizin);
    fs::create_dir_all(dizin.join("src")).unwrap();
    fs::write(
        dizin.join("Cargo.toml"),
        "[package]\nname = \"dil-sunucusu\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n\
         [[bin]]\nname = \"dil-sunucusu\"\npath = \"target/pas/src/main.rs\"\n",
    )
    .unwrap();
    let hesap = "/// İki sayıyı toplar.\ngenel işlev topla(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
    fs::write(dizin.join("src/hesap.pas"), hesap).unwrap();
    let ana = "modül hesap;\n\nişlev ana() {\n    tanımla şu = hesap::topla(1, 2);\n    tanımla değişken l = liste![şu];\n    l.ek\n}\n";
    fs::write(dizin.join("src/ana.pas"), ana).unwrap();

    let mut istemci = Istemci::baslat();
    let baslatma = istemci.iste("initialize", json!({ "processId": null, "rootUri": uri(&dizin), "capabilities": {} }));
    assert_eq!(baslatma["capabilities"]["definitionProvider"], true);
    istemci.bildir("initialized", json!({}));
    let ana_uri = uri(&dizin.join("src/ana.pas"));
    istemci.bildir(
        "textDocument/didOpen",
        json!({ "textDocument": { "uri": ana_uri, "languageId": "pas", "version": 1, "text": ana } }),
    );

    // Tanıma gitme: `topla` çağrısından `hesap.pas` içindeki tanıma.
    let tanim = istemci.dolu_cevap_bekle(
        "textDocument/definition",
        json!({ "textDocument": { "uri": ana_uri }, "position": konum(ana, "topla(1") }),
    );
    let tanim = if tanim.is_array() { tanim[0].clone() } else { tanim };
    let hedef = tanim.get("targetUri").or(tanim.get("uri")).and_then(Value::as_str).unwrap_or_default();
    assert!(hedef.ends_with("/src/hesap.pas"), "{tanim}");
    let aralik = tanim.get("targetSelectionRange").or(tanim.get("range")).unwrap();
    assert_eq!(aralik["start"], konum(hesap, "topla("), "{tanim}");

    // Hover Pas sözcükleriyle gösterilir.
    let hover = istemci.dolu_cevap_bekle(
        "textDocument/hover",
        json!({ "textDocument": { "uri": ana_uri }, "position": konum(ana, "topla(1") }),
    );
    let icerik = hover["contents"]["value"].as_str().unwrap_or_default();
    assert!(icerik.contains("işlev topla(a: i32, b: i32) -> i32"), "{hover}");
    assert!(icerik.contains("İki sayıyı toplar."), "{hover}");

    // Sözlükteki sözcüklerde önce Türkçe açıklama gelir.
    let hover = istemci.dolu_cevap_bekle(
        "textDocument/hover",
        json!({ "textDocument": { "uri": ana_uri }, "position": konum(ana, "liste![") }),
    );
    let icerik = hover["contents"]["value"].as_str().unwrap_or_default();
    assert!(icerik.starts_with("**`liste!`** (Rust'ta `vec!`): liste oluşturur"), "{hover}");

    // Tamamlama: `l.ek` yazılmışken `ekle` önerilir ve yazılmış `ek` değiştirilir.
    let imlec = konum(ana, "l.ek\n");
    let imlec = json!({ "line": imlec["line"], "character": imlec["character"].as_u64().unwrap() + 4 });
    let tamamlama = istemci
        .dolu_cevap_bekle("textDocument/completion", json!({ "textDocument": { "uri": ana_uri }, "position": imlec }));
    let ogeler = tamamlama.get("items").unwrap_or(&tamamlama).as_array().cloned().unwrap_or_default();
    let ekle =
        ogeler.iter().find(|o| o["label"].as_str().is_some_and(|e| e.starts_with("ekle"))).unwrap_or_else(|| {
            panic!(
                "`ekle` önerilmedi: {:?}",
                ogeler.iter().map(|o| o["label"].as_str().unwrap_or("")).collect::<Vec<_>>()
            )
        });
    let duzenleme = &ekle["textEdit"];
    let degisen = duzenleme.get("range").or(duzenleme.get("replace")).unwrap();
    assert_eq!(degisen["start"]["character"].as_u64(), Some(imlec["character"].as_u64().unwrap() - 2), "{ekle}");
    assert_eq!(degisen["end"], imlec, "{ekle}");
    assert!(duzenleme["newText"].as_str().unwrap().starts_with("ekle"), "{ekle}");
    let belge = ekle["documentation"]["value"].as_str().unwrap_or_default();
    assert!(belge.starts_with("**`ekle`** (Rust'ta `push`): listenin sonuna ekler"), "{ekle}");

    istemci.iste("shutdown", Value::Null);
    istemci.bildir("exit", Value::Null);
}

#[test]
fn turkce_tanilama_ve_hizli_duzeltme() {
    let dizin = Path::new(env!("CARGO_TARGET_TMPDIR")).join("dil-sunucusu-tanilama");
    let _ = fs::remove_dir_all(&dizin);
    fs::create_dir_all(dizin.join("src")).unwrap();
    fs::write(dizin.join("Cargo.toml"), "[package]\nname = \"tanilama\"\nversion = \"0.1.0\"\nedition = \"2024\"\n")
        .unwrap();
    // Diskteki dosya hatasız; hatalı hali editörde, kaydedilmemiş.
    fs::write(dizin.join("src/ana.pas"), "işlev ana() {}\n").unwrap();
    let ana = "işlev ana() {\n    tanımla ş = 1;\n    ş = 2;\n    yazdır!(\"{ş}\");\n}\n";

    let mut istemci = Istemci::baslat();
    let baslatma = istemci.iste(
        "initialize",
        json!({ "processId": null, "rootUri": uri(&dizin), "capabilities": {}, "initializationOptions": { "tanilamalar": true } }),
    );
    assert_eq!(baslatma["capabilities"]["codeActionProvider"]["codeActionKinds"][0], "quickfix");
    istemci.bildir("initialized", json!({}));
    let ana_uri = uri(&dizin.join("src/ana.pas"));
    istemci.bildir(
        "textDocument/didOpen",
        json!({ "textDocument": { "uri": ana_uri, "languageId": "pas", "version": 1, "text": ana } }),
    );

    let tanilamalar = istemci.bildirim_bekle("textDocument/publishDiagnostics", |p| {
        p["uri"] == ana_uri && p["diagnostics"].as_array().is_some_and(|d| !d.is_empty())
    });
    let hata = tanilamalar["diagnostics"].as_array().unwrap().iter().find(|d| d["code"] == "E0384").cloned().unwrap();
    assert_eq!(hata["severity"], 1);
    assert_eq!(hata["range"]["start"], konum(ana, "ş = 2"));
    assert!(hata["message"].as_str().unwrap().contains("`ş` değiştirilemez"), "{hata}");

    let eylemler = istemci.iste(
        "textDocument/codeAction",
        json!({ "textDocument": { "uri": ana_uri }, "range": hata["range"], "context": { "diagnostics": [hata] } }),
    );
    let eylem = &eylemler[0];
    assert_eq!(eylem["title"], "Ekle: değişken", "{eylemler}");
    let degisiklik = &eylem["edit"]["changes"][ana_uri.as_str()][0];
    assert_eq!(degisiklik["newText"], "değişken ");
    assert_eq!(degisiklik["range"]["start"], konum(ana, "ş = 1"));

    istemci.iste("shutdown", Value::Null);
    istemci.bildir("exit", Value::Null);
}
