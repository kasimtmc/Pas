// Desen çizici: basit şekilleri döndürerek ve tekrarlayarak güzel desenler çizeriz.

/// Köşe sayısı ve kenar uzunluğu verilen düzgün bir çokgen çizer.
işlev çokgen(k: &değişken Kaplumbağa, köşe: u32, kenar: f64) {
    için _ içinde 0..köşe {
        k.ileri(kenar);
        k.sağa_dön(360.0 / köşe olarak f64);
    }
}

/// Bir şekli her seferinde biraz döndürerek tekrar tekrar çizer.
işlev çiçek(k: &değişken Kaplumbağa, köşe: u32, kenar: f64, yaprak: u32) {
    için _ içinde 0..yaprak {
        çokgen(k, köşe, kenar);
        k.sağa_dön(360.0 / yaprak olarak f64);
    }
}

/// Kaplumbağayı çizmeden bir noktaya götürür.
işlev git(k: &değişken Kaplumbağa, x: f64, y: f64) {
    k.kalemi_kaldır();
    k.git(x, y);
    k.kalemi_indir();
}

işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
    tanımla renkler = ["kırmızı", "turuncu", "sarı", "yeşil", "mavi", "mor"];

    // Altıgen çiçek: her yaprak başka renkte
    git(&değişken k, -170.0, 0.0);
    için i içinde 0..12 {
        k.renk(renkler[i % renkler.uzunluk()]);
        çokgen(&değişken k, 6, 30.0);
        k.sağa_dön(30);
    }

    // Kare çiçek
    git(&değişken k, 0.0, 0.0);
    k.renk("lacivert");
    çiçek(&değişken k, 4, 60.0, 18);

    // Büyüyen spiral
    git(&değişken k, 180.0, 0.0);
    k.renk("turkuaz");
    için i içinde 1..=40 {
        k.ileri(i * 3);
        k.sağa_dön(91);
    }
}
