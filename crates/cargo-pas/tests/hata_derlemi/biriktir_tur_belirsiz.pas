işlev ana() {
    tanımla sayılar = liste![1, 2, 3];
    tanımla iki_kat = sayılar.yinele().dönüştür(|s| s * 2).biriktir();
    yazdır!("{:?}", iki_kat);
}
