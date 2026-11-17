package pas.eklenti

import com.intellij.lexer.LexerBase
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import com.intellij.psi.tree.TokenSet

class PasSozcukTuru(ad: String) : IElementType(ad, PasDili)

object PasSozcukler {
    @JvmField val ANAHTAR_SOZCUK = PasSozcukTuru("ANAHTAR_SOZCUK")
    @JvmField val ISIM = PasSozcukTuru("ISIM")
    /** `yazdır!` gibi bir makro adı (ünlem dahil). */
    @JvmField val MAKRO = PasSozcukTuru("MAKRO")
    /** Sözlükteki türler: `Metin`, `Liste`, … */
    @JvmField val HAZIR_TUR = PasSozcukTuru("HAZIR_TUR")
    /** Sözlükteki değerler: `Var`, `Yok`, `Tamam`, `Hata` */
    @JvmField val HAZIR_DEGER = PasSozcukTuru("HAZIR_DEGER")
    @JvmField val YASAM_SURESI = PasSozcukTuru("YASAM_SURESI")
    @JvmField val DIZGI = PasSozcukTuru("DIZGI")
    @JvmField val KARAKTER = PasSozcukTuru("KARAKTER")
    @JvmField val SAYI = PasSozcukTuru("SAYI")
    @JvmField val SATIR_YORUMU = PasSozcukTuru("SATIR_YORUMU")
    @JvmField val BELGE_YORUMU = PasSozcukTuru("BELGE_YORUMU")
    @JvmField val BLOK_YORUMU = PasSozcukTuru("BLOK_YORUMU")
    @JvmField val SOL_PARANTEZ = PasSozcukTuru("(")
    @JvmField val SAG_PARANTEZ = PasSozcukTuru(")")
    @JvmField val SOL_KOSELI = PasSozcukTuru("[")
    @JvmField val SAG_KOSELI = PasSozcukTuru("]")
    @JvmField val SOL_SUSLU = PasSozcukTuru("{")
    @JvmField val SAG_SUSLU = PasSozcukTuru("}")
    @JvmField val NOKTALI_VIRGUL = PasSozcukTuru(";")
    @JvmField val VIRGUL = PasSozcukTuru(",")
    @JvmField val NOKTA = PasSozcukTuru(".")
    @JvmField val ISLEC = PasSozcukTuru("ISLEC")

    @JvmField val YORUMLAR = TokenSet.create(SATIR_YORUMU, BELGE_YORUMU, BLOK_YORUMU)
    @JvmField val DIZGILER = TokenSet.create(DIZGI)
}

/**
 * Pas kaynak kodunu sözcüklere ayırır. `crates/pas-cekirdek/src/sozcukler.rs`
 * ile aynı kuralları izler; ek olarak renklendirme için isimleri sınıflar.
 *
 * Her sözcük kendi başına tamamlanır (blok yorumlar ve dizgiler tek bir
 * sözcüktür), bu yüzden durum tutmaya gerek yoktur.
 */
class PasLexer : LexerBase() {
    private var metin: CharSequence = ""
    private var son = 0
    private var basla = 0
    private var bitis = 0
    private var tur: IElementType? = null
    private var konum = 0

    override fun start(buffer: CharSequence, startOffset: Int, endOffset: Int, initialState: Int) {
        metin = buffer
        son = endOffset
        bitis = startOffset
        advance()
    }

    override fun getState() = 0
    override fun getTokenType() = tur
    override fun getTokenStart() = basla
    override fun getTokenEnd() = bitis
    override fun getBufferSequence() = metin
    override fun getBufferEnd() = son

    override fun advance() {
        basla = bitis
        if (basla >= son) {
            tur = null
            return
        }
        konum = basla
        tur = sonrakiSozcuk()
        bitis = konum
    }

    private fun bak(n: Int = 0): Char = if (konum + n < son) metin[konum + n] else '\u0000'

    private fun basliyorMu(s: String): Boolean {
        if (konum + s.length > son) return false
        return s.indices.all { metin[konum + it] == s[it] }
    }

    private fun kodNoktasi(): Int = Character.codePointAt(metin, konum)

    private fun ilerle() {
        konum += if (konum < son) Character.charCount(kodNoktasi()) else 1
    }

    private fun isimBaslangiciMi(): Boolean = konum < son && isimBaslangicKarakteriMi(kodNoktasi())

    private fun isimDevami() {
        while (konum < son && isimDevamKarakteriMi(kodNoktasi())) ilerle()
    }

    private fun sonrakiSozcuk(): IElementType {
        val c = bak()
        if (Character.isWhitespace(c)) {
            while (konum < son && Character.isWhitespace(bak())) konum++
            return TokenType.WHITE_SPACE
        }
        if (basliyorMu("//")) {
            val belge = (basliyorMu("///") && !basliyorMu("////")) || basliyorMu("//!")
            while (konum < son && bak() != '\n') konum++
            return if (belge) PasSozcukler.BELGE_YORUMU else PasSozcukler.SATIR_YORUMU
        }
        if (basliyorMu("/*")) {
            blokYorum()
            return PasSozcukler.BLOK_YORUMU
        }
        onekli()?.let { return it }
        if (isimBaslangiciMi()) {
            isimDevami()
            return isimSinifi(basla)
        }
        return when (c) {
            '"' -> {
                konum++
                tirnakli('"')
                PasSozcukler.DIZGI
            }
            '\'' -> karakterYaDaYasamSuresi()
            in '0'..'9' -> {
                sayi()
                PasSozcukler.SAYI
            }
            else -> {
                ilerle()
                when (c) {
                    '(' -> PasSozcukler.SOL_PARANTEZ
                    ')' -> PasSozcukler.SAG_PARANTEZ
                    '[' -> PasSozcukler.SOL_KOSELI
                    ']' -> PasSozcukler.SAG_KOSELI
                    '{' -> PasSozcukler.SOL_SUSLU
                    '}' -> PasSozcukler.SAG_SUSLU
                    ';' -> PasSozcukler.NOKTALI_VIRGUL
                    ',' -> PasSozcukler.VIRGUL
                    '.' -> PasSozcukler.NOKTA
                    else -> PasSozcukler.ISLEC
                }
            }
        }
    }

    /** Bir ismin anahtar sözcük, makro ya da hazır bir isim olup olmadığı. */
    private fun isimSinifi(baslangic: Int): IElementType {
        val sozcuk = metin.subSequence(baslangic, konum).toString()
        if (bak() == '!' && bak(1) != '=' && !Sozluk.anahtarSozcukMu(sozcuk)) {
            konum++
            return PasSozcukler.MAKRO
        }
        if (Sozluk.anahtarSozcukMu(sozcuk)) return PasSozcukler.ANAHTAR_SOZCUK
        return when (Sozluk.bul(sozcuk)?.sinif) {
            Sinif.TUR -> PasSozcukler.HAZIR_TUR
            Sinif.DEGER -> PasSozcukler.HAZIR_DEGER
            else -> PasSozcukler.ISIM
        }
    }

    /** `r"…"`, `r#"…"#`, `br"…"`, `b"…"`, `c"…"`, `b'…'` ve `r#isim`. */
    private fun onekli(): IElementType? {
        for (onek in listOf("br", "cr", "r")) {
            if (!basliyorMu(onek)) continue
            var diyez = 0
            while (bak(onek.length + diyez) == '#') diyez++
            if (bak(onek.length + diyez) == '"') {
                konum += onek.length + diyez + 1
                hamDizgi(diyez)
                return PasSozcukler.DIZGI
            }
            if (onek == "r" && diyez == 1) {
                val onceki = konum
                konum += 2
                if (isimBaslangiciMi()) {
                    isimDevami()
                    return PasSozcukler.ISIM
                }
                konum = onceki
            }
        }
        if ((bak() == 'b' || bak() == 'c') && bak(1) == '"') {
            konum += 2
            tirnakli('"')
            return PasSozcukler.DIZGI
        }
        if (bak() == 'b' && bak(1) == '\'') {
            konum += 2
            tirnakli('\'')
            return PasSozcukler.KARAKTER
        }
        return null
    }

    private fun tirnakli(tirnak: Char) {
        while (konum < son) {
            val c = bak()
            konum++
            if (c == '\\') konum++ else if (c == tirnak) return
        }
        konum = minOf(konum, son)
    }

    private fun hamDizgi(diyez: Int) {
        val kapanis = "\"" + "#".repeat(diyez)
        while (konum < son) {
            if (basliyorMu(kapanis)) {
                konum += kapanis.length
                return
            }
            konum++
        }
    }

    private fun blokYorum() {
        konum += 2
        var derinlik = 1
        while (derinlik > 0 && konum < son) {
            when {
                basliyorMu("/*") -> {
                    derinlik++
                    konum += 2
                }
                basliyorMu("*/") -> {
                    derinlik--
                    konum += 2
                }
                else -> konum++
            }
        }
        konum = minOf(konum, son)
    }

    /** `'a'` bir karakter, `'a` (ardından `'` gelmeyen) bir yaşam süresidir. */
    private fun karakterYaDaYasamSuresi(): IElementType {
        konum++
        if (bak() == '\\') {
            tirnakli('\'')
            return PasSozcukler.KARAKTER
        }
        if (konum < son) {
            val genislik = Character.charCount(kodNoktasi())
            if (bak(genislik) == '\'') {
                konum += genislik + 1
                return PasSozcukler.KARAKTER
            }
        }
        if (isimBaslangiciMi()) {
            isimDevami()
            return PasSozcukler.YASAM_SURESI
        }
        return PasSozcukler.ISLEC
    }

    private fun sayi() {
        val onaltilik = basliyorMu("0x") || basliyorMu("0X")
        var onceki = bak()
        konum++
        while (konum < son) {
            val c = bak()
            val kabul = c.isLetterOrDigit() && c.code < 128 ||
                c == '_' ||
                (c == '.' && bak(1) in '0'..'9') ||
                ((c == '+' || c == '-') && !onaltilik && (onceki == 'e' || onceki == 'E'))
            if (!kabul) break
            onceki = c
            konum++
        }
    }

    companion object {
        fun isimBaslangicKarakteriMi(kod: Int) = kod == '_'.code || Character.isUnicodeIdentifierStart(kod)

        fun isimDevamKarakteriMi(kod: Int) = Character.isUnicodeIdentifierPart(kod) && !Character.isIdentifierIgnorable(kod)
    }
}
