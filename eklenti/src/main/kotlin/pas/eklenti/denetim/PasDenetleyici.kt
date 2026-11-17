package pas.eklenti.denetim

import com.google.gson.JsonParser
import com.intellij.execution.process.CapturingProcessHandler
import com.intellij.lang.annotation.AnnotationHolder
import com.intellij.lang.annotation.ExternalAnnotator
import com.intellij.lang.annotation.HighlightSeverity
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
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
import java.nio.charset.StandardCharsets
import java.util.concurrent.atomic.AtomicBoolean

/** `cargo pas denetle --json` çıktısındaki bir tanılama. */
data class Tani(
    val seviye: String,
    val kod: String?,
    val mesaj: String,
    val dosya: String,
    val satir: Int,
    val sutun: Int,
    val bitisSatiri: Int,
    val bitisSutunu: Int,
    val notlar: List<String>,
    val aciklama: String?,
)

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
            araciBulamadiysaBildir(girdi.project)
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
                "error" -> HighlightSeverity.ERROR
                "warning" -> HighlightSeverity.WARNING
                else -> HighlightSeverity.WEAK_WARNING
            }
            holder.newAnnotation(onem, baslik(tani))
                .range(aralik(belge, tani))
                .tooltip(ipucu(tani))
                .create()
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

    private fun aralik(belge: Document, tani: Tani): TextRange {
        val baslangic = ofset(belge, tani.satir, tani.sutun)
        var bitis = ofset(belge, tani.bitisSatiri, tani.bitisSutunu)
        // Boş aralıklar görünmez; en az bir karakteri işaretle.
        if (bitis <= baslangic) bitis = minOf(baslangic + 1, belge.textLength)
        return TextRange(minOf(baslangic, bitis), bitis)
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
        private val bildirildi = AtomicBoolean(false)

        fun cozumle(satir: String): Tani? = try {
            val json = JsonParser.parseString(satir).asJsonObject
            val aralik = json.getAsJsonObject("span")
            fun JsonObjectAlani(ad: String) = json.get(ad)?.takeUnless { it.isJsonNull }?.asString
            Tani(
                seviye = json.get("level").asString,
                kod = JsonObjectAlani("code"),
                mesaj = json.get("message").asString,
                dosya = json.get("file").asString,
                satir = aralik.get("line").asInt,
                sutun = aralik.get("column").asInt,
                bitisSatiri = aralik.get("end_line").asInt,
                bitisSutunu = aralik.get("end_column").asInt,
                notlar = json.getAsJsonArray("notes")?.map { it.asString }.orEmpty(),
                aciklama = JsonObjectAlani("explanation"),
            )
        } catch (_: Exception) {
            null
        }

        private fun araciBulamadiysaBildir(project: Project) {
            if (!bildirildi.compareAndSet(false, true)) return
            NotificationGroupManager.getInstance()
                .getNotificationGroup("Pas")
                .createNotification(PasAraclari.KURULUM_YARDIMI, NotificationType.WARNING)
                .notify(project)
        }
    }
}
