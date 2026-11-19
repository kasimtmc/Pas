// Nitelik (trait): farklı türlerin ortak yapabildiği şeyler.
// Her hayvanın bir adı ve bir sesi vardır.
nitelik Hayvan {
    işlev ad(&kendi) -> Metin;
    işlev ses(&kendi) -> &'durağan str;

    // Gövdesi olan yöntemleri her hayvan hazır olarak alır.
    işlev tanıt(&kendi) {
        yazdır!("Ben {}, \"{}\" diye seslenirim!", kendi.ad(), kendi.ses());
    }
}

yapı Kedi {
    ad: Metin,
}

yapı Köpek {
    ad: Metin,
    yaş: u32,
}

yapı İnek;

// "Hayvan niteliğini her Kedi'ye uygula"
uygula Hayvan her Kedi {
    işlev ad(&kendi) -> Metin {
        kendi.ad.kopyala()
    }

    işlev ses(&kendi) -> &'durağan str {
        "miyav"
    }
}

uygula Hayvan her Köpek {
    işlev ad(&kendi) -> Metin {
        biçimle!("{} ({} yaşında)", kendi.ad, kendi.yaş)
    }

    işlev ses(&kendi) -> &'durağan str {
        "hav hav"
    }

    // Hazır gelen yöntemi kendimize göre değiştirebiliriz.
    işlev tanıt(&kendi) {
        yazdır!("Hav! Ben {}. Top atar mısın?", kendi.ad());
    }
}

uygula Hayvan her İnek {
    işlev ad(&kendi) -> Metin {
        "Sarıkız".metne_çevir()
    }

    işlev ses(&kendi) -> &'durağan str {
        "möö"
    }
}

// `&dinamik Hayvan`: Hayvan niteliğini taşıyan herhangi bir değer.
işlev iki_kez_seslen(hayvan: &dinamik Hayvan) {
    yazdır!("{0} {0}!", hayvan.ses());
}

işlev ana() {
    tanımla minnoş = Kedi { ad: "Minnoş".metne_çevir() };
    minnoş.tanıt();
    iki_kez_seslen(&minnoş);

    // Farklı türden hayvanlar aynı listede durabilir; her biri bir `Kutu` içinde.
    tanımla çiftlik: Liste<Kutu<dinamik Hayvan>> = liste![
        Kutu::yeni(minnoş),
        Kutu::yeni(Köpek { ad: "Karabaş".metne_çevir(), yaş: 3 }),
        Kutu::yeni(İnek),
    ];

    yazdır!("Çiftlikte {} hayvan var:", çiftlik.uzunluk());
    her hayvan için &çiftlik {
        hayvan.tanıt();
    }
}
