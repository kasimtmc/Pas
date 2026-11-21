# Proje: Bilgi yarışması

Kendi bilgi yarışmanı yap: sorular sırayla sorulur, her doğru cevap bir
puan kazandırır, sonunda puanına göre bir yorum gelir.

## Çalıştır

Bu dizinde şu komutu yaz:

```bash
cargo pas çalıştır
```

RustRover kullanıyorsan `src/ana.pas` dosyasını aç ve `işlev ana()` satırının
yanındaki ▶ düğmesine bas.

## Nasıl çalışıyor?

**Soruları tutmak**: Her soru ve cevabı bir `yapı`da durur; bütün sorular da
bir listede:

```
yapı Soru {
    soru: &'kalıcı str,
    cevap: &'kalıcı str,
}

tanımla sorular = liste![
    Soru { soru: "Türkiye'nin başkenti neresi?", cevap: "ankara" },
    Soru { soru: "Bir haftada kaç gün var?", cevap: "7" },
];
```

`&'kalıcı str` program boyunca yaşayan bir metin demektir: tırnak içine
yazdığın metinler böyledir.

**Soruları sormak**: `numaralandır` her soruyu sıra numarasıyla verir.
`biçimle!` soruyu numarasıyla birlikte bir metne çevirir:

```
her (sıra, soru) için sorular.elemanlar().numaralandır() {
    tanımla cevap = oku!(biçimle!("{}. {} ", sıra + 1, soru.soru));
    …
}
```

**Puan ve yorum**: Doğru cevaplar `puan` değişkeninde birikir. `yorum`
işlevi yüzdeye bakıp `eşle` ile bir yorum seçer; `50..=99` "50'den 99'a
kadar" demektir.

## Kendini geliştir

1. Kendi sorularını ekle; bir konu seç (hayvanlar, gezegenler, futbol…).
2. Her doğru cevaptan sonra o ana kadarki puanı yazdır.
3. Soruları karışık sırayla sor. İpucu: listeyi `değişken` yap, `rastgele!`
   ile bir sıra seç ve o soruyu `sorular.sil(sıra)` ile listeden çıkarıp sor.

---

Sonraki ders: [Proje: Hesap makinesi](../18-hesap-makinesi/README.md)
