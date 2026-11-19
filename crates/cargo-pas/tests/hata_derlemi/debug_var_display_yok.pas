#[derive(Debug)]
yapı Kedi {
    ad: Metin,
}

işlev ana() {
    tanımla kedi = Kedi { ad: Metin::from("Pamuk") };
    yazdır!("{}", kedi);
}
