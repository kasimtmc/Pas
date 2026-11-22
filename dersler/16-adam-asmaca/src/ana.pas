// Adam asmaca: bilgisayar bir kelime tutar, sen harf harf tahmin edersin.
// Her yanlış tahminde bir can gider; canın bitmeden kelimeyi bul!

/// Kelimenin bulunan harflerini gösterir, bulunmayanların yerine `_` koyar: "k _ d _"
işlev gösterim(kelime: &str, tahminler: &Liste<karakter>) -> Metin {
    tanımla değişken sonuç = Metin::yeni();
    için harf içinde kelime.karakterler() {
        eğer tahminler.içerir(&harf) {
            sonuç.ekle(harf);
        } değilse {
            sonuç.ekle('_');
        }
        sonuç.ekle(' ');
    }
    sonuç.kırp().metne_çevir()
}

/// Kelimenin bütün harfleri tahmin edildi mi?
işlev bulundu_mu(kelime: &str, tahminler: &Liste<karakter>) -> mantıksal {
    için harf içinde kelime.karakterler() {
        eğer !tahminler.içerir(&harf) {
            döndür yanlış;
        }
    }
    doğru
}

işlev ana() {
    tanımla kelimeler = ["kedi", "elma", "bulut", "kaplumbağa", "gökkuşağı", "bilgisayar"];
    tanımla kelime = kelimeler[rastgele!(0, kelimeler.uzunluk() - 1)];
    tanımla değişken tahminler: Liste<karakter> = Liste::yeni();
    tanımla değişken can = 6;

    yazdır!("Adam asmaca! Bir kelime tuttum, harflerini bulmaya çalış.");
    sürece can > 0 {
        yazdır!();
        yazdır!("{}", gösterim(kelime, &tahminler));
        yazdır!("Kalan can: {can}");
        tanımla harf = oku!("Bir harf yaz: ", karakter);

        eğer tahminler.içerir(&harf) {
            yazdır!("Bu harfi zaten denedin.");
            devam;
        }
        tahminler.ekle(harf);

        eğer kelime.içerir(harf) {
            yazdır!("Evet, `{harf}` var!");
        } değilse {
            can -= 1;
            yazdır!("Hayır, `{harf}` yok. Bir can gitti.");
        }

        eğer bulundu_mu(kelime, &tahminler) {
            yazdır!();
            yazdır!("Tebrikler! Kelimeyi buldun: {kelime}");
            döndür;
        }
    }
    yazdır!();
    yazdır!("Canın bitti! Kelime şuydu: {kelime}");
}
