package pas.eklenti

import com.intellij.extapi.psi.ASTWrapperPsiElement
import com.intellij.lang.ASTNode
import com.intellij.lang.ParserDefinition
import com.intellij.lang.PsiParser
import com.intellij.openapi.project.Project
import com.intellij.psi.FileViewProvider
import com.intellij.psi.PsiElement
import com.intellij.psi.tree.IFileElementType

/**
 * Şimdilik düz bir ağaç: dosya yalnızca sözcüklerden oluşur. Gerçek
 * sözdizimi denetimini derleyici yapar; renklendirme, yorum satırı ve
 * parantez eşleme için sözcükler yeterlidir.
 */
class PasAyristiriciTanimi : ParserDefinition {
    override fun createLexer(project: Project?) = PasLexer()

    override fun createParser(project: Project?) = PsiParser { kok, kurucu ->
        val isaret = kurucu.mark()
        while (!kurucu.eof()) kurucu.advanceLexer()
        isaret.done(kok)
        kurucu.treeBuilt
    }

    override fun getFileNodeType() = DOSYA

    override fun getCommentTokens() = PasSozcukler.YORUMLAR

    override fun getStringLiteralElements() = PasSozcukler.DIZGILER

    override fun createElement(node: ASTNode): PsiElement = ASTWrapperPsiElement(node)

    override fun createFile(viewProvider: FileViewProvider) = PasDosyasi(viewProvider)

    companion object {
        val DOSYA = IFileElementType(PasDili)
    }
}
