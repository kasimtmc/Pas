package pas.eklenti

import com.intellij.openapi.command.WriteCommandAction
import com.intellij.psi.codeStyle.CodeStyleManager
import com.intellij.psi.util.PsiTreeUtil
import com.intellij.testFramework.fixtures.BasePlatformTestCase

class PasYapiTest : BasePlatformTestCase() {
    private val ornek = """
        modül şekiller;

        /// Bir oyuncu.
        #[derive(Debug)]
        genel yapı Oyuncu {
            ad: Metin,
        }

        çeşit Renk { Kırmızı, Mavi }

        sabit EN_FAZLA: u32 = 10;

        uygula Oyuncu {
            genel işlev yeni(ad: &str) -> Kendi {
                Oyuncu { ad: ad.metne_çevir() }
            }

            işlev gizli(&kendi) {}
        }

        işlev ana() {
            tanımla f: işlev(i32) -> i32 = |x| x;
            işlev iç_işlev() {}
        }
    """.trimIndent()

    fun testOgelerAyristirilir() {
        val dosya = myFixture.configureByText("ana.pas", ornek)
        val ogeler = PsiTreeUtil.findChildrenOfType(dosya, PasOge::class.java).map { "${it.anahtarSozcuk} ${it.ad}" }
        assertEquals(
            listOf(
                "modül şekiller", "yapı Oyuncu", "çeşit Renk", "sabit EN_FAZLA", "uygula Oyuncu",
                "işlev yeni", "işlev gizli", "işlev ana", "işlev iç_işlev",
            ),
            ogeler,
        )
    }

    fun testYapiGorunumu() {
        val dosya = myFixture.configureByText("ana.pas", ornek)
        fun yazdir(oge: PasYapiOgesi, girinti: String = ""): List<String> =
            listOf(girinti + oge.presentableText) +
                oge.children.flatMap { yazdir(it as PasYapiOgesi, "$girinti  ") }
        assertEquals(
            listOf(
                "ana.pas",
                "  şekiller", "  Oyuncu", "  Renk", "  EN_FAZLA",
                "  uygula Oyuncu", "    yeni", "    gizli",
                // İşlev gövdesindeki öğeler gösterilmez.
                "  ana",
            ),
            yazdir(PasDosyaOgesi(dosya)),
        )
    }

    fun testUygulaBasligi() {
        val dosya = myFixture.configureByText("ana.pas", "uygula<T> Görüntüle için Kutu<T> {}")
        assertEquals("<T> Görüntüle için Kutu<T>", PsiTreeUtil.findChildOfType(dosya, PasOge::class.java)!!.ad)
    }

    fun testYarimKodCokmez() {
        val dosya = myFixture.configureByText("ana.pas", "işlev ana() {\n    eğer x {\n        yazdır!(\n}\n}\n} işlev")
        assertEquals(dosya.text, dosya.node.text)
        assertTrue(PsiTreeUtil.findChildrenOfType(dosya, PasOge::class.java).isNotEmpty())
    }

    fun testKatlama() {
        val dosya = myFixture.configureByText("ana.pas", "işlev ana() {\n    tanımla x = 1;\n}\n/* çok\nsatırlı */\nişlev tek() {}")
        val bolgeler = PasKatlama().buildFoldRegions(dosya, myFixture.editor.document, false)
        assertEquals(listOf("{...}", "/*...*/"), bolgeler.map { PasKatlama().getPlaceholderText(it.element) })
    }

    fun testGirintiDuzeltilir() {
        val dosya = myFixture.configureByText(
            "ana.pas",
            "işlev ana() {\ntanımla l = liste![\n1,\n2,\n];\n        eşle l.uzunluk() {\n0 => yazdır!(\"boş\"),\n_ => {\n// yorum\nyazdır!(\"dolu\")\n}\n}\n}\n",
        )
        WriteCommandAction.runWriteCommandAction(project) { CodeStyleManager.getInstance(project).reformat(dosya) }
        assertEquals(
            "işlev ana() {\n    tanımla l = liste![\n        1,\n        2,\n    ];\n    eşle l.uzunluk() {\n        0 => yazdır!(\"boş\"),\n" +
                "        _ => {\n            // yorum\n            yazdır!(\"dolu\")\n        }\n    }\n}\n",
            dosya.text,
        )
    }

    fun testEnterGirintiVerir() {
        myFixture.configureByText("ana.pas", "işlev ana() {<caret>}")
        myFixture.type("\n")
        myFixture.checkResult("işlev ana() {\n    <caret>\n}")
    }

    fun testAnaIslevindeCalistirDugmesi() {
        myFixture.configureByText("ana.pas", "genel işlev yardımcı() {}\n\nişlev a<caret>na() {\n    yardımcı();\n}\n")
        assertEquals(1, myFixture.findGuttersAtCaret().size)
        myFixture.configureByText("ana.pas", "işlev yard<caret>ımcı() {}\n")
        assertEquals(0, myFixture.findGuttersAtCaret().size)
    }
}
