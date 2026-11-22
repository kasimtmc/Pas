// Bilgi yarışması: sorular bir listede durur, her doğru cevap bir puan.

/// Bir soru ve cevabı
yapı Soru {
    soru: &'kalıcı str,
    cevap: &'kalıcı str,
}

/// Puana göre bir yorum
işlev yorum(puan: usize, toplam: usize) -> &'kalıcı str {
    eşle puan * 100 / toplam {
        100 => "Mükemmel! Hepsini bildin.",
        50..=99 => "Güzel! Biraz daha çalışırsan hepsini bilirsin.",
        _ => "Hiç sorun değil, bir daha dene!",
    }
}

işlev ana() {
    tanımla sorular = liste![
        Soru { soru: "Türkiye'nin başkenti neresi?", cevap: "ankara" },
        Soru { soru: "Bir haftada kaç gün var?", cevap: "7" },
        Soru { soru: "Gökkuşağında kaç renk var?", cevap: "7" },
        Soru { soru: "Kaplumbağalar yumurtlar mı? (evet/hayır)", cevap: "evet" },
    ];

    yazdır!("Bilgi yarışmasına hoş geldin! Cevaplarını küçük harfle yaz.");
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

    yazdır!();
    yazdır!("{} sorudan {} tanesini bildin.", sorular.uzunluk(), puan);
    yazdır!("{}", yorum(puan, sorular.uzunluk()));
}
