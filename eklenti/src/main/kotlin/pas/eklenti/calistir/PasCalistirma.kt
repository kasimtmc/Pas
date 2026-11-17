package pas.eklenti.calistir

import com.intellij.execution.ExecutionException
import com.intellij.execution.Executor
import com.intellij.execution.configurations.CommandLineState
import com.intellij.execution.configurations.ConfigurationFactory
import com.intellij.execution.configurations.ConfigurationTypeBase
import com.intellij.execution.configurations.ConfigurationTypeUtil
import com.intellij.execution.configurations.RunConfigurationBase
import com.intellij.execution.configurations.RunConfigurationOptions
import com.intellij.execution.configurations.RuntimeConfigurationError
import com.intellij.execution.process.ProcessHandler
import com.intellij.execution.process.ProcessHandlerFactory
import com.intellij.execution.process.ProcessTerminatedListener
import com.intellij.execution.runners.ExecutionEnvironment
import com.intellij.openapi.project.Project
import com.intellij.util.execution.ParametersListUtil
import pas.eklenti.PasAraclari
import pas.eklenti.PasSimgeleri
import java.io.File

class PasCalistirmaTuru : ConfigurationTypeBase(ID, "Pas", "Bir Pas programını derler ve çalıştırır", PasSimgeleri.DOSYA) {
    init {
        addFactory(PasYapilandirmaFabrikasi(this))
    }

    val fabrika: ConfigurationFactory get() = configurationFactories.single()

    companion object {
        const val ID = "PasCalistirma"

        fun ornek(): PasCalistirmaTuru = ConfigurationTypeUtil.findConfigurationType(PasCalistirmaTuru::class.java)
    }
}

class PasYapilandirmaFabrikasi(tur: PasCalistirmaTuru) : ConfigurationFactory(tur) {
    override fun getId() = PasCalistirmaTuru.ID

    override fun createTemplateConfiguration(project: Project) = PasYapilandirmasi(project, this, "Pas")

    override fun getOptionsClass() = PasYapilandirmaSecenekleri::class.java
}

class PasYapilandirmaSecenekleri : RunConfigurationOptions() {
    private val komutAlani = string(KOMUTLAR.first()).provideDelegate(this, "komut")
    private val dizinAlani = string("").provideDelegate(this, "calismaDizini")
    private val argumanAlani = string("").provideDelegate(this, "argumanlar")
    private val yayinAlani = property(false).provideDelegate(this, "yayin")

    var komut: String
        get() = komutAlani.getValue(this) ?: KOMUTLAR.first()
        set(deger) = komutAlani.setValue(this, deger)

    var calismaDizini: String
        get() = dizinAlani.getValue(this) ?: ""
        set(deger) = dizinAlani.setValue(this, deger)

    var argumanlar: String
        get() = argumanAlani.getValue(this) ?: ""
        set(deger) = argumanAlani.setValue(this, deger)

    var yayin: Boolean
        get() = yayinAlani.getValue(this)
        set(deger) = yayinAlani.setValue(this, deger)

    companion object {
        val KOMUTLAR = listOf("çalıştır", "derle", "denetle")
    }
}

class PasYapilandirmasi(project: Project, fabrika: ConfigurationFactory, ad: String) :
    RunConfigurationBase<PasYapilandirmaSecenekleri>(project, fabrika, ad) {

    public override fun getOptions() = super.getOptions() as PasYapilandirmaSecenekleri

    override fun getConfigurationEditor() = PasYapilandirmaDuzenleyicisi(project)

    override fun checkConfiguration() {
        val dizin = options.calismaDizini
        if (dizin.isBlank()) throw RuntimeConfigurationError("Proje dizini seçilmedi")
        if (!File(dizin, "Cargo.toml").isFile) {
            throw RuntimeConfigurationError("Seçilen dizinde `Cargo.toml` yok; bir Pas projesinin kök dizinini seç")
        }
    }

    override fun getState(executor: Executor, environment: ExecutionEnvironment) = PasCalistirmaDurumu(environment, this)
}

class PasCalistirmaDurumu(ortam: ExecutionEnvironment, private val yapilandirma: PasYapilandirmasi) :
    CommandLineState(ortam) {

    init {
        addConsoleFilters(PasKonumFiltresi(ortam.project, yapilandirma.options.calismaDizini))
    }

    override fun startProcess(): ProcessHandler {
        val secenekler = yapilandirma.options
        val argumanlar = buildList {
            add(secenekler.komut)
            if (secenekler.yayin) add("--yayın")
            val programArgumanlari = ParametersListUtil.parse(secenekler.argumanlar)
            if (programArgumanlari.isNotEmpty() && secenekler.komut == "çalıştır") {
                add("--")
                addAll(programArgumanlari)
            }
        }
        val komut = PasAraclari.komutSatiri(secenekler.calismaDizini, argumanlar, renkli = true)
            ?: throw ExecutionException(PasAraclari.KURULUM_YARDIMI)
        val isleyici = ProcessHandlerFactory.getInstance().createColoredProcessHandler(komut)
        ProcessTerminatedListener.attach(isleyici)
        return isleyici
    }
}
