// Hesap makinesi: iki sayı ve bir işlem sorar, sonucu yazar.
// İşlem yerine `son` yazınca biter.

/// İki sayı arasında verilen işlemi yapar; yapamazsa nedenini söyler.
işlev hesapla(a: f64, işlem: &str, b: f64) -> Sonuç<f64, Metin> {
    eşle işlem {
        "+" => Tamam(a + b),
        "-" => Tamam(a - b),
        "*" | "x" => Tamam(a * b),
        "/" => {
            eğer b == 0.0 {
                Hata("sıfıra bölünemez".metne_çevir())
            } değilse {
                Tamam(a / b)
            }
        }
        başka => Hata(biçimle!("`{başka}` bir işlem değil; +, -, * ya da / yaz")),
    }
}

işlev ana() {
    yazdır!("Hesap makinesi! Bitirmek için işlem yerine `son` yaz.");
    döngü {
        tanımla a = oku!("Birinci sayı: ", f64);
        tanımla işlem = oku!("İşlem (+ - * /): ");
        eğer işlem == "son" {
            kır;
        }
        tanımla b = oku!("İkinci sayı: ", f64);
        eşle hesapla(a, &işlem, b) {
            Tamam(sonuç) => yazdır!("{a} {işlem} {b} = {sonuç}"),
            Hata(neden) => yazdır!("Olmadı: {neden}"),
        }
    }
    yazdır!("Görüşürüz!");
}
