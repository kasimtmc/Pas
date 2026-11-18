package pas.eklenti

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.util.SystemInfo
import com.intellij.util.EnvironmentUtil
import com.intellij.openapi.vfs.VirtualFile
import java.io.File
import java.nio.charset.StandardCharsets

/** `cargo-pas` ve `cargo` programlarını bulur, komut satırlarını kurar. */
object PasAraclari {
    val cargoBin: File = System.getenv("CARGO_HOME")?.let { File(it, "bin") } ?: File(System.getProperty("user.home"), ".cargo/bin")

    /**
     * IDE masaüstünden açıldığında `PATH` içinde `~/.cargo/bin` olmayabilir;
     * bu yüzden oraya da bakılır.
     */
    fun bul(ad: String): File? {
        val dosyaAdi = if (SystemInfo.isWindows) "$ad.exe" else ad
        // Kabuğun ortamındaki PATH; masaüstünden açılan IDE'ninkinden daha doğrudur.
        val yol = EnvironmentUtil.getValue("PATH") ?: System.getenv("PATH").orEmpty()
        val dizinler = yol.split(File.pathSeparator).filter { it.isNotBlank() }.map(::File) + cargoBin
        return dizinler.map { File(it, dosyaAdi) }.firstOrNull { it.isFile && it.canExecute() }
    }

    fun cargoPas(): File? = bul("cargo-pas")

    const val KURULUM_YARDIMI =
        "`cargo-pas` bulunamadı. Kurmak için: cargo install --git https://github.com/kasimtmc/Pas cargo-pas"

    /** `cargo-pas <argümanlar>` komut satırı; araç yoksa `null`. */
    fun komutSatiri(calismaDizini: String, argumanlar: List<String>, renkli: Boolean = false): GeneralCommandLine? {
        val cargoPas = cargoPas() ?: return null
        return GeneralCommandLine(cargoPas.path)
            .withParameters(argumanlar)
            .withWorkingDirectory(File(calismaDizini).toPath())
            .withCharset(StandardCharsets.UTF_8)
            .withParentEnvironmentType(GeneralCommandLine.ParentEnvironmentType.CONSOLE)
            .apply {
                bul("cargo")?.let { withEnvironment("CARGO", it.path) }
                withEnvironment("CARGO_TERM_COLOR", if (renkli) "always" else "never")
            }
    }

    /** Dosyayı içeren Pas projesinin kökü (`Cargo.toml` dosyasının dizini). */
    fun projeKoku(dosya: VirtualFile): VirtualFile? {
        var dizin = if (dosya.isDirectory) dosya else dosya.parent
        while (dizin != null) {
            if (dizin.findChild("Cargo.toml") != null) return dizin
            dizin = dizin.parent
        }
        return null
    }
}
