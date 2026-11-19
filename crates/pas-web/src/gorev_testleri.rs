//! `dersler/*/gorevler.md` dosyalarındaki görevlerin denetimi: her görevin
//! çözümü, oyun alanındaki gibi derlenip çalıştırıldığında beklenen çıktıyı
//! vermeli; başlangıç kodu ise derlenmeli ama görevi kendiliğinden çözmemeli.

use std::path::Path;

use super::testler::calistir;

/// Görev dosyasının biçimi `betikler/oyun-alani.sh` içindeki okuyucuyla aynıdır.
#[derive(Debug, Default)]
struct Gorev {
    baslik: String,
    baslangic: String,
    cozum: String,
    /// (girdi, beklenen çıktı)
    ornekler: Vec<(String, String)>,
}

fn gorevleri_oku(metin: &str) -> Vec<Gorev> {
    let mut gorevler: Vec<Gorev> = Vec::new();
    let mut satirlar = metin.lines();
    let mut girdi = String::new();
    while let Some(satir) = satirlar.next() {
        if let Some(baslik) = satir.strip_prefix("## ") {
            gorevler.push(Gorev { baslik: baslik.to_owned(), ..Gorev::default() });
            continue;
        }
        let Some(bilgi) = satir.strip_prefix("```") else { continue };
        let blok: Vec<&str> = satirlar.by_ref().take_while(|s| !s.starts_with("```")).collect();
        let blok = blok.join("\n");
        let Some(gorev) = gorevler.last_mut() else { continue };
        match bilgi.trim() {
            "pas başlangıç" => gorev.baslangic = blok,
            "pas çözüm" => gorev.cozum = blok,
            "girdi" => girdi = blok,
            "çıktı" => gorev.ornekler.push((std::mem::take(&mut girdi), blok)),
            _ => {}
        }
    }
    gorevler
}

/// Satır sonlarındaki boşluklar ve sondaki boş satırlar önemsizdir.
fn duzelt(cikti: &str) -> String {
    cikti.lines().map(str::trim_end).collect::<Vec<_>>().join("\n").trim_end_matches('\n').to_owned()
}

fn gorevi_denetle(ders: &str, gorev: &Gorev) -> Vec<String> {
    let mut sorunlar = Vec::new();
    let ad = format!("{ders} / {}", gorev.baslik);
    if gorev.baslangic.is_empty() || gorev.cozum.is_empty() {
        sorunlar.push(format!("{ad}: başlangıç ya da çözüm yok"));
        return sorunlar;
    }
    let ornekler = if gorev.ornekler.is_empty() {
        vec![(String::new(), None)]
    } else {
        gorev.ornekler.iter().map(|(girdi, cikti)| (girdi.clone(), Some(cikti.clone()))).collect()
    };
    for (girdi, beklenen) in &ornekler {
        let girdi = if girdi.is_empty() { String::new() } else { format!("{girdi}\n") };
        let (cikti, sonuc) = calistir(&gorev.cozum, &girdi);
        if !sonuc["tanilamalar"].as_array().is_some_and(Vec::is_empty) {
            sorunlar.push(format!("{ad}: çözüm uyarı ya da hata veriyor: {}", sonuc["tanilamalar"]));
        }
        match beklenen {
            Some(beklenen) if duzelt(&cikti) != duzelt(beklenen) => {
                sorunlar.push(format!("{ad}: çözümün çıktısı beklenenden farklı:\n{cikti}\n--- beklenen:\n{beklenen}"));
            }
            None if sonuc["cizimler"].as_array().is_none_or(Vec::is_empty) => {
                sorunlar.push(format!("{ad}: çıktısı olmayan görevin çözümü bir şey çizmiyor"));
            }
            _ => {}
        }
        let (baslangic_ciktisi, sonuc) = calistir(&gorev.baslangic, &girdi);
        let hatalar = sonuc["tanilamalar"].as_array().into_iter().flatten().filter(|t| t["seviye"] == "hata").count();
        if hatalar > 0 {
            sorunlar.push(format!("{ad}: başlangıç kodu derlenmiyor"));
        }
        if beklenen.as_ref().is_some_and(|b| duzelt(&baslangic_ciktisi) == duzelt(b)) {
            sorunlar.push(format!("{ad}: başlangıç kodu görevi kendiliğinden çözüyor"));
        }
    }
    sorunlar
}

#[test]
fn gorevlerin_cozumleri_dogru() {
    let dersler = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../dersler");
    let mut gorevler = Vec::new();
    for girdi in std::fs::read_dir(&dersler).unwrap() {
        let dizin = girdi.unwrap().path();
        let Ok(metin) = std::fs::read_to_string(dizin.join("gorevler.md")) else { continue };
        let ders = dizin.file_name().unwrap().to_string_lossy().into_owned();
        let okunan = gorevleri_oku(&metin);
        assert!(!okunan.is_empty(), "{ders}: görev bulunamadı");
        gorevler.extend(okunan.into_iter().map(|gorev| (ders.clone(), gorev)));
    }
    assert!(gorevler.len() >= 20, "yalnızca {} görev bulundu", gorevler.len());

    let sorunlar: Vec<String> = std::thread::scope(|kapsam| {
        let isler: Vec<_> = gorevler
            .chunks(gorevler.len().div_ceil(8))
            .map(|parca| {
                kapsam.spawn(move || {
                    parca.iter().flat_map(|(ders, gorev)| gorevi_denetle(ders, gorev)).collect::<Vec<_>>()
                })
            })
            .collect();
        isler.into_iter().flat_map(|is| is.join().unwrap()).collect()
    });
    assert!(sorunlar.is_empty(), "{}", sorunlar.join("\n\n"));
}

#[test]
fn gorev_dosyasi_okunur() {
    let metin = "# Görevler\n\n## Bir\n\nAçıklama\n\n```\nörnek\n```\n\n```pas başlangıç\nb\n```\n\n```girdi\n5\n```\n\n```çıktı\n10\n```\n\n```çıktı\nx\n```\n\n```pas çözüm\nc\n```\n\n## İki\n";
    let gorevler = gorevleri_oku(metin);
    assert_eq!(gorevler.len(), 2);
    assert_eq!(gorevler[0].baslik, "Bir");
    assert_eq!(gorevler[0].baslangic, "b");
    assert_eq!(gorevler[0].cozum, "c");
    assert_eq!(gorevler[0].ornekler, vec![("5".to_owned(), "10".to_owned()), (String::new(), "x".to_owned())]);
    assert_eq!(gorevler[1].baslik, "İki");
}
