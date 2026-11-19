// `Sözlük` başka bir yerde tanımlı; `kullan` ile buraya alırız.
kullan std::collections::Sözlük;

işlev ana() {
    // Sözlük her anahtara bir değer bağlar: ülke → başkent.
    tanımla değişken başkentler = Sözlük::yeni();
    başkentler.yerleştir("Türkiye", "Ankara");
    başkentler.yerleştir("Japonya", "Tokyo");
    başkentler.yerleştir("Mısır", "Kahire");
    başkentler.yerleştir("Brezilya", "Brasilia");

    // Köşeli parantezle değere ulaşırız; anahtar yoksa program durur.
    yazdır!("Türkiye'nin başkenti: {}", başkentler["Türkiye"]);

    // `al` programı durdurmaz: anahtar yoksa `Yok` döner.
    yazdır!("Japonya: {:?}", başkentler.al("Japonya"));
    yazdır!("Fransa: {:?}", başkentler.al("Fransa"));

    eşle başkentler.al("Mısır") {
        Var(başkent) => yazdır!("Mısır'ın başkenti {başkent}."),
        Yok => yazdır!("Mısır'ı bilmiyorum."),
    }

    yazdır!("Brezilya var mı? {}", başkentler.anahtar_var_mı("Brezilya"));
    başkentler.sil("Brezilya");
    yazdır!("Sildikten sonra: {}", başkentler.anahtar_var_mı("Brezilya"));
    yazdır!("Sözlükte {} ülke var.", başkentler.uzunluk());

    // Aynı anahtara yeniden yerleştirirsek eski değerin yerine geçer.
    tanımla değişken puanlar = Sözlük::yeni();
    puanlar.yerleştir("Ayşe", 10);
    puanlar.yerleştir("Ayşe", 25);
    yazdır!("Ayşe'nin puanı: {}", puanlar["Ayşe"]);

    // Bir cümlede her kelimenin kaç kez geçtiğini sayalım.
    tanımla cümle = "bir berber bir berbere gel beraber bir berber dükkanı açalım demiş";
    tanımla değişken sayaç: Sözlük<&str, u32> = Sözlük::yeni();
    her kelime için cümle.böl(" ") {
        tanımla önceki = *sayaç.al(kelime).yoksa(&0);
        sayaç.yerleştir(kelime, önceki + 1);
    }

    // Sözlükteki anahtarların bir sırası yoktur. Düzenli yazdırmak için
    // anahtarları bir listeye alıp sıralarız.
    tanımla değişken kelimeler: Liste<_> = sayaç.anahtarlar().biriktir();
    kelimeler.sırala();
    her kelime için kelimeler {
        yazdır!("{kelime}: {}", sayaç[kelime]);
    }
}
