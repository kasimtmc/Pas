package pas.eklenti

import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import org.junit.Assert.assertEquals
import org.junit.Test

class PasLexerTest {
    private fun sozcukler(metin: String): List<Pair<IElementType, String>> {
        val lexer = PasLexer()
        lexer.start(metin)
        val sonuc = mutableListOf<Pair<IElementType, String>>()
        while (lexer.tokenType != null) {
            if (lexer.tokenType != TokenType.WHITE_SPACE) {
                sonuc += lexer.tokenType!! to metin.substring(lexer.tokenStart, lexer.tokenEnd)
            }
            lexer.advance()
        }
        return sonuc
    }

    @Test
    fun anahtarSozcuklerVeIsimler() {
        assertEquals(
            listOf(
                PasSozcukler.ANAHTAR_SOZCUK to "tanımla",
                PasSozcukler.ANAHTAR_SOZCUK to "değişken",
                PasSozcukler.ISIM to "sayılar",
                PasSozcukler.ISLEC to ":",
                PasSozcukler.HAZIR_TUR to "Liste",
                PasSozcukler.ISLEC to "=",
                PasSozcukler.MAKRO to "liste!",
                PasSozcukler.SOL_KOSELI to "[",
                PasSozcukler.SAYI to "1",
                PasSozcukler.SAG_KOSELI to "]",
                PasSozcukler.NOKTALI_VIRGUL to ";",
            ),
            sozcukler("tanımla değişken sayılar: Liste = liste![1];"),
        )
    }

    @Test
    fun ingilizceAnahtarSozcuklerDeGecerli() {
        assertEquals(PasSozcukler.ANAHTAR_SOZCUK, sozcukler("fn").single().first)
        assertEquals(PasSozcukler.MAKRO, sozcukler("println!").single().first)
    }

    @Test
    fun esitDegilMakroDegildir() {
        assertEquals(PasSozcukler.ISIM, sozcukler("x != y").first().first)
    }

    @Test
    fun dizgiVeYorumlarinIciKarismaz() {
        assertEquals(
            listOf(
                PasSozcukler.DIZGI to "\"yap \\\" eğer\"",
                PasSozcukler.SATIR_YORUMU to "// eğer",
                PasSozcukler.BLOK_YORUMU to "/* a /* iç */ b */",
                PasSozcukler.DIZGI to "r#\"ham \"dizgi\"\"#",
            ),
            sozcukler("\"yap \\\" eğer\" // eğer\n/* a /* iç */ b */ r#\"ham \"dizgi\"\"#"),
        )
        assertEquals(PasSozcukler.BELGE_YORUMU, sozcukler("/// belge").single().first)
    }

    @Test
    fun karakterVeYasamSuresi() {
        assertEquals(
            listOf(PasSozcukler.KARAKTER to "'ş'", PasSozcukler.ISLEC to "&", PasSozcukler.YASAM_SURESI to "'kalıcı"),
            sozcukler("'ş' &'kalıcı"),
        )
    }

    @Test
    fun hamIsim() {
        assertEquals(listOf(PasSozcukler.ISIM to "r#tür"), sozcukler("r#tür"))
    }

    @Test
    fun yarimKalanMetinCokmez() {
        // Yazılmakta olan kod da sözcüklere ayrılabilmeli.
        sozcukler("\"kapanmamış")
        sozcukler("/* kapanmamış")
        sozcukler("'")
        sozcukler("r##\"")
    }
}
