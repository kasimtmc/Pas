# Pas (Türkçe Rust)

**Pas** lets you write Rust with Turkish keywords. It is made for people who are new to programming
and not yet comfortable with English terms; its main audience is children and teenagers. Pas code is
translated to real Rust and compiled with the Rust compiler, and compiler errors are shown in Turkish.

**Pas**, Rust programlama dilini Türkçe anahtar sözcüklerle yazmayı sağlar. Programlamaya yeni
başlayan, İngilizce terimlere tam hakim olmayan kişiler için hazırlandı. Odak aldığı grup çocuklar
ve gençlerdir.

```
işlev ana() {
    tanımla ad = oku!("Adın ne? ");
    her i için 1..=3 {
        yazdır!("{i}. Merhaba {ad}!");
    }
}
```

## Özellikler

- `.pas` dosyaları için renklendirme
- Derleyici hatalarının Türkçe gösterilmesi ve hızlı düzeltmeler (Ctrl+.)
- `işlev ana()` üstünde **▶ Çalıştır** bağlantısı
- Tanıma gitme (F12), kullanıldığı yerleri bulma, üzerine gelince belge, akıllı tamamlama

## Kurulum

1. [Rust](https://rustup.rs)'ı kur.
2. `cargo pas` komutunu kur (eklenti bulamazsa bunu senin yerine terminalde çalıştırmayı önerir):
   - Linux ve macOS: `curl -fsSL https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.sh | sh`
   - Windows: `irm https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.ps1 | iex`
3. Tanıma gitme ve tamamlama için: `rustup component add rust-analyzer`

Yeni başlayanlar için Türkçe dersler ve daha fazlası:
[github.com/kasimtmc/Pas](https://github.com/kasimtmc/Pas)
