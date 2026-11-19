yapı Kedi {
    yaş: u32,
}

işlev ana() {
    tanımla kedi = Kedi { yaş: 1 };
    kedi.yaş = 2;
    yazdır!("{}", kedi.yaş);
}
