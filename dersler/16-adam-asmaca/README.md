# Proje: Adam asmaca

Bilgisayar bir kelime tutar, sen harf harf tahmin edersin. Her yanlış
tahminde bir can gider; canın bitmeden kelimeyi bulabilecek misin?

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Nasıl çalışıyor?

**Kelimeyi seçmek**: Kelimeler bir dizide durur; `rastgele!` bir sıra
numarası seçer:

```
tanımla kelimeler = ["kedi", "elma", "bulut", "kaplumbağa"];
tanımla kelime = kelimeler[rastgele!(0, kelimeler.uzunluk() - 1)];
```

**Tahminleri tutmak**: Tahmin edilen harfler bir `Liste<karakter>` içinde
birikir. `oku!("Bir harf yaz: ", karakter)` tek bir harf okur.

**Kelimeyi göstermek**: `gösterim` işlevi kelimenin her harfine bakar;
tahmin edildiyse harfi, edilmediyse `_` yazar: `k _ d _`.
`kelime.karakterler()` harfleri tek tek verir.

**Oyunun sonu**: `bulundu_mu` bütün harflerin tahmin edilip edilmediğine
bakar. Kelime bulununca `döndür;` oyunu bitirir; canlar biterse
`sürece can > 0` döngüsü kendiliğinden biter.

Harfleri küçük harfle yaz: Pas için `K` ile `k` farklı harflerdir.

## Kendini geliştir

1. Listeye kendi kelimelerini ekle.
2. Yanlış tahmin edilen harfleri de göster: "Denediğin harfler: a, t, z".
3. Can her azaldığında küçük bir adam çiz: `yazdır!(" O ")`,
   `yazdır!("/|\\")`… İpucu: `eşle can { … }`.

---

Sonraki ders: [Proje: Bilgi yarışması](../17-bilgi-yarismasi/README.md)
