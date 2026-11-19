# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Telefon rehberi

Rehbere Ali, Can ve Elif'i numaralarıyla ekle. Sonra Can'ın numarasını yazdır; rehberde olmayan Zeynep için `Zeynep rehberde yok` yazdır.

İpucu: `rehber.al(ad)` bir `Belki` verir; `eşle` ile `Var` ve `Yok` durumlarını ayır.

```pas başlangıç
kullan std::collections::Sözlük;

işlev ana() {
    tanımla değişken rehber = Sözlük::yeni();
    rehber.yerleştir("Ali", "555 11 22");
    // Can: 555 33 44, Elif: 555 55 66
}
```

```çıktı
Can: 555 33 44
Zeynep rehberde yok
```

```pas çözüm
kullan std::collections::Sözlük;

işlev ana() {
    tanımla değişken rehber = Sözlük::yeni();
    rehber.yerleştir("Ali", "555 11 22");
    rehber.yerleştir("Can", "555 33 44");
    rehber.yerleştir("Elif", "555 55 66");
    her ad için ["Can", "Zeynep"] {
        eşle rehber.al(ad) {
            Var(numara) => yazdır!("{ad}: {numara}"),
            Yok => yazdır!("{ad} rehberde yok"),
        }
    }
}
```

## Harf sayacı

`elma armut` metnindeki her harfin kaç kez geçtiğini say ve harfleri alfabe sırasıyla yazdır. Boşluğu sayma.

```
a: 2
e: 1
l: 1
m: 2
r: 1
t: 1
u: 1
```

İpucu: Ders 12'deki kelime sayacına benzer; `metin.karakterler()` her harfi tek tek verir.

```pas başlangıç
kullan std::collections::Sözlük;

işlev ana() {
    tanımla metin = "elma armut";
    tanımla değişken sayaç: Sözlük<karakter, u32> = Sözlük::yeni();
}
```

```çıktı
a: 2
e: 1
l: 1
m: 2
r: 1
t: 1
u: 1
```

```pas çözüm
kullan std::collections::Sözlük;

işlev ana() {
    tanımla metin = "elma armut";
    tanımla değişken sayaç: Sözlük<karakter, u32> = Sözlük::yeni();
    her harf için metin.karakterler() {
        eğer harf != ' ' {
            tanımla önceki = *sayaç.al(&harf).yoksa(&0);
            sayaç.yerleştir(harf, önceki + 1);
        }
    }
    tanımla değişken harfler: Liste<_> = sayaç.anahtarlar().biriktir();
    harfler.sırala();
    her harf için harfler {
        yazdır!("{harf}: {}", sayaç[harf]);
    }
}
```
