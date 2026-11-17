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
        "her" => "for": "bir dizinin her elemanı için döner: `her x içinde 0..10`",
        "eğer" => "if": "koşul doğruysa çalışır",
        "uygula" => "impl": "bir türe yöntem ya da nitelik ekler",
        "içinde" => "in": "`her` döngüsünde dolaşılacak dizi",
        "yap" => "let": "yeni bir değişken tanımlar: `yap x = 5;`",
        "döngü" => "loop": "sonsuz döngü",
        "eşle" => "match": "bir değeri desenlerle karşılaştırır",
        "modül" => "mod": "modül tanımlar",
        "taşı" => "move": "kapanışa (closure) değerleri taşır",
        "değişken" => "mut": "değeri sonradan değiştirilebilir yapar: `yap değişken x = 5;`",
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
    fn hicbir_pas_sozcugu_rust_sozcugu_degil() {
        // Bir Türkçe sözcük aynı zamanda başka bir girdinin Rust karşılığı
        // olursa ters çeviri belirsizleşir.
        for e in GIRDILER {
            assert!(rust_ile(e.pas).is_none(), "`{}` hem Pas hem Rust sözcüğü", e.pas);
        }
    }
}
