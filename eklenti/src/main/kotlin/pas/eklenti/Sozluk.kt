package pas.eklenti

import java.text.Normalizer

enum class Tur { KEYWORD, MACRO, TYPE, VALUE, FUNCTION, METHOD }

/** Sözlükteki bir Türkçe sözcük ve Rust karşılığı. */
data class SozlukGirdisi(val pas: String, val rust: String, val tur: Tur, val aciklama: String)

/**
 * Pas sözlüğü. Veriler derleme sırasında `cargo pas sözlük --json`
 * çıktısından üretilir (bkz. `SozlukVerisi.kt`); tek doğruluk kaynağı
 * `crates/pas-cekirdek/src/dictionary.rs` dosyasıdır.
 */
object Sozluk {
    val girdiler: List<SozlukGirdisi> = SOZLUK_VERISI

    private val pasIle = girdiler.associateBy { it.pas }

    /** Pas dosyalarında da geçerli olan İngilizce Rust anahtar sözcükleri. */
    private val rustAnahtarSozcukleri = setOf(
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
        "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "macro_rules", "union",
    )

    fun bul(sozcuk: String): SozlukGirdisi? = pasIle[nfc(sozcuk)]

    fun anahtarSozcukMu(sozcuk: String): Boolean =
        sozcuk in rustAnahtarSozcukleri || bul(sozcuk)?.tur == Tur.KEYWORD

    private fun nfc(s: String) =
        if (Normalizer.isNormalized(s, Normalizer.Form.NFC)) s else Normalizer.normalize(s, Normalizer.Form.NFC)
}
