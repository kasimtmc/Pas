# Ders 7: Listeler

Birden fazla değeri bir arada tutmak için liste kullanırız: alışveriş
listesi, sınav notları, oyuncular…

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**Liste oluşturmak**:

```
tanımla değişken meyveler = liste!["elma", "armut"];
meyveler.ekle("muz");
```

**`uzunluk()`** listede kaç eleman olduğunu söyler.

**Elemanlara erişmek**: Listedeki yerler **0'dan başlar**. İlk eleman
`meyveler[0]`, ikincisi `meyveler[1]`'dir. Listede olmayan bir yere erişmeye
çalışırsan program durur ve sana nedenini Türkçe söyler. Dene:
`meyveler[10]`.

**Listede dolaşmak**:

```
her meyve için &meyveler {
    yazdır!("{meyve}");
}
```

`&` işareti "listeyi ödünç al" demektir; liste döngüden sonra da kullanılabilir.

**Listeyi yazdırmak**: `{:?}` bütün listeyi köşeli parantez içinde yazar.

**Sayı biçimi**: `{:.1}` ondalıklı bir sayıyı virgülden sonra tek basamakla
yazar.

## Kendin dene

1. En sevdiğin beş oyunu bir listeye koy ve numaralı olarak yazdır
   ("1. …", "2. …").
2. Kullanıcıdan 5 sayı al ve en küçüğünü bul.
3. Notların arasında 50'den küçük olan kaç tane var, say.

---

Sonraki ders: [Yapılar](../08-yapilar/README.md)
