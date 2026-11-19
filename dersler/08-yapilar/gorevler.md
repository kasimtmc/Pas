# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Dikdörtgen yapısı

`Dikdörtgen` yapısına alanını döndüren `alan` yöntemini yaz.

İpucu: Yapının alanlarına `kendi.en` ve `kendi.boy` ile ulaşırsın.

```pas başlangıç
yapı Dikdörtgen {
    en: u32,
    boy: u32,
}

uygula Dikdörtgen {
    işlev alan(&kendi) -> u32 {
        // Burayı değiştir.
        0
    }
}

işlev ana() {
    tanımla kapı = Dikdörtgen { en: 4, boy: 6 };
    yazdır!("Alan: {}", kapı.alan());
}
```

```çıktı
Alan: 24
```

```pas çözüm
yapı Dikdörtgen {
    en: u32,
    boy: u32,
}

uygula Dikdörtgen {
    işlev alan(&kendi) -> u32 {
        kendi.en * kendi.boy
    }
}

işlev ana() {
    tanımla kapı = Dikdörtgen { en: 4, boy: 6 };
    yazdır!("Alan: {}", kapı.alan());
}
```

## Doğum günü

`Kedi` yapısına yaşını bir artıran `doğum_günü` yöntemini ekle. Minnoş iki doğum günü kutladıktan sonra `Minnoş 3 yaşında` yazmalı.

İpucu: Yöntem kediyi değiştireceği için `&değişken kendi` almalı.

```pas başlangıç
yapı Kedi {
    ad: Metin,
    yaş: u32,
}

uygula Kedi {
    // `doğum_günü` yöntemini buraya yaz.
}

işlev ana() {
    tanımla değişken kedi = Kedi { ad: "Minnoş".metne_çevir(), yaş: 1 };
    // İki kez doğum günü kutla.
    yazdır!("{} {} yaşında", kedi.ad, kedi.yaş);
}
```

```çıktı
Minnoş 3 yaşında
```

```pas çözüm
yapı Kedi {
    ad: Metin,
    yaş: u32,
}

uygula Kedi {
    işlev doğum_günü(&değişken kendi) {
        kendi.yaş += 1;
    }
}

işlev ana() {
    tanımla değişken kedi = Kedi { ad: "Minnoş".metne_çevir(), yaş: 1 };
    kedi.doğum_günü();
    kedi.doğum_günü();
    yazdır!("{} {} yaşında", kedi.ad, kedi.yaş);
}
```
