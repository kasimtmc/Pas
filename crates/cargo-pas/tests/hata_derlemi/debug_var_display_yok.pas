#[derive(Debug)]
yapı Kedi {
    ad: Metin,
}

işlev ana() {
    yap kedi = Kedi { ad: Metin::from("Pamuk") };
    yazdır!("{}", kedi);
}
