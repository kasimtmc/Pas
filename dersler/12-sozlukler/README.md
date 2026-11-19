# Ders 12: Sözlükler

Telefon rehberinde her isme bir numara, sözlükte her kelimeye bir anlam
karşılık gelir. Pas'ta da bir şeyi (anahtar) başka bir şeye (değer)
bağlamak için `Sözlük` kullanırız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`kullan`**: `Sözlük` Pas'ın içinde hazır gelir ama başka bir yerde
durur. Dosyanın başına şunu yazarak kullanıma alırız:

```
kullan std::collections::Sözlük;
```

**Sözlük oluşturmak ve doldurmak**:

```
tanımla değişken başkentler = Sözlük::yeni();
başkentler.yerleştir("Türkiye", "Ankara");
```

Aynı anahtara yeniden yerleştirirsen eski değerin yerine geçer.

**Değere ulaşmak**:

- `başkentler["Türkiye"]` değeri verir; anahtar yoksa program durur.
- `başkentler.al("Fransa")` programı durdurmaz; değeri `Var(…)` ya da `Yok`
  olarak verir.
- `al` bir `Belki` verdiği için `yoksa` ile anahtar yokken kullanılacak
  değeri söyleyebilirsin: `sayaç.al(kelime).yoksa(&0)`.

**Diğer yöntemler**: `anahtar_var_mı`, `sil`, `uzunluk`, `anahtarlar`,
`değerler`.

**Sıra yoktur**: Sözlük elemanları eklendiği sırayla tutmaz. Düzenli
yazdırmak istersen anahtarları bir listeye alıp sırala.

## Kendin dene

1. Sınıfındaki beş arkadaşının adını ve doğum ayını bir sözlüğe koy.
   Kullanıcıya bir isim sor ve o kişinin doğum ayını yazdır; isim yoksa
   "Bu kişiyi tanımıyorum" de.
2. Kelime sayacını değiştirerek her **harfin** kaç kez geçtiğini say.
   İpucu: `cümle.karakterler()`.
3. Bir alışveriş sepeti yap: ürün → fiyat. Sepetteki her şeyin toplam
   fiyatını `değerler()` ile hesapla.

---

Sonraki ders: [Hatalarla başa çıkmak](../13-sonuc-ve-hatalar/README.md)
