yapı Sayaç {
    değer: u32,
}

uygula Sayaç {
    işlev artır() {
        kendi.değer += 1;
    }
}

işlev ana() {
    yap değişken s = Sayaç { değer: 0 };
    s.artır();
}
