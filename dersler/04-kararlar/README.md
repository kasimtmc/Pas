# Ders 4: Kararlar

Programlar duruma göre farklı şeyler yapabilir. Bunun için `eğer` kullanırız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`eğer` ve `değilse`**:

```
eğer sayı > 0 {
    yazdır!("pozitif");
} değilse eğer sayı < 0 {
    yazdır!("negatif");
} değilse {
    yazdır!("sıfır");
}
```

**Karşılaştırmalar**: `>` büyük, `<` küçük, `>=` büyük ya da eşit, `<=` küçük
ya da eşit, `==` eşit, `!=` eşit değil.

Dikkat: `=` bir değişkene değer vermek içindir, `==` ise "eşit mi?" diye
sorar. Karıştırırsan Pas sana söyler.

**`&&` (ve), `||` (veya)**: İki koşulu birleştirir.
`yaş >= 7 && yaş <= 12` "yaş 7'den büyük ya da eşit **ve** 12'den küçük ya da
eşit" demektir.

**`mantıksal`**: Değeri `doğru` ya da `yanlış` olan bir türdür.
`oku!("Hava güzel mi? ", mantıksal)` "evet", "hayır", "doğru", "yanlış"
yanıtlarını anlar.

## Kendin dene

1. Bir sayı sor; 10'dan büyükse "büyük", küçükse "küçük" yazdır.
2. Yaşı sor ve sinemadaki bilet fiyatını söyle: 12 yaş altına 50 lira,
   diğerlerine 80 lira.
3. Üç sayı sor ve en büyüğünü bul.

---

Sonraki ders: [Döngüler](../05-donguler/README.md)
