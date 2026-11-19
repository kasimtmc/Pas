# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Toplam ve ortalama

Listedeki sayıların toplamını ve ortalamasını yazdır:

```
Toplam: 108
Ortalama: 18
```

İpucu: Ortalama, toplamın eleman sayısına bölümüdür. `sayılar.uzunluk() olarak i32` eleman sayısını tam sayıya çevirir.

```pas başlangıç
işlev ana() {
    yap sayılar = liste![4, 8, 15, 16, 23, 42];
}
```

```çıktı
Toplam: 108
Ortalama: 18
```

```pas çözüm
işlev ana() {
    yap sayılar = liste![4, 8, 15, 16, 23, 42];
    yap değişken toplam = 0;
    her sayı içinde &sayılar {
        toplam += sayı;
    }
    yazdır!("Toplam: {toplam}");
    yazdır!("Ortalama: {}", toplam / sayılar.uzunluk() olarak i32);
}
```

## Alışveriş listesi

Listeye `süt` ve `peynir` ekle, sonra her ürünü numarasıyla yazdır:

```
1. ekmek
2. yumurta
3. süt
4. peynir
```

İpucu: Numara için ayrı bir `değişken` sayaç tutabilirsin.

```pas başlangıç
işlev ana() {
    yap değişken alışveriş = liste!["ekmek", "yumurta"];
}
```

```çıktı
1. ekmek
2. yumurta
3. süt
4. peynir
```

```pas çözüm
işlev ana() {
    yap değişken alışveriş = liste!["ekmek", "yumurta"];
    alışveriş.ekle("süt");
    alışveriş.ekle("peynir");
    yap değişken sıra = 1;
    her ürün içinde &alışveriş {
        yazdır!("{sıra}. {ürün}");
        sıra += 1;
    }
}
```
