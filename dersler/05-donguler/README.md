# Ders 5: Döngüler

Bir işi tekrar tekrar yapmak için döngü kullanırız. Bilgisayarlar tekrardan
hiç sıkılmaz!

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`her … için`**: Bir aralıktaki her sayı için çalışır.

```
her i için 1..=10 {
    yazdır!("{i}");
}
```

`1..=10` 1'den 10'a kadar (10 dahil), `1..10` ise 1'den 9'a kadar demektir.

**`sürece`**: Koşul doğru olduğu sürece tekrar eder.

```
sürece geri_sayım > 0 {
    geri_sayım -= 1;
}
```

**`döngü`**: Sonsuza kadar döner. Çıkmak için **`kır`** kullanırız.
**`devam`** ise o turun geri kalanını atlayıp bir sonraki tura geçer.

## Kendin dene

1. 1'den 100'e kadar olan sayıların toplamını bul.
2. 1'den 30'a kadar sayıları yazdır ama 3'e bölünenlerin yerine "Fış" yaz.
3. Kullanıcı doğru şifreyi ("pas123") yazana kadar şifre soran bir program
   yaz. İpucu: `döngü` ve `kır`.

---

Sonraki ders: [İşlevler](../06-islevler/README.md)
