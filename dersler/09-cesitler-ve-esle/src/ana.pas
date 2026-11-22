// `çeşit`, birkaç seçenekten biri olabilen bir türdür.
çeşit Hava {
    Güneşli,
    Yağmurlu,
    Karlı,
}

// `eşle` bir değeri seçeneklerle karşılaştırır. Her seçeneği düşünmek zorundayız!
işlev öneri(hava: &Hava) -> &'kalıcı str {
    eşle hava {
        Hava::Güneşli => "Şapkanı al!",
        Hava::Yağmurlu => "Şemsiyeni unutma!",
        Hava::Karlı => "Kardan adam yapalım!",
    }
}

// `Belki`: bir değer olabilir (`Var`) de olmayabilir de (`Yok`).
işlev ilk_büyük(sayılar: &[i32], sınır: i32) -> Belki<i32> {
    için sayı içinde sayılar {
        eğer *sayı > sınır {
            döndür Var(*sayı);
        }
    }
    Yok
}

işlev ana() {
    tanımla havalar = [Hava::Güneşli, Hava::Yağmurlu, Hava::Karlı];
    için hava içinde &havalar {
        yazdır!("{}", öneri(hava));
    }

    tanımla sayılar = [4, 8, 15, 16, 23, 42];
    eşle ilk_büyük(&sayılar, 10) {
        Var(sayı) => yazdır!("10'dan büyük ilk sayı: {sayı}"),
        Yok => yazdır!("10'dan büyük sayı yok."),
    }
    eşle ilk_büyük(&sayılar, 100) {
        Var(sayı) => yazdır!("100'den büyük ilk sayı: {sayı}"),
        Yok => yazdır!("100'den büyük sayı yok."),
    }

    // `eşle` sayı aralıklarıyla da çalışır. `_` "geri kalan her şey" demektir.
    tanımla not = oku!("Sınav notun kaç? (0-100) ", u32);
    tanımla derece = eşle not {
        85..=100 => "Pekiyi",
        70..=84 => "İyi",
        50..=69 => "Orta",
        0..=49 => "Biraz daha çalışmalısın",
        _ => "Not 100'den büyük olamaz!",
    };
    yazdır!("Derecen: {derece}");
}
