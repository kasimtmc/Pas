package pas.eklenti

import com.intellij.formatting.Alignment
import com.intellij.formatting.Block
import com.intellij.formatting.ChildAttributes
import com.intellij.formatting.FormattingContext
import com.intellij.formatting.FormattingModel
import com.intellij.formatting.FormattingModelBuilder
import com.intellij.formatting.FormattingModelProvider
import com.intellij.formatting.Indent
import com.intellij.formatting.Spacing
import com.intellij.icons.AllIcons
import com.intellij.ide.structureView.StructureViewModel
import com.intellij.ide.structureView.StructureViewModelBase
import com.intellij.ide.structureView.StructureViewTreeElement
import com.intellij.ide.structureView.TreeBasedStructureViewBuilder
import com.intellij.ide.util.treeView.smartTree.TreeElement
import com.intellij.navigation.ItemPresentation
import com.intellij.pom.Navigatable
import com.intellij.lang.ASTNode
import com.intellij.lang.PsiStructureViewFactory
import com.intellij.lang.folding.FoldingBuilderEx
import com.intellij.lang.folding.FoldingDescriptor
import com.intellij.openapi.editor.Document
import com.intellij.openapi.editor.Editor
import com.intellij.openapi.project.DumbAware
import com.intellij.openapi.util.TextRange
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile
import com.intellij.psi.TokenType
import com.intellij.psi.formatter.common.AbstractBlock
import com.intellij.psi.util.PsiTreeUtil
import javax.swing.Icon

/** Çok satırlı blokları (`{ … }`) ve blok yorumlarını katlar. */
class PasKatlama : FoldingBuilderEx(), DumbAware {
    override fun buildFoldRegions(kok: PsiElement, belge: Document, hizli: Boolean): Array<FoldingDescriptor> {
        val bolgeler = mutableListOf<FoldingDescriptor>()
        PsiTreeUtil.processElements(kok) { oge ->
            val tur = oge.node.elementType
            val katlanir = tur == PasDugumler.BLOK || tur == PasSozcukler.BLOK_YORUMU
            if (katlanir && cokSatirli(oge.textRange, belge)) bolgeler += FoldingDescriptor(oge.node, oge.textRange)
            true
        }
        return bolgeler.toTypedArray()
    }

    private fun cokSatirli(aralik: TextRange, belge: Document) =
        belge.getLineNumber(aralik.startOffset) < belge.getLineNumber(aralik.endOffset)

    override fun getPlaceholderText(dugum: ASTNode) = if (dugum.elementType == PasDugumler.BLOK) "{...}" else "/*...*/"

    override fun isCollapsedByDefault(dugum: ASTNode) = false
}

/** Dosyadaki işlev, yapı, çeşit ve `uygula` bloklarını ağaç olarak gösterir. */
class PasYapiGorunumuFabrikasi : PsiStructureViewFactory {
    override fun getStructureViewBuilder(dosya: PsiFile) = object : TreeBasedStructureViewBuilder() {
        override fun createStructureViewModel(editor: Editor?): StructureViewModel = PasYapiModeli(dosya, editor)

        override fun isRootNodeShown() = false
    }
}

class PasYapiModeli(dosya: PsiFile, editor: Editor?) :
    StructureViewModelBase(dosya, editor, PasDosyaOgesi(dosya)),
    StructureViewModel.ElementInfoProvider {

    init {
        withSuitableClasses(PasOge::class.java)
    }

    override fun isAlwaysShowsPlus(oge: StructureViewTreeElement) = false

    override fun isAlwaysLeaf(oge: StructureViewTreeElement) = (oge.value as? PasOge)?.node?.elementType == PasDugumler.ISLEV
}

/** Yapı görünümündeki bir düğüm: dosyanın kendisi ya da bir öğe. */
abstract class PasYapiOgesi(private val psi: PsiElement) : StructureViewTreeElement, ItemPresentation {
    override fun getValue(): Any = psi

    override fun getPresentation(): ItemPresentation = this

    override fun navigate(odakla: Boolean) {
        (psi as? Navigatable)?.navigate(odakla)
    }

    override fun canNavigate() = (psi as? Navigatable)?.canNavigate() == true

    override fun canNavigateToSource() = (psi as? Navigatable)?.canNavigateToSource() == true

    override fun getIcon(open: Boolean): Icon? = null

    override fun getLocationString(): String? = null

    protected fun ogeler(liste: List<PasOge>): Array<TreeElement> =
        liste.filter { it.ad != null }.map(::PasOgeOgesi).toTypedArray()
}

class PasDosyaOgesi(private val dosya: PsiFile) : PasYapiOgesi(dosya) {
    override fun getPresentableText(): String = dosya.name

    override fun getChildren(): Array<TreeElement> = ogeler(PasOge.ogeleriTopla(dosya.node))
}

class PasOgeOgesi(private val oge: PasOge) : PasYapiOgesi(oge) {
    override fun getPresentableText(): String? =
        if (oge.node.elementType == PasDugumler.UYGULA) "${oge.anahtarSozcuk} ${oge.ad}" else oge.ad

    override fun getLocationString(): String? = oge.anahtarSozcuk.takeIf { oge.node.elementType != PasDugumler.UYGULA }

    override fun getIcon(open: Boolean): Icon? = when (oge.node.elementType) {
        PasDugumler.ISLEV -> AllIcons.Nodes.Function
        PasDugumler.YAPI -> AllIcons.Nodes.Class
        PasDugumler.CESIT -> AllIcons.Nodes.Enum
        PasDugumler.NITELIK -> AllIcons.Nodes.Interface
        PasDugumler.UYGULA -> AllIcons.Nodes.AbstractClass
        PasDugumler.MODUL -> AllIcons.Nodes.Package
        PasDugumler.SABIT -> AllIcons.Nodes.Constant
        PasDugumler.TUR_ADI -> AllIcons.Nodes.Type
        else -> null
    }

    override fun getChildren(): Array<TreeElement> = ogeler(oge.icOgeler)
}

/**
 * Girinti: blokların ve parantez gruplarının içi bir düzey içeride.
 * Boşluklara dokunulmaz; Ctrl+Alt+L yalnızca girintiyi düzeltir.
 */
class PasBicimlendirici : FormattingModelBuilder {
    override fun createModel(baglam: FormattingContext): FormattingModel {
        val dosya = baglam.containingFile
        val kok = PasBicimBlogu(dosya.node, Indent.getNoneIndent())
        return FormattingModelProvider.createFormattingModelForPsiFile(dosya, kok, baglam.codeStyleSettings)
    }
}

class PasBicimBlogu(dugum: ASTNode, private val girinti: Indent) : AbstractBlock(dugum, null, Alignment.createAlignment()) {
    override fun buildChildren(): List<Block> =
        node.getChildren(null)
            .filter { it.elementType != TokenType.WHITE_SPACE && it.textLength > 0 }
            .map { PasBicimBlogu(it, cocukGirintisi(it)) }

    private fun grupMu() = node.elementType in PasDugumler.GRUPLAR

    private fun cocukGirintisi(cocuk: ASTNode): Indent =
        if (grupMu() && cocuk.elementType !in ACILIS_KAPANIS) Indent.getNormalIndent() else Indent.getNoneIndent()

    override fun getIndent() = girinti

    override fun getAlignment(): Alignment? = null

    override fun getSpacing(cocuk1: Block?, cocuk2: Block): Spacing? = null

    override fun isLeaf() = node.firstChildNode == null

    override fun getChildAttributes(yeniSira: Int) =
        ChildAttributes(if (grupMu()) Indent.getNormalIndent() else Indent.getNoneIndent(), null)

    companion object {
        private val ACILIS_KAPANIS = setOf(
            PasSozcukler.SOL_SUSLU, PasSozcukler.SAG_SUSLU,
            PasSozcukler.SOL_PARANTEZ, PasSozcukler.SAG_PARANTEZ,
            PasSozcukler.SOL_KOSELI, PasSozcukler.SAG_KOSELI,
        )
    }
}
