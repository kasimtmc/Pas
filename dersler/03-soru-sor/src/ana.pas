işlev ana() {
    // `oku!` kullanıcının yazdığı satırı okur.
    yap ad = oku!("Adın ne? ");
    yazdır!("Merhaba {ad}!");

    // Sayı okumak için türünü de yazarız: `u32` artı (pozitif) bir tam sayıdır.
    yap yaş = oku!("Kaç yaşındasın? ", u32);
    yazdır!("Yaklaşık {} aydır yaşıyorsun!", yaş * 12);
    yazdır!("Yaklaşık {} gündür yaşıyorsun!", yaş * 365);

    // `f64` ondalıklı bir sayıdır. 1,45 da 1.45 de yazabilirsin.
    yap boy = oku!("Boyun kaç metre? ", f64);
    yazdır!("Boyun {} santimetre.", boy * 100.0);

    yap en_sevdiğin = oku!("En sevdiğin hayvan ne? ");
    yazdır!("{ad}, {en_sevdiğin} benim de en sevdiğim hayvan!");
}
