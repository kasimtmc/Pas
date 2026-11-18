package pas.eklenti

import com.intellij.openapi.util.SystemInfo
import com.intellij.util.system.CpuArch
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class PasKurulumTest {
    @Test
    fun ozetBulunur() {
        val ozetler = """
            1111  cargo-pas-x86_64-unknown-linux-musl
            ABCD  cargo-pas-x86_64-pc-windows-msvc.exe
            2222 *cargo-pas-aarch64-apple-darwin
        """.trimIndent()
        assertEquals("1111", PasKurulum.ozetiBul(ozetler, "cargo-pas-x86_64-unknown-linux-musl"))
        assertEquals("abcd", PasKurulum.ozetiBul(ozetler, "cargo-pas-x86_64-pc-windows-msvc.exe"))
        assertEquals("2222", PasKurulum.ozetiBul(ozetler, "cargo-pas-aarch64-apple-darwin"))
        assertNull(PasKurulum.ozetiBul(ozetler, "cargo-pas-x86_64-apple-darwin"))
    }

    @Test
    fun buSistemIcinDosyaAdi() {
        val ad = PasKurulum.dosyaAdi()
        if (SystemInfo.isLinux && CpuArch.isIntel64()) {
            assertEquals("cargo-pas-x86_64-unknown-linux-musl", ad)
        }
        // Yayın iş akışının ürettiği adlardan biri olmalı.
        val bilinenler = setOf(
            "cargo-pas-x86_64-unknown-linux-musl", "cargo-pas-aarch64-unknown-linux-musl",
            "cargo-pas-x86_64-pc-windows-msvc.exe", "cargo-pas-aarch64-apple-darwin", "cargo-pas-x86_64-apple-darwin",
        )
        assert(ad == null || ad in bilinenler) { "beklenmeyen dosya adı: $ad" }
    }
}
