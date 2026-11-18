//! Dil sunucusu üzerinden Türkçe tanılamalar ve hızlı düzeltmeler.
//!
//! Denetim, sunucunun kendisini `cargo-pas denetle --json --stdin` olarak
//! çalıştırmasıyla yapılır; sonuçlar LSP tanılamalarına, derleyicinin
//! önerileri de `codeAction` düzeltmelerine çevrilir. RustRover eklentisi bunu
//! kendisi yaptığı için özellik isteğe bağlıdır (`tanilamalar: true`).

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use serde_json::{Value, json};

use super::Olay;
use super::konum::Metin;

/// Bir denetimin sonucu.
pub struct DenetimSonucu {
    pub kok: PathBuf,
    /// Denetime standart girdiden verilen dosya ve o anki metni.
    pub pas_yolu: PathBuf,
    pub metin: String,
    /// `cargo pas denetle --json` satırları
    pub tanilar: Vec<Value>,
}

/// Projeyi arka planda denetler; bitince `Olay::Denetim` gönderir.
pub fn denetle(kok: PathBuf, pas_yolu: PathBuf, metin: String, gonderici: mpsc::Sender<Olay>) {
    thread::spawn(move || {
        let tanilar = calistir(&kok, &pas_yolu, &metin).unwrap_or_default();
        let _ = gonderici.send(Olay::Denetim(DenetimSonucu { kok, pas_yolu, metin, tanilar }));
    });
}

fn calistir(kok: &Path, pas_yolu: &Path, metin: &str) -> Option<Vec<Value>> {
    let goreli = pas_yolu.strip_prefix(kok).ok()?.to_string_lossy().replace('\\', "/");
    let mut surec = Command::new(std::env::current_exe().ok()?)
        .args(["denetle", "--json", "--stdin", &goreli])
        .current_dir(kok)
        .env("NO_COLOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    surec.stdin.take()?.write_all(metin.as_bytes()).ok()?;
    let cikti = surec.wait_with_output().ok()?;
    Some(String::from_utf8_lossy(&cikti.stdout).lines().filter_map(|s| serde_json::from_str(s).ok()).collect())
}

/// `cargo pas denetle --json` konumu (1'den başlayan satır ve karakter
/// sütunu) → LSP aralığı (0'dan başlayan satır, UTF-16 sütunu).
fn aralik(metin: &Metin, konum: &Value) -> Value {
    let nokta = |satir: &str, sutun: &str| {
        let ofset = metin.karakter_ofseti(konum[satir].as_u64().unwrap_or(1), konum[sutun].as_u64().unwrap_or(1));
        let (satir, sutun) = metin.utf16_konumu(ofset);
        json!({ "line": satir, "character": sutun })
    };
    json!({ "start": nokta("satir", "sutun"), "end": nokta("bitis_satiri", "bitis_sutunu") })
}

/// Bir dosyanın tanılamalarını LSP biçimine çevirir. Derleyicinin önerileri,
/// `codeAction` isteğinde düzeltmeye dönüştürülmek üzere `data` alanında taşınır.
pub fn lsp_tanilamalari(tanilar: &[Value], goreli: &str, metin: &str) -> Vec<Value> {
    let metin = Metin::yeni(metin);
    tanilar
        .iter()
        .filter(|t| t["dosya"] == goreli)
        .map(|t| {
            let mut mesaj = t["mesaj"].as_str().unwrap_or_default().to_owned();
            for not in t["notlar"].as_array().into_iter().flatten().filter_map(Value::as_str) {
                mesaj.push('\n');
                mesaj.push_str(not);
            }
            if let Some(aciklama) = t["aciklama"].as_str() {
                mesaj.push_str("\n\n");
                mesaj.push_str(aciklama);
            }
            let oneriler: Vec<Value> = t["oneriler"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|oneri| {
                    let degisiklikler: Vec<Value> = oneri["degisiklikler"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .map(|d| json!({ "range": aralik(&metin, &d["aralik"]), "newText": d["metin"] }))
                        .collect();
                    json!({ "mesaj": oneri["mesaj"], "kesin": oneri["kesin"], "degisiklikler": degisiklikler })
                })
                .collect();
            json!({
                "range": aralik(&metin, &t["aralik"]),
                "severity": match t["seviye"].as_str() {
                    Some("hata") => 1,
                    Some("uyarı") => 2,
                    _ => 3,
                },
                "code": t["kod"],
                "source": "pas",
                "message": mesaj,
                "data": { "oneriler": oneriler },
            })
        })
        .collect()
}

/// `textDocument/codeAction`: istekte gelen tanılamalardaki öneriler.
pub fn duzeltmeler(uri: &str, tanilamalar: &[Value]) -> Vec<Value> {
    let mut eylemler = Vec::new();
    for tanilama in tanilamalar {
        for oneri in tanilama["data"]["oneriler"].as_array().into_iter().flatten() {
            let degisiklikler = oneri["degisiklikler"].as_array().cloned().unwrap_or_default();
            eylemler.push(json!({
                "title": baslik(oneri, &degisiklikler),
                "kind": "quickfix",
                "diagnostics": [tanilama],
                "isPreferred": oneri["kesin"] == true,
                "edit": { "changes": { uri: degisiklikler } },
            }));
        }
    }
    eylemler
}

/// RustRover eklentisindeki gibi: tek bir eklemeyse "Ekle: …", tek bir
/// değiştirmeyse "Şununla değiştir: …", değilse derleyicinin mesajı.
fn baslik(oneri: &Value, degisiklikler: &[Value]) -> String {
    match degisiklikler {
        [tek] => {
            let metin = tek["newText"].as_str().unwrap_or_default().trim();
            if tek["range"]["start"] == tek["range"]["end"] {
                format!("Ekle: {metin}")
            } else {
                format!("Şununla değiştir: {metin}")
            }
        }
        _ => format!("Düzelt: {}", oneri["mesaj"].as_str().unwrap_or_default().replace('`', "")),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    fn ornek_tani() -> Value {
        json!({
            "seviye": "hata", "kod": "E0384", "mesaj": "`ş` değiştirilemez", "dosya": "src/ana.pas",
            "aralik": { "satir": 3, "sutun": 5, "bitis_satiri": 3, "bitis_sutunu": 10 },
            "notlar": ["yardım: başına `değişken` ekle"], "aciklama": "Açıklama.",
            "oneriler": [{ "mesaj": "başına `değişken` ekle", "kesin": true, "degisiklikler": [
                { "aralik": { "satir": 2, "sutun": 9, "bitis_satiri": 2, "bitis_sutunu": 9 }, "metin": "değişken " }
            ]}],
        })
    }

    #[test]
    fn tanilamalar_lsp_bicimine_cevrilir() {
        // "ş" 1 karakter ama 2 bayt; sütunlar karakter olarak sayılır.
        let metin = "işlev ana() {\n    yap ş = 1;\n    ş = 2;\n}\n";
        let lsp = lsp_tanilamalari(&[ornek_tani()], "src/ana.pas", metin);
        assert_eq!(lsp.len(), 1);
        assert_eq!(
            lsp[0]["range"],
            json!({ "start": { "line": 2, "character": 4 }, "end": { "line": 2, "character": 9 } })
        );
        assert_eq!(lsp[0]["severity"], 1);
        assert_eq!(lsp[0]["message"], "`ş` değiştirilemez\nyardım: başına `değişken` ekle\n\nAçıklama.");
        let degisiklik = &lsp[0]["data"]["oneriler"][0]["degisiklikler"][0];
        assert_eq!(degisiklik["range"]["start"], json!({ "line": 1, "character": 8 }));
        assert!(lsp_tanilamalari(&[ornek_tani()], "src/baska.pas", metin).is_empty());

        let eylemler = duzeltmeler("file:///a/src/ana.pas", &lsp);
        assert_eq!(eylemler[0]["title"], "Ekle: değişken");
        assert_eq!(eylemler[0]["isPreferred"], true);
        assert_eq!(eylemler[0]["edit"]["changes"]["file:///a/src/ana.pas"][0]["newText"], "değişken ");
    }
}
