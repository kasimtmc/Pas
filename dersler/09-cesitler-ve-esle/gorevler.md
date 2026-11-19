# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Trafik ışığı

`ne_yapmalı` işlevini, ışığa göre `Dur`, `Hazırlan` ya da `Geç` döndürecek şekilde tamamla.

```pas başlangıç
çeşit Işık {
    Kırmızı,
    Sarı,
    Yeşil,
}

işlev ne_yapmalı(ışık: &Işık) -> &'durağan str {
    // Burayı `eşle` ile değiştir.
    ""
}

işlev ana() {
    tanımla ışıklar = [Işık::Kırmızı, Işık::Sarı, Işık::Yeşil];
    her ışık için &ışıklar {
        yazdır!("{}", ne_yapmalı(ışık));
    }
}
```

```çıktı
Dur
Hazırlan
Geç
```

```pas çözüm
çeşit Işık {
    Kırmızı,
    Sarı,
    Yeşil,
}

işlev ne_yapmalı(ışık: &Işık) -> &'durağan str {
    eşle ışık {
        Işık::Kırmızı => "Dur",
        Işık::Sarı => "Hazırlan",
        Işık::Yeşil => "Geç",
    }
}

işlev ana() {
    tanımla ışıklar = [Işık::Kırmızı, Işık::Sarı, Işık::Yeşil];
    her ışık için &ışıklar {
        yazdır!("{}", ne_yapmalı(ışık));
    }
}
```

## Mevsimler

Ay numarasından mevsimi bulan `mevsim` işlevini yaz: 12, 1, 2 `Kış`; 3, 4, 5 `İlkbahar`; 6, 7, 8 `Yaz`; 9, 10, 11 `Sonbahar`. Başka bir sayı için `Böyle bir ay yok` döndürsün.

İpucu: `eşle` içinde `|` "veya" demektir, `3..=5` 3'ten 5'e kadar demektir, `_` geri kalan her şeyi yakalar.

```pas başlangıç
işlev mevsim(ay: u32) -> &'durağan str {
    // Burayı değiştir.
    ""
}

işlev ana() {
    her ay için [1, 4, 7, 10, 13] {
        yazdır!("{ay}: {}", mevsim(ay));
    }
}
```

```çıktı
1: Kış
4: İlkbahar
7: Yaz
10: Sonbahar
13: Böyle bir ay yok
```

```pas çözüm
işlev mevsim(ay: u32) -> &'durağan str {
    eşle ay {
        12 | 1 | 2 => "Kış",
        3..=5 => "İlkbahar",
        6..=8 => "Yaz",
        9..=11 => "Sonbahar",
        _ => "Böyle bir ay yok",
    }
}

işlev ana() {
    her ay için [1, 4, 7, 10, 13] {
        yazdır!("{ay}: {}", mevsim(ay));
    }
}
```
