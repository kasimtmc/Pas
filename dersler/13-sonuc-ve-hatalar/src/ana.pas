// Bazı işler başarısız olabilir: sıfıra bölmek, sayı olmayan bir metni
// sayıya çevirmek… Böyle işlevler `Sonuç` döndürür:
// başarılıysa `Tamam(değer)`, değilse `Hata(neden)`.
işlev böl(bölünen: i32, bölen: i32) -> Sonuç<i32, Metin> {
    eğer bölen == 0 {
        döndür Hata("sıfıra bölünemez".metne_çevir());
    }
    Tamam(bölünen / bölen)
}

// `?` işareti: sonuç `Hata` ise işlevden hemen o hatayla çıkar,
// `Tamam` ise içindeki değeri verir.
işlev hesapla(a: i32, b: i32, c: i32) -> Sonuç<i32, Metin> {
    yap ilk = böl(a, b)?;
    yap ikinci = böl(ilk, c)?;
    Tamam(ikinci)
}

/// Yazılanın geçerli bir yaş olup olmadığını denetler.
işlev yaşı_denetle(yazı: &str) -> Sonuç<u32, Metin> {
    yap yaş: u32 = eşle yazı.ayrıştır() {
        Tamam(yaş) => yaş,
        Hata(_) => döndür Hata(biçimle!("`{yazı}` bir sayı değil")),
    };
    eğer yaş > 130 {
        döndür Hata(biçimle!("{yaş} yaşında biri olamaz"));
    }
    Tamam(yaş)
}

işlev ana() {
    yazdır!("10 / 2 = {:?}", böl(10, 2));
    yazdır!("10 / 0 = {:?}", böl(10, 0));

    // `eşle` ile iki durumu da ele alırız.
    eşle böl(7, 0) {
        Tamam(sonuç) => yazdır!("Sonuç: {sonuç}"),
        Hata(neden) => yazdır!("Olmadı: {neden}"),
    }

    yazdır!("100 / 5 / 2 = {:?}", hesapla(100, 5, 2));
    yazdır!("100 / 0 / 2 = {:?}", hesapla(100, 0, 2));

    // `ayrıştır` metni sayıya çevirmeyi dener; o da `Sonuç` döndürür.
    her yazı içinde ["42", "kırk iki", " 7 "] {
        eşle yazı.kırp().ayrıştır::<i32>() {
            Tamam(sayı) => yazdır!("`{yazı}` bir sayı: {sayı}"),
            Hata(_) => yazdır!("`{yazı}` bir sayı değil"),
        }
    }

    // Geçerli bir yaş yazılana kadar sor.
    döngü {
        yap yazılan = oku!("Kaç yaşındasın? ");
        eşle yaşı_denetle(&yazılan) {
            Tamam(yaş) => {
                yazdır!("Gelecek yıl {} yaşında olacaksın!", yaş + 1);
                kır;
            }
            Hata(neden) => yazdır!("Hata: {neden}. Bir daha dene."),
        }
    }
}
