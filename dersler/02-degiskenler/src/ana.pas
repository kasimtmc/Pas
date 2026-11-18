işlev ana() {
    // `yap` ile bir değere isim veririz. Bu isimlere değişken denir.
    yap ad = "Elif";
    yap yaş = 11;
    yazdır!("{ad} {yaş} yaşında.");

    // Değişkenlerle hesap yapabiliriz.
    yap gelecek_yıl = yaş + 1;
    yazdır!("Gelecek yıl {gelecek_yıl} yaşında olacak.");
    yazdır!("10 yıl sonra {} yaşında olacak.", yaş + 10);

    // Değeri sonradan değişecek bir değişkenin başına `değişken` yazarız.
    yap değişken puan = 0;
    puan = puan + 10;
    puan += 5;
    yazdır!("Oyundaki puanın: {puan}");

    // Ondalıklı sayılar nokta ile yazılır.
    yap boy = 1.45;
    yazdır!("Boyun {boy} metre, yani {} santimetre.", boy * 100.0);

    // Dört işlem
    yap a = 17;
    yap b = 5;
    yazdır!("{a} + {b} = {}", a + b);
    yazdır!("{a} - {b} = {}", a - b);
    yazdır!("{a} x {b} = {}", a * b);
    yazdır!("{a} / {b} = {} (tam sayılarda bölüm)", a / b);
    yazdır!("{a} % {b} = {} (bölümden kalan)", a % b);
}
