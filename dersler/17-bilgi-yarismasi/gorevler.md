# Görevler

Bu görevleri [Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)nda yapıp **Denetle** düğmesiyle denetleyebilirsin.

## Puan yorumu

`yorum` işlevini tamamla: hepsi bildiyse `Mükemmel!`, en az yarısını bildiyse `Güzel!`, daha azını bildiyse `Bir daha dene!` döndürsün.

İpucu: "En az yarısı" `puan * 2 >= toplam` demektir.

```pas başlangıç
işlev yorum(puan: u32, toplam: u32) -> &'kalıcı str {
    // Burayı değiştir.
    ""
}

işlev ana() {
    yazdır!("{}", yorum(4, 4));
    yazdır!("{}", yorum(2, 4));
    yazdır!("{}", yorum(3, 4));
    yazdır!("{}", yorum(1, 4));
}
```

```çıktı
Mükemmel!
Güzel!
Güzel!
Bir daha dene!
```

```pas çözüm
işlev yorum(puan: u32, toplam: u32) -> &'kalıcı str {
    eğer puan == toplam {
        "Mükemmel!"
    } değilse eğer puan * 2 >= toplam {
        "Güzel!"
    } değilse {
        "Bir daha dene!"
    }
}

işlev ana() {
    yazdır!("{}", yorum(4, 4));
    yazdır!("{}", yorum(2, 4));
    yazdır!("{}", yorum(3, 4));
    yazdır!("{}", yorum(1, 4));
}
```

## Küçük yarışma

Listedeki soruları sırayla sor (`1. ` gibi numarasıyla). Cevap doğruysa `Doğru!`, değilse `Yanlış. Doğru cevap: 7` yazdır. Sonunda puanı `Puan: 1/2` biçiminde yazdır.

İpucu: `için (sıra, soru) içinde sorular.elemanlar().numaralandır()` her soruyu sırasıyla verir; sıra 0'dan başlar. Soruyu `oku!(biçimle!("{}. {} ", sıra + 1, soru.soru))` ile sorabilirsin.

```pas başlangıç
yapı Soru {
    soru: &'kalıcı str,
    cevap: &'kalıcı str,
}

işlev ana() {
    tanımla sorular = liste![
        Soru { soru: "Türkiye'nin başkenti neresi?", cevap: "ankara" },
        Soru { soru: "Bir haftada kaç gün var?", cevap: "7" },
    ];
    tanımla değişken puan = 0;
    // Soruları sor.
    yazdır!("Puan: {}/{}", puan, sorular.uzunluk());
}
```

```girdi
ankara
8
```

```çıktı
1. Türkiye'nin başkenti neresi? ankara
Doğru!
2. Bir haftada kaç gün var? 8
Yanlış. Doğru cevap: 7
Puan: 1/2
```

```girdi
ankara
7
```

```çıktı
1. Türkiye'nin başkenti neresi? ankara
Doğru!
2. Bir haftada kaç gün var? 7
Doğru!
Puan: 2/2
```

```pas çözüm
yapı Soru {
    soru: &'kalıcı str,
    cevap: &'kalıcı str,
}

işlev ana() {
    tanımla sorular = liste![
        Soru { soru: "Türkiye'nin başkenti neresi?", cevap: "ankara" },
        Soru { soru: "Bir haftada kaç gün var?", cevap: "7" },
    ];
    tanımla değişken puan = 0;
    için (sıra, soru) içinde sorular.elemanlar().numaralandır() {
        tanımla cevap = oku!(biçimle!("{}. {} ", sıra + 1, soru.soru));
        eğer cevap == soru.cevap {
            puan += 1;
            yazdır!("Doğru!");
        } değilse {
            yazdır!("Yanlış. Doğru cevap: {}", soru.cevap);
        }
    }
    yazdır!("Puan: {}/{}", puan, sorular.uzunluk());
}
```
