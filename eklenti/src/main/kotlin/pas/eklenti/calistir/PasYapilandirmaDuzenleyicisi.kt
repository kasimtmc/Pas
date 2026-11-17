package pas.eklenti.calistir

import com.intellij.openapi.fileChooser.FileChooserDescriptorFactory
import com.intellij.openapi.options.SettingsEditor
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.ComboBox
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.components.JBCheckBox
import com.intellij.ui.components.JBTextField
import com.intellij.ui.dsl.builder.AlignX
import com.intellij.ui.dsl.builder.panel
import javax.swing.JComponent

class PasYapilandirmaDuzenleyicisi(project: Project) : SettingsEditor<PasYapilandirmasi>() {
    private val komut = ComboBox(PasYapilandirmaSecenekleri.KOMUTLAR.toTypedArray())
    private val dizin = TextFieldWithBrowseButton().apply {
        addBrowseFolderListener(
            project,
            FileChooserDescriptorFactory.createSingleFolderDescriptor().withTitle("Pas Projesinin Dizini"),
        )
    }
    private val argumanlar = JBTextField()
    private val yayin = JBCheckBox("Hızlı çalışan (yayın) sürümü derle")

    override fun createEditor(): JComponent = panel {
        row("Komut:") { cell(komut) }
        row("Proje dizini:") { cell(dizin).align(AlignX.FILL) }
        row("Program argümanları:") { cell(argumanlar).align(AlignX.FILL) }
        row { cell(yayin) }
    }

    override fun resetEditorFrom(yapilandirma: PasYapilandirmasi) {
        val secenekler = yapilandirma.options
        komut.selectedItem = secenekler.komut
        dizin.text = secenekler.calismaDizini
        argumanlar.text = secenekler.argumanlar
        yayin.isSelected = secenekler.yayin
    }

    override fun applyEditorTo(yapilandirma: PasYapilandirmasi) {
        val secenekler = yapilandirma.options
        secenekler.komut = komut.selectedItem as? String ?: PasYapilandirmaSecenekleri.KOMUTLAR.first()
        secenekler.calismaDizini = dizin.text
        secenekler.argumanlar = argumanlar.text
        secenekler.yayin = yayin.isSelected
    }
}
