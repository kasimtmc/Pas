# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Üçgen

Kaplumbağaya kenarı 100 olan bir eşkenar üçgen çizdir: her köşede `ileri(100)` yürüyüp 120 derece **sağa** dönsün.

İpucu: Kare için 4 kez 90 derece dönüyorduk; üçgen için 3 kez 120 derece dön.

```pas başlangıç
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
}
```

```pas çözüm
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
    için _ içinde 0..3 {
        k.ileri(100);
        k.sağa_dön(120);
    }
}
```

## Merdiven

Dört basamaklı bir merdiven çiz. Her basamakta önce 30 yukarı, sonra 30 sağa yürü. Kaplumbağa yukarı bakarak başlar.

İpucu: Yukarı yürü, sağa dön, yürü, sonra sola dönerek yeniden yukarı bak.

```pas başlangıç
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
}
```

```pas çözüm
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
    için _ içinde 0..4 {
        k.ileri(30);
        k.sağa_dön(90);
        k.ileri(30);
        k.sola_dön(90);
    }
}
```
