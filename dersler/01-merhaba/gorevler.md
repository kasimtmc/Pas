# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Kendini tanıt

Ekrana alt alta şu iki satırı yazdır:

- `Merhaba, ben Pas!`
- `Programlamayı öğreniyorum.`

İpucu: Her satır için bir `yazdır!` kullan.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```çıktı
Merhaba, ben Pas!
Programlamayı öğreniyorum.
```

```pas çözüm
işlev ana() {
    yazdır!("Merhaba, ben Pas!");
    yazdır!("Programlamayı öğreniyorum.");
}
```

## Aynı satırda

`yaz!` ile üç kez yazarak tek bir satıra `Bir İki Üç` yazdır. Sonunda alt satıra geçmeyi unutma.

İpucu: `yaz!` alt satıra geçmez; en sonda boş bir `yazdır!()` alt satıra geçirir.

```pas başlangıç
işlev ana() {
    // Kodunu buraya yaz.
}
```

```çıktı
Bir İki Üç
```

```pas çözüm
işlev ana() {
    yaz!("Bir ");
    yaz!("İki ");
    yaz!("Üç");
    yazdır!();
}
```
