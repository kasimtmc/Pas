// Kaplumbağa kalemiyle yürüdüğü yere çizgi çizer. Ortadan başlar ve yukarı bakar.

/// Kenarları `kenar` uzunluğunda bir kare çizer.
işlev kare(k: &değişken Kaplumbağa, kenar: i32) {
    her _ içinde 0..4 {
        k.ileri(kenar);
        k.sağa_dön(90);
    }
}

/// Köşe sayısı verilen düzgün bir çokgen çizer: 3 üçgen, 6 altıgen…
işlev çokgen(k: &değişken Kaplumbağa, köşe: i32, kenar: i32) {
    her _ içinde 0..köşe {
        k.ileri(kenar);
        k.sağa_dön(360 / köşe);
    }
}

işlev ana() {
    yap değişken k = Kaplumbağa::yeni();

    // Bir kare
    kare(&değişken k, 80);

    // Kalemi kaldırıp çizmeden başka bir yere gidelim.
    k.kalemi_kaldır();
    k.git(-200, 0);
    k.kalemi_indir();

    // Beş köşeli yıldız: her köşede 144 derece dön.
    // Yıldız düz dursun diye önce sağa bakalım.
    k.sağa_dön(90);
    k.renk("turuncu");
    k.kalınlık(3);
    her _ içinde 0..5 {
        k.ileri(120);
        k.sağa_dön(144);
    }

    // Aynı yerde, her seferinde biraz dönerek çizilen altıgenler bir çiçek olur.
    k.kalemi_kaldır();
    k.git(200, 40);
    k.kalemi_indir();
    k.kalınlık(1);
    yap renkler = ["kırmızı", "mor", "mavi"];
    her i içinde 0..12 {
        k.renk(renkler[i % 3]);
        çokgen(&değişken k, 6, 30);
        k.sağa_dön(30);
    }

    // Ortasına sarı bir daire
    k.renk("sarı");
    k.kalınlık(4);
    k.daire(10);

    // Kaplumbağa son olarak aşağıda dursun.
    k.kalemi_kaldır();
    k.git(0, -120);
}
