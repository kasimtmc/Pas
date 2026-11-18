// İşlevler bir işi yapan, adı olan kod parçalarıdır. Bir kere yazıp
// istediğimiz kadar kullanabiliriz.

/// Birini adıyla selamlar.
işlev selamla(ad: &str) {
    yazdır!("Merhaba {ad}, hoş geldin!");
}

/// İki sayıyı toplar ve sonucu döndürür.
işlev topla(a: i32, b: i32) -> i32 {
    // Son satırın sonuna `;` koymazsak o değer döndürülür.
    a + b
}

/// Sayı çift mi?
işlev çift_mi(sayı: i32) -> mantıksal {
    sayı % 2 == 0
}

/// İki sayıdan büyük olanı bulur.
işlev büyüğü(a: i32, b: i32) -> i32 {
    eğer a > b {
        // `döndür` işlevden hemen çıkar.
        döndür a;
    }
    b
}

/// Bir dikdörtgenin alanı
işlev alan(en: f64, boy: f64) -> f64 {
    en * boy
}

işlev ana() {
    selamla("Deniz");
    selamla("Ece");

    yap sonuç = topla(3, 4);
    yazdır!("3 + 4 = {sonuç}");

    eğer çift_mi(sonuç) {
        yazdır!("{sonuç} çift.");
    } değilse {
        yazdır!("{sonuç} tek.");
    }

    yazdır!("7 ile 12'den büyük olanı: {}", büyüğü(7, 12));
    yazdır!("3 x 4,5 metrelik odanın alanı: {} metrekare", alan(3.0, 4.5));
}
