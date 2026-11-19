//! Pas sözlüğü: her Türkçe sözcüğün Rust karşılığı.
//!
//! Bu tablo projenin tek doğruluk kaynağıdır. Çevirici, hata mesajı
//! çevirisi ve IDE eklentisi (`cargo pas sözlük --json` üzerinden) hep
//! buradan beslenir.
//!
//! Çeviri, isimlerin tutarlı biçimde yeniden adlandırılmasıdır: `liste`
//! kaynakta her yerde `vec` olur. Bu yüzden anahtar sözcük olmayan
//! girdiler (`Liste`, `uzunluk`, …) kullanıcı tarafından değişken adı
//! olarak da kullanılabilir; program yine doğru çalışır. Yalnızca
//! [`Sinif::AnahtarSozcuk`] türündeki sözcükler gerçekten ayrılmıştır.

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sinif {
    /// Ayrılmış sözcük; isim olarak ancak `r#` ile kullanılabilir.
    AnahtarSozcuk,
    Makro,
    Tur,
    Deger,
    Islev,
    Yontem,
}

impl Sinif {
    /// Kullanıcıya gösterilecek Türkçe ad.
    pub fn ad(self) -> &'static str {
        match self {
            Sinif::AnahtarSozcuk => "anahtar sözcük",
            Sinif::Makro => "makro",
            Sinif::Tur => "tür",
            Sinif::Deger => "değer",
            Sinif::Islev => "işlev",
            Sinif::Yontem => "yöntem",
        }
    }

    /// Makine tarafından okunacak çıktılar (JSON) için kararlı kimlik.
    pub fn kimlik(self) -> &'static str {
        match self {
            Sinif::AnahtarSozcuk => "anahtar_sozcuk",
            Sinif::Makro => "makro",
            Sinif::Tur => "tur",
            Sinif::Deger => "deger",
            Sinif::Islev => "islev",
            Sinif::Yontem => "yontem",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Girdi {
    pub pas: &'static str,
    pub rust: &'static str,
    pub tur: Sinif,
    pub aciklama: &'static str,
}

macro_rules! girdiler {
    ($($tur:ident { $($pas:literal => $rust:literal : $aciklama:literal,)* })*) => {
        &[$($(Girdi { pas: $pas, rust: $rust, tur: Sinif::$tur, aciklama: $aciklama },)*)*]
    };
}

pub static GIRDILER: &[Girdi] = girdiler! {
    AnahtarSozcuk {
        "olarak" => "as": "tür dönüştürür: `5 olarak f64`",
        "eşzamansız" => "async": "eşzamansız işlev ya da blok",
        "bekle" => "await": "eşzamansız bir işin bitmesini bekler",
        "kır" => "break": "döngüden çıkar",
        "sabit" => "const": "değişmeyen sabit değer tanımlar",
        "devam" => "continue": "döngünün bir sonraki turuna geçer",
        "sandık" => "crate": "içinde bulunulan sandık (crate)",
        "dinamik" => "dyn": "dinamik nitelik nesnesi: `Kutu<dinamik Nitelik>`",
        "değilse" => "else": "`eğer` koşulu sağlanmazsa çalışır",
        "çeşit" => "enum": "birkaç seçenekten biri olabilen tür tanımlar",
        "harici" => "extern": "dışarıdan gelen kod",
        "yanlış" => "false": "mantıksal yanlış değeri",
        "işlev" => "fn": "işlev tanımlar",
        "her" => "for": "bir dizinin her elemanı için döner: `her x için 0..10`",
        "eğer" => "if": "koşul doğruysa çalışır",
        "uygula" => "impl": "bir türe yöntem ya da nitelik ekler",
        "için" => "in": "`her` döngüsünde dolaşılacak dizi: `her sayı için 1..=10`",
        "tanımla" => "let": "yeni bir değişken tanımlar: `tanımla x = 5;`",
        "döngü" => "loop": "sonsuz döngü",
        "eşle" => "match": "bir değeri desenlerle karşılaştırır",
        "modül" => "mod": "modül tanımlar",
        "taşı" => "move": "kapanışa (closure) değerleri taşır",
        "değişken" => "mut": "değeri sonradan değiştirilebilir yapar: `tanımla değişken x = 5;`",
        "genel" => "pub": "dışarıdan erişilebilir yapar",
        "referans" => "ref": "desen içinde referans alır",
        "döndür" => "return": "işlevden değer döndürür",
        "kendi" => "self": "yöntemin çağrıldığı değer",
        "Kendi" => "Self": "içinde bulunulan tür",
        "durağan" => "static": "program boyunca yaşayan değer",
        "yapı" => "struct": "alanlardan oluşan bir tür tanımlar",
        "üst" => "super": "bir üstteki modül",
        "nitelik" => "trait": "türlerin paylaştığı davranış (trait)",
        "doğru" => "true": "mantıksal doğru değeri",
        "tür" => "type": "tür takma adı tanımlar",
        "güvensiz" => "unsafe": "derleyicinin denetleyemediği kod",
        "kullan" => "use": "başka bir yerdeki ismi kullanıma alır",
        "öyleyse" => "where": "tür koşulları yazar",
        "sürece" => "while": "koşul doğru olduğu sürece döner",
        "makro_kuralları" => "macro_rules": "makro tanımlar",
    }
    Makro {
        "yazdır" => "println": "ekrana yazar ve alt satıra geçer",
        "yaz" => "print": "ekrana yazar, alt satıra geçmez",
        "hata_yazdır" => "eprintln": "hata çıktısına yazar ve alt satıra geçer",
        "biçimle" => "format": "biçimlendirilmiş bir `Metin` oluşturur",
        "liste" => "vec": "liste oluşturur: `liste![1, 2, 3]`",
        "panik" => "panic": "programı bir hata mesajıyla durdurur",
        "doğrula" => "assert": "koşul yanlışsa programı durdurur",
        "doğrula_eşit" => "assert_eq": "iki değer eşit değilse programı durdurur",
        "doğrula_farklı" => "assert_ne": "iki değer eşitse programı durdurur",
        "yapılacak" => "todo": "henüz yazılmamış kodu işaretler",
        "ulaşılamaz" => "unreachable": "hiç ulaşılmaması gereken kodu işaretler",
        "rastgele" => "pas_rastgele": "iki sayı arasında (ikisi de dahil) rastgele bir sayı seçer: `rastgele!(1, 6)`",
        "oku" => "pas_oku": "klavyeden bir satır okur: `oku!(\"Adın ne? \")`, sayı için `oku!(\"Yaşın kaç? \", u32)`",
    }
    Tur {
        "Metin" => "String": "değiştirilebilir metin",
        "Liste" => "Vec": "büyüyüp küçülebilen liste",
        "Belki" => "Option": "bir değer olabilir de olmayabilir de: `Var(x)` ya da `Yok`",
        "Sonuç" => "Result": "başarılı (`Tamam`) ya da hatalı (`Hata`) sonuç",
        "Kutu" => "Box": "değeri yığında (heap) tutan kutu",
        "Sözlük" => "HashMap": "anahtar-değer eşlemesi",
        "Küme" => "HashSet": "tekrarsız elemanlar kümesi",
        "mantıksal" => "bool": "`doğru` ya da `yanlış`",
        "karakter" => "char": "tek bir karakter: 'a'",
        "Kaplumbağa" => "Kaplumbaga": "kalemiyle yürüdüğü yere çizgi çizen kaplumbağa: `tanımla değişken k = Kaplumbağa::yeni();`",
    }
    Deger {
        "Var" => "Some": "`Belki` içinde bir değer var",
        "Yok" => "None": "`Belki` içinde değer yok",
        "Tamam" => "Ok": "`Sonuç` başarılı",
        "Hata" => "Err": "`Sonuç` hatalı",
    }
    Islev {
        "ana" => "main": "programın başladığı işlev",
        "girdi" => "stdin": "klavyeden okumak için standart girdi",
        "yeni" => "new": "yeni, boş bir değer oluşturur: `Liste::yeni()`",
    }
    Yontem {
        "uzunluk" => "len": "eleman sayısı",
        "ekle" => "push": "listenin sonuna ekler",
        "çıkar" => "pop": "listenin son elemanını çıkarır",
        "boş_mu" => "is_empty": "boş olup olmadığını söyler",
        "kopyala" => "clone": "değerin bir kopyasını oluşturur",
        "metne_çevir" => "to_string": "değeri `Metin`e çevirir",
        "içerir" => "contains": "bir elemanı içerip içermediğini söyler",
        "yinele" => "iter": "elemanlar üzerinde dolaşan yineleyici",
        "sarmalaç" => "unwrap": "`Var`/`Tamam` içindeki değeri alır, yoksa programı durdurur",
        "kırp" => "trim": "baştaki ve sondaki boşlukları atar",
        "ayrıştır" => "parse": "metni sayıya (ya da başka bir türe) çevirir",
        "satır_oku" => "read_line": "girdiden bir satır okur",
        "yoksa" => "unwrap_or": "`Var`/`Tamam` içindeki değeri, yoksa verilen değeri alır: `x.yoksa(0)`",
        "var_mı" => "is_some": "`Belki` içinde bir değer var mı?",
        "yok_mu" => "is_none": "`Belki` boş mu?",
        "al" => "get": "verilen sıradaki ya da anahtardaki değeri `Belki` olarak alır",
        "yerleştir" => "insert": "sözlüğe ya da listede bir yere değer yerleştirir",
        "sil" => "remove": "verilen sıradaki ya da anahtardaki değeri siler",
        "anahtar_var_mı" => "contains_key": "sözlükte bu anahtar var mı?",
        "anahtarlar" => "keys": "sözlüğün anahtarları üzerinde dolaşır",
        "değerler" => "values": "sözlüğün değerleri üzerinde dolaşır",
        "bul" => "find": "koşulu sağlayan ilk elemanı `Belki` olarak bulur",
        "sırala" => "sort": "listeyi küçükten büyüğe sıralar",
        "ters_çevir" => "reverse": "listeyi tersine çevirir",
        "toplam" => "sum": "elemanların toplamı: `liste.yinele().toplam::<i32>()`",
        "en_büyük" => "max": "en büyük eleman (ya da iki sayıdan büyük olanı)",
        "en_küçük" => "min": "en küçük eleman (ya da iki sayıdan küçük olanı)",
        "dönüştür" => "map": "her elemanı verilen kapanışla dönüştürür",
        "süz" => "filter": "yalnızca koşulu sağlayan elemanları bırakır",
        "biriktir" => "collect": "yineleyicideki elemanları bir listede (ya da başka bir yapıda) toplar",
        "numaralandır" => "enumerate": "her elemanı sıra numarasıyla birlikte verir: `(0, x)`",
        "böl" => "split": "metni verilen ayırıcıdan parçalara böler",
        "satırlar" => "lines": "metnin satırları üzerinde dolaşır",
        "karakterler" => "chars": "metnin karakterleri üzerinde dolaşır",
        "birleştir" => "join": "parçaları aralarına verilen metni koyarak birleştirir",
        "değiştir" => "replace": "metindeki bir parçayı başkasıyla değiştirir",
        "ile_başlar_mı" => "starts_with": "metin bununla mı başlıyor?",
        "ile_biter_mi" => "ends_with": "metin bununla mı bitiyor?",
        "mutlak" => "abs": "sayının mutlak değeri (eksi işareti olmadan)",
        "kuvvet" => "pow": "sayının kuvveti: `2.kuvvet(3)` → 8",
        "karekök" => "sqrt": "ondalıklı sayının karekökü",
        "yuvarla" => "round": "ondalıklı sayıyı en yakın tam sayıya yuvarlar",
        "ileri" => "pas_ileri": "kaplumbağayı baktığı yöne doğru yürütür: `k.ileri(100)`",
        "geri" => "pas_geri": "kaplumbağayı geri geri yürütür: `k.geri(50)`",
        "sağa_dön" => "pas_saga_don": "kaplumbağayı verilen derece kadar sağa döndürür: `k.sağa_dön(90)`",
        "sola_dön" => "pas_sola_don": "kaplumbağayı verilen derece kadar sola döndürür: `k.sola_dön(90)`",
        "kalemi_kaldır" => "pas_kalemi_kaldir": "kaplumbağanın kalemini kaldırır; çizmeden yürür",
        "kalemi_indir" => "pas_kalemi_indir": "kaplumbağanın kalemini indirir; yürüdüğü yere yeniden çizer",
        "renk" => "pas_renk": "kalemin rengini değiştirir: `k.renk(\"kırmızı\")` ya da `k.renk(\"#ff8800\")`",
        "kalınlık" => "pas_kalinlik": "kalemin kalınlığını değiştirir: `k.kalınlık(4)`",
        "git" => "pas_git": "kaplumbağayı bir noktaya götürür; orta (0, 0) noktasıdır: `k.git(50, -20)`",
        "daire" => "pas_daire": "kaplumbağanın olduğu yeri merkez alan bir daire çizer: `k.daire(40)`",
    }
};

struct Dizin {
    pas_ile: HashMap<&'static str, &'static Girdi>,
    rust_ile: HashMap<&'static str, &'static Girdi>,
}

fn dizin() -> &'static Dizin {
    static DIZIN: OnceLock<Dizin> = OnceLock::new();
    DIZIN.get_or_init(|| Dizin {
        pas_ile: GIRDILER.iter().map(|e| (e.pas, e)).collect(),
        rust_ile: GIRDILER.iter().map(|e| (e.rust, e)).collect(),
    })
}

/// Türkçe sözcüğün girdisini bulur. Sözcük NFC biçiminde olmalıdır.
pub fn pas_ile(kelime: &str) -> Option<&'static Girdi> {
    dizin().pas_ile.get(kelime).copied()
}

/// Rust isminin Türkçe girdisini bulur.
pub fn rust_ile(kelime: &str) -> Option<&'static Girdi> {
    dizin().rust_ile.get(kelime).copied()
}

/// Türkçe harfleri ASCII karşılıklarına indirir: `eğer` → `eger`.
pub fn ascii_yazim(kelime: &str) -> String {
    kelime
        .chars()
        .map(|c| match c {
            'ç' => 'c',
            'ğ' => 'g',
            'ı' => 'i',
            'ö' => 'o',
            'ş' => 's',
            'ü' => 'u',
            'Ç' => 'C',
            'Ğ' => 'G',
            'İ' => 'I',
            'Ö' => 'O',
            'Ş' => 'S',
            'Ü' => 'U',
            diger => diger,
        })
        .collect()
}

/// Türkçe harfleri olmadan yazılmış bir Pas sözcüğünün doğrusu: `eger` →
/// `eğer`. Yalnızca anahtar sözcükler, türler, değerler ve (`makro` ise)
/// makrolar için; yöntem ve işlev adları kullanıcının kendi adları olabilir.
pub fn turkce_yazimi(kelime: &str, makro: bool) -> Option<&'static Girdi> {
    static ASCII_ILE: OnceLock<HashMap<String, Vec<&'static Girdi>>> = OnceLock::new();
    let dizin = ASCII_ILE.get_or_init(|| {
        let mut dizin: HashMap<String, Vec<&'static Girdi>> = HashMap::new();
        for girdi in GIRDILER {
            let ascii = ascii_yazim(girdi.pas);
            if ascii != girdi.pas {
                dizin.entry(ascii).or_default().push(girdi);
            }
        }
        dizin
    });
    if pas_ile(kelime).is_some() || rust_ile(kelime).is_some() {
        return None;
    }
    dizin.get(kelime)?.iter().copied().find(|girdi| match girdi.tur {
        Sinif::Makro => makro,
        Sinif::AnahtarSozcuk | Sinif::Tur | Sinif::Deger => !makro,
        Sinif::Islev | Sinif::Yontem => false,
    })
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::sozcukler::{isim_baslangici_mi, isim_devami_mi};
    use std::collections::HashSet;

    #[test]
    fn sozcukler_benzersiz() {
        let mut pas = HashSet::new();
        let mut rust = HashSet::new();
        for e in GIRDILER {
            assert!(pas.insert(e.pas), "`{}` iki kez tanımlanmış", e.pas);
            assert!(rust.insert(e.rust), "`{}` iki kez tanımlanmış", e.rust);
        }
    }

    #[test]
    fn pas_sozcukleri_gecerli_isimler() {
        use unicode_normalization::is_nfc;
        for e in GIRDILER {
            let mut chars = e.pas.chars();
            assert!(chars.next().is_some_and(isim_baslangici_mi), "{}", e.pas);
            assert!(chars.all(isim_devami_mi), "{}", e.pas);
            assert!(is_nfc(e.pas), "`{}` NFC biçiminde değil", e.pas);
        }
    }

    #[test]
    fn turkce_harfsiz_yazim() {
        assert_eq!(turkce_yazimi("eger", false).map(|g| g.pas), Some("eğer"));
        assert_eq!(turkce_yazimi("islev", false).map(|g| g.pas), Some("işlev"));
        assert_eq!(turkce_yazimi("Sozluk", false).map(|g| g.pas), Some("Sözlük"));
        assert_eq!(turkce_yazimi("yazdir", true).map(|g| g.pas), Some("yazdır"));
        // Makro adları yalnızca `!` ile, yöntem adları hiç önerilmez.
        assert!(turkce_yazimi("yazdir", false).is_none());
        assert!(turkce_yazimi("degerler", false).is_none());
        // Zaten doğru yazılmış ya da Rust sözcüğü olanlar
        assert!(turkce_yazimi("eğer", false).is_none());
        assert!(turkce_yazimi("if", false).is_none());
    }

    #[test]
    fn hicbir_pas_sozcugu_rust_sozcugu_degil() {
        // Bir Türkçe sözcük aynı zamanda başka bir girdinin Rust karşılığı
        // olursa ters çeviri belirsizleşir.
        for e in GIRDILER {
            assert!(rust_ile(e.pas).is_none(), "`{}` hem Pas hem Rust sözcüğü", e.pas);
        }
    }
}
