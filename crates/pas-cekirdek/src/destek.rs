// Pas programlarına `cargo pas` tarafından eklenen destek kodu.
//
// Bu dosya derlenmez; metni, çevrilen her sandık kökünün başına tek satır
// olarak eklenir (satır numaraları kaymasın diye). Bu yüzden:
// - yorumlar yalnızca kendi satırlarında olmalı (satır sonunda `//` olmamalı),
// - Pas'taki adı `oku!` olan makro burada `pas_oku!` adını taşır.
// Kökte görünen adlar (`Kaplumbağa`). Satırın başında durur; rustc
// `kullan` önerilerini ilk `use`un önüne yazdığı için öneri ayrı bir satır olur.
#[allow(unused_imports)]
use crate::pas_destek::hazir::*;
#[macro_use]
#[allow(dead_code, unused_macros, unused_imports)]
mod pas_destek {
    use std::io::{BufRead, Write};

    pub fn satir_oku(soru: &str) -> String {
        girdi_ile(|girdi, yanki_ac| satir_oku_kaynaktan(soru, girdi, &mut std::io::stdout(), yanki_ac)).unwrap_or_default()
    }

    #[track_caller]
    pub fn deger_oku<T: std::str::FromStr>(soru: &str) -> T {
        match girdi_ile(|girdi, yanki_ac| deger_oku_kaynaktan(soru, girdi, &mut std::io::stdout(), yanki_ac)) {
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

    /// Girdi bittiyse `None` döner. `yanki_ac` açıksa okunan satır `cikti`ya da yazılır.
    pub fn satir_oku_kaynaktan(
        soru: &str,
        girdi: &mut (impl BufRead + ?Sized),
        cikti: &mut impl Write,
        yanki_ac: bool,
    ) -> Option<String> {
        let _ = write!(cikti, "{soru}");
        let _ = cikti.flush();
        let mut girilen = String::new();
        match girdi.read_line(&mut girilen) {
            Ok(0) | Err(_) => None,
            Ok(_) => {
                if yanki_ac {
                    let _ = writeln!(cikti, "{}", girilen.trim_end_matches(['\n', '\r']));
                }
                Some(girilen.trim().to_owned())
            }
        }
    }

    /// Geçerli bir değer yazılana kadar sorar; girdi bittiyse `None` döner.
    pub fn deger_oku_kaynaktan<T: std::str::FromStr>(
        soru: &str,
        girdi: &mut (impl BufRead + ?Sized),
        cikti: &mut impl Write,
        yanki_ac: bool,
    ) -> Option<T> {
        let tur = std::any::type_name::<T>();
        loop {
            let girilen = satir_oku_kaynaktan(soru, girdi, cikti, yanki_ac)?;
            if let Ok(deger) = turkce_yaziyi_duzelt(&girilen, tur).parse() {
                return Some(deger);
            }
            let _ = writeln!(cikti, "`{girilen}` {} değil, tekrar dene.", tur_aciklamasi(tur));
        }
    }

    /// `3,5` → `3.5`, `evet` → `true` gibi Türkçe yazımları Rust'ın anladığı biçime çevirir.
    fn turkce_yaziyi_duzelt(girilen: &str, tur: &str) -> String {
        match tur {
            "f32" | "f64" => girilen.replace(',', "."),
            "bool" => match girilen.to_lowercase().as_str() {
                "doğru" | "evet" | "e" => "true".to_owned(),
                "yanlış" | "hayır" | "h" => "false".to_owned(),
                diger => diger.to_owned(),
            },
            _ => girilen.to_owned(),
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
        T::aralikta_sec(en_kucuk, en_buyuk, rastgele_u64())
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
        fn aralikta_sec(en_kucuk: Self, en_buyuk: Self, rastgele: u64) -> Self;
    }

    macro_rules! pas_rastgele_tam_sayi {
        ($($tur:ty)*) => {$(
            impl RastgeleSayi for $tur {
                fn aralikta_sec(a: Self, b: Self, rastgele: u64) -> Self {
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
                fn aralikta_sec(a: Self, b: Self, rastgele: u64) -> Self {
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

    // ---- Kaplumbağa ile çizim ----
    // Pas'taki `Kaplumbağa` türü ve yöntemleri (`ileri`, `sağa_dön`…) sözlükte
    // buradaki adlara (`Kaplumbaga`, `pas_ileri`, `pas_saga_don`…) çevrilir.
    /// Kökte `kullan` gerekmeden görünen adlar. Kullanıcı aynı adla bir şey
    /// tanımlarsa onunki geçerli olur (glob ile alınan adlar gölgelenir).
    pub mod hazir {
        pub use super::Kaplumbaga;
    }

    /// Kaplumbağa komutlarının kabul ettiği sayılar: `ileri(100)`, `ileri(2.5)`.
    pub trait CizimSayisi: Copy {
        fn ondalik_deger(self) -> f64;
    }

    macro_rules! pas_cizim_sayisi {
        ($($tur:ty)*) => {$(
            impl CizimSayisi for $tur {
                fn ondalik_deger(self) -> f64 {
                    self as f64
                }
            }
        )*};
    }
    pas_cizim_sayisi!(i8 i16 i32 i64 isize u8 u16 u32 u64 usize f32 f64);

    /// Kalemiyle yürüdüğü yere çizgi çizen kaplumbağa. Ortadan (0, 0) başlar
    /// ve yukarı bakar. Program bitince çizimi SVG olarak verir: oyun alanında
    /// ekranda görünür, `cargo pas` ile `çizim.svg` dosyasına kaydedilir.
    pub struct Kaplumbaga {
        x: f64,
        y: f64,
        /// Derece; 0 sağa, 90 yukarı bakar.
        yon: f64,
        kalem_inik: bool,
        renk: String,
        kalem_kalinligi: f64,
        /// SVG öğelerinin başları; canlandırma için sonları çizim yazılırken eklenir.
        sekiller: Vec<String>,
        /// (en küçük x, en küçük y, en büyük x, en büyük y)
        kapladigi_alan: (f64, f64, f64, f64),
    }

    const EN_FAZLA_SEKIL: usize = 50_000;

    const RENKLER: &[(&str, &str)] = &[
        ("siyah", "#222222"),
        ("beyaz", "#ffffff"),
        ("kırmızı", "#e53935"),
        ("kirmizi", "#e53935"),
        ("yeşil", "#43a047"),
        ("yesil", "#43a047"),
        ("mavi", "#1e88e5"),
        ("sarı", "#fdd835"),
        ("sari", "#fdd835"),
        ("turuncu", "#fb8c00"),
        ("mor", "#8e24aa"),
        ("pembe", "#ec407a"),
        ("kahverengi", "#6d4c41"),
        ("gri", "#9e9e9e"),
        ("lacivert", "#1a237e"),
        ("turkuaz", "#00acc1"),
    ];

    impl Default for Kaplumbaga {
        fn default() -> Self {
            Kaplumbaga {
                x: 0.0,
                y: 0.0,
                yon: 90.0,
                kalem_inik: true,
                renk: "#222222".to_owned(),
                kalem_kalinligi: 2.0,
                sekiller: Vec::new(),
                kapladigi_alan: (0.0, 0.0, 0.0, 0.0),
            }
        }
    }

    impl Kaplumbaga {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn pas_ileri(&mut self, mesafe: impl CizimSayisi) {
            let (mesafe, yon_radyan) = (mesafe.ondalik_deger(), self.yon.to_radians());
            self.cizerek_git(self.x + mesafe * yon_radyan.cos(), self.y + mesafe * yon_radyan.sin());
        }

        pub fn pas_geri(&mut self, mesafe: impl CizimSayisi) {
            self.pas_ileri(-mesafe.ondalik_deger());
        }

        pub fn pas_saga_don(&mut self, derece: impl CizimSayisi) {
            self.yon = (self.yon - derece.ondalik_deger()).rem_euclid(360.0);
        }

        pub fn pas_sola_don(&mut self, derece: impl CizimSayisi) {
            self.yon = (self.yon + derece.ondalik_deger()).rem_euclid(360.0);
        }

        pub fn pas_kalemi_kaldir(&mut self) {
            self.kalem_inik = false;
        }

        pub fn pas_kalemi_indir(&mut self) {
            self.kalem_inik = true;
        }

        #[track_caller]
        pub fn pas_renk(&mut self, renk: &str) {
            self.renk = renk_kodu(renk);
        }

        pub fn pas_kalinlik(&mut self, kalem_kalinligi: impl CizimSayisi) {
            self.kalem_kalinligi = kalem_kalinligi.ondalik_deger().max(0.1);
        }

        pub fn pas_git(&mut self, x: impl CizimSayisi, y: impl CizimSayisi) {
            self.cizerek_git(x.ondalik_deger(), y.ondalik_deger());
        }

        /// Kaplumbağanın olduğu yeri merkez alan bir daire çizer.
        pub fn pas_daire(&mut self, daire_yaricapi: impl CizimSayisi) {
            let r = daire_yaricapi.ondalik_deger().abs();
            if self.kalem_inik && self.sekiller.len() < EN_FAZLA_SEKIL {
                self.sekiller.push(format!(
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{r:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"",
                    nokta(self.x),
                    nokta(-self.y),
                    self.renk,
                    self.kalem_kalinligi
                ));
                self.sinirlari_genislet(self.x - r, self.y - r);
                self.sinirlari_genislet(self.x + r, self.y + r);
            }
        }

        fn cizerek_git(&mut self, x: f64, y: f64) {
            if self.kalem_inik && self.sekiller.len() < EN_FAZLA_SEKIL {
                self.sekiller.push(format!(
                    "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"{}\"",
                    nokta(self.x),
                    nokta(-self.y),
                    nokta(x),
                    nokta(-y),
                    self.renk,
                    self.kalem_kalinligi
                ));
            }
            self.x = x;
            self.y = y;
            self.sinirlari_genislet(x, y);
        }

        fn sinirlari_genislet(&mut self, x: f64, y: f64) {
            let (a, b, c, d) = self.kapladigi_alan;
            self.kapladigi_alan = (a.min(x), b.min(y), c.max(x), d.max(y));
        }

        /// Çizim; şekiller sırayla belirir, en sonda kaplumbağa görünür.
        /// Canlandırmayı bilmeyen görüntüleyicilerde hepsi baştan görünür.
        pub fn svg(&self) -> String {
            let (x0, y0, x1, y1) = self.kapladigi_alan;
            let (genislik, yukseklik) = ((x1 - x0).max(400.0) + 60.0, (y1 - y0).max(400.0) + 60.0);
            let (orta_x, orta_y) = ((x0 + x1) / 2.0, -(y0 + y1) / 2.0);
            let gecikme = (3.0 / self.sekiller.len().max(1) as f64).min(0.05);
            let mut svg = format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{:.1} {:.1} {genislik:.1} {yukseklik:.1}\" width=\"{:.0}\" height=\"{:.0}\"><style>.c{{animation:g .01s both}}@keyframes g{{from{{opacity:0}}}}@media (prefers-reduced-motion:reduce){{.c{{animation:none}}}}line,circle{{stroke-linecap:round;vector-effect:non-scaling-stroke}}</style><rect x=\"{:.1}\" y=\"{:.1}\" width=\"{genislik:.1}\" height=\"{yukseklik:.1}\" fill=\"#ffffff\"/>",
                orta_x - genislik / 2.0,
                orta_y - yukseklik / 2.0,
                genislik,
                yukseklik,
                orta_x - genislik / 2.0,
                orta_y - yukseklik / 2.0,
            );
            for (cizim_no, sekil) in self.sekiller.iter().enumerate() {
                svg.push_str(&format!("{sekil} class=\"c\" style=\"animation-delay:{:.3}s\"/>", cizim_no as f64 * gecikme));
            }
            // Kaplumbağa: baktığı yöne dönük yeşil bir ok
            svg.push_str(&format!(
                "<g class=\"c\" style=\"animation-delay:{:.3}s\" transform=\"translate({:.1} {:.1}) rotate({:.1})\"><path d=\"M12 0 L-8 -8 L-4 0 L-8 8 Z\" fill=\"#2e7d32\" stroke=\"#1b5e20\" stroke-width=\"1.5\"/></g></svg>",
                self.sekiller.len() as f64 * gecikme,
                self.x,
                -self.y,
                -self.yon
            ));
            svg
        }
    }

    impl Drop for Kaplumbaga {
        fn drop(&mut self) {
            let svg = self.svg();
            // Oyun alanı (tohumu o verir) çizimi hata çıktısından alıp gösterir.
            if HAZIR_TOHUM.is_some() {
                eprintln!("[pas: çizim] {svg}");
                return;
            }
            static SAYAC: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let cizim_no = SAYAC.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            let ad = if cizim_no == 1 { "çizim.svg".to_owned() } else { format!("çizim-{cizim_no}.svg") };
            match std::fs::write(&ad, svg) {
                Ok(()) => eprintln!("Kaplumbağanın çizimi `{ad}` dosyasına kaydedildi; tarayıcıda açabilirsin."),
                Err(hata) => eprintln!("Kaplumbağanın çizimi `{ad}` dosyasına kaydedilemedi: {hata}"),
            }
        }
    }

    /// SVG'ye yazılacak koordinat; `-0.0` ya da `0.00000001` gibi değerler sıfır olur.
    fn nokta(deger: f64) -> f64 {
        if deger.abs() < 0.05 { 0.0 } else { deger }
    }

    /// Türkçe renk adı ya da `#ff8800` biçiminde bir renk.
    #[track_caller]
    fn renk_kodu(renk: &str) -> String {
        let renk = renk.trim();
        if let Some((_, kod)) = RENKLER.iter().find(|(ad, _)| *ad == renk) {
            return (*kod).to_owned();
        }
        let onaltilik_kod = renk.strip_prefix('#').is_some_and(|kod| {
            matches!(kod.len(), 3 | 6) && kod.chars().all(|c| c.is_ascii_hexdigit())
        });
        // Yalnızca onaltılık rakamlar içerdiği için SVG'ye güvenle yazılabilir.
        if onaltilik_kod {
            return renk.to_owned();
        }
        panic!(
            "`{renk}` bir renk değil; siyah, beyaz, kırmızı, yeşil, mavi, sarı, turuncu, mor, pembe, kahverengi, gri, lacivert, turkuaz ya da `#ff8800` gibi bir renk kodu yaz"
        );
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
