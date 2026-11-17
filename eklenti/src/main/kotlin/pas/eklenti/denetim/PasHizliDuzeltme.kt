package pas.eklenti.denetim

import com.intellij.codeInsight.intention.IntentionAction
import com.intellij.codeInsight.intention.PriorityAction
import com.intellij.codeInsight.intention.preview.IntentionPreviewInfo
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.TextRange
import com.intellij.psi.PsiFile
import pas.eklenti.PasDosyaTuru

/** Belgedeki bir aralığın yerine yazılacak metin. */
data class BelgeDegisikligi(val aralik: TextRange, val metin: String)

/**
 * Derleyicinin önerdiği düzeltmeyi Alt+Enter ile uygular. Değişiklikler,
 * tanılamanın üretildiği belge sürümüne göre hesaplanmıştır; belge o
 * zamandan beri değiştiyse düzeltme gösterilmez.
 */
class PasHizliDuzeltme(
    private val mesaj: String,
    private val degisiklikler: List<BelgeDegisikligi>,
    private val belgeDamgasi: Long,
    private val kesin: Boolean,
) : IntentionAction, PriorityAction {

    override fun getText(): String {
        val tek = degisiklikler.singleOrNull() ?: return "Düzelt: ${mesaj.replace("`", "")}"
        return if (tek.aralik.isEmpty) "Ekle: ${tek.metin.trim()}" else "Şununla değiştir: ${tek.metin.trim()}"
    }

    override fun getFamilyName() = "Pas derleyici önerisi"

    override fun getPriority() = if (kesin) PriorityAction.Priority.HIGH else PriorityAction.Priority.NORMAL

    override fun isAvailable(project: Project, editor: Editor?, file: PsiFile?) =
        editor != null && editor.document.modificationStamp == belgeDamgasi

    override fun startInWriteAction() = true

    override fun invoke(project: Project, editor: Editor?, file: PsiFile?) {
        val belge = editor?.document ?: return
        if (belge.modificationStamp != belgeDamgasi) return
        for (degisiklik in degisiklikler.sortedByDescending { it.aralik.startOffset }) {
            belge.replaceString(degisiklik.aralik.startOffset, degisiklik.aralik.endOffset, degisiklik.metin)
        }
    }

    /** Alt+Enter menüsünde, değişikliğin öncesini ve sonrasını gösterir. */
    override fun generatePreview(project: Project, editor: Editor, file: PsiFile): IntentionPreviewInfo {
        val belge = editor.document
        val ilk = degisiklikler.minOf { it.aralik.startOffset }
        val son = degisiklikler.maxOf { it.aralik.endOffset }
        if (son > belge.textLength) return IntentionPreviewInfo.EMPTY
        val bolgeBasi = belge.getLineStartOffset(belge.getLineNumber(ilk))
        val bolgeSonu = belge.getLineEndOffset(belge.getLineNumber(son))
        val once = belge.charsSequence.subSequence(bolgeBasi, bolgeSonu).toString()
        val sonra = StringBuilder(once)
        for (degisiklik in degisiklikler.sortedByDescending { it.aralik.startOffset }) {
            sonra.replace(degisiklik.aralik.startOffset - bolgeBasi, degisiklik.aralik.endOffset - bolgeBasi, degisiklik.metin)
        }
        return IntentionPreviewInfo.CustomDiff(PasDosyaTuru, once, sonra.toString())
    }
}
