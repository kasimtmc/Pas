package pas.eklenti

import com.intellij.codeInsight.editorActions.SimpleTokenSetQuoteHandler
import com.intellij.lang.BracePair
import com.intellij.lang.Commenter
import com.intellij.lang.PairedBraceMatcher
import com.intellij.psi.PsiFile
import com.intellij.psi.tree.IElementType

/** Ctrl+/ ve Ctrl+Shift+/ ile yorum satırı. */
class PasYorumlayici : Commenter {
    override fun getLineCommentPrefix() = "//"
    override fun getBlockCommentPrefix() = "/*"
    override fun getBlockCommentSuffix() = "*/"
    override fun getCommentedBlockCommentPrefix(): String? = null
    override fun getCommentedBlockCommentSuffix(): String? = null
}

class PasParantezEsleyici : PairedBraceMatcher {
    override fun getPairs() = arrayOf(
        BracePair(PasSozcukler.SOL_PARANTEZ, PasSozcukler.SAG_PARANTEZ, false),
        BracePair(PasSozcukler.SOL_KOSELI, PasSozcukler.SAG_KOSELI, false),
        BracePair(PasSozcukler.SOL_SUSLU, PasSozcukler.SAG_SUSLU, true),
    )

    override fun isPairedBracesAllowedBeforeType(lbraceType: IElementType, contextType: IElementType?) = true

    override fun getCodeConstructStart(file: PsiFile?, openingBraceOffset: Int) = openingBraceOffset
}

/** Tırnak yazınca kapanışını kendiliğinden ekler. */
class PasTirnakIsleyici : SimpleTokenSetQuoteHandler(PasSozcukler.DIZGI)
