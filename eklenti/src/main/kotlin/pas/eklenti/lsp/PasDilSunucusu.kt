package pas.eklenti.lsp

import com.intellij.execution.ExecutionException
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspIntegrationProvider
import com.intellij.platform.lsp.api.ProjectWideLspClientDescriptor
import pas.eklenti.PasAraclari
import pas.eklenti.PasKurulum

/**
 * Bir `.pas` dosyası açılınca `cargo pas lsp` dil sunucusunu başlatır.
 * Sunucu rust-analyzer'ı kullanarak tanıma gitme (Ctrl+tık), kullanıldığı
 * yerleri bulma, hover belgeleri ve akıllı tamamlama sağlar.
 */
class PasDilSunucusuSaglayici : LspIntegrationProvider {
    override fun fileOpened(
        project: Project,
        file: VirtualFile,
        clientStarter: LspIntegrationProvider.LspClientStarter,
    ) {
        if (!PasDilSunucusu.destekleniyor(file)) return
        if (PasAraclari.cargoPas() == null) {
            PasKurulum.eksikseBildir(project)
        } else {
            clientStarter.ensureClientStarted(PasDilSunucusu(project))
        }
    }
}

class PasDilSunucusu(project: Project) : ProjectWideLspClientDescriptor(project, "Pas") {
    override fun isSupportedFile(file: VirtualFile) = destekleniyor(file)

    override fun createCommandLine(): GeneralCommandLine =
        PasAraclari.komutSatiri(project.basePath ?: ".", listOf("lsp"))
            ?: throw ExecutionException(PasAraclari.KURULUM_YARDIMI)

    companion object {
        fun destekleniyor(dosya: VirtualFile) = dosya.extension == "pas"
    }
}
