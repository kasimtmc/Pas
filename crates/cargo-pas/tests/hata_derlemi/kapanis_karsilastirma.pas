işlev ana() {
    yap notlar = liste![40, 70, 90];
    yap geçenler: Liste<&i32> = notlar.yinele().süz(|not| not > 50).biriktir();
    yazdır!("{:?}", geçenler);
}
