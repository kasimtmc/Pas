# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Selamla

Kullanıcıya `Adın ne? ` diye sor ve cevabıyla selamla: `Merhaba Ayşe!`

İpucu: Soruyu `oku!("Adın ne? ")` ile sor.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```girdi
Ayşe
```

```çıktı
Adın ne? Ayşe
Merhaba Ayşe!
```

```girdi
Can
```

```çıktı
Adın ne? Can
Merhaba Can!
```

```pas çözüm
işlev ana() {
    tanımla ad = oku!("Adın ne? ");
    yazdır!("Merhaba {ad}!");
}
```

## Toplama makinesi

`Birinci sayı: ` ve `İkinci sayı: ` diye iki sayı sor, toplamlarını `Toplam: 12` biçiminde yazdır.

İpucu: Sayı okumak için türünü de yaz: `oku!("Birinci sayı: ", i32)`.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```girdi
5
7
```

```çıktı
Birinci sayı: 5
İkinci sayı: 7
Toplam: 12
```

```girdi
-3
10
```

```çıktı
Birinci sayı: -3
İkinci sayı: 10
Toplam: 7
```

```pas çözüm
işlev ana() {
    tanımla a = oku!("Birinci sayı: ", i32);
    tanımla b = oku!("İkinci sayı: ", i32);
    yazdır!("Toplam: {}", a + b);
}
```
