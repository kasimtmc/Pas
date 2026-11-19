# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Çift sayılar

1'den 10'a kadar olan çift sayıları alt alta yazdır.

İpucu: `her sayı içinde 1..=10` 1'den 10'a kadar döner.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```çıktı
2
4
6
8
10
```

```pas çözüm
işlev ana() {
    her sayı içinde 1..=10 {
        eğer sayı % 2 == 0 {
            yazdır!("{sayı}");
        }
    }
}
```

## Çarpım tablosu

`Hangi sayı? ` diye bir sayı sor ve onun çarpım tablosunu yazdır:

```
3 x 1 = 3
3 x 2 = 6
…
3 x 10 = 30
```

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```girdi
3
```

```çıktı
Hangi sayı? 3
3 x 1 = 3
3 x 2 = 6
3 x 3 = 9
3 x 4 = 12
3 x 5 = 15
3 x 6 = 18
3 x 7 = 21
3 x 8 = 24
3 x 9 = 27
3 x 10 = 30
```

```pas çözüm
işlev ana() {
    yap sayı = oku!("Hangi sayı? ", i32);
    her i içinde 1..=10 {
        yazdır!("{sayı} x {i} = {}", sayı * i);
    }
}
```

## Geri sayım

`sürece` kullanarak 5'ten 1'e geri say, sonra `Kalkış!` yazdır.

İpucu: Her turda sayacı 1 azaltmayı unutma, yoksa döngü hiç bitmez.

```pas başlangıç
işlev ana() {
    yap değişken sayaç = 5;
}
```

```çıktı
5
4
3
2
1
Kalkış!
```

```pas çözüm
işlev ana() {
    yap değişken sayaç = 5;
    sürece sayaç > 0 {
        yazdır!("{sayaç}");
        sayaç -= 1;
    }
    yazdır!("Kalkış!");
}
```
