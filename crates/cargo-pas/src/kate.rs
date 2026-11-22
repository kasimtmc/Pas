//! `cargo pas kate`: Kate düzenleyicisini Pas için hazırlar.
//!
//! Kate'te eklenti mağazası yoktur; yeni bir dil üç ayarla tanıtılır:
//! renklendirme tanımı (KSyntaxHighlighting XML), dil sunucusu ayarı
//! (`kate/lspclient/settings.json`) ve projedeki `.kateproject` dosyası
//! (Derleme eklentisinin Çalıştır hedefi).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use pas_cekirdek::sozluk::{GIRDILER, Sinif};
use serde_json::{Value, json};

use crate::gorunum::Gorunum;
use crate::proje::Proje;

/// Renklendirme tanımının adı; dil sunucusu ayarı bu adla eşleşir.
const DIL_ADI: &str = "Pas";

const RUST_ANAHTAR_SOZCUKLERI: &[&str] = &[
    "as",
    "async",
    "await",
    "break",
    "const",
    "continue",
    "crate",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "macro_rules",
    "union",
];

const TEMEL_TURLER: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "bool",
    "char", "str",
];

pub fn kur(argumanlar: &[String], gorunum: Gorunum) -> Result<ExitCode, String> {
    if !argumanlar.is_empty() {
        return Err("kullanım: cargo pas kate".into());
    }
    let yollar = Yollar::bul()?;

    let sozdizimi_yolu = yollar.veri.join("org.kde.syntax-highlighting/syntax/pas.xml");
    dosya_yaz(&sozdizimi_yolu, &sozdizimi())?;
    gorunum.durum("Kuruldu", &format!("renklendirme ({})", sozdizimi_yolu.display()));

    let cargo_pas = std::env::current_exe().map_err(|e| format!("cargo-pas'ın yeri bulunamadı: {e}"))?;
    let ayar_yolu = yollar.ayar.join("kate/lspclient/settings.json");
    let eski = match fs::read_to_string(&ayar_yolu) {
        Ok(metin) => Some(metin),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => return Err(format!("`{}` okunamadı: {e}", ayar_yolu.display())),
    };
    let yeni = lsp_ayarina_ekle(eski.as_deref(), &cargo_pas.to_string_lossy())
        .map_err(|e| format!("`{}` dosyası anlaşılamadı ({e}); dosyayı düzeltip yeniden dene", ayar_yolu.display()))?;
    dosya_yaz(&ayar_yolu, &yeni)?;
    gorunum.durum("Kuruldu", &format!("dil sunucusu ({})", ayar_yolu.display()));

    // Bir Pas projesinin içindeysek Çalıştır hedefini de ekle.
    if let Ok(proje) = std::env::current_dir().map_err(|e| e.to_string()).and_then(|d| Proje::bul(&d)) {
        let yol = proje.kok.join(".kateproject");
        if yol.exists() {
            gorunum.durum("Atlandı", &format!("`{}` zaten var", yol.display()));
        } else {
            let ad = proje.paket_adi.as_deref().unwrap_or("pas");
            dosya_yaz(&yol, &proje_dosyasi(ad))?;
            gorunum.durum("Oluşturuldu", &format!("Çalıştır hedefi ({})", yol.display()));
        }
    }

    eprintln!(
        "\nKate'i kapatıp yeniden aç. Sonra:\n  \
         • `.pas` dosyaları renkli görünür; hatalar Türkçe altı çizili gelir.\n  \
         • Çalıştırmak için: Derleme panelinde `Çalıştır` hedefini seç ve\n    \
           \"Derle ve Çalıştır\"a bas. Program alttaki uçbirimde çalışır.\n    \
           (Derleme paneli yoksa: Ayarlar → Kate'i Yapılandır → Eklentiler →\n    \
           \"Derleme ve Çalıştırma\" eklentisini aç.)\n  \
         • Yeni projeler (`cargo pas yeni`) Çalıştır hedefiyle birlikte gelir;\n    \
           eski bir projeye eklemek için o projenin içinde `cargo pas kate` yaz."
    );
    Ok(ExitCode::SUCCESS)
}

fn dosya_yaz(yol: &Path, icerik: &str) -> Result<(), String> {
    if let Some(ust) = yol.parent() {
        fs::create_dir_all(ust).map_err(|e| format!("`{}` oluşturulamadı: {e}", ust.display()))?;
    }
    fs::write(yol, icerik).map_err(|e| format!("`{}` yazılamadı: {e}", yol.display()))
}

/// Kate'in (Qt'nin) ayar ve veri dizinleri.
struct Yollar {
    ayar: PathBuf,
    veri: PathBuf,
}

impl Yollar {
    fn bul() -> Result<Yollar, String> {
        let ortam = |ad: &str| std::env::var_os(ad).filter(|d| !d.is_empty()).map(PathBuf::from);
        if cfg!(windows) {
            let yerel = ortam("LOCALAPPDATA").ok_or("LOCALAPPDATA tanımlı değil")?;
            return Ok(Yollar { ayar: yerel.clone(), veri: yerel });
        }
        let ev = ortam("HOME").ok_or("HOME tanımlı değil")?;
        if cfg!(target_os = "macos") {
            return Ok(Yollar { ayar: ev.join("Library/Preferences"), veri: ev.join("Library/Application Support") });
        }
        Ok(Yollar {
            ayar: ortam("XDG_CONFIG_HOME").unwrap_or_else(|| ev.join(".config")),
            veri: ortam("XDG_DATA_HOME").unwrap_or_else(|| ev.join(".local/share")),
        })
    }
}

/// Kate'in dil sunucusu ayarlarına Pas sunucusunu ekler; öteki ayarlara dokunmaz.
pub fn lsp_ayarina_ekle(eski: Option<&str>, cargo_pas: &str) -> Result<String, String> {
    let mut ayarlar = match eski.map(str::trim) {
        None | Some("") => json!({}),
        Some(metin) => serde_json::from_str::<Value>(metin).map_err(|e| e.to_string())?,
    };
    let Some(nesne) = ayarlar.as_object_mut() else { return Err("en dışta bir JSON nesnesi bekleniyordu".into()) };
    let sunucular = nesne.entry("servers").or_insert_with(|| json!({}));
    let Some(sunucular) = sunucular.as_object_mut() else { return Err("`servers` bir JSON nesnesi olmalı".into()) };
    sunucular.insert(
        "pas".into(),
        json!({
            "command": [cargo_pas, "lsp"],
            // Türkçe hata gösterimini (ve hızlı düzeltmeleri) açar.
            "initializationOptions": { "tanilamalar": true },
            "rootIndicationFileNames": ["Cargo.toml"],
            "highlightingModeRegex": format!("^{DIL_ADI}$"),
            "url": "https://github.com/kasimtmc/Pas",
        }),
    );
    Ok(serde_json::to_string_pretty(&ayarlar).expect("JSON") + "\n")
}

/// Kate projesi: dosya ağacı ve Derleme eklentisi için Çalıştır hedefi.
/// Çalıştırma komutu Kate'in uçbiriminde çalışır; `oku!` ile girdi alınabilir.
pub fn proje_dosyasi(ad: &str) -> String {
    let proje = json!({
        "name": ad,
        "files": [{ "directory": "src", "filters": ["*.pas"], "recursive": 1 }],
        "build": {
            "directory": ".",
            "default_target": "Çalıştır",
            "targets": [
                { "name": "Çalıştır", "build_cmd": "cargo pas derle", "run_cmd": "cargo pas çalıştır" },
                { "name": "Denetle", "build_cmd": "cargo pas denetle" },
            ],
        },
    });
    serde_json::to_string_pretty(&proje).expect("JSON") + "\n"
}

/// Pas sözlüğünden KSyntaxHighlighting renklendirme tanımı üretir.
pub fn sozdizimi() -> String {
    let liste = |ad: &str, sozcukler: &[&str]| {
        let mut satirlar = format!("    <list name=\"{ad}\">\n");
        for sozcuk in sozcukler {
            satirlar.push_str(&format!("      <item>{}</item>\n", xml_kacir(sozcuk)));
        }
        satirlar + "    </list>\n"
    };
    let sinif = |tur: Sinif| GIRDILER.iter().filter(|g| g.tur == tur).map(|g| g.pas).collect::<Vec<_>>();
    let mut anahtar = sinif(Sinif::AnahtarSozcuk);
    for sozcuk in RUST_ANAHTAR_SOZCUKLERI {
        if !anahtar.contains(sozcuk) {
            anahtar.push(sozcuk);
        }
    }
    let mut turler = sinif(Sinif::Tur);
    turler.extend(TEMEL_TURLER);
    let listeler = liste("anahtar", &anahtar) + &liste("turler", &turler) + &liste("degerler", &sinif(Sinif::Deger));

    // Kate tanımları sürüm numarasıyla önbelleğe alır: 0.10.1 → 1001.
    let surum = env!("CARGO_PKG_VERSION")
        .split('.')
        .map(|p| p.parse::<u32>().unwrap_or(0))
        .fold(0, |toplam, parca| toplam * 100 + parca);

    format!(
        r##"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE language>
<!-- `cargo pas kate` tarafından Pas sözlüğünden üretildi; elle değiştirme. -->
<!-- Pascal da `.pas` uzantısını kullanır; yüksek öncelik Pas'ı seçtirir. -->
<language name="{DIL_ADI}" section="Sources" version="{surum}" kateversion="5.79" extensions="*.pas" priority="10" author="Pas (https://github.com/kasimtmc/Pas)" license="MIT">
  <highlighting>
{listeler}    <contexts>
      <context name="Kod" attribute="Düz Metin" lineEndContext="#stay">
        <DetectSpaces/>
        <StringDetect String="///" attribute="Belge Yorumu" context="BelgeYorumu"/>
        <StringDetect String="//!" attribute="Belge Yorumu" context="BelgeYorumu"/>
        <Detect2Chars char="/" char1="/" attribute="Yorum" context="Yorum"/>
        <Detect2Chars char="/" char1="*" attribute="Yorum" context="BlokYorum" beginRegion="yorum"/>
        <RegExpr String="[bc]?r(#*)&quot;" attribute="Dizgi" context="HamDizgi"/>
        <RegExpr String="[bc]?&quot;" attribute="Dizgi" context="Dizgi"/>
        <RegExpr String="b?'(?:\\(?:u\{{[0-9a-fA-F]+\}}|.)|[^'\\])'" attribute="Karakter"/>
        <RegExpr String="'[\p{{L}}_][\p{{L}}\p{{N}}_]*" attribute="Ömür"/>
        <RegExpr String="(?:0x[0-9a-fA-F_]+|0o[0-7_]+|0b[01_]+|[0-9][0-9_]*(?:\.[0-9][0-9_]*)?(?:[eE][+-]?[0-9]+)?)(?:[iuf](?:8|16|32|64|128|size))?" attribute="Sayı"/>
        <RegExpr String="#!?\[" attribute="Öznitelik" context="Oznitelik"/>
        <RegExpr String="(?:işlev|fn)(?![\p{{L}}\p{{N}}_])" attribute="Anahtar Sözcük" context="IslevAdi"/>
        <RegExpr String="[\p{{L}}_][\p{{L}}\p{{N}}_]*!(?!=)" attribute="Makro"/>
        <keyword String="anahtar" attribute="Anahtar Sözcük"/>
        <keyword String="turler" attribute="Hazır Tür"/>
        <keyword String="degerler" attribute="Sabit"/>
        <RegExpr String="\p{{Lu}}[\p{{L}}\p{{N}}_]*" attribute="Tür"/>
        <RegExpr String="[\p{{L}}_][\p{{L}}\p{{N}}_]*" attribute="Düz Metin"/>
        <DetectChar char="{{" attribute="Düz Metin" beginRegion="blok"/>
        <DetectChar char="}}" attribute="Düz Metin" endRegion="blok"/>
        <RegExpr String="::|\.\.=?|=&gt;|-&gt;|[-+*/%=!&lt;&gt;&amp;|^]=?" attribute="İşleç"/>
      </context>
      <context name="IslevAdi" attribute="Düz Metin" lineEndContext="#pop" fallthroughContext="#pop">
        <DetectSpaces/>
        <RegExpr String="[\p{{L}}_][\p{{L}}\p{{N}}_]*" attribute="İşlev" context="#pop"/>
      </context>
      <context name="Yorum" attribute="Yorum" lineEndContext="#pop">
        <DetectSpaces/>
      </context>
      <context name="BelgeYorumu" attribute="Belge Yorumu" lineEndContext="#pop">
        <DetectSpaces/>
        <RegExpr String="`[^`]*`" attribute="Belge Kodu"/>
      </context>
      <context name="BlokYorum" attribute="Yorum" lineEndContext="#stay">
        <Detect2Chars char="*" char1="/" attribute="Yorum" context="#pop" endRegion="yorum"/>
        <Detect2Chars char="/" char1="*" attribute="Yorum" context="BlokYorum" beginRegion="yorum"/>
      </context>
      <context name="Dizgi" attribute="Dizgi" lineEndContext="#stay">
        <RegExpr String="\\(?:u\{{[0-9a-fA-F]+\}}|.)" attribute="Kaçış"/>
        <RegExpr String="\{{\{{|\}}\}}" attribute="Kaçış"/>
        <RegExpr String="\{{[^{{}}&quot;]*\}}" attribute="Yer Tutucu"/>
        <DetectChar char="&quot;" attribute="Dizgi" context="#pop"/>
      </context>
      <context name="HamDizgi" attribute="Dizgi" lineEndContext="#stay" dynamic="true">
        <StringDetect String="&quot;%1" attribute="Dizgi" context="#pop" dynamic="true"/>
      </context>
      <context name="Oznitelik" attribute="Öznitelik" lineEndContext="#stay">
        <DetectChar char="]" attribute="Öznitelik" context="#pop"/>
      </context>
    </contexts>
    <itemDatas>
      <itemData name="Düz Metin" defStyleNum="dsNormal" spellChecking="false"/>
      <itemData name="Anahtar Sözcük" defStyleNum="dsKeyword" spellChecking="false"/>
      <itemData name="Hazır Tür" defStyleNum="dsDataType" spellChecking="false"/>
      <itemData name="Tür" defStyleNum="dsDataType" spellChecking="false"/>
      <itemData name="Sabit" defStyleNum="dsConstant" spellChecking="false"/>
      <itemData name="İşlev" defStyleNum="dsFunction" spellChecking="false"/>
      <itemData name="Makro" defStyleNum="dsPreprocessor" spellChecking="false"/>
      <itemData name="Sayı" defStyleNum="dsDecVal" spellChecking="false"/>
      <itemData name="Dizgi" defStyleNum="dsString"/>
      <itemData name="Kaçış" defStyleNum="dsSpecialChar" spellChecking="false"/>
      <itemData name="Yer Tutucu" defStyleNum="dsSpecialString" spellChecking="false"/>
      <itemData name="Karakter" defStyleNum="dsChar" spellChecking="false"/>
      <itemData name="Ömür" defStyleNum="dsAttribute" spellChecking="false"/>
      <itemData name="Öznitelik" defStyleNum="dsAttribute" spellChecking="false"/>
      <itemData name="İşleç" defStyleNum="dsOperator" spellChecking="false"/>
      <itemData name="Yorum" defStyleNum="dsComment"/>
      <itemData name="Belge Yorumu" defStyleNum="dsDocumentation"/>
      <itemData name="Belge Kodu" defStyleNum="dsSpecialString" spellChecking="false"/>
    </itemDatas>
  </highlighting>
  <general>
    <comments>
      <comment name="singleLine" start="//" position="afterwhitespace"/>
      <comment name="multiLine" start="/*" end="*/" region="yorum"/>
    </comments>
    <keywords casesensitive="1"/>
  </general>
</language>
"##
    )
}

fn xml_kacir(metin: &str) -> String {
    metin.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn sozdizimi_sozlugu_icerir() {
        let xml = sozdizimi();
        for sozcuk in ["işlev", "tanımla", "için", "içinde", "eşle", "fn", "let"] {
            assert!(xml.contains(&format!("<item>{sozcuk}</item>")), "{sozcuk} yok");
        }
        assert!(
            xml.contains("<item>Metin</item>") && xml.contains("<item>u32</item>") && xml.contains("<item>Var</item>")
        );
        assert!(xml.contains(r#"extensions="*.pas""#));
        assert_eq!(xml.matches("<item>fn</item>").count(), 1);
        // format! kaçışlarından sonra düzenli ifadelerde tek süslü parantez kalmalı.
        assert!(xml.contains(r"\p{L}") && !xml.contains("{{L"));
    }

    #[test]
    fn lsp_ayari_otekilere_dokunmaz() {
        let eski = r#"{"servers": {"python": {"command": ["pylsp"]}}, "semanticHighlighting": true}"#;
        let yeni: Value = serde_json::from_str(&lsp_ayarina_ekle(Some(eski), "/ev/cargo-pas").unwrap()).unwrap();
        assert_eq!(yeni["servers"]["python"]["command"][0], "pylsp");
        assert_eq!(yeni["semanticHighlighting"], true);
        assert_eq!(yeni["servers"]["pas"]["command"], json!(["/ev/cargo-pas", "lsp"]));
        assert_eq!(yeni["servers"]["pas"]["highlightingModeRegex"], "^Pas$");
        assert_eq!(yeni["servers"]["pas"]["initializationOptions"]["tanilamalar"], true);

        // İkinci kez kurmak aynı sonucu verir.
        let tekrar = lsp_ayarina_ekle(Some(&yeni.to_string()), "/ev/cargo-pas").unwrap();
        assert_eq!(serde_json::from_str::<Value>(&tekrar).unwrap(), yeni);
    }

    #[test]
    fn lsp_ayari_bos_ya_da_bozuk() {
        let yeni: Value = serde_json::from_str(&lsp_ayarina_ekle(None, "cargo-pas").unwrap()).unwrap();
        assert!(yeni["servers"]["pas"].is_object());
        assert!(lsp_ayarina_ekle(Some("{ bozuk"), "cargo-pas").is_err());
        assert!(lsp_ayarina_ekle(Some("[]"), "cargo-pas").is_err());
    }

    #[test]
    fn proje_dosyasi_calistir_hedefi() {
        let proje: Value = serde_json::from_str(&proje_dosyasi("ciftlik")).unwrap();
        assert_eq!(proje["name"], "ciftlik");
        assert_eq!(proje["build"]["targets"][0]["run_cmd"], "cargo pas çalıştır");
    }
}
