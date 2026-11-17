package pas.eklenti

import com.intellij.codeInsight.completion.CompletionContributor
import com.intellij.codeInsight.completion.CompletionParameters
import com.intellij.codeInsight.completion.CompletionProvider
import com.intellij.codeInsight.completion.CompletionResultSet
import com.intellij.codeInsight.completion.CompletionType
import com.intellij.codeInsight.completion.InsertHandler
import com.intellij.codeInsight.completion.PrioritizedLookupElement
import com.intellij.codeInsight.lookup.LookupElement
import com.intellij.codeInsight.lookup.LookupElementBuilder
import com.intellij.patterns.PlatformPatterns
import com.intellij.util.ProcessingContext

/** Sözlükteki Türkçe sözcükleri ve dosyadaki isimleri önerir. */
class PasTamamlayici : CompletionContributor() {
    init {
        extend(CompletionType.BASIC, PlatformPatterns.psiElement().withLanguage(PasDili), Saglayici)
    }

    private object Saglayici : CompletionProvider<CompletionParameters>() {
        override fun addCompletions(
            parameters: CompletionParameters,
            context: ProcessingContext,
            result: CompletionResultSet,
        ) {
            val tur = parameters.position.node.elementType
            if (tur in PasSozcukler.YORUMLAR || tur == PasSozcukler.DIZGI || tur == PasSozcukler.KARAKTER) return

            for (girdi in Sozluk.girdiler) result.addElement(oge(girdi))

            val onerilenler = Sozluk.girdiler.mapTo(HashSet()) { it.pas }
            for (isim in dosyadakiIsimler(parameters.originalFile.text)) {
                if (onerilenler.add(isim)) result.addElement(LookupElementBuilder.create(isim))
            }
        }

        private fun oge(girdi: SozlukGirdisi): LookupElement {
            val makro = girdi.sinif == Sinif.MAKRO
            var oge = LookupElementBuilder.create(if (makro) "${girdi.pas}!" else girdi.pas)
                .withTypeText(if (makro) "${girdi.rust}!" else girdi.rust, true)
                .withTailText("  ${girdi.aciklama}", true)
                .withIcon(PasSimgeleri.DOSYA)
            oge = when (girdi.sinif) {
                Sinif.ANAHTAR_SOZCUK -> oge.bold()
                Sinif.MAKRO -> oge.withInsertHandler(parantezEkle(if (girdi.rust == "vec") "[]" else "()"))
                Sinif.YONTEM, Sinif.ISLEV -> oge.withInsertHandler(parantezEkle("()"))
                else -> oge
            }
            // Anahtar sözcükler dosyadaki isimlerden önce gelsin.
            return PrioritizedLookupElement.withPriority(oge, if (girdi.sinif == Sinif.ANAHTAR_SOZCUK) 2.0 else 1.0)
        }

        private fun parantezEkle(parantez: String) = InsertHandler<LookupElement> { baglam, _ ->
            val belge = baglam.document
            val ofset = baglam.tailOffset
            if (!belge.charsSequence.startsWith(parantez.substring(0, 1), ofset)) {
                belge.insertString(ofset, parantez)
            }
            baglam.editor.caretModel.moveToOffset(ofset + 1)
        }

        private fun dosyadakiIsimler(metin: CharSequence): Set<String> {
            val isimler = HashSet<String>()
            val lexer = PasLexer()
            lexer.start(metin)
            while (lexer.tokenType != null) {
                if (lexer.tokenType == PasSozcukler.ISIM) {
                    val isim = metin.subSequence(lexer.tokenStart, lexer.tokenEnd).toString()
                    // Tamamlama sırasında IDE'nin geçici olarak eklediği isim
                    if (!isim.contains("IntellijIdeaRulezzz")) isimler.add(isim)
                }
                lexer.advance()
            }
            return isimler
        }
    }
}
