# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Tek mi, çift mi?

`Bir sayı: ` diye bir sayı sor. Sayı çiftse `10 çift`, tekse `7 tek` yazdır.

İpucu: `sayı % 2 == 0` ise sayı çifttir.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```girdi
10
```

```çıktı
Bir sayı: 10
10 çift
```

```girdi
7
```

```çıktı
Bir sayı: 7
7 tek
```

```pas çözüm
işlev ana() {
    yap sayı = oku!("Bir sayı: ", i32);
    eğer sayı % 2 == 0 {
        yazdır!("{sayı} çift");
    } değilse {
        yazdır!("{sayı} tek");
    }
}
```

## Geçtin mi?

`Notun: ` diye bir not sor. Not 50 ya da daha büyükse `Geçtin!`, değilse `Kaldın, bir dahaki sefere!` yazdır. 100'den büyük bir not yazılırsa `Böyle bir not yok.` yazdır.

İpucu: Önce 100'den büyük mü diye bak, sonra `değilse eğer` ile 50'yi denetle.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```girdi
75
```

```çıktı
Notun: 75
Geçtin!
```

```girdi
50
```

```çıktı
Notun: 50
Geçtin!
```

```girdi
30
```

```çıktı
Notun: 30
Kaldın, bir dahaki sefere!
```

```girdi
120
```

```çıktı
Notun: 120
Böyle bir not yok.
```

```pas çözüm
işlev ana() {
    yap not = oku!("Notun: ", u32);
    eğer not > 100 {
        yazdır!("Böyle bir not yok.");
    } değilse eğer not >= 50 {
        yazdır!("Geçtin!");
    } değilse {
        yazdır!("Kaldın, bir dahaki sefere!");
    }
}
```
