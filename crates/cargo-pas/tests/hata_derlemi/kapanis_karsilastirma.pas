işlev ana() {
    tanımla notlar = liste![40, 70, 90];
    tanımla geçenler: Liste<&i32> = notlar.elemanlar().süz(|not| not > 50).biriktir();
    yazdır!("{:?}", geçenler);
}
