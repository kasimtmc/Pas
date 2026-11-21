# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Dikdörtgen

Eni 7, boyu 5 olan bir dikdörtgenin alanını ve çevresini hesaplayıp şöyle yazdır:

```text
Alan: 35
Çevre: 24
```

İpucu: Alan `en * boy`, çevre `2 * (en + boy)` olur.

```pas başlangıç
işlev ana() {
    tanımla en = 7;
    tanımla boy = 5;
    // Alanı ve çevreyi hesapla.
}
```

```çıktı
Alan: 35
Çevre: 24
```

```pas çözüm
işlev ana() {
    tanımla en = 7;
    tanımla boy = 5;
    yazdır!("Alan: {}", en * boy);
    yazdır!("Çevre: {}", 2 * (en + boy));
}
```

## Kumbara

Kumbarada 20 lira var. Önce 15 lira at, sonra 8 lira harca. Her adımdan sonra kumbarada kaç lira olduğunu yazdır:

```text
Kumbara: 20 lira
Kumbara: 35 lira
Kumbara: 27 lira
```

İpucu: `kumbara += 15;` kumbaraya 15 ekler.

```pas başlangıç
işlev ana() {
    tanımla değişken kumbara = 20;
    yazdır!("Kumbara: {kumbara} lira");
}
```

```çıktı
Kumbara: 20 lira
Kumbara: 35 lira
Kumbara: 27 lira
```

```pas çözüm
işlev ana() {
    tanımla değişken kumbara = 20;
    yazdır!("Kumbara: {kumbara} lira");
    kumbara += 15;
    yazdır!("Kumbara: {kumbara} lira");
    kumbara -= 8;
    yazdır!("Kumbara: {kumbara} lira");
}
```
