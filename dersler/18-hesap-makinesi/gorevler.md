# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Hesapla

`hesapla` işlevini tamamla: `+`, `-`, `*`, `/` işlemlerini yapsın. Sıfıra bölmede `sıfıra bölünemez` hatası döndürsün; bilinmeyen bir işlemde ise işlemi ters tırnak içinde yazarak şu hatayı döndürsün:

```text
`?` bir işlem değil
```

İpucu: `eşle işlem { "+" => Tamam(a + b), … }` ile başla. Geri kalan her şeyi yakalamak için en sona bir ad yaz (`başka => …`) ve hata metnini `biçimle!` ile oluştur.

```pas başlangıç
işlev hesapla(a: f64, işlem: &str, b: f64) -> Sonuç<f64, Metin> {
    // Burayı `eşle` ile değiştir.
    Hata("henüz yazılmadı".metne_çevir())
}

işlev ana() {
    için (a, işlem, b) içinde [(6.0, "+", 2.0), (5.0, "-", 8.0), (3.0, "*", 4.0), (1.0, "/", 0.0), (9.0, "?", 1.0)] {
        eşle hesapla(a, işlem, b) {
            Tamam(sonuç) => yazdır!("{a} {işlem} {b} = {sonuç}"),
            Hata(neden) => yazdır!("{a} {işlem} {b}: {neden}"),
        }
    }
}
```

```çıktı
6 + 2 = 8
5 - 8 = -3
3 * 4 = 12
1 / 0: sıfıra bölünemez
9 ? 1: `?` bir işlem değil
```

```pas çözüm
işlev hesapla(a: f64, işlem: &str, b: f64) -> Sonuç<f64, Metin> {
    eşle işlem {
        "+" => Tamam(a + b),
        "-" => Tamam(a - b),
        "*" => Tamam(a * b),
        "/" => {
            eğer b == 0.0 {
                Hata("sıfıra bölünemez".metne_çevir())
            } değilse {
                Tamam(a / b)
            }
        }
        başka => Hata(biçimle!("`{başka}` bir işlem değil")),
    }
}

işlev ana() {
    için (a, işlem, b) içinde [(6.0, "+", 2.0), (5.0, "-", 8.0), (3.0, "*", 4.0), (1.0, "/", 0.0), (9.0, "?", 1.0)] {
        eşle hesapla(a, işlem, b) {
            Tamam(sonuç) => yazdır!("{a} {işlem} {b} = {sonuç}"),
            Hata(neden) => yazdır!("{a} {işlem} {b}: {neden}"),
        }
    }
}
```

## Kalan

Hesap makinesine `%` (kalan) işlemini ekle: `7 % 3 = 1`. Sıfıra göre kalan da `sıfıra bölünemez` hatası versin.

İpucu: `/` koluna benzer bir `"%"` kolu ekle; kalanı `a % b` verir.

```pas başlangıç
işlev hesapla(a: f64, işlem: &str, b: f64) -> Sonuç<f64, Metin> {
    eşle işlem {
        "+" => Tamam(a + b),
        "-" => Tamam(a - b),
        "*" => Tamam(a * b),
        "/" => {
            eğer b == 0.0 {
                Hata("sıfıra bölünemez".metne_çevir())
            } değilse {
                Tamam(a / b)
            }
        }
        başka => Hata(biçimle!("`{başka}` bir işlem değil")),
    }
}

işlev ana() {
    için (a, işlem, b) içinde [(7.0, "%", 3.0), (10.0, "%", 5.0), (7.0, "%", 0.0)] {
        eşle hesapla(a, işlem, b) {
            Tamam(sonuç) => yazdır!("{a} {işlem} {b} = {sonuç}"),
            Hata(neden) => yazdır!("{a} {işlem} {b}: {neden}"),
        }
    }
}
```

```çıktı
7 % 3 = 1
10 % 5 = 0
7 % 0: sıfıra bölünemez
```

```pas çözüm
işlev hesapla(a: f64, işlem: &str, b: f64) -> Sonuç<f64, Metin> {
    eşle işlem {
        "+" => Tamam(a + b),
        "-" => Tamam(a - b),
        "*" => Tamam(a * b),
        "/" => {
            eğer b == 0.0 {
                Hata("sıfıra bölünemez".metne_çevir())
            } değilse {
                Tamam(a / b)
            }
        }
        "%" => {
            eğer b == 0.0 {
                Hata("sıfıra bölünemez".metne_çevir())
            } değilse {
                Tamam(a % b)
            }
        }
        başka => Hata(biçimle!("`{başka}` bir işlem değil")),
    }
}

işlev ana() {
    için (a, işlem, b) içinde [(7.0, "%", 3.0), (10.0, "%", 5.0), (7.0, "%", 0.0)] {
        eşle hesapla(a, işlem, b) {
            Tamam(sonuç) => yazdır!("{a} {işlem} {b} = {sonuç}"),
            Hata(neden) => yazdır!("{a} {işlem} {b}: {neden}"),
        }
    }
}
```
