# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Dönen üçgenler

`çokgen` işlevini kullanarak kenarı 80 olan 12 üçgen çiz. Her üçgenden sonra kaplumbağayı 30 derece **sağa** döndür.

İpucu: Bir üçgen `çokgen(&değişken k, 3, 80.0)` ile çizilir; bunu bir `her` döngüsünde 12 kez yap.

```pas başlangıç
işlev çokgen(k: &değişken Kaplumbağa, köşe: u32, kenar: f64) {
    için _ içinde 0..köşe {
        k.ileri(kenar);
        k.sağa_dön(360.0 / köşe olarak f64);
    }
}

işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
}
```

```pas çözüm
işlev çokgen(k: &değişken Kaplumbağa, köşe: u32, kenar: f64) {
    için _ içinde 0..köşe {
        k.ileri(kenar);
        k.sağa_dön(360.0 / köşe olarak f64);
    }
}

işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
    için _ içinde 0..12 {
        çokgen(&değişken k, 3, 80.0);
        k.sağa_dön(30);
    }
}
```

## Kare spiral

Her adımda biraz daha uzun yürüyen bir kare spiral çiz: 1'den 20'ye kadar her `i` için `i * 10` adım yürü ve 90 derece **sağa** dön.

İpucu: `için i içinde 1..=20` 1'den 20'ye kadar döner; `k.ileri(i * 10)` her seferinde daha uzun yürür.

```pas başlangıç
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
}
```

```pas çözüm
işlev ana() {
    tanımla değişken k = Kaplumbağa::yeni();
    için i içinde 1..=20 {
        k.ileri(i * 10);
        k.sağa_dön(90);
    }
}
```
