# Ders 14: Nitelikler

Kedi, köpek ve inek birbirinden farklıdır ama hepsinin bir adı ve bir sesi
vardır. Farklı türlerin ortak yapabildiği şeyleri **nitelik** ile
tanımlarız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**Nitelik tanımlamak**: niteliği taşıyan her türün hangi yöntemleri olması
gerektiğini yazarız. Gövdesi olan yöntemleri her tür hazır olarak alır:

```
nitelik Hayvan {
    işlev ad(&kendi) -> Metin;
    işlev ses(&kendi) -> &'kalıcı str;

    işlev tanıt(&kendi) {
        yazdır!("Ben {}, \"{}\" diye seslenirim!", kendi.ad(), kendi.ses());
    }
}
```

**Niteliği bir türe uygulamak**: Bu satırı "Kedi için Hayvan niteliğini
uygula" diye okuyabilirsin:

```
uygula Hayvan için Kedi {
    işlev ad(&kendi) -> Metin { kendi.ad.kopyala() }
    işlev ses(&kendi) -> &'kalıcı str { "miyav" }
}
```

Bir yöntemi unutursan Pas hangisinin eksik olduğunu söyler. Hazır gelen bir
yöntemi (`tanıt`) kendi türüne göre yeniden yazabilirsin; `Köpek` öyle
yapıyor.

**`dinamik`**: `&dinamik Hayvan` "Hayvan niteliğini taşıyan herhangi bir
değer" demektir. Böylece bir işlev kediyle de köpekle de çalışır.

**Farklı türler aynı listede**: Bir listede yalnızca tek türden değerler
durabilir. Her hayvanı bir `Kutu`ya koyarsak hepsi `Kutu<dinamik Hayvan>`
olur:

```
tanımla çiftlik: Liste<Kutu<dinamik Hayvan>> = liste![
    Kutu::yeni(minnoş),
    Kutu::yeni(İnek),
];
```

Nitelikleri sen yazmadan da kullanıyordun: `yazdır!` ile `{}` kullanarak
yazdırılabilen türlerin hepsi `Display` adlı bir niteliği taşır.

## Kendin dene

1. Çiftliğe bir `Ördek` ekle.
2. `Hayvan` niteliğine `bacak_sayısı(&kendi) -> u32` yöntemini ekle. Pas
   hangi türlerde eksik olduğunu söyleyecek. Sonra çiftlikteki toplam
   bacak sayısını yazdır.
3. Yeni bir nitelik yaz: `Şekil`, `alan(&kendi) -> f64` yöntemiyle. `Kare`
   ve `Daire` yapılarına uygula ve alanlarını yazdır.

---

Sonraki ders: [Kaplumbağa ile çizim](../15-kaplumbaga/README.md)
