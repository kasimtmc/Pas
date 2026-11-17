//! Derlenen programı çalıştırır; çökerse (panik) mesajını Türkçeleştirip
//! Pas kaynağındaki yerini gösterir.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

use pas_cekirdek::mesajlar::panik_mesajini_cevir;
use pas_cekirdek::{SatirDizini, Yon};

use crate::gorunum::Gorunum;
use crate::tanilama::{Kaynaklar, konumu_goster};

pub fn calistir(
    program_dosyasi: &Path,
    argumanlar: &[String],
    kaynaklar: &mut Kaynaklar,
    gorunum: Gorunum,
) -> Result<ExitCode, String> {
    let mut surec = Command::new(program_dosyasi)
        .args(argumanlar)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("program çalıştırılamadı: {e}"))?;

    let hata_ciktisi = surec.stderr.take().expect("hata çıktısı yönlendirildi");
    let mut okuyucu = BufReader::new(hata_ciktisi);
    let mut tampon = Vec::new();
    let mut panikte = false;
    let mut hata_akisi = std::io::stderr();
    loop {
        tampon.clear();
        match okuyucu.read_until(b'\n', &mut tampon) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let satir = String::from_utf8_lossy(&tampon);
        let icerik = satir.trim_end_matches(['\n', '\r']);

        if let Some(baslik) = panik_basligi(icerik, kaynaklar, gorunum) {
            let _ = write!(hata_akisi, "{baslik}");
            panikte = true;
        } else if panikte && icerik.starts_with("note: run with `RUST_BACKTRACE=1`") {
            let _ = writeln!(
                hata_akisi,
                "{}: çökmenin hangi işlev çağrılarından geçtiğini görmek için programı \
                 `RUST_BACKTRACE=1` ortam değişkeniyle çalıştırabilirsin",
                gorunum.not("not")
            );
            panikte = false;
        } else if panikte {
            let _ = writeln!(hata_akisi, "  {}", gorunum.kalin(&panik_mesajini_cevir(icerik)));
        } else {
            let _ = hata_akisi.write_all(&tampon);
        }
    }

    let durum = surec.wait().map_err(|e| format!("program beklenirken hata: {e}"))?;
    Ok(match durum.code() {
        Some(kod) => ExitCode::from(u8::try_from(kod).unwrap_or(1)),
        None => ExitCode::from(1),
    })
}

/// `thread 'main' (1234) panicked at kaynak/main.rs:5:34:`
fn panik_basligi(satir: &str, kaynaklar: &mut Kaynaklar, gorunum: Gorunum) -> Option<String> {
    let kalan = satir.strip_prefix("thread '")?;
    let is_parcacigi = &kalan[..kalan.find('\'')?];
    let konum = kalan.split(" panicked at ").nth(1)?.strip_suffix(':')?;
    let mut parcalar = konum.rsplitn(3, ':');
    let sutun: usize = parcalar.next()?.parse().ok()?;
    let satir_no: usize = parcalar.next()?.parse().ok()?;
    let dosya_adi = parcalar.next()?;

    let mut cikti = String::new();
    let is_parcacigi = if is_parcacigi == "main" { "ana".to_owned() } else { is_parcacigi.to_owned() };
    cikti.push_str(&format!(
        "\n{}{}\n",
        gorunum.hata("hata"),
        gorunum.kalin(&format!(": program çöktü (panik, `{is_parcacigi}` iş parçacığında)"))
    ));
    match kaynaklar.al(dosya_adi) {
        Some(dosya) => {
            let rust_ofseti = SatirDizini::yeni(&dosya.rust_metni).ofset(satir_no, sutun).unwrap_or(0);
            let ofset = match &dosya.harita {
                Some(harita) => harita.rusttan_pasa(rust_ofseti, Yon::Baslangic),
                None => rust_ofseti,
            };
            konumu_goster(&mut cikti, &gorunum, &dosya, ofset);
        }
        None => cikti.push_str(&format!("  {} {}\n", gorunum.kenar("-->"), kaynaklar.yollari_duzelt(konum))),
    }
    Some(cikti)
}
