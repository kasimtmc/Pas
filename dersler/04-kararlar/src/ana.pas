işlev ana() {
    tanımla sayı = oku!("Bir sayı yaz: ", i32);

    // `eğer` bir koşul doğruysa çalışır, değilse `değilse` kısmı çalışır.
    eğer sayı > 0 {
        yazdır!("{sayı} pozitif bir sayı.");
    } değilse eğer sayı < 0 {
        yazdır!("{sayı} negatif bir sayı.");
    } değilse {
        yazdır!("Bu sıfır!");
    }

    // `%` bölümden kalanı verir. 2'ye bölünce kalan 0 ise sayı çifttir.
    // `==` "eşit mi?" diye sorar; `=` ise bir değer vermek içindir.
    eğer sayı % 2 == 0 {
        yazdır!("{sayı} çift.");
    } değilse {
        yazdır!("{sayı} tek.");
    }

    // `&&` "ve", `||` "veya" demektir.
    tanımla yaş = oku!("Kaç yaşındasın? ", u32);
    eğer yaş >= 7 && yaş <= 12 {
        yazdır!("7 ile 12 yaş arasındasın.");
    }
    eğer yaş < 7 || yaş > 12 {
        yazdır!("7 ile 12 yaş arasında değilsin.");
    }

    // `eğer` bir değer de verebilir.
    tanımla durum = eğer yaş < 18 { "çocuk" } değilse { "yetişkin" };
    yazdır!("Durum: {durum}");

    // `mantıksal` bir değer ya `doğru` ya da `yanlış` olur.
    tanımla hava_güzel = oku!("Hava güzel mi? (evet/hayır) ", mantıksal);
    eğer hava_güzel {
        yazdır!("Hadi dışarı çıkalım!");
    } değilse {
        yazdır!("Evde kitap okuyalım.");
    }
}
