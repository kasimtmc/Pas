package pas.eklenti.calistir

import com.intellij.execution.filters.Filter
import com.intellij.execution.filters.OpenFileHyperlinkInfo
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.LocalFileSystem
import java.io.File

/** Konsoldaki `--> src/ana.pas:3:5` konumlarını tıklanabilir yapar. */
class PasKonumFiltresi(private val project: Project, private val kok: String) : Filter {
    override fun applyFilter(satir: String, toplamUzunluk: Int): Filter.Result? {
        val eslesme = DESEN.find(satir) ?: return null
        val (yol, satirNo, sutunNo) = eslesme.destructured
        val dosya = LocalFileSystem.getInstance().findFileByIoFile(File(kok, yol)) ?: return null
        val satirBasi = toplamUzunluk - satir.length
        val bilgi = OpenFileHyperlinkInfo(project, dosya, satirNo.toInt() - 1, sutunNo.toInt() - 1)
        val yolAraligi = eslesme.groups[1]!!.range
        return Filter.Result(satirBasi + yolAraligi.first, satirBasi + eslesme.range.last + 1, bilgi)
    }

    companion object {
        val DESEN = Regex("""--> (\S+?\.(?:pas|rs)):(\d+):(\d+)""")
    }
}
