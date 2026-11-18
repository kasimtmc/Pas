# Ders 9: Çeşitler ve eşle

Bazı şeyler birkaç seçenekten biri olabilir: hava güneşli, yağmurlu ya da
karlıdır. Bunun için `çeşit`, seçenekleri karşılaştırmak için `eşle`
kullanırız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**Çeşit tanımlamak**:

```
çeşit Hava {
    Güneşli,
    Yağmurlu,
    Karlı,
}
```

**`eşle`** bir değeri her seçenekle karşılaştırır. Pas her seçeneği
düşündüğünden emin olur: birini unutursan sana hatırlatır!

```
eşle hava {
    Hava::Güneşli => "Şapkanı al!",
    Hava::Yağmurlu => "Şemsiyeni unutma!",
    Hava::Karlı => "Kardan adam yapalım!",
}
```

`eşle` sayı aralıklarıyla da çalışır (`85..=100 => …`). `_` ise "geri kalan
her şey" demektir.

**`Belki`**: Bir değer olabilir de olmayabilir de. Değer varsa `Var(değer)`,
yoksa `Yok` olur. Bir listede aradığını bulamayabilirsin; `Belki` tam bunun
içindir.

## Kendin dene

1. `Hava` çeşidine `Rüzgârlı` seçeneğini ekle ve programı çalıştır. Pas ne
   diyor? `öneri` işlevine yeni kolu ekle.
2. Haftanın günleri için bir çeşit yaz ve hafta sonu olup olmadığını söyleyen
   bir işlev ekle.
3. Bir listede bir ismi arayan ve buldu ise sırasını `Var(sıra)`, bulamadıysa
   `Yok` döndüren bir işlev yaz.

---

Sonraki ders: [Proje: Taş, kâğıt, makas](../10-tas-kagit-makas/README.md)
