# Ders 15: Kaplumbağa ile çizim

Elinde kalemi olan bir kaplumbağa düşün: "ileri yürü" dersen yürüdüğü yere
çizgi çizer, "sağa dön" dersen döner. Döngüler ve işlevlerle ona kareler,
yıldızlar, çiçekler çizdirebilirsin!

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

Çizim bu dizindeki `çizim.svg` dosyasına kaydedilir; onu tarayıcıda aç.
[Pas Oyun Alanı](https://kasimtmc.github.io/Pas/#ders=15-kaplumbaga)nda ise
çizim çıktının üstünde, kaplumbağa çizerken görünür.

## Neler öğreniyoruz?

**Kaplumbağa oluşturmak**: Kaplumbağa ortadan başlar ve yukarı bakar. Komut
verdikçe değişeceği için `değişken` olmalı:

```
tanımla değişken k = Kaplumbağa::yeni();
```

**Komutlar**:

| Komut | Ne yapar? |
|---|---|
| `k.ileri(100)` | baktığı yöne 100 adım yürür |
| `k.geri(50)` | geri geri 50 adım yürür |
| `k.sağa_dön(90)` | 90 derece sağa döner |
| `k.sola_dön(45)` | 45 derece sola döner |
| `k.kalemi_kaldır()` | kalemi kaldırır, çizmeden yürür |
| `k.kalemi_indir()` | kalemi indirir, yeniden çizer |
| `k.renk("kırmızı")` | kalemin rengini değiştirir |
| `k.kalınlık(4)` | kalemin kalınlığını değiştirir |
| `k.git(50, -20)` | bir noktaya gider; orta (0, 0) noktasıdır |
| `k.daire(30)` | olduğu yeri merkez alan bir daire çizer |

Renkler: siyah, beyaz, kırmızı, yeşil, mavi, sarı, turuncu, mor, pembe,
kahverengi, gri, lacivert, turkuaz. `"#ff8800"` gibi renk kodları da olur.

**Döngüyle şekiller**: Kare dört kez "ileri yürü, 90 derece dön"dür. Bir
şeklin çevresini dolaşan kaplumbağa toplam 360 derece döner; bu yüzden
altıgende her köşede `360 / 6` derece döneriz.

```
her _ için 0..4 {
    k.ileri(100);
    k.sağa_dön(90);
}
```

`_` "bu sayıyı kullanmayacağım" demektir.

**Kaplumbağayı işleve vermek**: İşlev kaplumbağayı yürütüp döndüreceği için
onu `&değişken` ile ödünç alır:

```
işlev kare(k: &değişken Kaplumbağa, kenar: i32) { … }

kare(&değişken k, 80);
```

## Kendin dene

1. Bir üçgen ve bir sekizgen çiz. Her köşede kaç derece dönmen gerekiyor?
2. `rastgele!` ile her turda rastgele bir renk seç ve 36 kare çizerek bir
   çiçek yap (her kareden sonra 10 derece dön).
3. Her adımda biraz daha uzun yürüyen bir spiral çiz:
   `k.ileri(i * 2)` ve `k.sağa_dön(91)`.
4. Kendi adının ilk harfini çiz!

---

Tebrikler, bütün dersleri bitirdin! Şimdi kendi projeni başlatma zamanı:
[dersler sayfasına](../README.md#kendi-projeni-başlat) dön.
