işlev ana() {
    // `liste!` ile bir liste oluştururuz. Değişecekse `değişken` olmalı.
    yap değişken meyveler = liste!["elma", "armut"];
    meyveler.ekle("muz");
    meyveler.ekle("çilek");
    yazdır!("Sepette {} meyve var.", meyveler.uzunluk());

    // Listedeki yerler 0'dan başlar: ilk eleman [0], ikincisi [1]...
    yazdır!("İlk meyve: {}", meyveler[0]);
    yazdır!("Son meyve: {}", meyveler[meyveler.uzunluk() - 1]);

    // Listedeki her eleman için:
    her meyve içinde &meyveler {
        yazdır!("- {meyve}");
    }

    // Boş bir listeyle başlayıp kullanıcının yazdıklarını ekleyelim.
    yap değişken notlar: Liste<u32> = Liste::yeni();
    her sıra içinde 1..=3 {
        yap not = oku!(biçimle!("{sıra}. sınav notun: "), u32);
        notlar.ekle(not);
    }

    yap değişken toplam = 0;
    yap değişken en_yüksek = 0;
    her not içinde &notlar {
        toplam += not;
        eğer *not > en_yüksek {
            en_yüksek = *not;
        }
    }
    yazdır!("Notların: {:?}", notlar);
    // `{:.1}` sayıyı virgülden sonra tek basamakla yazar.
    yazdır!("Ortalaman: {:.1}", toplam olarak f64 / notlar.uzunluk() olarak f64);
    yazdır!("En yüksek notun: {en_yüksek}");
}
