package pas.eklenti

import com.intellij.codeInsight.lookup.Lookup
import com.intellij.testFramework.fixtures.BasePlatformTestCase
import pas.eklenti.calistir.PasKonumFiltresi
import pas.eklenti.denetim.PasDenetleyici

class PasEklentiTest : BasePlatformTestCase() {
    /** Tamamlamayı açar ve verilen öneriyi seçer (tek öneri varsa IDE kendisi ekler). */
    private fun tamamla(secim: String) {
        val oneriler = myFixture.completeBasic() ?: return
        val oge = oneriler.firstOrNull { it.lookupString == secim }
        assertNotNull("`$secim` önerilmedi: ${oneriler.map { it.lookupString }}", oge)
        myFixture.lookup.currentItem = oge
        myFixture.finishLookup(Lookup.NORMAL_SELECT_CHAR)
    }

    fun testDosyaTuru() {
        val dosya = myFixture.configureByText("ana.pas", "işlev ana() {}")
        assertEquals(PasDosyaTuru, dosya.fileType)
        assertEquals(PasDili, dosya.language)
    }

    fun testTamamlamaSozluguOnerir() {
        myFixture.configureByText("ana.pas", "işlev ana() {\n    ya<caret>\n}")
        val oneriler = myFixture.completeBasic()?.map { it.lookupString }.orEmpty()
        assertContainsElements(oneriler, "yap", "yazdır!", "yaz!")
    }

    fun testTamamlamaDosyadakiIsimleriOnerir() {
        myFixture.configureByText("ana.pas", "işlev ana() {\n    yap sayılar = 1;\n    say<caret>\n}")
        tamamla("sayılar")
        assertTrue(myFixture.editor.document.text.contains("    sayılar\n"))
    }

    fun testMakroParantezEkler() {
        myFixture.configureByText("ana.pas", "işlev ana() {\n    yazdı<caret>\n}")
        tamamla("yazdır!")
        myFixture.type("\"Merhaba\"")
        assertTrue(myFixture.editor.document.text, myFixture.editor.document.text.contains("yazdır!(\"Merhaba\")"))
    }

    fun testYorumSatiri() {
        myFixture.configureByText("ana.pas", "yap x = 1;<caret>")
        myFixture.performEditorAction("CommentByLineComment")
        myFixture.checkResult("//yap x = 1;")
    }

    fun testKonumFiltresiDeseni() {
        val eslesme = PasKonumFiltresi.DESEN.find("  --> src/şekiller.pas:12:5")!!
        assertEquals(listOf("src/şekiller.pas", "12", "5"), eslesme.destructured.toList())
    }

    fun testTaniCozumleme() {
        val satir = """{"seviye":"hata","kod":"E0384","mesaj":"`x` değiştirilemez","dosya":"src/ana.pas",""" +
            """"aralik":{"satir":3,"sutun":5,"bitis_satiri":3,"bitis_sutunu":10},"etiketler":[],"notlar":["yardım: a"],"aciklama":null}"""
        val tani = PasDenetleyici.cozumle(satir)!!
        assertEquals("E0384", tani.kod)
        assertEquals(3, tani.satir)
        assertEquals(listOf("yardım: a"), tani.notlar)
        assertNull(tani.aciklama)
        assertNull(PasDenetleyici.cozumle("Derleniyor…"))
    }
}
