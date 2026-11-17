//! Pas çekirdeği: Türkçe anahtar sözcüklerle yazılmış Rust (Pas) kodunu
//! Rust'a çevirir, iki kaynak arasındaki konumları eşler ve derleyici
//! mesajlarını Türkçeleştirir.
//!
//! ```
//! let ceviri = pas_cekirdek::rusta_cevir("işlev ana() { yazdır!(\"Merhaba!\"); }");
//! assert_eq!(ceviri.rust, "fn main() { println!(\"Merhaba!\"); }");
//! ```

pub mod ceviri;
pub mod kaynak_haritasi;
pub mod mesajlar;
pub mod sozcukler;
pub mod sozluk;

pub use ceviri::{Ceviri, Secenekler, pasa_cevir, rusta_cevir, rusta_cevir_secenekle};
pub use kaynak_haritasi::{KaynakHaritasi, SatirDizini, Yon};

/// Pas kaynak dosyalarının uzantısı.
pub const UZANTI: &str = "pas";
