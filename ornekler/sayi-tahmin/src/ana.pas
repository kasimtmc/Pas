// Sayı tahmin oyunu: bilgisayar 1 ile 100 arasında bir sayı tutar,
// sen de onu bulmaya çalışırsın.

işlev ana() {
    yazdır!("Sayı tahmin oyununa hoş geldin!");
    yap ad = oku!("Adın ne? ");
    yap tutulan: u32 = rastgele!(1, 100);
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
