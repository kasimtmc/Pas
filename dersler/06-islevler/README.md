# Ders 6: İşlevler

İşlev, bir işi yapan ve adı olan kod parçasıdır. Bir kere yazıp istediğin
kadar kullanabilirsin. `ana` da bir işlevdir!

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**İşlev tanımlamak**:

```
işlev selamla(ad: &str) {
    yazdır!("Merhaba {ad}!");
}
```

Parantez içindekiler işlevin *parametreleridir*: işleve dışarıdan verilen
bilgiler. Her parametrenin türünü yazarız. `&str` bir yazı demektir.

**Değer döndürmek**: `->` işaretinden sonra işlevin döndüreceği türü yazarız.
Son satırın sonuna `;` koymazsak o değer döndürülür:

```
işlev topla(a: i32, b: i32) -> i32 {
    a + b
}
```

**`döndür`** işlevden hemen çıkar ve verilen değeri döndürür.

## Kendin dene

1. Bir sayının karesini döndüren `kare(sayı: i32) -> i32` işlevini yaz.
2. Bir sınavdan geçip geçmediğini söyleyen
   `geçti_mi(not: u32) -> mantıksal` işlevini yaz (50 ve üstü geçer).
3. `topla` işlevinin son satırına `;` koy. Pas ne diyor?

---

Sonraki ders: [Listeler](../07-listeler/README.md)
