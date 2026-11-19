# Ders 13: Hatalarla başa çıkmak

Her iş yolunda gitmez: bir sayı sıfıra bölünemez, "kırk iki" yazısı sayıya
çevrilemez. Programın çökmek yerine böyle durumları ele alması için
`Sonuç` kullanırız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`Sonuç`** iki şeyden biridir:

- `Tamam(değer)`: iş başarıyla bitti, işte sonuç.
- `Hata(neden)`: bir sorun çıktı, işte nedeni.

```
işlev böl(bölünen: i32, bölen: i32) -> Sonuç<i32, Metin> {
    eğer bölen == 0 {
        döndür Hata("sıfıra bölünemez".metne_çevir());
    }
    Tamam(bölünen / bölen)
}
```

`Sonuç<i32, Metin>` şu demektir: başarılıysa bir `i32`, değilse bir `Metin`.

**İki durumu da ele almak**: `eşle` her iki durumu düşünmeni sağlar:

```
eşle böl(7, 0) {
    Tamam(sonuç) => yazdır!("Sonuç: {sonuç}"),
    Hata(neden) => yazdır!("Olmadı: {neden}"),
}
```

**`?` işareti**: `Sonuç` döndüren bir işlevin içinde, başka bir işlevin
sonucunun arkasına `?` koyarsan: sonuç `Hata` ise işlev hemen o hatayla
biter, `Tamam` ise içindeki değer alınır. Böylece her adımda `eşle`
yazmana gerek kalmaz.

```
yap ilk = böl(a, b)?;
```

**`ayrıştır()`**: metni sayıya çevirmeyi dener ve bir `Sonuç` verir.
`oku!("…", u32)` da arka planda bunu kullanır; yanlış yazılırsa tekrar sorar.

**`sarmalaç()` ne zaman?** `sarmalaç()` `Tamam` içindeki değeri alır ama
`Hata` gelirse programı durdurur. Kısa denemelerde işe yarar; gerçek
programlarda hatayı `eşle` ya da `?` ile ele almak daha iyidir.

## Kendin dene

1. `böl` işlevinin yanına `kök_al(sayı: f64) -> Sonuç<f64, Metin>` yaz:
   eksi sayılar için hata döndürsün, diğerleri için `sayı.karekök()`.
2. Kullanıcıdan iki sayı iste ve bölümlerini yazdır. Sayı olmayan bir şey
   ya da sıfır yazılırsa ne olduğunu Türkçe anlat.
3. `yaşı_denetle` işlevine eksi yaşları da ekle. İpucu: `u32` yerine `i32`
   kullanman gerekecek.

---

Sonraki ders: [Nitelikler](../14-nitelikler/README.md)
