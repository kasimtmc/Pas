//! Kaynak kodu sözcüklere (token) ayıran basit bir sözcük çözümleyici.
//!
//! Rust sözdizimini tam olarak ayrıştırmaz; isimleri dizgilerden,
//! yorumlardan ve karakter sabitlerinden ayırt edecek kadarını bilir.
//! Kaynağın her baytı tam olarak bir sözcüğe aittir; böylece çevirici
//! değiştirmediği her şeyi olduğu gibi bırakabilir.

use unicode_ident::{is_xid_continue, is_xid_start};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Whitespace,
    LineComment,
    BlockComment,
    Ident,
    /// `r#isim`
    RawIdent,
    /// `'a`, `'static`
    Lifetime,
    /// `"…"`, `b"…"`, `c"…"`
    Str,
    /// `r"…"`, `r#"…"#`, `br"…"`, `cr"…"`
    RawStr,
    /// `'a'`, `b'a'`
    Char,
    Number,
    /// Tek karakterlik noktalama; `::` gibi birleşik işaretler ayrı sözcüklerdir.
    Punct,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }

    /// Boşluk ve yorumlar anlam taşımaz.
    pub fn is_trivia(&self) -> bool {
        matches!(self.kind, TokenKind::Whitespace | TokenKind::LineComment | TokenKind::BlockComment)
    }
}

pub fn is_ident_start(c: char) -> bool {
    c == '_' || is_xid_start(c)
}

pub fn is_ident_continue(c: char) -> bool {
    is_xid_continue(c)
}

pub fn is_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    chars.next().is_some_and(is_ident_start) && chars.all(is_ident_continue)
}

pub fn tokenize(src: &str) -> Vec<Token> {
    let mut lexer = Lexer { src, pos: 0 };
    let mut tokens = Vec::new();
    while lexer.pos < src.len() {
        let start = lexer.pos;
        let kind = lexer.next_kind();
        tokens.push(Token { kind, start, end: lexer.pos });
    }
    tokens
}

struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl Lexer<'_> {
    fn rest(&self) -> &str {
        &self.src[self.pos..]
    }

    fn peek(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        self.rest().chars().nth(n)
    }

    fn bump(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn eat_while(&mut self, f: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&f) {
            self.bump();
        }
    }

    fn next_kind(&mut self) -> TokenKind {
        let rest = self.rest();
        let c = self.peek().expect("kaynağın sonuna gelinmedi");
        if c.is_whitespace() {
            self.eat_while(char::is_whitespace);
            return TokenKind::Whitespace;
        }
        if rest.starts_with("//") {
            self.eat_while(|c| c != '\n');
            return TokenKind::LineComment;
        }
        if rest.starts_with("/*") {
            self.block_comment();
            return TokenKind::BlockComment;
        }
        if let Some(kind) = self.prefixed() {
            return kind;
        }
        if is_ident_start(c) {
            self.eat_while(is_ident_continue);
            return TokenKind::Ident;
        }
        match c {
            '"' => {
                self.bump();
                self.quoted('"');
                TokenKind::Str
            }
            '\'' => self.char_or_lifetime(),
            '0'..='9' => {
                self.number();
                TokenKind::Number
            }
            _ => {
                self.bump();
                TokenKind::Punct
            }
        }
    }

    /// Önekli sabitler (`r"…"`, `b'…'`, …) ve `r#isim`.
    fn prefixed(&mut self) -> Option<TokenKind> {
        let rest = self.rest();
        for prefix in ["br", "cr", "r"] {
            let Some(after) = rest.strip_prefix(prefix) else { continue };
            let hashes = after.len() - after.trim_start_matches('#').len();
            if after[hashes..].starts_with('"') {
                self.pos += prefix.len() + hashes + 1;
                self.raw_string_body(hashes);
                return Some(TokenKind::RawStr);
            }
            if prefix == "r" && hashes == 1 && after[1..].starts_with(is_ident_start) {
                self.pos += 2;
                self.eat_while(is_ident_continue);
                return Some(TokenKind::RawIdent);
            }
        }
        for prefix in ["b", "c"] {
            let Some(after) = rest.strip_prefix(prefix) else { continue };
            if after.starts_with('"') {
                self.pos += 2;
                self.quoted('"');
                return Some(TokenKind::Str);
            }
            if prefix == "b" && after.starts_with('\'') {
                self.pos += 2;
                self.quoted('\'');
                return Some(TokenKind::Char);
            }
        }
        None
    }

    /// Açılış tırnağı tüketilmiş olmalı; kapanış tırnağını da tüketir.
    fn quoted(&mut self, quote: char) {
        while let Some(c) = self.bump() {
            if c == '\\' {
                self.bump();
            } else if c == quote {
                return;
            }
        }
    }

    fn raw_string_body(&mut self, hashes: usize) {
        let closing = format!("\"{}", "#".repeat(hashes));
        match self.rest().find(&closing) {
            Some(i) => self.pos += i + closing.len(),
            None => self.pos = self.src.len(),
        }
    }

    fn block_comment(&mut self) {
        self.pos += 2;
        let mut depth = 1;
        while depth > 0 && self.pos < self.src.len() {
            if self.rest().starts_with("/*") {
                depth += 1;
                self.pos += 2;
            } else if self.rest().starts_with("*/") {
                depth -= 1;
                self.pos += 2;
            } else {
                self.bump();
            }
        }
    }

    /// `'a'` bir karakter, `'a` (ardından `'` gelmeyen) bir yaşam süresidir.
    fn char_or_lifetime(&mut self) -> TokenKind {
        match (self.peek_nth(1), self.peek_nth(2)) {
            (Some('\\'), _) => {
                self.bump();
                self.quoted('\'');
                TokenKind::Char
            }
            (Some(c), Some('\'')) => {
                self.pos += 1 + c.len_utf8() + 1;
                TokenKind::Char
            }
            (Some(c), _) if is_ident_start(c) => {
                self.bump();
                self.eat_while(is_ident_continue);
                TokenKind::Lifetime
            }
            _ => {
                self.bump();
                TokenKind::Punct
            }
        }
    }

    fn number(&mut self) {
        let hex = self.rest().starts_with("0x") || self.rest().starts_with("0X");
        let mut prev = self.bump().unwrap_or('0');
        while let Some(c) = self.peek() {
            let accept = c.is_ascii_alphanumeric()
                || c == '_'
                || (c == '.' && self.peek_nth(1).is_some_and(|n| n.is_ascii_digit()))
                || (matches!(c, '+' | '-') && !hex && matches!(prev, 'e' | 'E'));
            if !accept {
                break;
            }
            prev = c;
            self.bump();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenKind::*;

    fn kinds(src: &str) -> Vec<(TokenKind, &str)> {
        tokenize(src).into_iter().filter(|t| t.kind != Whitespace).map(|t| (t.kind, t.text(src))).collect()
    }

    #[test]
    fn covers_every_byte() {
        let src = "işlev ana() { yap ş = r#\"a\"#; } // yorum\n/* iç /* içe */ */ 'a' 'b";
        let tokens = tokenize(src);
        let mut pos = 0;
        for t in &tokens {
            assert_eq!(t.start, pos);
            pos = t.end;
        }
        assert_eq!(pos, src.len());
    }

    #[test]
    fn turkish_identifiers() {
        assert_eq!(kinds("yap sayılar"), [(Ident, "yap"), (Ident, "sayılar")]);
        assert_eq!(kinds("ğüşıöç_İ"), [(Ident, "ğüşıöç_İ")]);
    }

    #[test]
    fn strings_hide_words() {
        assert_eq!(kinds(r#""yap \" eğer""#), [(Str, r#""yap \" eğer""#)]);
        assert_eq!(kinds(r###"r##"a "# b"##"###), [(RawStr, r###"r##"a "# b"##"###)]);
        assert_eq!(kinds(r#"b"x" br"y" c"z""#), [(Str, r#"b"x""#), (RawStr, r#"br"y""#), (Str, r#"c"z""#)]);
    }

    #[test]
    fn raw_identifiers() {
        assert_eq!(kinds("r#tür"), [(RawIdent, "r#tür")]);
        assert_eq!(kinds("break"), [(Ident, "break")]);
    }

    #[test]
    fn chars_and_lifetimes() {
        assert_eq!(kinds("'ş'"), [(Char, "'ş'")]);
        assert_eq!(kinds(r"'\''"), [(Char, r"'\''")]);
        assert_eq!(kinds("&'durağan"), [(Punct, "&"), (Lifetime, "'durağan")]);
        assert_eq!(kinds("b'a'"), [(Char, "b'a'")]);
    }

    #[test]
    fn comments() {
        assert_eq!(kinds("// yap\nx"), [(LineComment, "// yap"), (Ident, "x")]);
        assert_eq!(kinds("/* a /* b */ c */x"), [(BlockComment, "/* a /* b */ c */"), (Ident, "x")]);
    }

    #[test]
    fn numbers() {
        assert_eq!(kinds("1.5e-3"), [(Number, "1.5e-3")]);
        assert_eq!(kinds("0..10"), [(Number, "0"), (Punct, "."), (Punct, "."), (Number, "10")]);
        assert_eq!(kinds("0xe-1"), [(Number, "0xe"), (Punct, "-"), (Number, "1")]);
        assert_eq!(kinds("2.uzunluk"), [(Number, "2"), (Punct, "."), (Ident, "uzunluk")]);
    }
}
