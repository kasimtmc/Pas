işlev ana() {
    yap sayılar = liste![1, 2, 3];
    yap iki_kat = sayılar.yinele().dönüştür(|s| s * 2).biriktir();
    yazdır!("{:?}", iki_kat);
}
