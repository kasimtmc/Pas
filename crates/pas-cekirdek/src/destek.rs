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
        girdi_ile(|girdi, yanki| satir_oku_kaynaktan(soru, girdi, &mut std::io::stdout(), yanki)).unwrap_or_default()
    }

    #[track_caller]
    pub fn deger_oku<T: std::str::FromStr>(soru: &str) -> T {
        match girdi_ile(|girdi, yanki| deger_oku_kaynaktan(soru, girdi, &mut std::io::stdout(), yanki)) {
            Some(deger) => deger,
            None => panic!("girdi bitti; okunacak değer kalmadı"),
        }
    }

    /// Programın girdisi önceden biliniyorsa (oyun alanında olduğu gibi)
    /// klavye yerine buradan okunur. `cargo pas` bunu hep boş bırakır.
    const HAZIR_GIRDI: Option<&str> = None;

    /// Girdiyi klavyeden ya da hazır girdiden okur. Hazır girdide okunan
    /// satırlar ekrana da yazılır (yankı); böylece çıktı, klavyeyle yazılmış
    /// gibi görünür. Hazır girdi biterse program durur ve bunu hata çıktısına
    /// yazar; oyun alanı kullanıcıdan yeni bir satır ister.
    fn girdi_ile<T>(islem: impl FnOnce(&mut dyn BufRead, bool) -> Option<T>) -> Option<T> {
        let Some(metin) = HAZIR_GIRDI else { return islem(&mut std::io::stdin().lock(), false) };
        static OKUNAN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let mut kalan = &metin.as_bytes()[OKUNAN.load(std::sync::atomic::Ordering::Relaxed)..];
        let sonuc = islem(&mut kalan, true);
        OKUNAN.store(metin.len() - kalan.len(), std::sync::atomic::Ordering::Relaxed);
        if sonuc.is_none() {
            let _ = std::io::stdout().flush();
            eprintln!("[pas: girdi bitti]");
            std::process::exit(0);
        }
        sonuc
    }

    /// Girdi bittiyse `None` döner. `yanki` açıksa okunan satır `cikti`ya da yazılır.
    pub fn satir_oku_kaynaktan(
        soru: &str,
        girdi: &mut (impl BufRead + ?Sized),
        cikti: &mut impl Write,
        yanki: bool,
    ) -> Option<String> {
        let _ = write!(cikti, "{soru}");
        let _ = cikti.flush();
        let mut satir = String::new();
        match girdi.read_line(&mut satir) {
            Ok(0) | Err(_) => None,
            Ok(_) => {
                if yanki {
                    let _ = writeln!(cikti, "{}", satir.trim_end_matches(['\n', '\r']));
                }
                Some(satir.trim().to_owned())
            }
        }
    }

    /// Geçerli bir değer yazılana kadar sorar; girdi bittiyse `None` döner.
    pub fn deger_oku_kaynaktan<T: std::str::FromStr>(
        soru: &str,
        girdi: &mut (impl BufRead + ?Sized),
        cikti: &mut impl Write,
        yanki: bool,
    ) -> Option<T> {
        let tur = std::any::type_name::<T>();
        loop {
            let satir = satir_oku_kaynaktan(soru, girdi, cikti, yanki)?;
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

    // ---- Türkçe çıktı ----
    // Pas'ın yazdırma makroları her argümanı `(&Sar(&x)).pas_turkce()` diye
    // sarmalar. "Otomatik referans uzmanlaşması": `Debug` uygulayan türler
    // için `TurkceSecimi` seçilir ve değer Türkçe yazılır (`doğru`, `Var(…)`,
    // `Yok`, `Tamam(…)`, `Hata(…)`); diğer türler `DuzSecim` ile olduğu gibi
    // geçer, böylece derleyicinin hata mesajları değişmez.
    pub struct Sar<'a, T: ?Sized>(pub &'a T);

    pub struct Turkce<'a, T: ?Sized>(pub &'a T);

    pub trait TurkceSecimi<'a, T: ?Sized> {
        fn pas_turkce(&self) -> Turkce<'a, T>;
    }

    impl<'a, T: ?Sized + std::fmt::Debug> TurkceSecimi<'a, T> for Sar<'a, T> {
        fn pas_turkce(&self) -> Turkce<'a, T> {
            Turkce(self.0)
        }
    }

    pub trait DuzSecim<'a, T: ?Sized> {
        fn pas_turkce(&self) -> &'a T;
    }

    impl<'a, T: ?Sized> DuzSecim<'a, T> for &Sar<'a, T> {
        fn pas_turkce(&self) -> &'a T {
            self.0
        }
    }

    impl<T: ?Sized + std::fmt::Display> std::fmt::Display for Turkce<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if std::any::type_name::<T>().trim_start_matches('&') == "bool" {
                let deger = self.0.to_string();
                return f.pad(if deger == "true" { "doğru" } else { "yanlış" });
            }
            std::fmt::Display::fmt(self.0, f)
        }
    }

    impl<T: ?Sized + std::fmt::Debug> std::fmt::Debug for Turkce<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let ham = if f.alternate() { format!("{:#?}", self.0) } else { format!("{:?}", self.0) };
            f.pad(&hata_ayiklamayi_turkcelestir(&ham))
        }
    }

    macro_rules! pas_bicim_aktar {
        ($($nitelik:ident)*) => {$(
            impl<T: ?Sized + std::fmt::$nitelik> std::fmt::$nitelik for Turkce<'_, T> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::$nitelik::fmt(self.0, f)
                }
            }
        )*};
    }
    pas_bicim_aktar!(LowerHex UpperHex Octal Binary LowerExp UpperExp Pointer);

    /// `Debug` çıktısındaki `Some`, `None`, `Ok`, `Err`, `true`, `false`
    /// isimlerini Türkçeleştirir; tırnak içindeki metinlere dokunmaz.
    pub fn hata_ayiklamayi_turkcelestir(metin: &str) -> String {
        let mut sonuc = String::with_capacity(metin.len());
        let mut karakterler = metin.char_indices().peekable();
        while let Some((i, c)) = karakterler.next() {
            if c == '"' || c == '\'' {
                sonuc.push(c);
                while let Some((_, d)) = karakterler.next() {
                    sonuc.push(d);
                    if d == '\\' {
                        if let Some((_, e)) = karakterler.next() {
                            sonuc.push(e);
                        }
                    } else if d == c {
                        break;
                    }
                }
            } else if c.is_alphabetic() || c == '_' {
                let mut son = i + c.len_utf8();
                while let Some(&(j, d)) = karakterler.peek() {
                    if !(d.is_alphanumeric() || d == '_') {
                        break;
                    }
                    son = j + d.len_utf8();
                    karakterler.next();
                }
                sonuc.push_str(match &metin[i..son] {
                    "Some" => "Var",
                    "None" => "Yok",
                    "Ok" => "Tamam",
                    "Err" => "Hata",
                    "true" => "doğru",
                    "false" => "yanlış",
                    isim => isim,
                });
            } else {
                sonuc.push(c);
            }
        }
        sonuc
    }

    // ---- Rastgele sayılar ----
    /// Oyun alanı her cevapta programı baştan çalıştırır; rastgele sayılar
    /// değişmesin diye tohumu o verir. `cargo pas` bunu hep boş bırakır.
    const HAZIR_TOHUM: Option<u64> = None;

    /// `en_kucuk` ile `en_buyuk` arasında (ikisi de dahil) rastgele bir sayı.
    pub fn rastgele<T: RastgeleSayi>(en_kucuk: T, en_buyuk: T) -> T {
        T::aralikta(en_kucuk, en_buyuk, rastgele_u64())
    }

    /// splitmix64; ilk çağrıda saatten (ya da hazır tohumdan) tohumlanır.
    fn rastgele_u64() -> u64 {
        static DURUM: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let mut durum = DURUM.load(std::sync::atomic::Ordering::Relaxed);
        if durum == 0 {
            durum = HAZIR_TOHUM.unwrap_or_else(|| {
                let simdi = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH);
                simdi.map(|s| s.as_nanos() as u64).unwrap_or(1)
            });
        }
        durum = durum.wrapping_add(0x9E37_79B9_7F4A_7C15);
        DURUM.store(durum, std::sync::atomic::Ordering::Relaxed);
        let mut z = durum;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// `rastgele!` ile üretilebilen sayı türleri.
    pub trait RastgeleSayi: Copy {
        fn aralikta(en_kucuk: Self, en_buyuk: Self, rastgele: u64) -> Self;
    }

    macro_rules! pas_rastgele_tam_sayi {
        ($($tur:ty)*) => {$(
            impl RastgeleSayi for $tur {
                fn aralikta(a: Self, b: Self, rastgele: u64) -> Self {
                    let (a, b) = if a <= b { (a, b) } else { (b, a) };
                    let genislik = (b as i128 - a as i128 + 1) as u128;
                    (a as i128 + (rastgele as u128 % genislik) as i128) as $tur
                }
            }
        )*};
    }
    pas_rastgele_tam_sayi!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize);

    macro_rules! pas_rastgele_ondalik {
        ($($tur:ty)*) => {$(
            impl RastgeleSayi for $tur {
                fn aralikta(a: Self, b: Self, rastgele: u64) -> Self {
                    let oran = (rastgele >> 11) as f64 / (1u64 << 53) as f64;
                    (a as f64 + oran * (b as f64 - a as f64)) as $tur
                }
            }
        )*};
    }
    pas_rastgele_ondalik!(f32 f64);

    macro_rules! pas_rastgele {
        ($en_kucuk:expr, $en_buyuk:expr) => {
            $crate::pas_destek::rastgele($en_kucuk, $en_buyuk)
        };
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
