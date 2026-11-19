//! rustc ve çalışma zamanı (panik) mesajlarının Türkçe çevirileri.
//!
//! Mesajlar kalıplarla eşleştirilir: `{}` herhangi bir metni yakalar ve
//! Türkçe şablonda `{0}`, `{1}`, … olarak kullanılır. Yakalanan kod
//! parçaları ([`pasa_cevir`] ile) Pas'a çevrilir; böylece `String` yerine
//! `Metin` görünür. Kalıbı olmayan mesajlar İngilizce kalır, ama içlerindeki
//! `ters tırnaklı` kod parçaları yine Pas'a çevrilir.

use crate::ceviri::pasa_cevir;

/// (İngilizce kalıp, Türkçe şablon). Özel kalıplar genel olanlardan önce gelmeli.
const DERLEYICI: &[(&str, &str)] = &[
    // Yeni başlayanların sık yaptığı hatalar için özel açıklamalar (genel kalıplardan önce)
    ("`{}` doesn't implement `std::fmt::Display`", "`{0}` doğrudan `{{}}` ile yazdırılamaz"),
    (
        "cannot add `{}` to `Option<{}>`",
        "`Belki<{1}>` ile `{0}` toplanamaz; önce içindeki değeri `eşle`, `.yoksa(…)` ya da `.sarmalaç()` ile çıkar",
    ),
    (
        "can't compare `{float}` with `{integer}`",
        "ondalıklı sayı ile tam sayı karşılaştırılamaz; tam sayıyı `1.0` gibi noktalı yaz",
    ),
    (
        "expected floating-point number, found integer",
        "ondalıklı sayı bekleniyordu ama tam sayı bulundu; sayıyı `1.0` gibi noktalı yaz",
    ),
    (
        "expected `f{}`, found integer",
        "`f{0}` (ondalıklı sayı) bekleniyordu ama tam sayı bulundu; sayıyı `10.0` gibi noktalı yaz",
    ),
    (
        "cannot find value `self` in this scope",
        "`kendi` yalnızca ilk parametresi `kendi` olan yöntemlerde kullanılabilir",
    ),
    (
        "`self` value is a keyword only available in methods with a `self` parameter",
        "`kendi` yalnızca ilk parametresi `kendi` olan yöntemlerde kullanılabilir",
    ),
    ("this function doesn't have a `self` parameter", "bu işlevin `kendi` parametresi yok"),
    (
        "add a `self` receiver parameter to make the associated `fn` a method",
        "yöntem yapmak için ilk parametre olarak `&kendi` ya da `&değişken kendi` ekle",
    ),
    ("this is an associated function, not a method", "bu bir yöntem değil, türe bağlı bir işlev"),
    ("method `{}` not found for this struct", "bu yapıda `{0}` adında bir yöntem yok"),
    (
        "found the following associated functions; to be used as methods, functions must have a `self` parameter",
        "şu işlevler bulundu; yöntem olarak kullanılabilmeleri için `kendi` parametresi almaları gerekir",
    ),
    ("the candidate is defined in an impl for the type `{}`", "aday, `{0}` türünün `uygula` bloğunda tanımlı"),
    ("use associated function syntax instead", "bunun yerine `Tür::işlev()` biçiminde çağır"),
    (
        "cannot assign to `{}`, as `{}` is not declared as mutable",
        "`{1}` `değişken` olarak tanımlanmadığı için `{0}` değiştirilemez",
    ),
    ("try adding a return type", "işleve bir dönüş türü eklemeyi dene"),
    ("expected because this is `{}`", "bu `{0}` olduğu için bekleniyor"),
    (
        "you can convert {} `{}` to {} `{}` and panic if the converted value doesn't fit",
        "`{1}` değerini `{3}` türüne çevirebilirsin (değer sığmazsa program durur)",
    ),
    ("you can convert {} `{}` to {} `{}`{}", "`{1}` değerini `{3}` türüne çevirebilirsin"),
    ("`+` cannot be used to concatenate two `&str` strings", "`+` ile iki `&str` metin birleştirilemez"),
    (
        "string concatenation requires an owned `{}` on the left",
        "metinleri `+` ile birleştirmek için soldakinin bir `{0}` olması gerekir",
    ),
    ("create an owned `{}` from a string reference", "soldaki metinden bir `{0}` oluştur"),
    (
        "the `{}` expression is missing a block after this condition",
        "`{0}` koşulundan sonra süslü parantezli bir blok (`{{ … }}`) gelmeli",
    ),
    (
        "you might have meant to write this as part of a block",
        "bunu bir bloğun (`{{ … }}`) içine yazmak istemiş olabilirsin",
    ),
    ("`{}` does not implement `{}`", "`{0}` türü `{1}` niteliğini uygulamıyor"),
    ("function cannot return without recursing", "bu işlev kendini durmadan çağırıyor; hiç bitmeyecek"),
    ("cannot return without recursing", "kendini çağırmadan hiç geri dönemiyor"),
    ("recursive call site", "kendini burada çağırıyor"),
    (
        "a `loop` may express intention better if this is on purpose",
        "bilerek yapıyorsan `döngü` kullanmak daha anlaşılır olur",
    ),
    // İsim çözümleme
    ("cannot find value `{}` in this scope", "`{0}` adında bir değer bulunamadı"),
    ("cannot find function `{}` in this scope", "`{0}` adında bir işlev bulunamadı"),
    ("cannot find type `{}` in this scope", "`{0}` adında bir tür bulunamadı"),
    ("cannot find macro `{}` in this scope", "`{0}!` adında bir makro bulunamadı"),
    ("cannot find struct, variant or union type `{}` in this scope", "`{0}` adında bir yapı ya da seçenek bulunamadı"),
    ("failed to resolve: use of undeclared type `{}`", "`{0}` türü tanımlanmamış"),
    (
        "failed to resolve: use of unresolved module or unlinked crate `{}`",
        "`{0}` adında bir modül ya da sandık bulunamadı",
    ),
    ("failed to resolve: {}", "bulunamadı: {0}"),
    ("unresolved import `{}`", "`{0}` kullanıma alınamadı (bulunamadı)"),
    ("not found in this scope", "bu kapsamda bulunamadı"),
    ("a local variable with a similar name exists", "benzer adlı bir yerel değişken var"),
    ("a function with a similar name exists", "benzer adlı bir işlev var"),
    ("a macro with a similar name exists", "benzer adlı bir makro var"),
    ("consider importing this {}", "bunu `kullan` ile kullanıma almayı düşün"),
    ("consider importing one of these {}", "bunlardan birini `kullan` ile kullanıma almayı düşün"),
    ("use of undeclared type `{}`", "`{0}` türü tanımlanmamış"),
    ("`main` function not found in crate `{}`", "`{0}` sandığında `ana` işlevi bulunamadı"),
    ("consider adding a `main` function to `{}`", "`{0}` dosyasına bir `işlev ana() {{ … }}` eklemelisin"),
    ("trying to load file for module `{}` with non-ascii identifier name", "`{0}` modülünün dosyası bulunamadı"),
    // Türler
    ("mismatched types", "türler uyuşmuyor"),
    // `süz(|not| not > 50)`: `süz` elemanları iki kez ödünç verir.
    (
        "expected `&&_`, found integer",
        "burada ödünç alınmış bir değer (`&&`) var; sayıyla karşılaştırmak için değişkenin başına `**` koy (örneğin `**x > 50`)",
    ),
    ("expected `{}`, found `{}`", "`{0}` bekleniyordu ama `{1}` bulundu"),
    ("expected `{}` because of return type", "dönüş türü yüzünden `{0}` bekleniyor"),
    ("expected due to this", "bu yüzden bekleniyor"),
    ("arguments to this function are incorrect", "bu işleve verilen argümanlar hatalı"),
    ("arguments to this method are incorrect", "bu yönteme verilen argümanlar hatalı"),
    (
        "this function takes {} argument{} but {} argument{} supplied",
        "bu işlev {0} argüman alıyor ama {2} argüman verildi",
    ),
    (
        "this method takes {} argument{} but {} argument{} supplied",
        "bu yöntem {0} argüman alıyor ama {2} argüman verildi",
    ),
    (
        "implicitly returns `()` as its body has no tail or `return` expression",
        "gövdesi bir değer döndürmediği için `()` döndürüyor",
    ),
    ("remove this semicolon to return this value", "bu değeri döndürmek için noktalı virgülü kaldır"),
    ("`if` and `else` have incompatible types", "`eğer` ve `değilse` kolları farklı türde değerler veriyor"),
    ("`match` arms have incompatible types", "`eşle` kolları farklı türde değerler veriyor"),
    ("`if` may be missing an `else` clause", "`eğer` ifadesinin `değilse` kolu eksik olabilir"),
    ("type annotations needed", "türü açıkça yazman gerekiyor"),
    ("consider giving `{}` an explicit type", "`{0}` için türü açıkça yazmayı düşün"),
    ("no method named `{}` found for {} `{}` in the current scope", "`{2}` türünde `{0}` adında bir yöntem yok"),
    ("method not found in `{}`", "`{0}` türünde böyle bir yöntem yok"),
    ("no field `{}` on type `{}`", "`{1}` türünde `{0}` adında bir alan yok"),
    ("unknown field", "bilinmeyen alan"),
    ("struct `{}` has no field named `{}`", "`{0}` yapısında `{1}` adında bir alan yok"),
    ("missing field `{}` in initializer of `{}`", "`{1}` oluşturulurken `{0}` alanı eksik"),
    ("missing fields {} in initializer of `{}`", "`{1}` oluşturulurken şu alanlar eksik: {0}"),
    ("cannot add `{}` to `{}`", "`{1}` ile `{0}` toplanamaz"),
    ("cannot subtract `{}` from `{}`", "`{1}` değerinden `{0}` çıkarılamaz"),
    ("cannot multiply `{}` by `{}`", "`{0}` ile `{1}` çarpılamaz"),
    ("cannot divide `{}` by `{}`", "`{0}`, `{1}` ile bölünemez"),
    ("binary operation `{}` cannot be applied to type `{}`", "`{0}` işlemi `{1}` türüne uygulanamaz"),
    ("can't compare `{}` with `{}`", "`{0}` ile `{1}` karşılaştırılamaz"),
    ("`{}` doesn't implement `{}`", "`{0}` türü `{1}` niteliğini uygulamıyor"),
    (
        "`{}` cannot be formatted using `{:?}` because it doesn't implement `{}`",
        "`{0}` `{{:?}}` ile yazdırılamaz çünkü `{1}` niteliğini uygulamıyor",
    ),
    ("consider annotating `{}` with `{}`", "`{0}` tanımının üstüne `{1}` ekle"),
    ("`{}` cannot be formatted with the default formatter", "`{0}` doğrudan `{{}}` ile yazdırılamaz"),
    ("unsatisfied trait bound", "bu tür istenen niteliği karşılamıyor"),
    ("`{}` doesn't satisfy the trait bound", "`{0}` istenen niteliği karşılamıyor"),
    ("`{}` implements trait `{}`", "`{0}` türü `{1}` niteliğini uyguluyor"),
    ("the type must implement `{}`", "türün `{0}` niteliğini uygulaması gerekiyor"),
    ("required for the cast from `{}` to `{}`", "`{0}` değerini `{1}` olarak kullanabilmek için gerekli"),
    (
        "this trait has no implementations, consider adding one",
        "bu niteliği uygulayan hiçbir tür yok; `uygula … her …` ile bir türe uygula",
    ),
    (
        "the trait bound `{}: RastgeleSayi` is not satisfied",
        "`rastgele!` yalnızca sayılarla çalışır; `{0}` bir sayı değil",
    ),
    ("expected {} `{}`\n   found {} `{}`", "beklenen: `{1}`, bulunan: `{3}`"),
    ("expected {} `{}`\n{}found {} `{}`", "beklenen: `{1}`, bulunan: `{4}`"),
    ("expected because of this", "bu yüzden bekleniyor"),
    ("consider borrowing here", "burada ödünç almayı (`&`) düşün"),
    // Tekli işlemler ve dizinleme
    ("cannot apply unary operator `{}` to type `{}`", "`{1}` türüne `{0}` işlemi uygulanamaz"),
    ("cannot apply unary operator `{}`", "`{0}` işlemi uygulanamaz"),
    (
        "unsigned values cannot be negated",
        "işaretsiz (`u` ile başlayan) türler eksi olamaz; eksi sayılar için `i32` gibi bir tür kullan",
    ),
    (
        "the type `{}` cannot be indexed by `{}`",
        "`{0}` türünde köşeli parantezle (`[…]`) `{1}` sırasındaki elemana ulaşılamaz",
    ),
    (
        "string indices are ranges of `usize`",
        "metinlerde köşeli parantez içine tek bir sayı değil, bir aralık yazılır: `ad[0..2]`",
    ),
    ("you can use `{}` or `{}`\n{}", "bunun yerine `{0}` ya da `{1}` kullanabilirsin"),
    // Nitelikler
    ("not all trait items implemented, missing: {}", "niteliğin bütün yöntemleri yazılmamış; eksik: {0}"),
    ("missing `{}` in implementation", "`{0}` burada eksik"),
    ("`{}` from trait", "nitelikteki `{0}`"),
    ("implement the missing item: `{}`", "eksik yöntemi ekle: `{0}`"),
    ("implement the missing items: `{}`", "eksik yöntemleri ekle: `{0}`"),
    // Çeşitler
    (
        "no variant, associated function, or constant named `{}` found for enum `{}` in the current scope",
        "`{1}` çeşidinde `{0}` adında bir seçenek yok",
    ),
    ("variant, associated function, or constant not found in `{}`", "`{0}` içinde bulunamadı"),
    (
        "variant, associated function, or constant `{}` not found for this enum",
        "bu çeşitte `{0}` adında bir seçenek yok",
    ),
    (
        "no function or associated item named `{}` found for {} `{}` in the current scope",
        "`{2}` türünde `{0}` adında bir işlev yok",
    ),
    ("function or associated item not found in `{}`", "`{0}` içinde böyle bir işlev yok"),
    // `?` işareti
    (
        "the `?` operator can only be used in a function that returns `{}` or `{}` (or another type that implements `{}`)",
        "`?` işareti yalnızca `{0}` ya da `{1}` döndüren bir işlevin içinde kullanılabilir",
    ),
    ("cannot use the `?` operator in a function that returns `{}`", "`{0}` döndüren bir işlevde `?` kullanılamaz"),
    (
        "this function should return `{}` or `{}` to accept `?`",
        "`?` kullanabilmek için bu işlev `{0}` ya da `{1}` döndürmeli",
    ),
    (
        "consider adding return type",
        "işleve bir dönüş türü eklemeyi düşün; `ana` içindeysen `?` yerine `eşle` ya da `.sarmalaç()` kullan",
    ),
    (
        "this `{}` may be an `{}` variant, which should be handled",
        "bu `{0}` bir `{1}` olabilir; bu durumu (örneğin `eşle` ile) ele almalısın",
    ),
    ("use `let _ = ...` to ignore the resulting value", "sonucu bilerek yok saymak için `yap _ = ...` yaz"),
    ("the trait bound `{}: {}` is not satisfied", "`{0}` türü `{1}` niteliğini karşılamıyor"),
    ("the trait `{}` is not implemented for `{}`", "`{1}` türü `{0}` niteliğini uygulamıyor"),
    // Değiştirilebilirlik ve sahiplik
    (
        "cannot assign twice to immutable variable `{}`",
        "`{0}` değiştirilemez bir değişken; ona ikinci kez değer verilemez",
    ),
    ("cannot assign twice to immutable variable", "değiştirilemez değişkene ikinci kez değer verilemez"),
    (
        "cannot assign to `{}`, as it is not declared as mutable",
        "`{0}` `değişken` olarak tanımlanmadığı için değeri değiştirilemez",
    ),
    ("cannot assign", "değer verilemez"),
    ("first assignment to `{}`", "`{0}` için ilk değer burada verildi"),
    (
        "cannot borrow `{}` as mutable, as it is not declared as mutable",
        "`{0}` `değişken` olarak tanımlanmadığı için değiştirilemez",
    ),
    ("cannot borrow as mutable", "değiştirilebilir olarak ödünç alınamaz"),
    ("consider changing this to be mutable", "değiştirebilmek için başına `değişken` ekle"),
    ("consider making this binding mutable", "değerini sonradan değiştirebilmek için başına `değişken` ekle"),
    (
        "consider making this binding mutable: `{}`",
        "değerini sonradan değiştirebilmek için başına `değişken` ekle: `{0}`",
    ),
    ("to modify the original value, take a borrow instead", "asıl değeri değiştirmek için onu ödünç al"),
    ("borrow of moved value: `{}`", "`{0}` taşındıktan sonra ödünç alınmaya çalışıldı"),
    ("use of moved value: `{}`", "`{0}` taşındıktan sonra kullanılmaya çalışıldı"),
    ("value moved here", "değer burada taşındı"),
    ("value moved here, in previous iteration of loop", "değer döngünün önceki turunda burada taşındı"),
    ("value borrowed here after move", "değer taşındıktan sonra burada ödünç alındı"),
    ("value used here after move", "değer taşındıktan sonra burada kullanıldı"),
    (
        "move occurs because `{}` has type `{}`, which does not implement the `Copy` trait",
        "`{0}` değerinin türü `{1}` ve bu tür kendiliğinden kopyalanamıyor (`Copy` değil); bu yüzden taşındı",
    ),
    (
        "consider cloning the value if the performance cost is acceptable",
        "değerin bir kopyasını (`.kopyala()`) kullanabilirsin",
    ),
    (
        "cannot borrow `{}` as mutable more than once at a time",
        "`{0}` aynı anda birden fazla kez değiştirilebilir olarak ödünç alınamaz",
    ),
    (
        "cannot borrow `{}` as mutable because it is also borrowed as immutable",
        "`{0}` okunmak üzere ödünç alınmışken değiştirilebilir olarak ödünç alınamaz",
    ),
    (
        "cannot borrow `{}` as immutable because it is also borrowed as mutable",
        "`{0}` değiştirilmek üzere ödünç alınmışken okunmak üzere ödünç alınamaz",
    ),
    ("first mutable borrow occurs here", "ilk değiştirilebilir ödünç burada"),
    ("second mutable borrow occurs here", "ikinci değiştirilebilir ödünç burada"),
    ("first borrow later used here", "ilk ödünç daha sonra burada kullanılıyor"),
    ("mutable borrow occurs here", "değiştirilebilir ödünç burada"),
    ("immutable borrow occurs here", "okuma amaçlı ödünç burada"),
    ("immutable borrow later used here", "okuma amaçlı ödünç daha sonra burada kullanılıyor"),
    ("mutable borrow later used here", "değiştirilebilir ödünç daha sonra burada kullanılıyor"),
    ("borrow later used here", "ödünç daha sonra burada kullanılıyor"),
    ("`{}` does not live long enough", "`{0}` yeterince uzun yaşamıyor"),
    ("borrowed value does not live long enough", "ödünç alınan değer yeterince uzun yaşamıyor"),
    ("`{}` dropped here while still borrowed", "`{0}` hâlâ ödünç alınmışken burada yok edildi"),
    ("used binding `{}` isn't initialized", "`{0}` değişkenine henüz bir değer verilmemiş"),
    ("binding declared here but left uninitialized", "değişken burada tanımlandı ama değer verilmedi"),
    ("missing lifetime specifier", "yaşam süresi belirtilmemiş"),
    ("expected named lifetime parameter", "burada bir yaşam süresi adı bekleniyordu"),
    (
        "this function's return type contains a borrowed value, but there is no value for it to be borrowed from",
        "bu işlev ödünç alınmış bir değer (`&`) döndürüyor ama ödünç alınabilecek bir girdisi yok",
    ),
    (
        "consider using the `{}` lifetime, but this is uncommon unless you're returning a borrowed value from a `{}` or a `{}`",
        "`{0}` yaşam süresini kullanabilirsin ama bu yalnızca `{1}` ya da `{2}` bir değeri döndürürken işe yarar",
    ),
    (
        "instead, you are more likely to want to return an owned value",
        "büyük ihtimalle ödünç yerine değerin kendisini döndürmek istiyorsun (örneğin `&str` yerine `Metin`)",
    ),
    // Desenler
    ("non-exhaustive patterns: {} not covered", "`eşle` tüm durumları kapsamıyor: {0} eksik"),
    ("pattern {} not covered", "{0} deseni eksik"),
    ("unreachable pattern", "bu desene hiçbir zaman ulaşılamaz"),
    // Sözdizimi ve parantezler
    ("add `;` here", "buraya `;` ekle"),
    ("closing delimiter possibly meant for this", "bu kapanış işareti belki şunun içindi"),
    ("this opening brace...", "bu açılış süslü parantezi..."),
    ("...matches this closing brace", "...bu kapanışla eşleşiyor"),
    ("you might have meant to compare for equality", "karşılaştırmak istediysen `=` yerine `==` yaz"),
    ("a macro named `{}` exists in another namespace", "`{0}!` adında bir makro var"),
    ("use `!` to invoke the macro", "makroyu çağırmak için adının sonuna `!` koy"),
    // İşlevler, alanlar, desenler
    ("argument #{} of type `{}` is missing", "{0}. argüman (`{1}` türünde) eksik"),
    ("provide the argument", "eksik argümanı ver"),
    ("provide the arguments", "eksik argümanları ver"),
    ("function defined here", "işlev burada tanımlı"),
    ("method defined here", "yöntem burada tanımlı"),
    ("`{}` defined here", "`{0}` burada tanımlı"),
    ("available field is: `{}`", "kullanılabilen alan: `{0}`"),
    ("available fields are: {}", "kullanılabilen alanlar: {0}"),
    ("missing `{}`", "`{0}` eksik"),
    ("the matched value is of type `{}`", "eşlenen değerin türü `{0}`"),
    (
        "ensure that all possible cases are being handled{}",
        "eksik durumlar için kol ekle ya da en sona her şeyi yakalayan `_ => …` kolunu koy",
    ),
    ("convert the identifier to snake case", "adı küçük harflerle yaz"),
    ("convert the identifier to upper camel case", "adı büyük harfle başlat"),
    ("remove the whole `{}` item", "bütün `{0}` satırını kaldır"),
    // Türler ve işlemler
    ("no implementation for `{}` and `{}`", "`{0}` ve `{1}` işlemleri yapılamaz"),
    ("no implementation for `{}`", "`{0}` işlemi yapılamaz"),
    (
        "the literal `{}` does not fit into the type `{}` whose range is `{}`",
        "`{0}` sayısı `{1}` türüne sığmıyor; bu türün aralığı `{2}`",
    ),
    ("type annotations needed for `{}`", "`{0}` için türü açıkça yazman gerekiyor"),
    ("type must be known at this point", "tür burada bilinmeli"),
    (
        "consider giving `{}` an explicit type, where the type for type parameter `{}` is specified",
        "`{0}` için türü açıkça yaz; `{1}` yerine gelecek türü de belirt",
    ),
    (
        "consider using `{}` to unwrap the `{}` value, panicking if the value is a `{}`",
        "değeri almak için `{0}` kullanabilirsin; değer `{2}` ise program durur",
    ),
    ("`{}` is not an iterator", "`{0}` üzerinde dolaşılamaz"),
    (
        "if you want to iterate between `start` until a value `end`, use the exclusive range syntax `start..end` or the inclusive range syntax `start..=end`",
        "bir aralıkta dolaşmak için `0..10` (10 hariç) ya da `0..=10` (10 dahil) yaz",
    ),
    ("required for `{}` to implement `{}`", "`{0}` türünün `{1}` niteliğini uygulaması gerekiyor"),
    // Yazdırma (biçim dizgileri)
    ("required by this formatting parameter", "bu yazdırma yeri bunu gerektiriyor"),
    (
        "in format strings you may be able to use `{:?}` (or `{:#?}` for pretty-print) instead",
        "`{{}}` yerine `{{:?}}` (daha okunaklı görünüm için `{{:#?}}`) yazmayı dene; yapının üstüne `#[derive(Debug)]` eklemeyi unutma",
    ),
    (
        "{} positional argument{} in format string, but no arguments were given",
        "yazdırma dizgisinde {0} tane `{{}}` var ama hiç değer verilmemiş",
    ),
    (
        "{} positional argument{} in format string, but there {} {} argument{}",
        "yazdırma dizgisinde {0} tane `{{}}` var ama {3} değer verilmiş",
    ),
    ("argument never used", "bu değer hiç kullanılmıyor (dizgide onun için `{{}}` yok)"),
    ("multiple unused formatting arguments", "birden fazla değer hiç kullanılmıyor"),
    ("formatting specifier missing", "`{{}}` eksik"),
    // Sözdizimi
    ("expected identifier, found keyword `{}`", "bir isim bekleniyordu ama ayrılmış sözcük `{0}` bulundu"),
    ("expected identifier, found reserved keyword `{}`", "bir isim bekleniyordu ama ayrılmış sözcük `{0}` bulundu"),
    ("escape `{}` to use it as an identifier", "`{0}` sözcüğünü isim olarak kullanmak için başına `r#` ekle"),
    ("expected identifier, found keyword", "bir isim bekleniyordu ama ayrılmış sözcük bulundu"),
    ("expected one of {}, found {}", "şunlardan biri bekleniyordu: {0}; ama {1} bulundu"),
    ("expected {}, found {}", "{0} bekleniyordu ama {1} bulundu"),
    ("expected one of {}", "şunlardan biri bekleniyordu: {0}"),
    ("expected `{}`", "`{0}` bekleniyordu"),
    ("unexpected token", "beklenmeyen sözcük"),
    ("unexpected closing delimiter: `{}`", "beklenmeyen kapanış işareti: `{0}`"),
    ("mismatched closing delimiter: `{}`", "kapanış işareti açılışla uyuşmuyor: `{0}`"),
    ("this file contains an unclosed delimiter", "bu dosyada kapatılmamış bir parantez var"),
    ("unclosed delimiter", "kapatılmamış işaret"),
    ("unterminated double quote string", "kapatılmamış dizgi: sonuna `\"` eklemeyi unutma"),
    ("missing `fn` for function definition", "işlev tanımında `işlev` sözcüğü eksik"),
    ("unknown character escape: `{}`", "bilinmeyen kaçış karakteri: `{0}`"),
    ("integer literal is too large", "sayı çok büyük"),
    ("literal out of range for `{}`", "sayı `{0}` türünün sınırları dışında"),
    ("this arithmetic operation will overflow", "bu işlem taşma yapacak (sonuç bu sayı türüne sığmıyor)"),
    ("this operation will panic at runtime", "bu işlem program çalışırken çökmeye (paniğe) yol açacak"),
    (
        "index out of bounds: the length is {} but the index is {}",
        "dizin sınırların dışında: uzunluk {0} ama dizin {1}",
    ),
    ("attempt to divide `{}` by zero", "`{0}` sıfıra bölünmeye çalışılıyor"),
    // Uyarılar
    (
        "found both `{}` and `{}` as identifiers, which look alike",
        "`{0}` ve `{1}` birbirine çok benzeyen iki ayrı isim; aynı şeyi mi kastettin?",
    ),
    ("other identifier used here", "diğer isim burada"),
    ("this identifier can be confused with `{}`", "bu isim `{0}` ile karıştırılabilir"),
    ("fields {} are never read", "şu alanlar hiç okunmamış: {0}"),
    (
        "`{}` has a derived impl for the trait `{}`, but this is intentionally ignored during dead code analysis",
        "`{0}` için `{1}` türetilmiş olması, alanların kullanıldığı anlamına gelmez",
    ),
    ("unused variable: `{}`", "`{0}` değişkeni hiç kullanılmamış"),
    ("if this is intentional, prefix it with an underscore", "bilerek yaptıysan başına alt çizgi koy"),
    ("if this is intentional, prefix it with an underscore: `{}`", "bilerek yaptıysan başına alt çizgi koy: `{0}`"),
    ("variable does not need to be mutable", "burada `değişken` yazmana gerek yok, değeri hiç değişmiyor"),
    ("remove this `mut`", "bu `değişken` sözcüğünü kaldır"),
    ("variable `{}` is assigned to, but never used", "`{0}` değişkenine değer veriliyor ama hiç kullanılmıyor"),
    ("consider using `{}` instead", "bunun yerine `{0}` yazabilirsin"),
    ("maybe it is overwritten before being read?", "değeri okunmadan önce üzerine yeni bir değer yazılıyor olabilir"),
    ("value assigned to `{}` is never read", "`{0}` değişkenine verilen değer hiç okunmamış"),
    ("this value is reassigned later and never used", "bu değer sonradan değiştiriliyor ve hiç kullanılmıyor"),
    (
        "`{}` is overwritten here before the previous value is read",
        "`{0}` önceki değeri okunmadan burada yeni değer alıyor",
    ),
    ("unused import: `{}`", "kullanılmayan `kullan`: `{0}`"),
    ("unused imports: {}", "kullanılmayan `kullan`lar: {0}"),
    ("function `{}` is never used", "`{0}` işlevi hiç kullanılmamış"),
    ("method `{}` is never used", "`{0}` yöntemi hiç kullanılmamış"),
    ("struct `{}` is never constructed", "`{0}` yapısı hiç oluşturulmamış"),
    ("enum `{}` is never used", "`{0}` çeşidi hiç kullanılmamış"),
    ("variant `{}` is never constructed", "`{0}` seçeneği hiç oluşturulmamış"),
    ("field `{}` is never read", "`{0}` alanı hiç okunmamış"),
    ("constant `{}` is never used", "`{0}` sabiti hiç kullanılmamış"),
    ("unused `{}` that must be used", "kullanılması gereken `{0}` değeri kullanılmamış"),
    ("field in this struct", "bu yapıdaki alan"),
    ("fields in this struct", "bu yapıdaki alanlar"),
    ("field in this variant", "bu seçenekteki alan"),
    ("fields in this variant", "bu seçenekteki alanlar"),
    ("variant in this enum", "bu çeşitteki seçenek"),
    ("variants in this enum", "bu çeşitteki seçenekler"),
    ("unreachable statement", "bu satıra hiçbir zaman ulaşılamaz"),
    ("unreachable expression", "bu ifadeye hiçbir zaman ulaşılamaz"),
    ("any code following this expression is unreachable", "bu ifadeden sonraki koda hiçbir zaman ulaşılamaz"),
    ("variable `{}` should have a snake case name", "`{0}` değişkeninin adı küçük harflerle ve alt çizgiyle yazılmalı"),
    ("function `{}` should have a snake case name", "`{0}` işlevinin adı küçük harflerle ve alt çizgiyle yazılmalı"),
    (
        "type `{}` should have an upper camel case name",
        "`{0}` türünün adı büyük harfle başlamalı ve sözcükler bitişik yazılmalı",
    ),
    ("convert the identifier to snake case: `{}`", "şöyle yazabilirsin: `{0}`"),
    ("convert the identifier to upper camel case: `{}`", "şöyle yazabilirsin: `{0}`"),
];

/// Çalışma zamanında (panik) görülen mesajlar.
const PANIK: &[(&str, &str)] = &[
    (
        "index out of bounds: the len is {} but the index is {}",
        "dizin sınırların dışında: uzunluk {0} ama istenen dizin {1}",
    ),
    ("attempt to add with overflow", "toplamada taşma oldu: sonuç bu sayı türüne sığmıyor"),
    (
        "attempt to subtract with overflow",
        "çıkarmada taşma oldu: sonuç bu sayı türüne sığmıyor (işaretsiz sayılar sıfırın altına inemez)",
    ),
    ("attempt to multiply with overflow", "çarpmada taşma oldu: sonuç bu sayı türüne sığmıyor"),
    ("attempt to negate with overflow", "eksi alırken taşma oldu"),
    ("attempt to divide by zero", "sıfıra bölmeye çalışıldı"),
    ("attempt to calculate the remainder with a divisor of zero", "sıfıra göre kalan hesaplanmaya çalışıldı"),
    ("called `{}` on a `{}` value", "`{1}` değeri üzerinde `{0}` çağrıldı (içinde değer yoktu)"),
    ("called `{}` on an `{}` value: {}", "`{1}` değeri üzerinde `{0}` çağrıldı: {2}"),
    ("explicit panic", "panik! çağrıldı"),
    ("not yet implemented: {}", "henüz yazılmadı: {0}"),
    ("not yet implemented", "henüz yazılmadı (yapılacak!)"),
    ("internal error: entered unreachable code: {}", "ulaşılamaz denen koda ulaşıldı: {0}"),
    ("internal error: entered unreachable code", "ulaşılamaz denen koda ulaşıldı"),
    ("assertion failed: {}", "doğrulama başarısız: `{0}`"),
    ("assertion `left == right` failed: {}", "doğrulama başarısız (`sol == sağ`): {0}"),
    ("assertion `left == right` failed", "doğrulama başarısız: `sol == sağ` değil"),
    ("assertion `left != right` failed: {}", "doğrulama başarısız (`sol != sağ`): {0}"),
    ("assertion `left != right` failed", "doğrulama başarısız: `sol != sağ` değil"),
    ("  left: {}", "   sol: {0}"),
    (" right: {}", "   sağ: {0}"),
];

/// Yakalanan düz metin içinde çevrilen sözcükler.
const DUZ_METIN: &[(&str, &str)] = &[
    ("{float}", "{ondalıklı sayı}"),
    ("ParseIntError { kind: InvalidDigit }", "metin bir tam sayı değil"),
    ("ParseIntError { kind: Empty }", "metin boş, sayıya çevrilemez"),
    ("ParseIntError { kind: PosOverflow }", "sayı bu türe sığmayacak kadar büyük"),
    ("ParseIntError { kind: NegOverflow }", "sayı bu türe sığmayacak kadar küçük"),
    ("ParseFloatError { kind: Invalid }", "metin bir ondalıklı sayı değil"),
    ("ParseFloatError { kind: Empty }", "metin boş, sayıya çevrilemez"),
    (", or ", " veya "),
    (" or ", " veya "),
    (" and ", " ve "),
    ("integer", "tam sayı"),
    ("floating-point number", "ondalıklı sayı"),
    ("possible tokens", "olası sözcük"),
    ("reserved keyword", "ayrılmış sözcük"),
    ("keyword", "ayrılmış sözcük"),
    ("identifier", "isim"),
];

/// Çocuklar için hata kodu açıklamaları.
const ACIKLAMALAR: &[(&str, &str)] = &[
    (
        "E0004",
        "`eşle` ifadesi olabilecek bütün durumları kapsamalı. Eksik durumları ekle ya da en sona her şeyi yakalayan `_ => …` kolunu koy.",
    ),
    (
        "E0046",
        "Bir niteliği bir türe uygularken niteliğin gövdesiz bütün yöntemlerini yazmalısın. Eksik yöntemi `uygula … her …` bloğunun içine ekle.",
    ),
    ("E0063", "Bir yapı oluştururken bütün alanlarına değer vermelisin. Eksik alanı da yaz: `Kedi { ad: …, yaş: 1 }`"),
    (
        "E0061",
        "İşleve verdiğin argüman sayısı, işlevin beklediği sayıdan farklı. İşlevin tanımına bakıp her parametre için bir değer ver.",
    ),
    (
        "E0424",
        "`kendi`, yöntemin çağrıldığı değerdir ve yalnızca ilk parametresi `kendi` olan yöntemlerde kullanılabilir: `işlev artır(&değişken kendi) { … }`",
    ),
    (
        "E0594",
        "Bir yapının alanını değiştirebilmek için yapıyı tutan değişkenin `değişken` olarak tanımlanması gerekir: `yap değişken kedi = …;`",
    ),
    ("E0765", "Bir metni açan `\"` işaretinin bir de kapatanı olmalı. Metnin sonuna `\"` koymayı unutma."),
    (
        "E0282",
        "Pas bu değerin türünü kendi başına çıkaramadı. Değişkenin yanına türünü yaz: `yap sayılar: Liste<i32> = …`",
    ),
    (
        "E0283",
        "Pas bu değerin türünü kendi başına çıkaramadı. Değişkenin yanına türünü yaz: `yap sayılar: Liste<i32> = …`",
    ),
    (
        "E0106",
        "Ödünç alınan (`&`) bir değer döndürüyorsan, bu değerin ne kadar yaşayacağını (yaşam süresini) belirtmen gerekir. Çoğu zaman en kolayı ödünç yerine `Metin` gibi sahip olunan bir değer döndürmektir.",
    ),
    (
        "E0277",
        "Bu tür, burada istenen yeteneğe (niteliğe) sahip değil. Mesajda hangi türün hangi niteliği karşılamadığı yazıyor: başka bir tür kullanmayı ya da o niteliği türüne uygulamayı dene.",
    ),
    (
        "E0308",
        "Beklenen tür ile verdiğin değerin türü farklı. Örneğin sayı beklenen bir yere metin yazmış olabilirsin; ya da bir işlev değer döndürmesi gerekirken son satırın sonuna `;` koymuş olabilirsin.",
    ),
    (
        "E0369",
        "Bu işlem (örneğin `+` ya da `==`) bu türler arasında yapılamıyor. İki tarafın da aynı türde olduğundan emin ol.",
    ),
    ("E0381", "Bir değişkeni kullanmadan önce ona bir değer vermelisin."),
    (
        "E0382",
        "Bir değer başka bir yere taşındığında (başka bir değişkene verildiğinde ya da bir işleve gönderildiğinde) eski adıyla artık kullanılamaz. Kopyasına ihtiyacın varsa `.kopyala()` kullan; sadece okuyacaksan `&` ile ödünç ver.",
    ),
    (
        "E0384",
        "Pas'ta değişkenler kendiliğinden değiştirilemez. Değerini sonradan değiştirmek istiyorsan değişkeni `yap değişken x = …;` diye tanımla.",
    ),
    ("E0412", "Bu isimde bir tür yok. Yazımını kontrol et ya da türü `kullan` ile kullanıma al."),
    ("E0423", "Bu isim bir değer gibi kullanılmış ama bir değer değil (örneğin bir tür olabilir)."),
    (
        "E0425",
        "Kullandığın isim burada tanımlı değil. Yazımını kontrol et (büyük-küçük harfler de önemli) ya da kullanmadan önce `yap` ile tanımla.",
    ),
    ("E0432", "`kullan` ile almak istediğin şey bulunamadı. Yolunu ve yazımını kontrol et."),
    ("E0433", "Bu isim bulunamadı. Yazımını kontrol et ya da `kullan` ile kullanıma al."),
    (
        "E0499",
        "Bir değeri aynı anda yalnızca tek bir yerden değiştirebilirsin. Bu kural, programının beklenmedik biçimde bozulmasını engeller.",
    ),
    ("E0502", "Bir değeri okurken aynı anda değiştiremezsin. Önce okumayı bitir, sonra değiştir."),
    ("E0596", "Bunu değiştirebilmek için değişkenin `değişken` olarak tanımlanması gerekir: `yap değişken x = …;`"),
    (
        "E0597",
        "Ödünç aldığın değer, sen onu kullanmayı bitirmeden yok oluyor. Değeri daha dış bir blokta tanımlamayı dene.",
    ),
    ("E0599", "Bu türün böyle bir yöntemi, seçeneği ya da işlevi yok. Adını ve hangi türe ait olduğunu kontrol et."),
    (
        "E0600",
        "Bu işlem bu türle yapılamaz. Örneğin `u32` gibi işaretsiz türler eksi olamaz; eksi sayılar için `i32` kullan.",
    ),
    ("E0601", "Her Pas programının başladığı bir `ana` işlevi olmalı: `işlev ana() { … }`"),
    ("E0609", "Bu yapının böyle bir alanı yok. Alanın adını kontrol et."),
    ("E0614", "Bu değer bir referans değil; başına `*` koyamazsın."),
    ("E0618", "Bu bir işlev değil; arkasına `()` koyarak çağıramazsın."),
];

/// Bir derleyici mesajını Türkçeye çevirir.
pub fn derleyici_mesajini_cevir(mesaj: &str) -> String {
    cevir(DERLEYICI, mesaj)
}

/// Programın panik dışındaki çalışma zamanı hata satırlarını (yığın taşması
/// gibi) Türkçeye çevirir; bilinmeyen satırlar için `None`.
pub fn calisma_mesajini_cevir(satir: &str) -> Option<String> {
    if let Some(kalan) = satir.strip_prefix("thread '")
        && let Some((is_parcacigi, _)) = kalan.split_once('\'')
        && satir.ends_with("has overflowed its stack")
    {
        let is_parcacigi = if is_parcacigi == "main" { "ana" } else { is_parcacigi };
        return Some(format!(
            "program çöktü: `{is_parcacigi}` iş parçacığının yığını taştı. Bir işlev kendini (doğrudan ya da dolaylı) durmadan çağırıyor olabilir."
        ));
    }
    match satir {
        "fatal runtime error: stack overflow" => Some(String::new()),
        _ if satir.starts_with("fatal runtime error: stack overflow") => Some(String::new()),
        _ => None,
    }
}

/// Bir panik mesajı satırını Türkçeye çevirir.
pub fn panik_mesajini_cevir(mesaj: &str) -> String {
    cevir(PANIK, mesaj)
}

/// Bir hata kodunun çocuklara yönelik açıklaması.
pub fn aciklama(kod: &str) -> Option<&'static str> {
    ACIKLAMALAR.iter().find(|(c, _)| *c == kod).map(|(_, metin)| *metin)
}

/// Kullanıcıya gösterilmesine gerek olmayan mesajlar (özetler, lint notları).
pub fn gereksiz_mi(mesaj: &str) -> bool {
    mesaj.starts_with("aborting due to")
        || mesaj.starts_with("For more information about")
        || mesaj.starts_with("Some errors have detailed explanations")
        || (mesaj.ends_with("on by default") && mesaj.contains("#["))
        || mesaj.ends_with("warning emitted")
        || mesaj.ends_with("warnings emitted")
        || mesaj.starts_with("requested on the command line with")
        || mesaj.starts_with("the following other types implement trait")
        // `{}` ile yazdırılamayan türlerde ana mesajı tekrar eder
        || mesaj.starts_with("the trait `std::fmt::Display` is not implemented for")
        || mesaj.starts_with("a module named `self` exists in another namespace")
        || mesaj.contains("`Turkce<")
        // Bir niteliği uygulayan türlerin uzun listesi
        || (mesaj.contains(" implements trait `") && mesaj.contains('\n'))
        // Destek kodunun (`oku!`) iç ayrıntıları
        || mesaj.contains("`deger_oku`")
        || mesaj.contains("`satir_oku`")
}

/// `oku!` makrosunun sık yapılan yanlış kullanımları için ipucu. `satir`,
/// hatanın gösterdiği Pas satırı; `etiket`, Türkçeleştirilmiş birincil etiket.
pub fn oku_ipucu(kod: &str, satir: &str, etiket: &str) -> Option<&'static str> {
    if !satir.contains("oku!(") {
        return None;
    }
    match kod {
        "E0308" if etiket.contains("`Metin` bulundu") => {
            Some("`oku!` metin okur; sayı okumak için türünü de yaz: `oku!(\"Sayı: \", u32)`")
        }
        "E0308" if etiket.contains("`&str` bekleniyordu") => {
            Some("`oku!` içine soruyu tırnak içinde yaz: `oku!(\"Adın ne? \")`")
        }
        "E0277" => Some("`oku!` yalnızca sayı, `Metin`, `mantıksal` ve `karakter` gibi türleri okuyabilir"),
        _ => None,
    }
}

fn cevir(kaliplar: &[(&str, &str)], mesaj: &str) -> String {
    for (kalip, sablon) in kaliplar {
        if let Some(yakalananlar) = kalipla_esle(kalip, mesaj) {
            return doldur(sablon, &yakalananlar);
        }
    }
    // Yalnızca bir türden oluşan etiketler (`Option<&{integer}>`) koddur.
    if !mesaj.is_empty() && !mesaj.contains(char::is_whitespace) && !mesaj.contains('`') {
        return yakalanani_pasa_cevir(mesaj);
    }
    kod_parcalarini_pasa_cevir(mesaj)
}

fn kalipla_esle<'m>(kalip: &str, mesaj: &'m str) -> Option<Vec<&'m str>> {
    let mut parcalar = kalip.split("{}");
    let mut kalan = mesaj.strip_prefix(parcalar.next()?)?;
    let parcalar: Vec<&str> = parcalar.collect();
    let mut yakalananlar = Vec::with_capacity(parcalar.len());
    for (i, parca) in parcalar.iter().enumerate() {
        if i + 1 == parcalar.len() {
            yakalananlar.push(kalan.strip_suffix(parca)?);
            kalan = "";
        } else {
            let yer = kalan.find(parca)?;
            yakalananlar.push(&kalan[..yer]);
            kalan = &kalan[yer + parca.len()..];
        }
    }
    kalan.is_empty().then_some(yakalananlar)
}

fn doldur(sablon: &str, yakalananlar: &[&str]) -> String {
    let mut cikti = String::with_capacity(sablon.len() + 16);
    let mut kalan = sablon;
    while let Some(acilis) = kalan.find('{') {
        cikti.push_str(&kalan[..acilis].replace("}}", "}"));
        let sonrasi = &kalan[acilis + 1..];
        if let Some(ayiklanan) = sonrasi.strip_prefix('{') {
            cikti.push('{');
            kalan = ayiklanan;
            continue;
        }
        let kapanis = sonrasi.find('}').expect("şablonda kapanmamış `{`");
        let dizin: usize = sonrasi[..kapanis].parse().expect("şablonda geçersiz yakalama numarası");
        cikti.push_str(&yakalanani_pasa_cevir(yakalananlar[dizin]));
        kalan = &sonrasi[kapanis + 1..];
    }
    cikti.push_str(&kalan.replace("}}", "}"));
    cikti
}

fn yakalanani_pasa_cevir(yakalanan: &str) -> String {
    // Dosya yolları (`src/main.rs`) çevrilmez; onları çağıran taraf düzeltir.
    if yakalanan.contains(".rs") {
        return yakalanan.to_owned();
    }
    let mut metin = pasa_cevir(yakalanan);
    // Kod parçalarının içine dokunmadan yalnızca düz metni çevir.
    if !metin.contains('`') && metin.chars().any(char::is_alphabetic) {
        for (ingilizce, turkce) in DUZ_METIN {
            metin = metin.replace(ingilizce, turkce);
        }
    } else {
        let mut cikti = String::new();
        for (i, parca_) in metin.split('`').enumerate() {
            if i > 0 {
                cikti.push('`');
            }
            if i % 2 == 0 {
                let mut parca_ = parca_.to_owned();
                for (ingilizce, turkce) in DUZ_METIN {
                    parca_ = parca_.replace(ingilizce, turkce);
                }
                cikti.push_str(&parca_);
            } else {
                cikti.push_str(parca_);
            }
        }
        metin = cikti;
    }
    metin
}

/// İngilizce bir mesajdaki `kod` parçalarını Pas'a çevirir.
fn kod_parcalarini_pasa_cevir(mesaj: &str) -> String {
    mesaj
        .split('`')
        .enumerate()
        .map(|(i, parca_)| if i % 2 == 1 { pasa_cevir(parca_) } else { parca_.to_owned() })
        .collect::<Vec<_>>()
        .join("`")
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn sablonlar_dogru_bicimde() {
        for (kalip, sablon) in DERLEYICI.iter().chain(PANIK) {
            let bosluk_sayisi = kalip.matches("{}").count();
            let yakalananlar: Vec<&str> = (0..bosluk_sayisi).map(|_| "x").collect();
            // Şablon geçersizse `doldur` panikler.
            doldur(sablon, &yakalananlar);
            assert!(!kalip.contains("{}{}"), "yan yana yakalamalar belirsiz: {kalip}");
        }
    }

    #[test]
    fn derleyici_mesajlari() {
        assert_eq!(
            derleyici_mesajini_cevir("cannot find value `sayi` in this scope"),
            "`sayi` adında bir değer bulunamadı"
        );
        assert_eq!(
            derleyici_mesajini_cevir("expected `String`, found `Option<i32>`"),
            "`Metin` bekleniyordu ama `Belki<i32>` bulundu"
        );
        assert_eq!(
            derleyici_mesajini_cevir("this function takes 1 argument but 2 arguments were supplied"),
            "bu işlev 1 argüman alıyor ama 2 argüman verildi"
        );
        assert_eq!(
            derleyici_mesajini_cevir("expected one of `,`, `.`, or `;`, found `x`"),
            "şunlardan biri bekleniyordu: `,`, `.` veya `;`; ama `x` bulundu"
        );
        assert_eq!(
            derleyici_mesajini_cevir("expected integer, found `&str`"),
            "tam sayı bekleniyordu ama `&str` bulundu"
        );
        assert_eq!(
            derleyici_mesajini_cevir("consider adding a `main` function to `src/main.rs`"),
            "`src/main.rs` dosyasına bir `işlev ana() { … }` eklemelisin"
        );
    }

    #[test]
    fn bilinmeyen_mesajlardaki_kod_pasa_cevrilir() {
        assert_eq!(
            derleyici_mesajini_cevir("some new message about `Vec<String>`"),
            "some new message about `Liste<Metin>`"
        );
    }

    #[test]
    fn tur_etiketleri_cevrilir() {
        assert_eq!(derleyici_mesajini_cevir("Option<&{integer}>"), "Belki<&{tam sayı}>");
        assert_eq!(derleyici_mesajini_cevir("&str"), "&str");
    }

    #[test]
    fn calisma_mesajlari() {
        assert_eq!(
            calisma_mesajini_cevir("thread 'main' has overflowed its stack").as_deref(),
            Some(
                "program çöktü: `ana` iş parçacığının yığını taştı. Bir işlev kendini (doğrudan ya da dolaylı) durmadan çağırıyor olabilir."
            )
        );
        assert_eq!(calisma_mesajini_cevir("fatal runtime error: stack overflow").as_deref(), Some(""));
        assert_eq!(calisma_mesajini_cevir("merhaba"), None);
    }

    #[test]
    fn panik_mesajlari() {
        assert_eq!(
            panik_mesajini_cevir("called `Option::unwrap()` on a `None` value"),
            "`Yok` değeri üzerinde `Belki::sarmalaç()` çağrıldı (içinde değer yoktu)"
        );
        assert_eq!(
            panik_mesajini_cevir("index out of bounds: the len is 3 but the index is 10"),
            "dizin sınırların dışında: uzunluk 3 ama istenen dizin 10"
        );
        assert_eq!(panik_mesajini_cevir("Kendi mesajım"), "Kendi mesajım");
    }

    #[test]
    fn gereksiz_mesajlar() {
        assert!(gereksiz_mi("aborting due to 2 previous errors"));
        assert!(gereksiz_mi("`#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default"));
        assert!(!gereksiz_mi("mismatched types"));
    }
}
