package pas.eklenti.calistir

import com.intellij.execution.actions.ConfigurationContext
import com.intellij.execution.actions.LazyRunConfigurationProducer
import com.intellij.execution.lineMarker.ExecutorAction
import com.intellij.execution.lineMarker.RunLineMarkerContributor
import com.intellij.icons.AllIcons
import com.intellij.openapi.util.Ref
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.psi.PsiElement
import com.intellij.psi.util.PsiTreeUtil
import pas.eklenti.PasAraclari
import pas.eklenti.PasSozcukler

/** Bir `.pas` dosyasında sağ tık > Çalıştır için yapılandırma üretir. */
class PasCalistirmaUreticisi : LazyRunConfigurationProducer<PasYapilandirmasi>() {
    override fun getConfigurationFactory() = PasCalistirmaTuru.ornek().fabrika

    override fun setupConfigurationFromContext(
        yapilandirma: PasYapilandirmasi,
        baglam: ConfigurationContext,
        kaynak: Ref<PsiElement>,
    ): Boolean {
        val kok = projeKoku(baglam) ?: return false
        yapilandirma.options.calismaDizini = kok.path
        yapilandirma.options.komut = "çalıştır"
        yapilandirma.name = "Pas: ${kok.name}"
        return true
    }

    override fun isConfigurationFromContext(yapilandirma: PasYapilandirmasi, baglam: ConfigurationContext): Boolean {
        val kok = projeKoku(baglam) ?: return false
        return yapilandirma.options.calismaDizini == kok.path && yapilandirma.options.komut == "çalıştır"
    }

    private fun projeKoku(baglam: ConfigurationContext): VirtualFile? {
        val dosya = baglam.location?.virtualFile ?: return null
        if (dosya.extension != "pas") return null
        return PasAraclari.projeKoku(dosya)
    }
}

/** `işlev ana()` satırının yanına ▶ düğmesi koyar. */
class PasCalistirmaIsareti : RunLineMarkerContributor() {
    override fun getInfo(oge: PsiElement): Info? {
        if (oge.node.elementType != PasSozcukler.ISIM || oge.text != "ana") return null
        val onceki = PsiTreeUtil.prevVisibleLeaf(oge) ?: return null
        if (onceki.text != "işlev" && onceki.text != "fn") return null
        return Info(AllIcons.RunConfigurations.TestState.Run, ExecutorAction.getActions(0)) { "Çalıştır" }
    }
}
