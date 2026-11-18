# Ders 3: Soru sormak

Şimdi programımız kullanıcıyla konuşacak! `oku!` ile klavyeden yazılanları
okuyacağız.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Neler öğreniyoruz?

**`oku!`** soruyu yazar ve kullanıcının yazdığı satırı okur:

```
yap ad = oku!("Adın ne? ");
```

**Sayı okumak** için sorudan sonra türünü de yazarız:

```
yap yaş = oku!("Kaç yaşındasın? ", u32);
yap boy = oku!("Boyun kaç metre? ", f64);
```

- `u32`: artı (pozitif) bir tam sayı (0, 1, 2, …)
- `i32`: eksi de olabilen bir tam sayı (…, -1, 0, 1, …)
- `f64`: ondalıklı bir sayı (1,45 de 1.45 de yazabilirsin)

Sayı beklenirken başka bir şey yazılırsa `oku!` uyarır ve yeniden sorar.
Dene: yaş sorulduğunda "on bir" yaz!

## Kendin dene

1. Kullanıcıya iki sayı sor ve toplamlarını yazdır.
2. Doğum yılını sor ve bu yıl kaç yaşına gireceğini hesapla.
3. Bir dikdörtgenin enini ve boyunu (`f64`) sor, alanını yazdır.

---

Sonraki ders: [Kararlar](../04-kararlar/README.md)
