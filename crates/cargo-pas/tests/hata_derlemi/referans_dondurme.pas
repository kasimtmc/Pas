işlev selam() -> &str {
    tanımla metin = biçimle!("Merhaba");
    &metin
}

işlev ana() {
    yazdır!("{}", selam());
}
