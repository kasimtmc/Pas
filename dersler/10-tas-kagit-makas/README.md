# Ders 10: Proje: Taş, kâğıt, makas

Şimdiye kadar öğrendiğin her şeyi bir araya getirip bilgisayara karşı bir oyun
yapıyoruz!

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Oyunda neler kullanıyoruz?

- **`çeşit Hamle`**: taş, kâğıt ya da makas
- **`uygula`** ile hamlelere yöntemler: adını söyleme, yazıdan hamleye
  çevirme, başka bir hamleyi yenip yenmediğini bulma
- **`eşle`** ile kuralları yazma. `(kendi, öteki)` iki değeri birlikte
  karşılaştırır; `|` "veya" demektir
- **`Belki`**: kullanıcı anlamadığımız bir şey yazarsa `Yok` döner
- **`sürece`** ile oyun döngüsü, **`devam`** ile turu yeniden başlatma
- **`oku!`** ile kullanıcının hamlesini okuma

Bilgisayarın hamlesi saatin nanosaniyelerinden seçilir. Bu tam olarak
rastgele değildir ama oyun için yeterlidir.

## Kendin dene

1. Oyunu 5 tur kazananın kazanacağı şekilde değiştir.
2. Kaç kez berabere kaldığınızı da say ve oyun sonunda yazdır.
3. "Taş, kâğıt, makas, kertenkele, Spock" kurallarını araştır ve oyuna iki
   yeni hamle ekle. Hangi yerleri değiştirmen gerektiğini Pas sana
   söyleyecek!

---

Sonraki ders: [Listelerle sihir](../11-yineleyiciler/README.md)
