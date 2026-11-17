//! Terminal renkleri ve cargo tarzı durum satırları.

use std::io::IsTerminal;

#[derive(Debug, Clone, Copy)]
pub struct Style {
    color: bool,
}

impl Style {
    /// `NO_COLOR` tanımlı değilse ve hata çıktısı bir terminalse renk kullanır.
    pub fn detect() -> Self {
        let color = match std::env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => true,
            Ok("never") => false,
            _ => std::env::var_os("NO_COLOR").is_none() && std::io::stderr().is_terminal(),
        };
        Style { color }
    }

    /// Renksiz çıktı (testler için).
    #[cfg(test)]
    pub fn plain() -> Self {
        Style { color: false }
    }

    fn paint(&self, code: &str, text: &str) -> String {
        if self.color { format!("\x1b[{code}m{text}\x1b[0m") } else { text.to_owned() }
    }

    pub fn bold(&self, text: &str) -> String {
        self.paint("1", text)
    }

    pub fn error(&self, text: &str) -> String {
        self.paint("1;31", text)
    }

    pub fn warning(&self, text: &str) -> String {
        self.paint("1;33", text)
    }

    pub fn gutter(&self, text: &str) -> String {
        self.paint("1;34", text)
    }

    pub fn success(&self, text: &str) -> String {
        self.paint("1;32", text)
    }

    pub fn note(&self, text: &str) -> String {
        self.paint("1;36", text)
    }

    /// cargo'nun `   Compiling x` satırlarına benzeyen bir durum satırı.
    pub fn status(&self, word: &str, message: &str) {
        eprintln!("{} {message}", self.success(&format!("{word:>15}")));
    }
}
