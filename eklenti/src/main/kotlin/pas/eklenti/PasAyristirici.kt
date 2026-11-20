package pas.eklenti

import com.intellij.extapi.psi.ASTWrapperPsiElement
import com.intellij.lang.ASTNode
import com.intellij.lang.ParserDefinition
import com.intellij.lang.PsiBuilder
import com.intellij.lang.PsiParser
import com.intellij.openapi.project.Project
import com.intellij.psi.FileViewProvider
import com.intellij.psi.PsiElement
import com.intellij.psi.TokenType
import com.intellij.psi.tree.IElementType
import com.intellij.psi.tree.IFileElementType
import com.intellij.psi.tree.TokenSet

class PasDugumTuru(ad: String) : IElementType(ad, PasDili)

object PasDugumler {
    /** `{ … }` */
    @JvmField val BLOK = PasDugumTuru("BLOK")
    /** `( … )` */
    @JvmField val PARANTEZ_GRUBU = PasDugumTuru("PARANTEZ_GRUBU")
    /** `[ … ]` */
    @JvmField val KOSELI_GRUBU = PasDugumTuru("KOSELI_GRUBU")

    @JvmField val ISLEV = PasDugumTuru("ISLEV")
    @JvmField val YAPI = PasDugumTuru("YAPI")
    @JvmField val CESIT = PasDugumTuru("CESIT")
    @JvmField val NITELIK = PasDugumTuru("NITELIK")
    @JvmField val UYGULA = PasDugumTuru("UYGULA")
    @JvmField val MODUL = PasDugumTuru("MODUL")
    @JvmField val SABIT = PasDugumTuru("SABIT")
    @JvmField val TUR_ADI = PasDugumTuru("TUR_ADI")

    @JvmField val GRUPLAR = TokenSet.create(BLOK, PARANTEZ_GRUBU, KOSELI_GRUBU)
    @JvmField val OGELER = TokenSet.create(ISLEV, YAPI, CESIT, NITELIK, UYGULA, MODUL, SABIT, TUR_ADI)

    /** Öğeyi başlatan anahtar sözcükler (Pas ve Rust yazımıyla). */
    val OGE_SOZCUKLERI: Map<String, IElementType> = mapOf(
        "işlev" to ISLEV, "fn" to ISLEV,
        "yapı" to YAPI, "struct" to YAPI,
        "çeşit" to CESIT, "enum" to CESIT,
        "nitelik" to NITELIK, "trait" to NITELIK,
        "uygula" to UYGULA, "impl" to UYGULA,
        "modül" to MODUL, "mod" to MODUL,
        "sabit" to SABIT, "const" to SABIT, "kalıcı" to SABIT, "static" to SABIT,
        "tür" to TUR_ADI, "type" to TUR_ADI,
    )

    /** Bir öğeden önce gelebilen niteleyiciler: `genel işlev`, `güvensiz işlev`, … */
    val NITELEYICILER = setOf(
        "genel", "pub", "güvensiz", "unsafe", "eşzamansız", "async", "sabit", "const", "harici", "extern",
    )
}

/**
 * Pas kaynağını blok ve öğe yapısına ayırır: süslü/normal/köşeli parantez
 * grupları ile `işlev`, `yapı`, `uygula` gibi öğeler. İfadeleri ayrıştırmaz
 * ve hiç hata üretmez; sözdizimi hatalarını derleyici Türkçe olarak bildirir.
 * Katlama, girinti ve yapı görünümü bu ağaca dayanır.
 */
class PasAyristirici : PsiParser {
    /** Bir öğenin başlayabilmesi için önceki anlamlı sözcük. */
    private var onceki: String? = null

    override fun parse(kok: IElementType, kurucu: PsiBuilder): ASTNode {
        onceki = null
        val isaret = kurucu.mark()
        icerik(kurucu, kapanis = null)
        while (!kurucu.eof()) ilerle(kurucu) // eşleşmeyen kapanışlar
        isaret.done(kok)
        return kurucu.treeBuilt
    }

    private fun ilerle(k: PsiBuilder) {
        onceki = k.tokenText
        k.advanceLexer()
    }

    /** Verilen kapanış işaretine (ya da dosya sonuna) kadar okur. */
    private fun icerik(k: PsiBuilder, kapanis: IElementType?) {
        while (!k.eof() && k.tokenType != kapanis) {
            if (k.tokenType in KAPANISLAR) {
                // Başka bir grubun kapanışı: o grup bitirsin.
                if (kapanis != null) return
                ilerle(k)
                continue
            }
            val oge = ogeTuru(k)
            if (oge != null) ogeOku(k, oge) else sozcukYaDaGrup(k)
        }
    }

    private fun ogeTuru(k: PsiBuilder): IElementType? {
        if (k.tokenType != PasSozcukler.ANAHTAR_SOZCUK) return null
        val tur = PasDugumler.OGE_SOZCUKLERI[k.tokenText] ?: return null
        val oncekiUygun = onceki == null || onceki in setOf(";", "{", "}", "]") || onceki in PasDugumler.NITELEYICILER
        if (!oncekiUygun) return null
        // `sabit işlev` ve `güvensiz uygula` gibi durumlarda `sabit` bir niteleyicidir.
        if (tur == PasDugumler.SABIT && k.lookAhead(1) == PasSozcukler.ANAHTAR_SOZCUK) return null
        return tur
    }

    /** Öğe: anahtar sözcükten `;` ya da gövde bloğunun sonuna kadar. */
    private fun ogeOku(k: PsiBuilder, tur: IElementType) {
        val isaret = k.mark()
        ilerle(k)
        while (!k.eof()) {
            when (k.tokenType) {
                PasSozcukler.NOKTALI_VIRGUL -> {
                    ilerle(k)
                    break
                }
                PasSozcukler.SOL_SUSLU -> {
                    grup(k, PasSozcukler.SAG_SUSLU, PasDugumler.BLOK)
                    break
                }
                in KAPANISLAR -> break
                else -> sozcukYaDaGrup(k)
            }
        }
        isaret.done(tur)
    }

    private fun sozcukYaDaGrup(k: PsiBuilder) {
        when (k.tokenType) {
            PasSozcukler.SOL_SUSLU -> grup(k, PasSozcukler.SAG_SUSLU, PasDugumler.BLOK)
            PasSozcukler.SOL_PARANTEZ -> grup(k, PasSozcukler.SAG_PARANTEZ, PasDugumler.PARANTEZ_GRUBU)
            PasSozcukler.SOL_KOSELI -> grup(k, PasSozcukler.SAG_KOSELI, PasDugumler.KOSELI_GRUBU)
            else -> ilerle(k)
        }
    }

    private fun grup(k: PsiBuilder, kapanis: IElementType, tur: IElementType) {
        // `genel(sandık) işlev` içindeki parantezden sonra da öğe başlayabilir.
        val genelSonrasi = onceki == "genel" || onceki == "pub"
        val isaret = k.mark()
        ilerle(k)
        icerik(k, kapanis)
        if (k.tokenType == kapanis) ilerle(k)
        isaret.done(tur)
        if (genelSonrasi && tur == PasDugumler.PARANTEZ_GRUBU) onceki = "genel"
    }

    companion object {
        private val KAPANISLAR = TokenSet.create(PasSozcukler.SAG_SUSLU, PasSozcukler.SAG_PARANTEZ, PasSozcukler.SAG_KOSELI)
    }
}

/** `işlev`, `yapı`, `uygula` gibi bir öğe. */
class PasOge(dugum: ASTNode) : ASTWrapperPsiElement(dugum) {
    /** Öğeyi başlatan anahtar sözcük (`işlev`, `fn`, …). */
    val anahtarSozcuk: String
        get() = node.getChildren(null).firstOrNull { PasDugumler.OGE_SOZCUKLERI.containsKey(it.text) }?.text ?: ""

    /**
     * Öğenin adı. `uygula` için başlığın tamamıdır (`Oyuncu`,
     * `Görüntüle her Oyuncu`); adı olmayan öğeler için `null`.
     */
    val ad: String?
        get() {
            val cocuklar = node.getChildren(null).toList()
            val sozcukSirasi = cocuklar.indexOfFirst { PasDugumler.OGE_SOZCUKLERI.containsKey(it.text) }
            if (sozcukSirasi < 0) return null
            val sonrasi = cocuklar.drop(sozcukSirasi + 1).filter { it.elementType != TokenType.WHITE_SPACE && it.elementType !in PasSozcukler.YORUMLAR }
            if (node.elementType == PasDugumler.UYGULA) {
                val baslik = sonrasi.takeWhile { it.elementType != PasDugumler.BLOK && it.elementType != PasSozcukler.NOKTALI_VIRGUL }
                if (baslik.isEmpty()) return null
                val metin = containingFile.text.substring(baslik.first().startOffset, baslik.last().textRange.endOffset)
                return metin.replace(Regex("\\s+"), " ")
            }
            // `kalıcı değişken SAYAC` gibi durumlarda `değişken` atlanır.
            return sonrasi.firstOrNull { it.elementType == PasSozcukler.ISIM }?.text
        }

    override fun getName() = ad

    /** Yapı görünümünde altında gösterilecek öğeler (`uygula` içindeki yöntemler gibi). */
    val icOgeler: List<PasOge>
        get() {
            if (node.elementType == PasDugumler.ISLEV) return emptyList()
            val blok = node.findChildByType(PasDugumler.BLOK) ?: return emptyList()
            return ogeleriTopla(blok)
        }

    companion object {
        /** Bir düğümün altındaki öğeleri, öğelerin içine inmeden toplar. */
        fun ogeleriTopla(dugum: ASTNode): List<PasOge> {
            val sonuc = mutableListOf<PasOge>()
            fun gez(d: ASTNode) {
                for (cocuk in d.getChildren(null)) {
                    val psi = cocuk.psi
                    if (psi is PasOge) sonuc += psi else if (cocuk.elementType in PasDugumler.GRUPLAR) gez(cocuk)
                }
            }
            gez(dugum)
            return sonuc
        }
    }
}

class PasAyristiriciTanimi : ParserDefinition {
    override fun createLexer(project: Project?) = PasLexer()

    override fun createParser(project: Project?) = PasAyristirici()

    override fun getFileNodeType() = DOSYA

    override fun getCommentTokens() = PasSozcukler.YORUMLAR

    override fun getStringLiteralElements() = PasSozcukler.DIZGILER

    override fun createElement(node: ASTNode): PsiElement =
        if (node.elementType in PasDugumler.OGELER) PasOge(node) else ASTWrapperPsiElement(node)

    override fun createFile(viewProvider: FileViewProvider) = PasDosyasi(viewProvider)

    companion object {
        val DOSYA = IFileElementType(PasDili)
    }
}
