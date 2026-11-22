işlev ana() {
    tanımla notlar: Liste<i32> = liste![72, 45, 90, 38, 100, 64, 81];
    yazdır!("Notlar: {:?}", notlar);

    // Kapanış (closure): adı olmayan küçük bir işlev.
    // `|sayı|` girdisidir, sağındaki de sonucu.
    tanımla iki_katı = |sayı: i32| sayı * 2;
    yazdır!("21'in iki katı: {}", iki_katı(21));

    // `dönüştür` her elemanı kapanıştan geçirir,
    // `biriktir` çıkanları yeni bir listede toplar.
    tanımla artmış: Liste<i32> = notlar.elemanlar().dönüştür(|not| not + 5).biriktir();
    yazdır!("Herkese 5 puan: {:?}", artmış);

    // `süz` yalnızca koşulu sağlayan elemanları bırakır.
    // `elemanlar` elemanları ödünç verir (`&72`); `*` ile değerin kendisini alırız.
    tanımla geçenler: Liste<i32> = notlar.elemanlar().dönüştür(|not| *not).süz(|not| *not >= 50).biriktir();
    yazdır!("Geçenler: {:?}", geçenler);

    // Hepsini art arda yazabiliriz: 50'nin altında kalanları bul ve 10 puan artır.
    tanımla kalanlar: Liste<i32> =
        notlar.elemanlar().dönüştür(|not| *not).süz(|not| *not < 50).dönüştür(|not| not + 10).biriktir();
    yazdır!("Kalanlara 10 puan: {:?}", kalanlar);

    // Toplam, en büyük ve en küçük
    tanımla toplam: i32 = notlar.elemanlar().toplam();
    yazdır!("Toplam: {toplam}");
    yazdır!("Ortalama: {:.1}", toplam olarak f64 / notlar.uzunluk() olarak f64);
    // Liste boş olabileceği için en büyük eleman `Belki` olarak gelir.
    yazdır!("En yüksek: {:?}", notlar.elemanlar().en_büyük());
    yazdır!("En düşük: {:?}", notlar.elemanlar().en_küçük());

    // Sıralamak listeyi değiştirir; asıl liste bozulmasın diye kopyasını sıralarız.
    tanımla değişken sıralı = notlar.kopyala();
    sıralı.sırala();
    sıralı.ters_çevir();

    // `numaralandır` her elemanı sırasıyla birlikte verir: (0, 100), (1, 90)…
    yazdır!("Sıralama:");
    için (sıra, not) içinde sıralı.elemanlar().numaralandır() {
        yazdır!("{}. {}", sıra + 1, not);
    }

    // `bul` koşulu sağlayan ilk elemanı bulur.
    tanımla isimler = liste!["Deniz", "Ali", "Ayşe", "Can"];
    yazdır!("A ile başlayan ilk isim: {:?}", isimler.elemanlar().bul(|isim| isim.ile_başlar_mı("A")));
    yazdır!("Z ile başlayan ilk isim: {:?}", isimler.elemanlar().bul(|isim| isim.ile_başlar_mı("Z")));
}
