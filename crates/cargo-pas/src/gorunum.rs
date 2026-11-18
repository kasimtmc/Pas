//! Terminal renkleri ve cargo tarzı durum satırları.

use std::io::IsTerminal;

#[derive(Debug, Clone, Copy)]
pub struct Gorunum {
    renkli: bool,
}

impl Gorunum {
    /// Rengi ortama göre açar: `CARGO_TERM_COLOR`, yoksa hata çıktısı bir terminal mi.
    /// `NO_COLOR` her zaman önceliklidir (https://no-color.org).
    pub fn algila() -> Self {
        if std::env::var_os("NO_COLOR").is_some_and(|d| !d.is_empty()) {
            return Gorunum { renkli: false };
        }
        let renkli = match std::env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => true,
            Ok("never") => false,
            _ => std::io::stderr().is_terminal(),
        };
        Gorunum { renkli }
    }

    /// Renksiz çıktı (testler için).
    #[cfg(test)]
    pub fn duz() -> Self {
        Gorunum { renkli: false }
    }

    fn boya(&self, kod: &str, metin: &str) -> String {
        if self.renkli { format!("\x1b[{kod}m{metin}\x1b[0m") } else { metin.to_owned() }
    }

    pub fn kalin(&self, metin: &str) -> String {
        self.boya("1", metin)
    }

    pub fn hata(&self, metin: &str) -> String {
        self.boya("1;31", metin)
    }

    pub fn uyari(&self, metin: &str) -> String {
        self.boya("1;33", metin)
    }

    pub fn kenar(&self, metin: &str) -> String {
        self.boya("1;34", metin)
    }

    pub fn basari(&self, metin: &str) -> String {
        self.boya("1;32", metin)
    }

    pub fn not(&self, metin: &str) -> String {
        self.boya("1;36", metin)
    }

    /// cargo'nun `   Compiling x` satırlarına benzeyen bir durum satırı.
    pub fn durum(&self, kelime: &str, mesaj: &str) {
        eprintln!("{} {mesaj}", self.basari(&format!("{kelime:>15}")));
    }
}
