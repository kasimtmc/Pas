# Proje: Desen çizici

Basit şekilleri döndürerek ve tekrarlayarak çiçekler, yıldızlar ve
spiraller çizeceğiz. İşlevler ve döngüler bir araya gelince ortaya
şaşırtıcı desenler çıkar!

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

Çizim bu dizindeki `çizim.svg` dosyasına kaydedilir; onu tarayıcıda aç.
[Pas Oyun Alanı](https://kasimtmc.github.io/Pas/#ders=19-desen-cizici)nda ise
çizim çıktının üstünde görünür.

## Nasıl çalışıyor?

**Çokgen**: Kaplumbağa bir şeklin çevresini dolaşınca toplam 360 derece
döner. Bu yüzden `köşe` köşeli bir şekilde her köşede `360 / köşe` derece
döneriz:

```
işlev çokgen(k: &değişken Kaplumbağa, köşe: u32, kenar: f64) {
    her _ için 0..köşe {
        k.ileri(kenar);
        k.sağa_dön(360.0 / köşe olarak f64);
    }
}
```

`köşe olarak f64` tam sayıyı ondalıklı sayıya çevirir; böylece 360'ı 7'ye
bile tam bölebiliriz.

**Çiçek**: Aynı şekli her seferinde biraz döndürerek tekrar çizersek bir
çiçek olur. `çiçek` işlevi `çokgen`i kullanır: işlevler başka işlevleri
çağırabilir.

**Renkler**: `renkler[i % renkler.uzunluk()]` sırayla bütün renkleri dolaşır;
`%` liste bitince başa döndürür.

**Spiral**: Her adımda biraz daha uzun yürüyüp 90 dereceden biraz fazla
(`91`) dönünce kare, kendi etrafında dönen bir spirale dönüşür.

## Kendini geliştir

1. `çiçek` işlevini beşgenlerle, sekizgenlerle dene. Yaprak sayısını değiştir.
2. Spiralde `91` yerine `121`, `144` ya da `89` dene. Ne değişiyor?
3. Kendi desenini tasarla ve **Paylaş** ile arkadaşlarına gönder!

---

Tebrikler, bütün dersleri ve projeleri bitirdin! Şimdi kendi projeni başlatma
zamanı: [dersler sayfasına](../README.md#kendi-projeni-başlat) dön.
