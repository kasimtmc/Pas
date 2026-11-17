//! Pas ⇄ Rust çevirisi.
//!
//! Çeviri sözcük düzeyindedir: sözlükteki isimler karşılıklarıyla
//! değiştirilir, geri kalan her şey (boşluklar, yorumlar, dizgiler)
//! olduğu gibi kalır. Satır sayısı hiç değişmez. Bunun üç istisnası var:
//!
//! * Biçim dizgilerindeki isimler (`yazdır!("{uzunluk}")`) de çevrilir.
//! * `mod çalış;` gibi ASCII olmayan modül bildirimlerine rustc'nin
//!   istediği `#[path = "…"]` niteliği eklenir.
//! * `yaz!(…)` çıktıyı hemen ekrana basar (`print!` + `flush`); yoksa
//!   girdi okumadan önce yazılan soru ekranda görünmez.

use std::collections::HashMap;

use unicode_normalization::{UnicodeNormalization, is_nfc};

use crate::dictionary::{self, Kind};
use crate::lexer::{Token, TokenKind, is_identifier, tokenize};
use crate::source_map::{Segment, SourceMap};

#[derive(Debug, Clone)]
pub struct Translation {
    pub rust: String,
    pub map: SourceMap,
}

#[derive(Default)]
pub struct Options<'a> {
    /// ASCII olmayan ya da sözlükte bulunan bir modül adı için (`mod çalış;`)
    /// `#[path]` niteliğine yazılacak yolu üretir. Verilmezse `ad.rs` kullanılır.
    pub module_path: Option<&'a dyn Fn(&str) -> String>,
}

/// Pas kaynağını Rust'a çevirir.
pub fn to_rust(src: &str) -> Translation {
    to_rust_with(src, &Options::default())
}

pub fn to_rust_with(src: &str, options: &Options) -> Translation {
    let (rust, map) = Translator::new(src, Direction::ToRust).run(options);
    Translation { rust, map }
}

/// Rust kaynağını Pas'a çevirir. Hata mesajlarındaki kod parçaları ve
/// `cargo pas çevir --ters` için kullanılır.
pub fn to_pas(src: &str) -> String {
    Translator::new(src, Direction::ToPas).run(&Options::default()).0
}

/// Biçim dizgisi alan makrolar ve biçim dizgisinden önce gelen virgül sayısı.
const FORMAT_MACROS: &[(&str, usize)] = &[
    ("println", 0),
    ("print", 0),
    ("eprintln", 0),
    ("eprint", 0),
    ("format", 0),
    ("format_args", 0),
    ("panic", 0),
    ("todo", 0),
    ("unimplemented", 0),
    ("unreachable", 0),
    ("write", 1),
    ("writeln", 1),
    ("assert", 1),
    ("debug_assert", 1),
    ("assert_eq", 2),
    ("assert_ne", 2),
    ("debug_assert_eq", 2),
    ("debug_assert_ne", 2),
];

const FLUSH_STDOUT: &str = ";::std::io::Write::flush(&mut ::std::io::stdout()).ok();}";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    ToRust,
    ToPas,
}

struct Translator<'a> {
    src: &'a str,
    direction: Direction,
    tokens: Vec<Token>,
    /// Anlamlı (boşluk ve yorum olmayan) sözcüklerin `tokens` içindeki sıraları.
    sig: Vec<usize>,
    out: String,
    map: SourceMap,
    /// `src` içinde çıktıya kopyalanmış son konum.
    copied: usize,
}

impl<'a> Translator<'a> {
    fn new(src: &'a str, direction: Direction) -> Self {
        let tokens = tokenize(src);
        let sig = (0..tokens.len()).filter(|&i| !tokens[i].is_trivia()).collect();
        Translator {
            src,
            direction,
            tokens,
            sig,
            out: String::with_capacity(src.len() + src.len() / 8),
            map: SourceMap::default(),
            copied: 0,
        }
    }

    fn run(mut self, options: &Options) -> (String, SourceMap) {
        let mut before: HashMap<usize, String> = HashMap::new();
        let mut after: HashMap<usize, String> = HashMap::new();
        if self.direction == Direction::ToRust {
            self.plan_module_paths(options, &mut before);
            self.plan_flushing_print(&mut before, &mut after);
        }

        let mut format = FormatTracker::default();
        for k in 0..self.sig.len() {
            let ti = self.sig[k];
            let token = self.tokens[ti];
            if let Some(text) = before.get(&ti) {
                self.replace(token.start..token.start, text);
            }
            match token.kind {
                TokenKind::Ident => {
                    if let Some(word) = self.translate_word(token.text(self.src)) {
                        self.replace(token.start..token.end, &word);
                    }
                }
                TokenKind::Lifetime => self.translate_lifetime(token),
                TokenKind::Str | TokenKind::RawStr if format.wants_string() => {
                    self.translate_format_string(token);
                    format.string_seen();
                }
                TokenKind::Punct => format.observe_punct(&self, k),
                _ => {}
            }
            if let Some(text) = after.get(&ti) {
                self.replace(token.end..token.end, text);
            }
        }
        self.copy_to(self.src.len());
        (self.out, self.map)
    }

    fn copy_to(&mut self, pos: usize) {
        self.out.push_str(&self.src[self.copied..pos]);
        self.copied = pos;
    }

    fn replace(&mut self, range: std::ops::Range<usize>, text: &str) {
        self.copy_to(range.start);
        self.map.push(Segment {
            pas_start: range.start,
            pas_len: range.len(),
            rust_start: self.out.len(),
            rust_len: text.len(),
        });
        self.out.push_str(text);
        self.copied = range.end;
    }

    fn sig_token(&self, k: usize) -> Option<Token> {
        self.sig.get(k).map(|&i| self.tokens[i])
    }

    fn sig_text(&self, k: usize) -> &'a str {
        self.sig_token(k).map_or("", |t| t.text(self.src))
    }

    /// Bir ismin çeviri yönündeki karşılığı; değişmeyecekse `None`.
    fn translate_word(&self, word: &str) -> Option<String> {
        match self.direction {
            Direction::ToRust => dictionary::by_pas(&nfc(word)).map(|e| e.rust.to_owned()),
            Direction::ToPas => match dictionary::by_rust(word) {
                Some(entry) => Some(entry.pas.to_owned()),
                // Rust'ta sıradan bir isim olan `için`, Pas'ta ayrılmış bir sözcük.
                None => dictionary::by_pas(&nfc(word)).filter(|e| e.kind == Kind::Keyword).map(|_| format!("r#{word}")),
            },
        }
    }

    /// Bir ismin Rust karşılığı (çevrilmeyecekse kendisi).
    fn rust_name(&self, word: &'a str) -> std::borrow::Cow<'a, str> {
        match self.direction {
            Direction::ToRust => match dictionary::by_pas(&nfc(word)) {
                Some(entry) => entry.rust.into(),
                None => word.into(),
            },
            Direction::ToPas => word.into(),
        }
    }

    fn translate_lifetime(&mut self, token: Token) {
        let name = &token.text(self.src)[1..];
        let entry = match self.direction {
            Direction::ToRust => dictionary::by_pas(&nfc(name)).map(|e| (e, e.rust)),
            Direction::ToPas => dictionary::by_rust(name).map(|e| (e, e.pas)),
        };
        // Anahtar sözcüklerden yalnızca `'static` geçerli bir yaşam süresidir.
        if let Some((entry, word)) = entry
            && (entry.kind != Kind::Keyword || entry.rust == "static")
        {
            self.replace(token.start..token.end, &format!("'{word}"));
        }
    }

    /// `"{uzunluk} {x:?}"` gibi biçim dizgilerindeki isimleri çevirir.
    fn translate_format_string(&mut self, token: Token) {
        let text = token.text(self.src);
        let (Some(open), Some(close)) = (text.find('"'), text.rfind('"')) else { return };
        if close <= open {
            return;
        }
        let content_start = token.start + open + 1;
        let content = &text[open + 1..close];
        let mut i = 0;
        while i < content.len() {
            let rest = &content[i..];
            if rest.starts_with("{{") || rest.starts_with("}}") {
                i += 2;
            } else if let Some(inner) = rest.strip_prefix('{') {
                let arg_len = inner.find([':', '}']).unwrap_or(inner.len());
                let arg = &inner[..arg_len];
                if is_identifier(arg)
                    && let Some(word) = self.translate_word(arg)
                {
                    let start = content_start + i + 1;
                    self.replace(start..start + arg.len(), &word);
                }
                i += 1 + arg_len;
            } else {
                i += rest.chars().next().map_or(1, char::len_utf8);
            }
        }
    }

    /// `mod çalış;` → `#[path = "çalış.rs"] mod çalış;`
    fn plan_module_paths(&self, options: &Options, before: &mut HashMap<usize, String>) {
        for k in 0..self.sig.len() {
            let Some(token) = self.sig_token(k) else { continue };
            if token.kind != TokenKind::Ident || self.rust_name(token.text(self.src)) != "mod" {
                continue;
            }
            let Some(name) = self.sig_token(k + 1).filter(|t| t.kind == TokenKind::Ident) else {
                continue;
            };
            if self.sig_text(k + 2) != ";" {
                continue;
            }
            let name = nfc(name.text(self.src));
            if name.is_ascii() && dictionary::by_pas(&name).is_none() {
                continue;
            }

            // Nitelik, görünürlük belirtecinden (`genel`, `genel(sandık)`) önce gelmeli.
            let mut j = k;
            if j > 0
                && self.sig_text(j - 1) == ")"
                && let Some(open) = self.matching_open(j - 1)
                && open > 0
                && self.rust_name(self.sig_text(open - 1)) == "pub"
            {
                j = open - 1;
            } else if j > 0 && self.rust_name(self.sig_text(j - 1)) == "pub" {
                j -= 1;
            }
            // Zaten bir `#[path]` niteliği varsa dokunma.
            let mut has_path = false;
            let mut a = j;
            while a > 0 && self.sig_text(a - 1) == "]" {
                let Some(open) = self.matching_open(a - 1) else { break };
                if open == 0 || self.sig_text(open - 1) != "#" {
                    break;
                }
                has_path |= self.sig_text(open + 1) == "path";
                a = open - 1;
            }
            if has_path {
                continue;
            }

            let path = match options.module_path {
                Some(module_path) => module_path(&name),
                None => format!("{name}.rs"),
            };
            before.insert(self.sig[j], format!("#[path = {path:?}] "));
        }
    }

    /// `yaz!(…)` → `{print!(…);::std::io::Write::flush(…).ok();}`
    fn plan_flushing_print(&self, before: &mut HashMap<usize, String>, after: &mut HashMap<usize, String>) {
        for k in 0..self.sig.len() {
            let Some(token) = self.sig_token(k) else { continue };
            let word = token.text(self.src);
            if token.kind != TokenKind::Ident || word == "print" || self.rust_name(word) != "print" {
                continue;
            }
            if self.sig_text(k + 1) != "!" || !matches!(self.sig_text(k + 2), "(" | "[" | "{") {
                continue;
            }
            if let Some(close) = self.matching_close(k + 2) {
                before.insert(self.sig[k], "{".to_owned());
                after.insert(self.sig[close], FLUSH_STDOUT.to_owned());
            }
        }
    }

    fn matching_close(&self, open: usize) -> Option<usize> {
        let mut depth = 0usize;
        for k in open..self.sig.len() {
            match self.sig_text(k) {
                "(" | "[" | "{" => depth += 1,
                ")" | "]" | "}" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(k);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn matching_open(&self, close: usize) -> Option<usize> {
        let mut depth = 0usize;
        for k in (0..=close).rev() {
            match self.sig_text(k) {
                ")" | "]" | "}" => depth += 1,
                "(" | "[" | "{" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(k);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

/// Hangi dizginin bir biçim dizgisi olduğunu izler.
#[derive(Default)]
struct FormatTracker {
    depth: usize,
    stack: Vec<FormatCall>,
}

struct FormatCall {
    /// Makro çağrısının parantezlerinin içindeki derinlik.
    depth: usize,
    /// Biçim dizgisinden önce görülmesi gereken virgül sayısı.
    commas_left: usize,
    done: bool,
}

impl FormatTracker {
    fn top(&mut self) -> Option<&mut FormatCall> {
        let depth = self.depth;
        self.stack.last_mut().filter(|call| call.depth == depth)
    }

    fn wants_string(&mut self) -> bool {
        self.top().is_some_and(|call| call.commas_left == 0 && !call.done)
    }

    fn string_seen(&mut self) {
        if let Some(call) = self.top() {
            call.done = true;
        }
    }

    fn observe_punct(&mut self, t: &Translator, k: usize) {
        match t.sig_text(k) {
            "(" | "[" | "{" => {
                self.depth += 1;
                if k >= 2
                    && t.sig_text(k - 1) == "!"
                    && t.sig_token(k - 2).is_some_and(|tok| tok.kind == TokenKind::Ident)
                {
                    let name = t.rust_name(t.sig_text(k - 2));
                    if let Some(&(_, commas)) = FORMAT_MACROS.iter().find(|(m, _)| *m == name) {
                        self.stack.push(FormatCall { depth: self.depth, commas_left: commas, done: false });
                    }
                }
            }
            ")" | "]" | "}" => {
                if self.top().is_some() {
                    self.stack.pop();
                }
                self.depth = self.depth.saturating_sub(1);
            }
            "," => {
                if let Some(call) = self.top() {
                    call.commas_left = call.commas_left.saturating_sub(1);
                }
            }
            _ => {}
        }
    }
}

fn nfc(word: &str) -> std::borrow::Cow<'_, str> {
    if is_nfc(word) { word.into() } else { word.nfc().collect::<String>().into() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_map::Bias;

    fn rust(src: &str) -> String {
        to_rust(src).rust
    }

    #[test]
    fn hello_world() {
        assert_eq!(
            rust("işlev ana() {\n    yazdır!(\"Merhaba dünya!\");\n}\n"),
            "fn main() {\n    println!(\"Merhaba dünya!\");\n}\n"
        );
    }

    #[test]
    fn keywords_and_std_names() {
        assert_eq!(
            rust("olsun değişken l: Liste<Metin> = liste![]; eğer l.boş_mu() { döndür Yok; } değilse { kır; }"),
            "let mut l: Vec<String> = vec![]; if l.is_empty() { return None; } else { break; }"
        );
        assert_eq!(rust("için i içinde 0..10 {}"), "for i in 0..10 {}");
    }

    #[test]
    fn english_rust_still_works() {
        let src = "fn main() { let mut x = 5; if x > 2 { println!(\"{x}\"); } }";
        assert_eq!(rust(src), src);
    }

    #[test]
    fn strings_and_comments_are_untouched() {
        assert_eq!(
            rust("olsun s = \"olsun eğer\"; // eğer olsun\n/* işlev */ 'ı'"),
            "let s = \"olsun eğer\"; // eğer olsun\n/* işlev */ 'ı'"
        );
    }

    #[test]
    fn raw_identifiers_escape_keywords() {
        assert_eq!(rust("olsun r#tür = 1;"), "let r#tür = 1;");
    }

    #[test]
    fn format_string_arguments() {
        assert_eq!(
            rust("yazdır!(\"{uzunluk} {{uzunluk}} {x:?} {ana:>5}\", x = 1);"),
            "println!(\"{len} {{uzunluk}} {x:?} {main:>5}\", x = 1);"
        );
        // Biçim dizgisi olmayan dizgilere dokunulmaz.
        assert_eq!(
            rust("yazdır!(\"{}\", \"{uzunluk}\"); olsun s = \"{uzunluk}\";"),
            "println!(\"{}\", \"{uzunluk}\"); let s = \"{uzunluk}\";"
        );
        assert_eq!(rust("doğrula_eşit!(a, \"{ana}\", \"{ana}\");"), "assert_eq!(a, \"{ana}\", \"{main}\");");
    }

    #[test]
    fn lifetimes() {
        assert_eq!(rust("&'durağan str"), "&'static str");
        assert_eq!(rust("<'a>"), "<'a>");
    }

    #[test]
    fn print_flushes() {
        assert_eq!(rust("yaz!(\"Adın: \");"), format!("{{print!(\"Adın: \"){FLUSH_STDOUT};"));
        assert_eq!(rust("print!(\"a\");"), "print!(\"a\");");
        assert_eq!(rust("makro_kuralları! yaz { () => {} }"), "macro_rules! print { () => {} }");
    }

    #[test]
    fn non_ascii_modules_get_a_path() {
        assert_eq!(rust("mod çalış;"), "#[path = \"çalış.rs\"] mod çalış;");
        assert_eq!(rust("genel modül çalış;"), "#[path = \"çalış.rs\"] pub mod çalış;");
        assert_eq!(rust("genel(sandık) modül çalış;"), "#[path = \"çalış.rs\"] pub(crate) mod çalış;");
        assert_eq!(rust("#[path = \"x.rs\"] mod çalış;"), "#[path = \"x.rs\"] mod çalış;");
        assert_eq!(rust("modül oyun;"), "mod oyun;");
        assert_eq!(rust("modül çalış { }"), "mod çalış { }");
        // Sözlükteki bir isim çevrilir ama dosyası yine Türkçe adıyla aranır.
        assert_eq!(rust("modül liste;"), "#[path = \"liste.rs\"] mod vec;");

        let module_path = |name: &str| format!("oyun/{name}.rs");
        let options = Options { module_path: Some(&module_path) };
        assert_eq!(to_rust_with("mod şekil;", &options).rust, "#[path = \"oyun/şekil.rs\"] mod şekil;");
    }

    #[test]
    fn decomposed_input_is_normalized() {
        // "değilse" with "g" + U+0306 (birleşik breve)
        assert_eq!(rust("de\u{67}\u{306}ilse"), "else");
    }

    #[test]
    fn reverse_translation() {
        assert_eq!(
            to_pas("fn main() { let mut v: Vec<String> = vec![]; println!(\"{}\", v.len()); }"),
            "işlev ana() { olsun değişken v: Liste<Metin> = liste![]; yazdır!(\"{}\", v.uzunluk()); }"
        );
        assert_eq!(to_pas("let için = 1;"), "olsun r#için = 1;");
        assert_eq!(to_pas("`Option<i32>`"), "`Belki<i32>`");
        assert_eq!(to_pas("&'static str"), "&'durağan str");
    }

    #[test]
    fn round_trip() {
        let src = "işlev ana() {\n    olsun değişken sayılar = liste![1, 2];\n    sayılar.ekle(3);\n    yazdır!(\"{}\", sayılar.uzunluk());\n}\n";
        assert_eq!(to_pas(&rust(src)), src);
    }

    #[test]
    fn source_map_points_back_to_pas() {
        let src = "işlev ana() { olsun x = yok_böyle; }";
        let t = to_rust(src);
        let rust_start = t.rust.find("yok_böyle").unwrap();
        let rust_end = rust_start + "yok_böyle".len();
        let (start, end) = t.map.rust_range_to_pas(rust_start, rust_end);
        assert_eq!(&src[start..end], "yok_böyle");

        let fn_end = t.rust.find("fn").unwrap() + 2;
        assert_eq!(t.map.rust_range_to_pas(0, fn_end), (0, "işlev".len()));
        assert_eq!(t.map.pas_to_rust(src.find("x").unwrap(), Bias::Start), t.rust.find("x").unwrap());
    }

    #[test]
    fn line_numbers_are_preserved() {
        let src = "işlev ana() {\n  yaz!(\"a\");\n  modül çalış;\n}\n";
        assert_eq!(rust(src).lines().count(), src.lines().count());
    }
}
