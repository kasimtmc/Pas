nitelik Hayvan {
    işlev ses(&kendi) -> Metin;
}

yapı Kedi;

işlev seslen(hayvan: &dinamik Hayvan) {
    yazdır!("{}", hayvan.ses());
}

işlev ana() {
    seslen(&Kedi);
}
