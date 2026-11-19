# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Şekillerin alanı

`Kare` ve `Daire` için `alan` yöntemlerini tamamla. Alanları virgülden sonra iki basamakla yazdırılıyor:

```
Kare: 16.00
Daire: 12.57
```

İpucu: Dairenin alanı π × yarıçap × yarıçap'tır; π için `std::f64::consts::PI` kullan.

```pas başlangıç
nitelik Şekil {
    işlev alan(&kendi) -> f64;
}

yapı Kare {
    kenar: f64,
}

yapı Daire {
    yarıçap: f64,
}

uygula Şekil her Kare {
    işlev alan(&kendi) -> f64 {
        // Burayı değiştir.
        0.0
    }
}

uygula Şekil her Daire {
    işlev alan(&kendi) -> f64 {
        // Burayı değiştir.
        0.0
    }
}

işlev ana() {
    yap kare = Kare { kenar: 4.0 };
    yap daire = Daire { yarıçap: 2.0 };
    yazdır!("Kare: {:.2}", kare.alan());
    yazdır!("Daire: {:.2}", daire.alan());
}
```

```çıktı
Kare: 16.00
Daire: 12.57
```

```pas çözüm
nitelik Şekil {
    işlev alan(&kendi) -> f64;
}

yapı Kare {
    kenar: f64,
}

yapı Daire {
    yarıçap: f64,
}

uygula Şekil her Kare {
    işlev alan(&kendi) -> f64 {
        kendi.kenar * kendi.kenar
    }
}

uygula Şekil her Daire {
    işlev alan(&kendi) -> f64 {
        std::f64::consts::PI * kendi.yarıçap * kendi.yarıçap
    }
}

işlev ana() {
    yap kare = Kare { kenar: 4.0 };
    yap daire = Daire { yarıçap: 2.0 };
    yazdır!("Kare: {:.2}", kare.alan());
    yazdır!("Daire: {:.2}", daire.alan());
}
```
