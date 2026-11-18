package pas.eklenti.denetim

import com.google.gson.JsonObject
import com.google.gson.JsonParser
import com.intellij.execution.process.CapturingProcessHandler
import com.intellij.lang.annotation.AnnotationHolder
import com.intellij.lang.annotation.ExternalAnnotator
import com.intellij.lang.annotation.HighlightSeverity
import com.intellij.openapi.diagnostic.thisLogger
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.TextRange
import com.intellij.openapi.util.text.StringUtil
import com.intellij.openapi.vfs.VfsUtilCore
import com.intellij.psi.PsiDocumentManager
import com.intellij.psi.PsiFile
import pas.eklenti.PasAraclari
import pas.eklenti.PasKurulum
import java.nio.charset.StandardCharsets

/** `cargo pas denetle --json` çıktısındaki bir tanılama. */
data class Tani(
    val seviye: String,
    val kod: String?,
    val mesaj: String,
    val dosya: String,
    val konum: Konum,
    val notlar: List<String>,
    val oneriler: List<Oneri>,
    val aciklama: String?,
)

/** 1'den başlayan satır ve (karakter olarak) sütun numaralarıyla bir aralık. */
data class Konum(val satir: Int, val sutun: Int, val bitisSatiri: Int, val bitisSutunu: Int)

/** Derleyicinin önerdiği bir düzeltme: `kesin` ise rustc güvenle uygulanabilir der. */
data class Oneri(val mesaj: String, val kesin: Boolean, val degisiklikler: List<Pair<Konum, String>>)

/**
 * Pas dosyalarını derleyiciyle denetleyip hataları editörde Türkçe gösterir.
 * Düzenlenen dosyanın kaydedilmemiş hali `--stdin` ile verilir; böylece
 * hatalar dosya kaydedilmeden de görünür.
 */
class PasDenetleyici : ExternalAnnotator<PasDenetleyici.Girdi, List<Tani>>() {
    class Girdi(val project: Project, val kok: String, val dosya: String, val metin: String)

    override fun collectInformation(file: PsiFile, editor: Editor, hasErrors: Boolean) = collectInformation(file)

    override fun collectInformation(file: PsiFile): Girdi? {
        val sanal = file.virtualFile ?: return null
        val kok = PasAraclari.projeKoku(sanal) ?: return null
        val goreli = VfsUtilCore.getRelativePath(sanal, kok) ?: return null
        return Girdi(file.project, kok.path, goreli, file.text)
    }

    override fun doAnnotate(girdi: Girdi): List<Tani>? {
        val komut = PasAraclari.komutSatiri(girdi.kok, listOf("denetle", "--json", "--stdin", girdi.dosya))
        if (komut == null) {
            PasKurulum.eksikseBildir(girdi.project)
            return null
        }
        return try {
            val isleyici = CapturingProcessHandler(komut)
            isleyici.processInput.use { it.write(girdi.metin.toByteArray(StandardCharsets.UTF_8)) }
            val cikti = isleyici.runProcess(ZAMAN_ASIMI_MS)
            if (cikti.isTimeout) return null
            cikti.stdoutLines.mapNotNull(::cozumle).filter { it.dosya == girdi.dosya }
        } catch (e: Exception) {
            thisLogger().warn("cargo pas denetle çalıştırılamadı", e)
            null
        }
    }

    override fun apply(file: PsiFile, tanilar: List<Tani>, holder: AnnotationHolder) {
        val belge = PsiDocumentManager.getInstance(file.project).getDocument(file) ?: return
        for (tani in tanilar) {
            val onem = when (tani.seviye) {
                "hata" -> HighlightSeverity.ERROR
                "uyarı" -> HighlightSeverity.WARNING
                else -> HighlightSeverity.WEAK_WARNING
            }
            var ek = holder.newAnnotation(onem, baslik(tani))
                .range(isaretlenecekAralik(belge, tani.konum))
                .tooltip(ipucu(tani))
            for (oneri in tani.oneriler) {
                val degisiklikler = oneri.degisiklikler.map { (konum, metin) -> BelgeDegisikligi(aralik(belge, konum), metin) }
                ek = ek.withFix(PasHizliDuzeltme(oneri.mesaj, degisiklikler, belge.modificationStamp, oneri.kesin))
            }
            ek.create()
        }
    }

    private fun baslik(tani: Tani) = if (tani.kod != null) "${tani.mesaj} [${tani.kod}]" else tani.mesaj

    private fun ipucu(tani: Tani): String = buildString {
        append("<html><b>").append(StringUtil.escapeXmlEntities(baslik(tani))).append("</b>")
        for (not in tani.notlar) {
            append("<br>").append(StringUtil.escapeXmlEntities(not).replace("\n", "<br>"))
        }
        tani.aciklama?.let { append("<br><br><i>").append(StringUtil.escapeXmlEntities(it)).append("</i>") }
        append("</html>")
    }

    private fun aralik(belge: Document, konum: Konum): TextRange {
        val baslangic = ofset(belge, konum.satir, konum.sutun)
        return TextRange(baslangic, maxOf(baslangic, ofset(belge, konum.bitisSatiri, konum.bitisSutunu)))
    }

    /** Boş aralıklar görünmez; en az bir karakteri işaretle. */
    private fun isaretlenecekAralik(belge: Document, konum: Konum): TextRange {
        val aralik = aralik(belge, konum)
        return if (aralik.isEmpty) TextRange(aralik.startOffset, minOf(aralik.startOffset + 1, belge.textLength)) else aralik
    }

    /** 1'den başlayan satır ve karakter sütununu belge ofsetine çevirir. */
    private fun ofset(belge: Document, satir: Int, sutun: Int): Int {
        if (belge.lineCount == 0) return 0
        val satirNo = (satir - 1).coerceIn(0, belge.lineCount - 1)
        val metin = belge.charsSequence
        val satirSonu = belge.getLineEndOffset(satirNo)
        var ofset = belge.getLineStartOffset(satirNo)
        var kalan = sutun - 1
        while (kalan > 0 && ofset < satirSonu) {
            ofset += Character.charCount(Character.codePointAt(metin, ofset))
            kalan--
        }
        return ofset
    }

    companion object {
        private const val ZAMAN_ASIMI_MS = 120_000

        fun cozumle(satir: String): Tani? = try {
            val json = JsonParser.parseString(satir).asJsonObject
            fun istegeBagli(ad: String) = json.get(ad)?.takeUnless { it.isJsonNull }?.asString
            Tani(
                seviye = json.get("seviye").asString,
                kod = istegeBagli("kod"),
                mesaj = json.get("mesaj").asString,
                dosya = json.get("dosya").asString,
                konum = konum(json.getAsJsonObject("aralik")),
                notlar = json.getAsJsonArray("notlar")?.map { it.asString }.orEmpty(),
                oneriler = json.getAsJsonArray("oneriler")?.map { o ->
                    val oneri = o.asJsonObject
                    Oneri(
                        mesaj = oneri.get("mesaj").asString,
                        kesin = oneri.get("kesin")?.asBoolean ?: false,
                        degisiklikler = oneri.getAsJsonArray("degisiklikler").map { d ->
                            val degisiklik = d.asJsonObject
                            konum(degisiklik.getAsJsonObject("aralik")) to degisiklik.get("metin").asString
                        },
                    )
                }.orEmpty(),
                aciklama = istegeBagli("aciklama"),
            )
        } catch (_: Exception) {
            null
        }

        private fun konum(json: JsonObject) = Konum(
            satir = json.get("satir").asInt,
            sutun = json.get("sutun").asInt,
            bitisSatiri = json.get("bitis_satiri").asInt,
            bitisSutunu = json.get("bitis_sutunu").asInt,
        )
    }
}
