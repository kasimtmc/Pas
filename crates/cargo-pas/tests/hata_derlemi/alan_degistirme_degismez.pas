yapı Kedi {
    yaş: u32,
}

işlev ana() {
    yap kedi = Kedi { yaş: 1 };
    kedi.yaş = 2;
    yazdır!("{}", kedi.yaş);
}
