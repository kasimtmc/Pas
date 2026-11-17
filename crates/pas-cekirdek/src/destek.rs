// Pas programlarına `cargo pas` tarafından eklenen destek kodu.
//
// Bu dosya derlenmez; metni, çevrilen her sandık kökünün başına tek satır
// olarak eklenir (satır numaraları kaymasın diye). Bu yüzden:
// - yorumlar yalnızca kendi satırlarında olmalı (satır sonunda `//` olmamalı),
// - Pas'taki adı `oku!` olan makro burada `pas_oku!` adını taşır.
#[macro_use]
#[allow(dead_code, unused_macros)]
mod pas_destek {
    use std::io::{BufRead, Write};

    pub fn satir_oku(soru: &str) -> String {
        let mut girdi = std::io::stdin().lock();
        satir_oku_kaynaktan(soru, &mut girdi, &mut std::io::stdout()).unwrap_or_default()
    }

    #[track_caller]
    pub fn deger_oku<T: std::str::FromStr>(soru: &str) -> T {
        let mut girdi = std::io::stdin().lock();
        match deger_oku_kaynaktan(soru, &mut girdi, &mut std::io::stdout()) {
            Some(deger) => deger,
            None => panic!("girdi bitti; okunacak değer kalmadı"),
        }
    }

    /// Girdi bittiyse `None` döner.
    pub fn satir_oku_kaynaktan(soru: &str, girdi: &mut impl BufRead, cikti: &mut impl Write) -> Option<String> {
        let _ = write!(cikti, "{soru}");
        let _ = cikti.flush();
        let mut satir = String::new();
        match girdi.read_line(&mut satir) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(satir.trim().to_owned()),
        }
    }

    /// Geçerli bir değer yazılana kadar sorar; girdi bittiyse `None` döner.
    pub fn deger_oku_kaynaktan<T: std::str::FromStr>(
        soru: &str,
        girdi: &mut impl BufRead,
        cikti: &mut impl Write,
    ) -> Option<T> {
        let tur = std::any::type_name::<T>();
        loop {
            let satir = satir_oku_kaynaktan(soru, girdi, cikti)?;
            if let Ok(deger) = turkce_yaziyi_duzelt(&satir, tur).parse() {
                return Some(deger);
            }
            let _ = writeln!(cikti, "`{satir}` {} değil, tekrar dene.", tur_aciklamasi(tur));
        }
    }

    /// `3,5` → `3.5`, `evet` → `true` gibi Türkçe yazımları Rust'ın anladığı biçime çevirir.
    fn turkce_yaziyi_duzelt(satir: &str, tur: &str) -> String {
        match tur {
            "f32" | "f64" => satir.replace(',', "."),
            "bool" => match satir.to_lowercase().as_str() {
                "doğru" | "evet" | "e" => "true".to_owned(),
                "yanlış" | "hayır" | "h" => "false".to_owned(),
                diger => diger.to_owned(),
            },
            _ => satir.to_owned(),
        }
    }

    fn tur_aciklamasi(tur: &str) -> &'static str {
        match tur {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => {
                "geçerli bir tam sayı"
            }
            "f32" | "f64" => "geçerli bir ondalıklı sayı",
            "bool" => "`doğru` ya da `yanlış`",
            "char" => "tek bir karakter",
            _ => "geçerli bir değer",
        }
    }

    macro_rules! pas_oku {
        () => {
            $crate::pas_destek::satir_oku("")
        };
        ($soru:expr) => {
            $crate::pas_destek::satir_oku(&$soru)
        };
        ($soru:expr, $tur:ty) => {
            $crate::pas_destek::deger_oku::<$tur>(&$soru)
        };
    }
}
