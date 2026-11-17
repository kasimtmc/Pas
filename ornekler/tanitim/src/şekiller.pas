genel çeşit Şekil {
    Kare(f64),
    Daire(f64),
}

genel işlev alan(şekil: &Şekil) -> f64 {
    eşle şekil {
        Şekil::Kare(kenar) => kenar * kenar,
        Şekil::Daire(yarıçap) => 3.14159 * yarıçap * yarıçap,
    }
}
