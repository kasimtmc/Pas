# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Kazanan kim?

İki hamleye bakıp sonucu söyleyen `sonuç` işlevini tamamla: birinci hamle kazanırsa `Sen kazandın!`, ikinci kazanırsa `Ben kazandım!`, aynıysa `Berabere!` döndürsün.

İpucu: `eşle (senin, benim)` iki hamleyi birlikte karşılaştırır. Kazandığın durumları ve berabere durumlarını yaz; geri kalanı `_` ile yakala.

```pas başlangıç
çeşit Hamle {
    Taş,
    Kağıt,
    Makas,
}

işlev sonuç(senin: &Hamle, benim: &Hamle) -> &'kalıcı str {
    // Burayı `eşle` ile değiştir.
    ""
}

işlev ana() {
    yazdır!("{}", sonuç(&Hamle::Taş, &Hamle::Makas));
    yazdır!("{}", sonuç(&Hamle::Taş, &Hamle::Kağıt));
    yazdır!("{}", sonuç(&Hamle::Makas, &Hamle::Makas));
    yazdır!("{}", sonuç(&Hamle::Kağıt, &Hamle::Taş));
}
```

```çıktı
Sen kazandın!
Ben kazandım!
Berabere!
Sen kazandın!
```

```pas çözüm
çeşit Hamle {
    Taş,
    Kağıt,
    Makas,
}

işlev sonuç(senin: &Hamle, benim: &Hamle) -> &'kalıcı str {
    eşle (senin, benim) {
        (Hamle::Taş, Hamle::Makas) | (Hamle::Makas, Hamle::Kağıt) | (Hamle::Kağıt, Hamle::Taş) => "Sen kazandın!",
        (Hamle::Taş, Hamle::Taş) | (Hamle::Kağıt, Hamle::Kağıt) | (Hamle::Makas, Hamle::Makas) => "Berabere!",
        _ => "Ben kazandım!",
    }
}

işlev ana() {
    yazdır!("{}", sonuç(&Hamle::Taş, &Hamle::Makas));
    yazdır!("{}", sonuç(&Hamle::Taş, &Hamle::Kağıt));
    yazdır!("{}", sonuç(&Hamle::Makas, &Hamle::Makas));
    yazdır!("{}", sonuç(&Hamle::Kağıt, &Hamle::Taş));
}
```
