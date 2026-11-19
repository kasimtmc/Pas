// `yapı` birbirine ait bilgileri bir arada tutar.
yapı Kedi {
    ad: Metin,
    yaş: u32,
    açlık: u32,
}

// `uygula` ile yapıya yetenekler (yöntemler) ekleriz.
uygula Kedi {
    /// Yeni bir kedi oluşturur.
    işlev yeni(ad: &str, yaş: u32) -> Kedi {
        Kedi { ad: ad.metne_çevir(), yaş, açlık: 3 }
    }

    // `kendi`, yöntemin çağrıldığı kedidir.
    işlev miyavla(&kendi) {
        yazdır!("{}: Miyav!", kendi.ad);
    }

    // Kediyi değiştireceğimiz için `&değişken kendi` yazarız.
    işlev besle(&değişken kendi) {
        eğer kendi.açlık > 0 {
            kendi.açlık -= 1;
        }
        yazdır!("{} mama yedi. Açlığı: {}", kendi.ad, kendi.açlık);
    }

    işlev tok_mu(&kendi) -> mantıksal {
        kendi.açlık == 0
    }

    işlev doğum_günü(&değişken kendi) {
        kendi.yaş += 1;
        yazdır!("İyi ki doğdun {}! Artık {} yaşındasın.", kendi.ad, kendi.yaş);
    }
}

işlev ana() {
    tanımla değişken pamuk = Kedi::yeni("Pamuk", 3);
    tanımla değişken boncuk = Kedi::yeni("Boncuk", 1);

    pamuk.miyavla();
    boncuk.miyavla();

    // `!` "değil" demektir: tok değilken beslemeye devam et.
    sürece !pamuk.tok_mu() {
        pamuk.besle();
    }
    yazdır!("{} artık tok!", pamuk.ad);

    boncuk.doğum_günü();
}
