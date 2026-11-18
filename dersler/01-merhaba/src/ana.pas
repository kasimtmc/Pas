// Bu bir yorum. Bilgisayar yorumları okumaz; yorumlar insanlar içindir.

// Her Pas programı `ana` işlevinden başlar. Süslü parantezlerin { }
// arasındaki her şey sırayla, yukarıdan aşağıya çalışır.
işlev ana() {
    yazdır!("Merhaba dünya!");
    yazdır!("Benim adım Pas. Seninle tanıştığıma sevindim.");

    // `yaz!` alt satıra geçmez, `yazdır!` geçer.
    yaz!("Bu cümle ");
    yaz!("ve bu cümle ");
    yazdır!("aynı satırda.");

    // Boş bir satır bırakmak için:
    yazdır!();
    yazdır!("Hoşça kal!");
}
