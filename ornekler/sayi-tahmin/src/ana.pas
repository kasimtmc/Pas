// Sayı tahmin oyunu: bilgisayar 1 ile 100 arasında bir sayı tutar,
// sen de onu bulmaya çalışırsın.

kullan std::time::{SystemTime, UNIX_EPOCH};

/// Saatin nanosaniyelerinden basit bir "rastgele" sayı üretir.
işlev rastgele_sayı(en_küçük: u32, en_büyük: u32) -> u32 {
    yap nanosaniye = SystemTime::now().duration_since(UNIX_EPOCH).sarmalaç().subsec_nanos();
    en_küçük + nanosaniye % (en_büyük - en_küçük + 1)
}

işlev ana() {
    yazdır!("Sayı tahmin oyununa hoş geldin!");
    yap ad = oku!("Adın ne? ");
    yap tutulan = rastgele_sayı(1, 100);
    yazdır!("Merhaba {ad}! 1 ile 100 arasında bir sayı tuttum. Bakalım bulabilecek misin?");

    yap değişken deneme = 0;
    döngü {
        yap tahmin = oku!("Tahminin: ", u32);
        deneme += 1;
        eğer tahmin < tutulan {
            yazdır!("Daha büyük bir sayı dene.");
        } değilse eğer tahmin > tutulan {
            yazdır!("Daha küçük bir sayı dene.");
        } değilse {
            yazdır!("Tebrikler {ad}! {deneme} denemede buldun.");
            kır;
        }
    }
}
