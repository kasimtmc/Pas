# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Gizli kelime

`gösterim` işlevini tamamla: kelimenin tahmin edilen harflerini göstersin, edilmeyenlerin yerine `_` koysun, harflerin arasına boşluk koysun. Örneğin `kedi` kelimesinde `e` ve `i` tahmin edildiyse `_ e _ i`.

İpucu: `kelime.karakterler()` harfleri tek tek verir; `sonuç.ekle(harf)` metnin sonuna harf ekler. Sondaki fazla boşluğu `sonuç.kırp().metne_çevir()` atar.

```pas başlangıç
işlev gösterim(kelime: &str, tahminler: &Liste<karakter>) -> Metin {
    tanımla değişken sonuç = Metin::yeni();
    // Kelimenin her harfine bak.
    sonuç
}

işlev ana() {
    yazdır!("{}", gösterim("kedi", &liste!['e', 'i']));
    yazdır!("{}", gösterim("elma", &liste![]));
    yazdır!("{}", gösterim("bulut", &liste!['b', 'u', 'l', 't']));
}
```

```çıktı
_ e _ i
_ _ _ _
b u l u t
```

```pas çözüm
işlev gösterim(kelime: &str, tahminler: &Liste<karakter>) -> Metin {
    tanımla değişken sonuç = Metin::yeni();
    için harf içinde kelime.karakterler() {
        eğer tahminler.içerir(&harf) {
            sonuç.ekle(harf);
        } değilse {
            sonuç.ekle('_');
        }
        sonuç.ekle(' ');
    }
    sonuç.kırp().metne_çevir()
}

işlev ana() {
    yazdır!("{}", gösterim("kedi", &liste!['e', 'i']));
    yazdır!("{}", gösterim("elma", &liste![]));
    yazdır!("{}", gösterim("bulut", &liste!['b', 'u', 'l', 't']));
}
```

## Bulundu mu?

`bulundu_mu` işlevini tamamla: kelimenin bütün harfleri tahmin edildiyse `doğru`, değilse `yanlış` döndürsün.

İpucu: Tahmin edilmemiş tek bir harf bulursan hemen `döndür yanlış;` de. Döngü biterse hepsi bulunmuştur.

```pas başlangıç
işlev bulundu_mu(kelime: &str, tahminler: &Liste<karakter>) -> mantıksal {
    // Burayı değiştir.
    yanlış
}

işlev ana() {
    yazdır!("{}", bulundu_mu("kedi", &liste!['k', 'e', 'd', 'i']));
    yazdır!("{}", bulundu_mu("kedi", &liste!['k', 'e']));
    yazdır!("{}", bulundu_mu("elma", &liste!['a', 'm', 'l', 'e', 'z']));
}
```

```çıktı
doğru
yanlış
doğru
```

```pas çözüm
işlev bulundu_mu(kelime: &str, tahminler: &Liste<karakter>) -> mantıksal {
    için harf içinde kelime.karakterler() {
        eğer !tahminler.içerir(&harf) {
            döndür yanlış;
        }
    }
    doğru
}

işlev ana() {
    yazdır!("{}", bulundu_mu("kedi", &liste!['k', 'e', 'd', 'i']));
    yazdır!("{}", bulundu_mu("kedi", &liste!['k', 'e']));
    yazdır!("{}", bulundu_mu("elma", &liste!['a', 'm', 'l', 'e', 'z']));
}
```

## Harf var mı?

`Harf: ` diye bir harf sor. Harf `kaplumbağa` kelimesinde varsa harfi ters tırnak içinde yazıp "var!", yoksa "yok." de:

```text
`a` var!
`z` yok.
```

İpucu: Tek bir harf okumak için `oku!("Harf: ", karakter)`, kelimede olup olmadığını anlamak için `kelime.içerir(harf)` kullan.

```pas başlangıç
işlev ana() {
    tanımla kelime = "kaplumbağa";
}
```

```girdi
a
```

```çıktı
Harf: a
`a` var!
```

```girdi
z
```

```çıktı
Harf: z
`z` yok.
```

```girdi
ğ
```

```çıktı
Harf: ğ
`ğ` var!
```

```pas çözüm
işlev ana() {
    tanımla kelime = "kaplumbağa";
    tanımla harf = oku!("Harf: ", karakter);
    eğer kelime.içerir(harf) {
        yazdır!("`{harf}` var!");
    } değilse {
        yazdır!("`{harf}` yok.");
    }
}
```
