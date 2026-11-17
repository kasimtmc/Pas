package pas.eklenti

import com.intellij.extapi.psi.PsiFileBase
import com.intellij.lang.Language
import com.intellij.openapi.fileTypes.LanguageFileType
import com.intellij.openapi.util.IconLoader
import com.intellij.psi.FileViewProvider
import javax.swing.Icon

object PasDili : Language("Pas") {
    private fun readResolve(): Any = PasDili

    override fun getDisplayName() = "Pas (Türkçe Rust)"
}

object PasSimgeleri {
    @JvmField
    val DOSYA: Icon = IconLoader.getIcon("/icons/pas.svg", PasSimgeleri::class.java)
}

object PasDosyaTuru : LanguageFileType(PasDili) {
    override fun getName() = "Pas"

    override fun getDescription() = "Pas (Türkçe Rust) dosyası"

    override fun getDefaultExtension() = "pas"

    override fun getIcon() = PasSimgeleri.DOSYA
}

class PasDosyasi(viewProvider: FileViewProvider) : PsiFileBase(viewProvider, PasDili) {
    override fun getFileType() = PasDosyaTuru

    override fun toString() = "Pas dosyası"
}
