işlev selam() -> &str {
    yap metin = biçimle!("Merhaba");
    &metin
}

işlev ana() {
    yazdır!("{}", selam());
}
