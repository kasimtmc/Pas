package pas.eklenti

import com.intellij.ide.BrowserUtil
import com.intellij.notification.Notification
import com.intellij.notification.NotificationAction
import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.progress.ProgressIndicator
import com.intellij.openapi.progress.Task
import com.intellij.openapi.project.Project
import com.intellij.openapi.util.SystemInfo
import com.intellij.util.io.HttpRequests
import com.intellij.util.system.CpuArch
import java.io.File
import java.nio.file.Files
import java.nio.file.StandardCopyOption
import java.security.MessageDigest
import java.util.concurrent.atomic.AtomicBoolean

/**
 * `cargo-pas` ya da Rust eksikse kullanıcıyı bilgilendirir; `cargo-pas`'ı
 * GitHub'daki son yayından indirip kurabilir.
 */
object PasKurulum {
    private const val YAYIN_ADRESI = "https://github.com/kasimtmc/Pas/releases/latest/download"
    private val bildirildi = AtomicBoolean(false)

    /** Bu sistem için yayındaki dosyanın adı; desteklenmeyen sistemde `null`. */
    fun dosyaAdi(): String? {
        val islemci = when {
            CpuArch.isArm64() -> "aarch64"
            CpuArch.isIntel64() -> "x86_64"
            else -> return null
        }
        return when {
            SystemInfo.isWindows && islemci == "x86_64" -> "cargo-pas-x86_64-pc-windows-msvc.exe"
            SystemInfo.isMac -> "cargo-pas-$islemci-apple-darwin"
            SystemInfo.isLinux -> "cargo-pas-$islemci-unknown-linux-musl"
            else -> null
        }
    }

    /** `SHA256SUMS` içeriğinden bir dosyanın özetini bulur. */
    fun ozetiBul(ozetler: String, dosya: String): String? =
        ozetler.lineSequence()
            .map { it.trim().split(Regex("\\s+")) }
            .firstOrNull { it.size == 2 && it[1].removePrefix("*") == dosya }
            ?.get(0)?.lowercase()

    /** Araçlardan biri eksikse oturum başına bir kez bildirim gösterir. */
    fun eksikseBildir(project: Project?) {
        if (PasAraclari.cargoPas() != null || !bildirildi.compareAndSet(false, true)) return
        val grup = NotificationGroupManager.getInstance().getNotificationGroup("Pas")
        val bildirim = if (PasAraclari.bul("cargo") == null) {
            grup.createNotification(
                "Pas için Rust gerekli",
                "Pas programlarını derlemek için önce Rust'ı kurman gerekiyor.",
                NotificationType.WARNING,
            ).addAction(NotificationAction.createSimpleExpiring("Rust'ı kur (rustup.rs)") {
                BrowserUtil.browse("https://rustup.rs")
            })
        } else {
            grup.createNotification(
                "cargo-pas bulunamadı",
                "Pas dosyalarını çalıştırmak ve hataları görmek için <code>cargo-pas</code> gerekli.",
                NotificationType.WARNING,
            ).addAction(NotificationAction.createSimpleExpiring("İndir ve kur") { indirVeKur(project) })
        }
        bildirim.notify(project)
    }

    fun indirVeKur(project: Project?) {
        object : Task.Backgroundable(project, "cargo-pas indiriliyor", true) {
            override fun run(gosterge: ProgressIndicator) {
                val sonuc = runCatching { kur(gosterge) }
                val grup = NotificationGroupManager.getInstance().getNotificationGroup("Pas")
                val bildirim: Notification = sonuc.fold(
                    onSuccess = { yol ->
                        grup.createNotification("cargo-pas kuruldu", yol.path, NotificationType.INFORMATION)
                    },
                    onFailure = { hata ->
                        grup.createNotification(
                            "cargo-pas kurulamadı",
                            "${hata.message}<br>${PasAraclari.KURULUM_YARDIMI}",
                            NotificationType.ERROR,
                        )
                    },
                )
                bildirim.notify(project)
            }
        }.queue()
    }

    private fun kur(gosterge: ProgressIndicator): File {
        val dosya = dosyaAdi() ?: error("Bu sistem için hazır cargo-pas yok")
        val ozetler = HttpRequests.request("$YAYIN_ADRESI/SHA256SUMS").readString(gosterge)
        val beklenen = ozetiBul(ozetler, dosya) ?: error("$dosya için özet bulunamadı")

        val gecici = Files.createTempFile("cargo-pas", null).toFile()
        try {
            gosterge.text = "$dosya indiriliyor"
            HttpRequests.request("$YAYIN_ADRESI/$dosya").saveToFile(gecici, gosterge)
            val ozet = MessageDigest.getInstance("SHA-256").digest(gecici.readBytes()).joinToString("") { "%02x".format(it) }
            check(ozet == beklenen) { "İndirilen dosyanın özeti tutmuyor; dosya bozuk olabilir" }

            val hedef = File(PasAraclari.cargoBin, if (SystemInfo.isWindows) "cargo-pas.exe" else "cargo-pas")
            hedef.parentFile.mkdirs()
            Files.move(gecici.toPath(), hedef.toPath(), StandardCopyOption.REPLACE_EXISTING)
            hedef.setExecutable(true)
            return hedef
        } finally {
            gecici.delete()
        }
    }
}
