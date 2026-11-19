import org.jetbrains.intellij.platform.gradle.TestFrameworkType
import java.io.ByteArrayOutputStream
import javax.inject.Inject

plugins {
    id("org.jetbrains.kotlin.jvm") version "2.4.21"
    id("org.jetbrains.intellij.platform") version "2.20.1"
}

group = "pas"
version = "0.6.0"

repositories {
    mavenCentral()
    intellijPlatform {
        defaultRepositories()
    }
}

// Derleme ve testler için kullanılacak IDE. Varsayılan olarak Toolbox'ın
// kurduğu RustRover kullanılır; başka bir kurulum `-Ppas.ide.yolu=…` ile
// verilebilir. Hiçbiri yoksa RustRover indirilir.
val yerelIde: String? = providers.gradleProperty("pas.ide.yolu").orNull
    ?: File(System.getProperty("user.home"), ".local/share/JetBrains/Toolbox/apps/rustrover")
        .takeIf { it.resolve("product-info.json").isFile }?.path

dependencies {
    intellijPlatform {
        if (yerelIde != null) local(yerelIde) else rustRover("2026.3")
        testFramework(TestFrameworkType.Platform)
    }
    testImplementation("junit:junit:4.13.2")
    testImplementation("org.opentest4j:opentest4j:1.3.0")
}

kotlin {
    jvmToolchain(25)
}

intellijPlatform {
    buildSearchableOptions = false
    pluginConfiguration {
        ideaVersion {
            sinceBuild = "263"
        }
    }
    // `./gradlew publishPlugin` JetBrains Marketplace'e gönderir (bkz. yayin.yml).
    publishing {
        token = providers.environmentVariable("JETBRAINS_TOKEN")
    }
}

/** Sözlüğü `cargo pas sözlük --json` çıktısından Kotlin koduna çevirir. */
abstract class SozlukUret : DefaultTask() {
    @get:InputFile
    @get:PathSensitive(PathSensitivity.RELATIVE)
    abstract val sozlukKaynagi: RegularFileProperty

    @get:Internal
    abstract val calismaAlani: DirectoryProperty

    @get:OutputDirectory
    abstract val cikti: DirectoryProperty

    @get:Inject
    abstract val exec: ExecOperations

    @TaskAction
    fun uret() {
        val json = ByteArrayOutputStream()
        exec.exec {
            workingDir = calismaAlani.get().asFile
            commandLine("cargo", "run", "-q", "-p", "cargo-pas", "--", "pas", "sozluk", "--json")
            standardOutput = json
        }
        @Suppress("UNCHECKED_CAST")
        val girdiler = groovy.json.JsonSlurper().parseText(json.toString(Charsets.UTF_8)) as List<Map<String, String>>
        fun kt(s: String?) = "\"" + s!!.replace("\\", "\\\\").replace("\"", "\\\"").replace("$", "\\$") + "\""
        val satirlar = girdiler.joinToString("\n") { g ->
            "    SozlukGirdisi(${kt(g["pas"])}, ${kt(g["rust"])}, Sinif.${g["tur"]!!.uppercase()}, ${kt(g["aciklama"])}),"
        }
        val dosya = cikti.get().asFile.resolve("pas/eklenti/SozlukVerisi.kt")
        dosya.parentFile.mkdirs()
        dosya.writeText(
            "// Bu dosya `cargo pas sözlük --json` çıktısından üretilir; elle düzenleme.\n" +
                "package pas.eklenti\n\ninternal val SOZLUK_VERISI: List<SozlukGirdisi> = listOf(\n$satirlar\n)\n"
        )
    }
}

val sozlukUret = tasks.register<SozlukUret>("sozlukUret") {
    calismaAlani = layout.projectDirectory.dir("..")
    sozlukKaynagi = layout.projectDirectory.file("../crates/pas-cekirdek/src/sozluk.rs")
    cikti = layout.buildDirectory.dir("generated/sozluk")
}

sourceSets.main {
    kotlin.srcDir(sozlukUret)
}
