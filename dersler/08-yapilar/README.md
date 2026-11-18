# Ders 8: Yapılar

Bir kedinin adı, yaşı ve ne kadar aç olduğu gibi birbirine ait bilgileri bir
arada tutmak için `yapı` kullanırız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**Yapı tanımlamak**:

```
yapı Kedi {
    ad: Metin,
    yaş: u32,
}
```

`Metin` değiştirilebilen bir yazıdır. Yapıların adları büyük harfle başlar.

**`uygula`** ile yapıya yöntemler (yetenekler) ekleriz. `kendi`, yöntemin
çağrıldığı değerdir:

```
uygula Kedi {
    işlev miyavla(&kendi) {
        yazdır!("{}: Miyav!", kendi.ad);
    }
}
```

Yöntem yapıyı değiştirecekse `&değişken kendi` yazarız.

**Kullanmak**: `Kedi::yeni("Pamuk", 3)` yeni bir kedi oluşturur;
`pamuk.miyavla()` yöntemi çağırır; `pamuk.ad` bir alanı okur.

**`!`** bir koşulun önüne gelince "değil" demektir: `!pamuk.tok_mu()`.

## Kendin dene

1. Kedilere `oyna` yöntemi ekle: her oynayışta açlık 1 artsın.
2. Bir `Köpek` yapısı oluştur ve `havla` yöntemini yaz.
3. `Dikdörtgen` yapısı yap (`en` ve `boy` alanları) ve alanını döndüren
   `alan(&kendi) -> f64` yöntemini yaz.

---

Sonraki ders: [Çeşitler ve eşle](../09-cesitler-ve-esle/README.md)
