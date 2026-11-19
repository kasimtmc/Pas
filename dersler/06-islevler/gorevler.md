# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Kare

`kare` işlevini, verilen sayının karesini döndürecek şekilde tamamla. `ana` işlevi 1'den 5'e kadar sayıların karelerini yazdırıyor.

İpucu: Son satırın sonuna `;` koymazsan o değer döndürülür.

```pas başlangıç
işlev kare(sayı: i32) -> i32 {
    // Burayı değiştir.
    0
}

işlev ana() {
    her i içinde 1..=5 {
        yazdır!("{}", kare(i));
    }
}
```

```çıktı
1
4
9
16
25
```

```pas çözüm
işlev kare(sayı: i32) -> i32 {
    sayı * sayı
}

işlev ana() {
    her i içinde 1..=5 {
        yazdır!("{}", kare(i));
    }
}
```

## En büyüğü

Üç sayının en büyüğünü döndüren `en_büyüğü` işlevini yaz.

İpucu: Bir `değişken` ile şimdiye kadarki en büyüğü tut, diğerlerini onunla karşılaştır.

```pas başlangıç
işlev en_büyüğü(a: i32, b: i32, c: i32) -> i32 {
    // Burayı değiştir.
    a
}

işlev ana() {
    yazdır!("{}", en_büyüğü(3, 9, 4));
    yazdır!("{}", en_büyüğü(10, 2, 7));
    yazdır!("{}", en_büyüğü(1, 5, 8));
}
```

```çıktı
9
10
8
```

```pas çözüm
işlev en_büyüğü(a: i32, b: i32, c: i32) -> i32 {
    yap değişken büyük = a;
    eğer b > büyük {
        büyük = b;
    }
    eğer c > büyük {
        büyük = c;
    }
    büyük
}

işlev ana() {
    yazdır!("{}", en_büyüğü(3, 9, 4));
    yazdır!("{}", en_büyüğü(10, 2, 7));
    yazdır!("{}", en_büyüğü(1, 5, 8));
}
```
