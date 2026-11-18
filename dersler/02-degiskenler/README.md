# Ders 2: Değişkenler

Programlar bilgileri hatırlamak için *değişken* kullanır. Değişken, bir
değere verdiğimiz isimdir.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`yap`** ile bir değişken oluştururuz:

```
yap ad = "Elif";
yap yaş = 11;
```

**Yazdırırken** değişkenin adını süslü parantez içine yazarız:
`yazdır!("{ad} {yaş} yaşında.");`. Bir hesabın sonucunu yazdırmak için
boş `{}` koyup hesabı virgülden sonra yazarız:
`yazdır!("{}", yaş + 10);`

**`değişken`**: Pas'ta değişkenler kendiliğinden değiştirilemez. Değerini
sonradan değiştirmek istiyorsan başına `değişken` yaz:

```
yap değişken puan = 0;
puan += 5;    // puan = puan + 5 ile aynı
```

**Sayılar**: `11` gibi tam sayılar ve `1.45` gibi ondalıklı sayılar vardır.
Ondalıklı sayılar kodda **nokta** ile yazılır.

**İşlemler**: `+` toplama, `-` çıkarma, `*` çarpma, `/` bölme, `%` bölümden
kalan. Dikkat: iki tam sayıyı bölersen sonuç da tam sayı olur (`17 / 5` → `3`).

## Kendin dene

1. Kendi adını ve yaşını değişkenlere koyup bir cümle yazdır.
2. Bir `değişken` ile para biriktirme oyunu yap: 0 lira ile başla, üç kez
   para ekle ve her seferinde toplamı yazdır.
3. `yap` ile oluşturduğun (başında `değişken` olmayan) bir değeri değiştirmeye
   çalış. Pas ne diyor? Önerisini uygula.

---

Sonraki ders: [Soru sormak](../03-soru-sor/README.md)
