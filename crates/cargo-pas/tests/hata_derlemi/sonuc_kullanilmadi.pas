işlev böl(a: i32, b: i32) -> Sonuç<i32, Metin> {
    eğer b == 0 {
        döndür Hata("sıfır".metne_çevir());
    }
    Tamam(a / b)
}

işlev ana() {
    böl(4, 2);
}
