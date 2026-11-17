//! Pas çekirdeği: Türkçe anahtar sözcüklerle yazılmış Rust (Pas) kodunu
//! Rust'a çevirir, iki kaynak arasındaki konumları eşler ve derleyici
//! mesajlarını Türkçeleştirir.
//!
//! ```
//! let ceviri = pas_cekirdek::to_rust("işlev ana() { yazdır!(\"Merhaba!\"); }");
//! assert_eq!(ceviri.rust, "fn main() { println!(\"Merhaba!\"); }");
//! ```

pub mod dictionary;
pub mod lexer;
pub mod messages;
pub mod source_map;
pub mod translate;

pub use source_map::{Bias, LineIndex, SourceMap};
pub use translate::{Options, Translation, to_pas, to_rust, to_rust_with};

/// Pas kaynak dosyalarının uzantısı.
pub const EXTENSION: &str = "pas";
