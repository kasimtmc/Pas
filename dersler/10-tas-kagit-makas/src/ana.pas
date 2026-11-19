// Taş, kâğıt, makas! Bilgisayara karşı oyna.
// Taş makası, makas kâğıdı, kâğıt taşı yener.

çeşit Hamle {
    Taş,
    Kağıt,
    Makas,
}

uygula Hamle {
    işlev ad(&kendi) -> &'durağan str {
        eşle kendi {
            Hamle::Taş => "taş",
            Hamle::Kağıt => "kâğıt",
            Hamle::Makas => "makas",
        }
    }

    /// Kullanıcının yazdığını bir hamleye çevirir.
    işlev metinden(metin: &str) -> Belki<Hamle> {
        eşle metin {
            "taş" | "t" => Var(Hamle::Taş),
            "kağıt" | "kâğıt" | "k" => Var(Hamle::Kağıt),
            "makas" | "m" => Var(Hamle::Makas),
            _ => Yok,
        }
    }

    işlev yener_mi(&kendi, öteki: &Hamle) -> mantıksal {
        eşle (kendi, öteki) {
            (Hamle::Taş, Hamle::Makas) | (Hamle::Makas, Hamle::Kağıt) | (Hamle::Kağıt, Hamle::Taş) => doğru,
            _ => yanlış,
        }
    }
}

/// Rastgele bir hamle seçer.
işlev bilgisayarın_hamlesi() -> Hamle {
    eşle rastgele!(0, 2) {
        0 => Hamle::Taş,
        1 => Hamle::Kağıt,
        _ => Hamle::Makas,
    }
}

işlev ana() {
    yazdır!("Taş, kâğıt, makas! Önce 3 tur kazanan oyunu alır.");
    tanımla değişken senin_puanın = 0;
    tanımla değişken benim_puanım = 0;

    sürece senin_puanın < 3 && benim_puanım < 3 {
        tanımla yazılan = oku!("Hamlen (taş / kâğıt / makas): ");
        tanımla senin = eşle Hamle::metinden(&yazılan) {
            Var(hamle) => hamle,
            Yok => {
                yazdır!("Bunu anlamadım. taş, kâğıt ya da makas yaz.");
                devam;
            }
        };
        tanımla benim = bilgisayarın_hamlesi();
        yazdır!("Sen: {}, ben: {}", senin.ad(), benim.ad());

        eğer senin.yener_mi(&benim) {
            senin_puanın += 1;
            yazdır!("Bu turu sen kazandın!");
        } değilse eğer benim.yener_mi(&senin) {
            benim_puanım += 1;
            yazdır!("Bu turu ben kazandım!");
        } değilse {
            yazdır!("Berabere!");
        }
        yazdır!("Skor: sen {senin_puanın}, ben {benim_puanım}");
        yazdır!();
    }

    eğer senin_puanın == 3 {
        yazdır!("Tebrikler, oyunu sen kazandın!");
    } değilse {
        yazdır!("Bu sefer ben kazandım. Bir daha oynayalım mı?");
    }
}
