// Pas'a hoş geldin! Bu örnek dilin temel parçalarını gösterir.

modül şekiller;

kullan şekiller::{Şekil, alan};

/// Bir hayvanın türü.
çeşit Hayvan {
    Kedi,
    Köpek,
    Kuş { kanat_açıklığı: f64 },
}

yapı Oyuncu {
    ad: Metin,
    puan: u32,
}

uygula Oyuncu {
    işlev yeni(ad: &str) -> Kendi {
        Oyuncu { ad: ad.metne_çevir(), puan: 0 }
    }

    işlev puan_ekle(&değişken kendi, miktar: u32) {
        kendi.puan += miktar;
    }
}

işlev ses(hayvan: &Hayvan) -> &'durağan str {
    eşle hayvan {
        Hayvan::Kedi => "miyav",
        Hayvan::Köpek => "hav hav",
        Hayvan::Kuş { kanat_açıklığı } eğer *kanat_açıklığı > 1.0 => "gak gak",
        Hayvan::Kuş { .. } => "cik cik",
    }
}

işlev böl(a: i32, b: i32) -> Sonuç<i32, Metin> {
    eğer b == 0 {
        döndür Hata("sıfıra bölünemez".metne_çevir());
    }
    Tamam(a / b)
}

işlev ana() {
    yazdır!("Merhaba dünya!");

    // Değişkenler
    olsun değişken oyuncu = Oyuncu::yeni("Ayşe");
    oyuncu.puan_ekle(10);
    oyuncu.puan_ekle(5);
    yazdır!("{} şu an {} puanda", oyuncu.ad, oyuncu.puan);

    // Listeler ve döngüler
    olsun değişken sayılar = liste![3, 1, 4, 1, 5];
    sayılar.ekle(9);
    olsun değişken toplam = 0;
    için sayı içinde sayılar.yinele() {
        toplam += sayı;
    }
    yazdır!("{} sayının toplamı: {toplam}", sayılar.uzunluk());

    olsun değişken geri_sayım = 3;
    sürece geri_sayım > 0 {
        yaz!("{geri_sayım}... ");
        geri_sayım -= 1;
    }
    yazdır!("Kalkış!");

    // Eşleme
    olsun hayvanlar = [Hayvan::Kedi, Hayvan::Köpek, Hayvan::Kuş { kanat_açıklığı: 0.3 }];
    için h içinde &hayvanlar {
        yazdır!("{}", ses(h));
    }

    // Belki ve Sonuç
    olsun ilk_çift: Belki<&i32> = sayılar.yinele().find(|s| *s % 2 == 0);
    eşle ilk_çift {
        Var(s) => yazdır!("İlk çift sayı: {s}"),
        Yok => yazdır!("Çift sayı yok"),
    }
    eşle böl(10, 0) {
        Tamam(sonuç) => yazdır!("Sonuç: {sonuç}"),
        Hata(mesaj) => yazdır!("Hata: {mesaj}"),
    }

    // Modüller
    için şekil içinde [Şekil::Kare(4.0), Şekil::Daire(1.0)] {
        yazdır!("Alan: {:.2}", alan(&şekil));
    }
}
