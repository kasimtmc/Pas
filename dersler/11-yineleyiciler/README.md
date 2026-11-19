# Ders 11: Listelerle sihir

Bir listedeki her sayıyı ikiyle çarpmak, yalnızca geçen notları ayırmak ya
da en yüksek notu bulmak için döngü yazmana gerek yok. Pas'ın hazır
araçlarıyla bunları tek satırda yapabilirsin.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**Kapanış (closure)**: adı olmayan küçük bir işlevdir. Girdisi iki dikey
çizginin arasına yazılır:

```
tanımla iki_katı = |sayı: i32| sayı * 2;
yazdır!("{}", iki_katı(21)); // 42
```

**`yinele()`** listenin elemanlarını sırayla verir. Arkasına şunları
ekleyebilirsin:

| Yöntem | Ne yapar? |
|---|---|
| `dönüştür(\|x\| …)` | her elemanı kapanıştan geçirir |
| `süz(\|x\| …)` | yalnızca koşulu sağlayanları bırakır |
| `biriktir()` | çıkan elemanları yeni bir listede toplar |
| `toplam()` | elemanları toplar |
| `en_büyük()`, `en_küçük()` | en büyük ve en küçük elemanı `Belki` olarak verir |
| `numaralandır()` | her elemanı sırasıyla verir: `(0, x)`, `(1, y)`… |
| `bul(\|x\| …)` | koşulu sağlayan ilk elemanı `Belki` olarak verir |

Bunları art arda yazabilirsin:

```
tanımla kalanlar: Liste<i32> =
    notlar.yinele().dönüştür(|not| *not).süz(|not| *not < 50).dönüştür(|not| not + 10).biriktir();
```

**`*` işareti**: `yinele()` elemanları ödünç verir; `72` yerine `&72` gelir.
`*not` ödünç alınan değerin kendisidir. Unutursan Pas sana hatırlatır.

**Sıralamak**: `sırala()` listeyi küçükten büyüğe, `ters_çevir()` tersine
dizer. İkisi de listenin kendisini değiştirir; listeyi `değişken` yap ya da
`kopyala()` ile bir kopyasını sırala.

## Kendin dene

1. Notlardan 90 ve üstü olanların listesini çıkar ve kaç tane olduklarını yazdır.
2. `liste![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]` listesindeki çift sayıların
   karelerinin toplamını tek satırda bul (`sayı % 2 == 0` çiftleri seçer).
3. İsimler listesindeki en uzun ismi bul. İpucu: her ismi uzunluğuyla
   dönüştürüp `en_büyük()` kullanabilirsin.

---

Sonraki ders: [Sözlükler](../12-sozlukler/README.md)
