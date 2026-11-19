# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Kareler listesi

`dönüştür` ve `biriktir` ile sayıların karelerinden yeni bir liste yap ve yazdır: `[1, 4, 9, 16, 25, 36]`

İpucu: `tanımla kareler: Liste<i32> = sayılar.yinele().dönüştür(|s| s * s).biriktir();`

```pas başlangıç
işlev ana() {
    tanımla sayılar = liste![1, 2, 3, 4, 5, 6];
}
```

```çıktı
[1, 4, 9, 16, 25, 36]
```

```pas çözüm
işlev ana() {
    tanımla sayılar = liste![1, 2, 3, 4, 5, 6];
    tanımla kareler: Liste<i32> = sayılar.yinele().dönüştür(|s| s * s).biriktir();
    yazdır!("{:?}", kareler);
}
```

## Uzun isimler

`süz` ile dört harften uzun isimleri seç ve yazdır: `["Deniz", "Mehmet"]`

İpucu: `isim.uzunluk() > 4` uzun isimleri seçer.

```pas başlangıç
işlev ana() {
    tanımla isimler = liste!["Ali", "Deniz", "Can", "Elif", "Mehmet"];
}
```

```çıktı
["Deniz", "Mehmet"]
```

```pas çözüm
işlev ana() {
    tanımla isimler = liste!["Ali", "Deniz", "Can", "Elif", "Mehmet"];
    tanımla uzunlar: Liste<&&str> = isimler.yinele().süz(|isim| isim.uzunluk() > 4).biriktir();
    yazdır!("{:?}", uzunlar);
}
```
