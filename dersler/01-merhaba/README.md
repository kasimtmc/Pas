# Ders 1: Merhaba dünya!

İlk programına hoş geldin! Bu derste bilgisayara bir şeyler yazdırmayı
öğreneceğiz.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`işlev ana()`**: Her Pas programı buradan başlar. Süslü parantezlerin
`{ }` arasındaki satırlar yukarıdan aşağıya sırayla çalışır.

**`yazdır!`**: Tırnak içindeki yazıyı ekrana yazar ve alt satıra geçer.
Sonundaki `!` işareti onun bir *makro* olduğunu gösterir; şimdilik bunu
"özel bir komut" diye düşünebilirsin.

```
yazdır!("Merhaba dünya!");
```

**`yaz!`**: `yazdır!` gibidir ama alt satıra geçmez.

**`;`**: Her komutun sonuna noktalı virgül koyarız. Unutursan Pas sana
Türkçe olarak hatırlatır!

**`//`**: İki eğik çizgiden sonra yazılanlar *yorumdur*. Bilgisayar onları
okumaz; kodunu açıklamak için yazarsın.

## Kendin dene

1. Programa kendi adını yazdıran bir satır ekle.
2. `yaz!` kullanarak "Bir, iki, üç!" yazısını üç ayrı komutla ama aynı
   satırda yazdır.
3. Bir satırın sonundaki `;` işaretini sil ve programı çalıştır. Pas ne diyor?
   Sonra geri koy.

---

Sonraki ders: [Değişkenler](../02-degiskenler/README.md)
