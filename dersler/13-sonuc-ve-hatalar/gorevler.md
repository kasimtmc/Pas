# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Güvenli bölme

`böl` işlevini tamamla: bölen 0 ise `Hata("sıfıra bölünemez")`, değilse `Tamam(bölüm)` döndürsün.

İpucu: Metni `Metin`e çevirmek için `"sıfıra bölünemez".metne_çevir()` kullan.

```pas başlangıç
işlev böl(a: i32, b: i32) -> Sonuç<i32, Metin> {
    // Burayı değiştir.
    Tamam(0)
}

işlev ana() {
    için (a, b) içinde [(10, 2), (7, 0), (9, 3)] {
        eşle böl(a, b) {
            Tamam(bölüm) => yazdır!("{a} / {b} = {bölüm}"),
            Hata(neden) => yazdır!("{a} / {b}: {neden}"),
        }
    }
}
```

```çıktı
10 / 2 = 5
7 / 0: sıfıra bölünemez
9 / 3 = 3
```

```pas çözüm
işlev böl(a: i32, b: i32) -> Sonuç<i32, Metin> {
    eğer b == 0 {
        döndür Hata("sıfıra bölünemez".metne_çevir());
    }
    Tamam(a / b)
}

işlev ana() {
    için (a, b) içinde [(10, 2), (7, 0), (9, 3)] {
        eşle böl(a, b) {
            Tamam(bölüm) => yazdır!("{a} / {b} = {bölüm}"),
            Hata(neden) => yazdır!("{a} / {b}: {neden}"),
        }
    }
}
```

## Sayı mı?

Her yazıyı sayıya çevirmeyi dene. Sayıysa iki katını, değilse `sayı değil` yazdır:

```text
12 → 24
on → sayı değil
7 → 14
```

İpucu: `yazı.ayrıştır::<i32>()` bir `Sonuç` verir.

```pas başlangıç
işlev ana() {
    için yazı içinde ["12", "on", "7"] {
    }
}
```

```çıktı
12 → 24
on → sayı değil
7 → 14
```

```pas çözüm
işlev ana() {
    için yazı içinde ["12", "on", "7"] {
        eşle yazı.ayrıştır::<i32>() {
            Tamam(sayı) => yazdır!("{yazı} → {}", sayı * 2),
            Hata(_) => yazdır!("{yazı} → sayı değil"),
        }
    }
}
```
