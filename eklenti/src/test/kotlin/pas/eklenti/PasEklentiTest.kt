package pas.eklenti

import com.intellij.codeInsight.intention.preview.IntentionPreviewInfo
import com.intellij.codeInsight.lookup.Lookup
import com.intellij.testFramework.fixtures.BasePlatformTestCase
import pas.eklenti.calistir.PasKonumFiltresi
import com.intellij.openapi.command.WriteCommandAction
import com.intellij.openapi.util.TextRange
import pas.eklenti.denetim.BelgeDegisikligi
import pas.eklenti.denetim.Konum
import pas.eklenti.denetim.Oneri
import pas.eklenti.denetim.PasDenetleyici
import pas.eklenti.denetim.PasHizliDuzeltme

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
            """"aralik":{"satir":3,"sutun":5,"bitis_satiri":3,"bitis_sutunu":10},"etiketler":[],"notlar":["yardım: a"],""" +
            """"oneriler":[{"mesaj":"m","kesin":true,"degisiklikler":[{"aralik":{"satir":2,"sutun":9,"bitis_satiri":2,""" +
            """"bitis_sutunu":9},"metin":"değişken "}]}],"aciklama":null}"""
        val tani = PasDenetleyici.cozumle(satir)!!
        assertEquals("E0384", tani.kod)
        assertEquals(Konum(3, 5, 3, 10), tani.konum)
        assertEquals(listOf("yardım: a"), tani.notlar)
        assertEquals(listOf(Oneri("m", true, listOf(Konum(2, 9, 2, 9) to "değişken "))), tani.oneriler)
        assertNull(tani.aciklama)
        assertNull(PasDenetleyici.cozumle("Derleniyor…"))
    }

    fun testHizliDuzeltmeEkler() {
        myFixture.configureByText("ana.pas", "işlev ana() {\n    yap x = 1;\n    x = 2;\n}")
        val belge = myFixture.editor.document
        val ofset = belge.text.indexOf("x = 1")
        val duzeltme = PasHizliDuzeltme(
            "değerini sonradan değiştirebilmek için başına `değişken` ekle",
            listOf(BelgeDegisikligi(TextRange(ofset, ofset), "değişken ")),
            belge.modificationStamp,
            kesin = true,
        )
        assertEquals("Ekle: değişken", duzeltme.text)
        assertTrue(duzeltme.isAvailable(project, myFixture.editor, myFixture.file))
        val onizleme = duzeltme.generatePreview(project, myFixture.editor, myFixture.file) as IntentionPreviewInfo.CustomDiff
        assertEquals("    yap x = 1;", onizleme.originalText())
        assertEquals("    yap değişken x = 1;", onizleme.modifiedText())
        WriteCommandAction.runWriteCommandAction(project) { duzeltme.invoke(project, myFixture.editor, myFixture.file) }
        assertEquals("işlev ana() {\n    yap değişken x = 1;\n    x = 2;\n}", belge.text)
        // Belge değişti; aynı düzeltme artık geçerli değil.
        assertFalse(duzeltme.isAvailable(project, myFixture.editor, myFixture.file))
    }

    fun testHizliDuzeltmeDegistirir() {
        myFixture.configureByText("ana.pas", "yap kullanilmayan = 3;")
        val belge = myFixture.editor.document
        val duzeltme = PasHizliDuzeltme(
            "m",
            listOf(BelgeDegisikligi(TextRange(4, 17), "_kullanilmayan")),
            belge.modificationStamp,
            kesin = true,
        )
        assertEquals("Şununla değiştir: _kullanilmayan", duzeltme.text)
        WriteCommandAction.runWriteCommandAction(project) { duzeltme.invoke(project, myFixture.editor, myFixture.file) }
        assertEquals("yap _kullanilmayan = 3;", belge.text)
    }
}
