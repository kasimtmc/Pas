işlev ana() {
    // `her … içinde` bir aralıktaki her sayı için bir kez çalışır.
    // 1..=5 demek 1, 2, 3, 4, 5 demektir.
    her tur içinde 1..=5 {
        yazdır!("{tur}. tur");
    }

    // Çarpım tablosu
    yap sayı = oku!("Hangi sayının çarpım tablosunu görmek istersin? ", u32);
    her i içinde 1..=10 {
        yazdır!("{sayı} x {i} = {}", sayı * i);
    }

    // `sürece`, koşul doğru olduğu sürece tekrar eder.
    yap değişken geri_sayım = 5;
    sürece geri_sayım > 0 {
        yaz!("{geri_sayım}... ");
        geri_sayım -= 1;
    }
    yazdır!("Kalkış!");

    // `döngü` sonsuza kadar döner; `kır` ile döngüden çıkarız.
    yap değişken toplam = 0;
    döngü {
        yap girilen = oku!("Bir sayı yaz (bitirmek için 0): ", i32);
        eğer girilen == 0 {
            kır;
        }
        toplam += girilen;
    }
    yazdır!("Yazdığın sayıların toplamı: {toplam}");
}
