# Proje: Hesap makinesi

İki sayı ve bir işlem sorar, sonucu yazar. Sıfıra bölmek ya da olmayan bir
işlem yazmak gibi hataları programı durdurmadan, Türkçe olarak anlatır.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Nasıl çalışıyor?

**Hesaplamak**: `hesapla` işlevi işleme `eşle` ile bakar ve bir `Sonuç`
döndürür: başarılıysa `Tamam(sonuç)`, değilse `Hata(neden)`.

```
eşle işlem {
    "+" => Tamam(a + b),
    "*" | "x" => Tamam(a * b),
    başka => Hata(biçimle!("`{başka}` bir işlem değil")),
}
```

`|` "veya" demektir: `*` de `x` de çarpar. En sondaki `başka` geri kalan her
şeyi yakalar ve yazılan işlemi o adla kullanmamızı sağlar.

**Sıfıra bölme**: Bölmede bölen `0.0` ise program çökmesin diye `Hata`
döndürülür.

**Ondalıklı sayılar**: Sayılar `f64` olarak okunur; `2,5` da `2.5` de olur.

**Bitirmek**: Hesaplar bir `döngü` içinde tekrar eder; işlem yerine `son`
yazılınca `kır` döngüden çıkar.

## Kendini geliştir

1. Kalan (`%`) ve üs alma işlemlerini ekle. İpucu: üs için
   `a.kuvvet(b olarak i32)`.
2. Bir önceki sonucu hatırla: birinci sayı yerine `son` yazılınca
   bir önceki sonuç kullanılsın.
3. Yapılan bütün hesapları bir listede tut ve program biterken yazdır.

---

Sonraki ders: [Proje: Desen çizici](../19-desen-cizici/README.md)
