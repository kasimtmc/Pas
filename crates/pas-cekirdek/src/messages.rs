//! rustc ve çalışma zamanı (panik) mesajlarının Türkçe çevirileri.
//!
//! Mesajlar kalıplarla eşleştirilir: `{}` herhangi bir metni yakalar ve
//! Türkçe şablonda `{0}`, `{1}`, … olarak kullanılır. Yakalanan kod
//! parçaları ([`to_pas`] ile) Pas'a çevrilir; böylece `String` yerine
//! `Metin` görünür. Kalıbı olmayan mesajlar İngilizce kalır, ama içlerindeki
//! `ters tırnaklı` kod parçaları yine Pas'a çevrilir.

use crate::translate::to_pas;

/// (İngilizce kalıp, Türkçe şablon). Özel kalıplar genel olanlardan önce gelmeli.
const COMPILER: &[(&str, &str)] = &[
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
    ("consider importing this {}", "bunu `kullan` ile kullanıma almayı düşün ({0})"),
    ("`main` function not found in crate `{}`", "`{0}` sandığında `ana` işlevi bulunamadı"),
    ("consider adding a `main` function to `{}`", "`{0}` dosyasına bir `işlev ana() {{ … }}` eklemelisin"),
    ("trying to load file for module `{}` with non-ascii identifier name", "`{0}` modülünün dosyası bulunamadı"),
    // Türler
    ("mismatched types", "türler uyuşmuyor"),
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
    ("`{}` cannot be formatted with the default formatter", "`{0}` doğrudan `{{}}` ile yazdırılamaz"),
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
    ("consider changing this to be mutable", "bunu `değişken` yapmayı düşün"),
    ("consider making this binding mutable", "bu değişkeni `değişken` yapmayı düşün"),
    ("consider making this binding mutable: `{}`", "bu değişkeni `değişken` yapmayı düşün: `{0}`"),
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
    // Desenler
    ("non-exhaustive patterns: {} not covered", "`eşle` tüm durumları kapsamıyor: {0} eksik"),
    ("pattern {} not covered", "{0} deseni eksik"),
    ("unreachable pattern", "bu desene hiçbir zaman ulaşılamaz"),
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
    ("unused variable: `{}`", "`{0}` değişkeni hiç kullanılmamış"),
    ("if this is intentional, prefix it with an underscore: `{}`", "bilerek yaptıysan başına alt çizgi koy: `{0}`"),
    ("variable does not need to be mutable", "bu değişkenin `değişken` olmasına gerek yok"),
    ("remove this `mut`", "bu `değişken` sözcüğünü kaldır"),
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
const PANIC: &[(&str, &str)] = &[
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
const PROSE: &[(&str, &str)] = &[
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
const EXPLANATIONS: &[(&str, &str)] = &[
    (
        "E0004",
        "`eşle` ifadesi olabilecek bütün durumları kapsamalı. Eksik durumları ekle ya da en sona her şeyi yakalayan `_ => …` kolunu koy.",
    ),
    (
        "E0061",
        "İşleve verdiğin argüman sayısı, işlevin beklediği sayıdan farklı. İşlevin tanımına bakıp her parametre için bir değer ver.",
    ),
    (
        "E0106",
        "Ödünç alınan (`&`) bir değer döndürüyorsan, bu değerin ne kadar yaşayacağını (yaşam süresini) belirtmen gerekir. Çoğu zaman en kolayı ödünç yerine `Metin` gibi sahip olunan bir değer döndürmektir.",
    ),
    (
        "E0277",
        "Bu tür, istenen yeteneğe (niteliğe) sahip değil. Örneğin kendi yapını `{}` ile yazdırmak istiyorsan yapının üstüne `#[derive(Debug)]` ekleyip `{:?}` ile yazdırmayı dene.",
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
        "Pas'ta değişkenler kendiliğinden değiştirilemez. Değerini sonradan değiştirmek istiyorsan değişkeni `olsun değişken x = …;` diye tanımla.",
    ),
    ("E0412", "Bu isimde bir tür yok. Yazımını kontrol et ya da türü `kullan` ile kullanıma al."),
    ("E0423", "Bu isim bir değer gibi kullanılmış ama bir değer değil (örneğin bir tür olabilir)."),
    (
        "E0425",
        "Kullandığın isim burada tanımlı değil. Yazımını kontrol et (büyük-küçük harfler de önemli) ya da kullanmadan önce `olsun` ile tanımla.",
    ),
    ("E0432", "`kullan` ile almak istediğin şey bulunamadı. Yolunu ve yazımını kontrol et."),
    ("E0433", "Bu isim bulunamadı. Yazımını kontrol et ya da `kullan` ile kullanıma al."),
    (
        "E0499",
        "Bir değeri aynı anda yalnızca tek bir yerden değiştirebilirsin. Bu kural, programının beklenmedik biçimde bozulmasını engeller.",
    ),
    ("E0502", "Bir değeri okurken aynı anda değiştiremezsin. Önce okumayı bitir, sonra değiştir."),
    ("E0596", "Bunu değiştirebilmek için değişkenin `değişken` olarak tanımlanması gerekir: `olsun değişken x = …;`"),
    (
        "E0597",
        "Ödünç aldığın değer, sen onu kullanmayı bitirmeden yok oluyor. Değeri daha dış bir blokta tanımlamayı dene.",
    ),
    ("E0599", "Bu türün böyle bir yöntemi yok. Yöntemin adını ve hangi türe ait olduğunu kontrol et."),
    ("E0601", "Her Pas programının başladığı bir `ana` işlevi olmalı: `işlev ana() { … }`"),
    ("E0609", "Bu yapının böyle bir alanı yok. Alanın adını kontrol et."),
    ("E0614", "Bu değer bir referans değil; başına `*` koyamazsın."),
    ("E0618", "Bu bir işlev değil; arkasına `()` koyarak çağıramazsın."),
];

/// Bir derleyici mesajını Türkçeye çevirir.
pub fn translate_compiler_message(message: &str) -> String {
    translate(COMPILER, message)
}

/// Bir panik mesajı satırını Türkçeye çevirir.
pub fn translate_panic_message(message: &str) -> String {
    translate(PANIC, message)
}

/// Bir hata kodunun çocuklara yönelik açıklaması.
pub fn explanation(code: &str) -> Option<&'static str> {
    EXPLANATIONS.iter().find(|(c, _)| *c == code).map(|(_, text)| *text)
}

/// Kullanıcıya gösterilmesine gerek olmayan mesajlar (özetler, lint notları).
pub fn is_noise(message: &str) -> bool {
    message.starts_with("aborting due to")
        || message.starts_with("For more information about")
        || message.starts_with("Some errors have detailed explanations")
        || (message.ends_with("on by default") && message.contains("#["))
        || message.ends_with("warning emitted")
        || message.ends_with("warnings emitted")
        || message.starts_with("requested on the command line with")
}

fn translate(patterns: &[(&str, &str)], message: &str) -> String {
    for (pattern, template) in patterns {
        if let Some(captures) = match_pattern(pattern, message) {
            return fill(template, &captures);
        }
    }
    code_spans_to_pas(message)
}

fn match_pattern<'m>(pattern: &str, message: &'m str) -> Option<Vec<&'m str>> {
    let mut parts = pattern.split("{}");
    let mut rest = message.strip_prefix(parts.next()?)?;
    let parts: Vec<&str> = parts.collect();
    let mut captures = Vec::with_capacity(parts.len());
    for (i, part) in parts.iter().enumerate() {
        if i + 1 == parts.len() {
            captures.push(rest.strip_suffix(part)?);
            rest = "";
        } else {
            let at = rest.find(part)?;
            captures.push(&rest[..at]);
            rest = &rest[at + part.len()..];
        }
    }
    rest.is_empty().then_some(captures)
}

fn fill(template: &str, captures: &[&str]) -> String {
    let mut out = String::with_capacity(template.len() + 16);
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open].replace("}}", "}"));
        let after = &rest[open + 1..];
        if let Some(stripped) = after.strip_prefix('{') {
            out.push('{');
            rest = stripped;
            continue;
        }
        let close = after.find('}').expect("şablonda kapanmamış `{`");
        let index: usize = after[..close].parse().expect("şablonda geçersiz yakalama numarası");
        out.push_str(&capture_to_pas(captures[index]));
        rest = &after[close + 1..];
    }
    out.push_str(&rest.replace("}}", "}"));
    out
}

fn capture_to_pas(capture: &str) -> String {
    // Dosya yolları (`src/main.rs`) çevrilmez; onları çağıran taraf düzeltir.
    if capture.contains(".rs") {
        return capture.to_owned();
    }
    let mut text = to_pas(capture);
    // Kod parçalarının içine dokunmadan yalnızca düz metni çevir.
    if !text.contains('`') && text.chars().any(char::is_alphabetic) {
        for (english, turkish) in PROSE {
            text = text.replace(english, turkish);
        }
    } else {
        let mut out = String::new();
        for (i, piece) in text.split('`').enumerate() {
            if i > 0 {
                out.push('`');
            }
            if i % 2 == 0 {
                let mut piece = piece.to_owned();
                for (english, turkish) in PROSE {
                    piece = piece.replace(english, turkish);
                }
                out.push_str(&piece);
            } else {
                out.push_str(piece);
            }
        }
        text = out;
    }
    text
}

/// İngilizce bir mesajdaki `kod` parçalarını Pas'a çevirir.
fn code_spans_to_pas(message: &str) -> String {
    message
        .split('`')
        .enumerate()
        .map(|(i, piece)| if i % 2 == 1 { to_pas(piece) } else { piece.to_owned() })
        .collect::<Vec<_>>()
        .join("`")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_are_well_formed() {
        for (pattern, template) in COMPILER.iter().chain(PANIC) {
            let holes = pattern.matches("{}").count();
            let captures: Vec<&str> = (0..holes).map(|_| "x").collect();
            // Şablon geçersizse `fill` panikler.
            fill(template, &captures);
            assert!(!pattern.contains("{}{}"), "yan yana yakalamalar belirsiz: {pattern}");
        }
    }

    #[test]
    fn compiler_messages() {
        assert_eq!(
            translate_compiler_message("cannot find value `sayi` in this scope"),
            "`sayi` adında bir değer bulunamadı"
        );
        assert_eq!(
            translate_compiler_message("expected `String`, found `Option<i32>`"),
            "`Metin` bekleniyordu ama `Belki<i32>` bulundu"
        );
        assert_eq!(
            translate_compiler_message("this function takes 1 argument but 2 arguments were supplied"),
            "bu işlev 1 argüman alıyor ama 2 argüman verildi"
        );
        assert_eq!(
            translate_compiler_message("expected one of `,`, `.`, or `;`, found `x`"),
            "şunlardan biri bekleniyordu: `,`, `.` veya `;`; ama `x` bulundu"
        );
        assert_eq!(
            translate_compiler_message("expected integer, found `&str`"),
            "tam sayı bekleniyordu ama `&str` bulundu"
        );
        assert_eq!(
            translate_compiler_message("consider adding a `main` function to `src/main.rs`"),
            "`src/main.rs` dosyasına bir `işlev ana() { … }` eklemelisin"
        );
    }

    #[test]
    fn unknown_messages_keep_code_in_pas() {
        assert_eq!(
            translate_compiler_message("some new message about `Vec<String>`"),
            "some new message about `Liste<Metin>`"
        );
    }

    #[test]
    fn panic_messages() {
        assert_eq!(
            translate_panic_message("called `Option::unwrap()` on a `None` value"),
            "`Yok` değeri üzerinde `Belki::sarmalaç()` çağrıldı (içinde değer yoktu)"
        );
        assert_eq!(
            translate_panic_message("index out of bounds: the len is 3 but the index is 10"),
            "dizin sınırların dışında: uzunluk 3 ama istenen dizin 10"
        );
        assert_eq!(translate_panic_message("Kendi mesajım"), "Kendi mesajım");
    }

    #[test]
    fn noise() {
        assert!(is_noise("aborting due to 2 previous errors"));
        assert!(is_noise("`#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default"));
        assert!(!is_noise("mismatched types"));
    }
}
