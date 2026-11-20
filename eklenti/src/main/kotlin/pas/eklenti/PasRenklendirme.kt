package pas.eklenti

import com.intellij.openapi.editor.DefaultLanguageHighlighterColors as Varsayilan
import com.intellij.openapi.editor.colors.TextAttributesKey
import com.intellij.openapi.editor.colors.TextAttributesKey.createTextAttributesKey
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase
import com.intellij.openapi.fileTypes.SyntaxHighlighterFactory
import com.intellij.openapi.options.colors.AttributesDescriptor
import com.intellij.openapi.options.colors.ColorDescriptor
import com.intellij.openapi.options.colors.ColorSettingsPage
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.psi.tree.IElementType

object PasRenkleri {
    val ANAHTAR_SOZCUK = createTextAttributesKey("PAS_ANAHTAR_SOZCUK", Varsayilan.KEYWORD)
    val ISIM = createTextAttributesKey("PAS_ISIM", Varsayilan.IDENTIFIER)
    val MAKRO = createTextAttributesKey("PAS_MAKRO", Varsayilan.STATIC_METHOD)
    val HAZIR_TUR = createTextAttributesKey("PAS_HAZIR_TUR", Varsayilan.CLASS_NAME)
    val HAZIR_DEGER = createTextAttributesKey("PAS_HAZIR_DEGER", Varsayilan.CONSTANT)
    val YASAM_SURESI = createTextAttributesKey("PAS_YASAM_SURESI", Varsayilan.LABEL)
    val DIZGI = createTextAttributesKey("PAS_DIZGI", Varsayilan.STRING)
    val KARAKTER = createTextAttributesKey("PAS_KARAKTER", Varsayilan.STRING)
    val SAYI = createTextAttributesKey("PAS_SAYI", Varsayilan.NUMBER)
    val SATIR_YORUMU = createTextAttributesKey("PAS_SATIR_YORUMU", Varsayilan.LINE_COMMENT)
    val BLOK_YORUMU = createTextAttributesKey("PAS_BLOK_YORUMU", Varsayilan.BLOCK_COMMENT)
    val BELGE_YORUMU = createTextAttributesKey("PAS_BELGE_YORUMU", Varsayilan.DOC_COMMENT)
    val PARANTEZ = createTextAttributesKey("PAS_PARANTEZ", Varsayilan.PARENTHESES)
    val KOSELI = createTextAttributesKey("PAS_KOSELI", Varsayilan.BRACKETS)
    val SUSLU = createTextAttributesKey("PAS_SUSLU", Varsayilan.BRACES)
    val NOKTALI_VIRGUL = createTextAttributesKey("PAS_NOKTALI_VIRGUL", Varsayilan.SEMICOLON)
    val VIRGUL = createTextAttributesKey("PAS_VIRGUL", Varsayilan.COMMA)
    val NOKTA = createTextAttributesKey("PAS_NOKTA", Varsayilan.DOT)
    val ISLEC = createTextAttributesKey("PAS_ISLEC", Varsayilan.OPERATION_SIGN)
}

class PasRenklendirici : SyntaxHighlighterBase() {
    override fun getHighlightingLexer() = PasLexer()

    override fun getTokenHighlights(tur: IElementType): Array<TextAttributesKey> {
        val anahtar = when (tur) {
            PasSozcukler.ANAHTAR_SOZCUK -> PasRenkleri.ANAHTAR_SOZCUK
            PasSozcukler.ISIM -> PasRenkleri.ISIM
            PasSozcukler.MAKRO -> PasRenkleri.MAKRO
            PasSozcukler.HAZIR_TUR -> PasRenkleri.HAZIR_TUR
            PasSozcukler.HAZIR_DEGER -> PasRenkleri.HAZIR_DEGER
            PasSozcukler.YASAM_SURESI -> PasRenkleri.YASAM_SURESI
            PasSozcukler.DIZGI -> PasRenkleri.DIZGI
            PasSozcukler.KARAKTER -> PasRenkleri.KARAKTER
            PasSozcukler.SAYI -> PasRenkleri.SAYI
            PasSozcukler.SATIR_YORUMU -> PasRenkleri.SATIR_YORUMU
            PasSozcukler.BLOK_YORUMU -> PasRenkleri.BLOK_YORUMU
            PasSozcukler.BELGE_YORUMU -> PasRenkleri.BELGE_YORUMU
            PasSozcukler.SOL_PARANTEZ, PasSozcukler.SAG_PARANTEZ -> PasRenkleri.PARANTEZ
            PasSozcukler.SOL_KOSELI, PasSozcukler.SAG_KOSELI -> PasRenkleri.KOSELI
            PasSozcukler.SOL_SUSLU, PasSozcukler.SAG_SUSLU -> PasRenkleri.SUSLU
            PasSozcukler.NOKTALI_VIRGUL -> PasRenkleri.NOKTALI_VIRGUL
            PasSozcukler.VIRGUL -> PasRenkleri.VIRGUL
            PasSozcukler.NOKTA -> PasRenkleri.NOKTA
            PasSozcukler.ISLEC -> PasRenkleri.ISLEC
            else -> return emptyArray()
        }
        return arrayOf(anahtar)
    }
}

class PasRenklendiriciFabrikasi : SyntaxHighlighterFactory() {
    override fun getSyntaxHighlighter(project: Project?, virtualFile: VirtualFile?) = PasRenklendirici()
}

/** Ayarlar > Düzenleyici > Renk Şeması > Pas sayfası. */
class PasRenkAyarlari : ColorSettingsPage {
    override fun getIcon() = PasSimgeleri.DOSYA

    override fun getHighlighter() = PasRenklendirici()

    override fun getDemoText() = """
        /// Bir oyuncunun bilgileri.
        yapı Oyuncu {
            ad: Metin,
            puan: u32,
        }

        işlev en_iyisi<'a>(oyuncular: &'a [Oyuncu]) -> Belki<&'a Oyuncu> {
            oyuncular.elemanlar().max_by_key(|o| o.puan)
        }

        işlev ana() {
            tanımla değişken liste = liste![1, 2, 3]; // yorum
            liste.ekle(4);
            eşle liste.çıkar() {
                Var(son) => yazdır!("Son sayı: {son}"),
                Yok => yazdır!("Liste boş"),
            }
            /* blok yorum */
            tanımla harf = 'ş';
        }
    """.trimIndent()

    override fun getAdditionalHighlightingTagToDescriptorMap() = null

    override fun getAttributeDescriptors() = arrayOf(
        AttributesDescriptor("Anahtar sözcük", PasRenkleri.ANAHTAR_SOZCUK),
        AttributesDescriptor("İsim", PasRenkleri.ISIM),
        AttributesDescriptor("Makro", PasRenkleri.MAKRO),
        AttributesDescriptor("Hazır tür (Metin, Liste, …)", PasRenkleri.HAZIR_TUR),
        AttributesDescriptor("Hazır değer (Var, Yok, Tamam, Hata)", PasRenkleri.HAZIR_DEGER),
        AttributesDescriptor("Yaşam süresi", PasRenkleri.YASAM_SURESI),
        AttributesDescriptor("Dizgi", PasRenkleri.DIZGI),
        AttributesDescriptor("Karakter", PasRenkleri.KARAKTER),
        AttributesDescriptor("Sayı", PasRenkleri.SAYI),
        AttributesDescriptor("Yorumlar//Satır yorumu", PasRenkleri.SATIR_YORUMU),
        AttributesDescriptor("Yorumlar//Blok yorumu", PasRenkleri.BLOK_YORUMU),
        AttributesDescriptor("Yorumlar//Belge yorumu", PasRenkleri.BELGE_YORUMU),
        AttributesDescriptor("İşaretler//Parantez", PasRenkleri.PARANTEZ),
        AttributesDescriptor("İşaretler//Köşeli parantez", PasRenkleri.KOSELI),
        AttributesDescriptor("İşaretler//Süslü parantez", PasRenkleri.SUSLU),
        AttributesDescriptor("İşaretler//Noktalı virgül", PasRenkleri.NOKTALI_VIRGUL),
        AttributesDescriptor("İşaretler//Virgül", PasRenkleri.VIRGUL),
        AttributesDescriptor("İşaretler//Nokta", PasRenkleri.NOKTA),
        AttributesDescriptor("İşaretler//İşleç", PasRenkleri.ISLEC),
    )

    override fun getColorDescriptors(): Array<ColorDescriptor> = ColorDescriptor.EMPTY_ARRAY

    override fun getDisplayName() = "Pas"
}
