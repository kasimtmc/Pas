# Pas: Türkçe Rust

Pas, Rust programlama dilini Türkçe anahtar sözcüklerle yazmayı sağlar.
Henüz İngilizce bilmeyen çocuklar ve gençler için hazırlandı. Yazılan kod
gerçek Rust'a çevrilip Rust derleyicisiyle derlenir. Hata mesajları da
Türkçe gösterilir ve Pas kodundaki doğru satırı işaret eder.

```
işlev ana() {
    olsun değişken sayılar = liste![3, 1, 4];
    sayılar.ekle(1);

    için sayı içinde sayılar.yinele() {
        eğer sayı % 2 == 0 {
            yazdır!("{sayı} çift");
        } değilse {
            yazdır!("{sayı} tek");
        }
    }
}
```

## Kurulum

[Rust](https://rustup.rs) kurulu olmalı. Sonra bu depoda:

```bash
cargo install --path crates/cargo-pas
```

## Kullanım

```bash
cargo pas yeni ilk-oyunum
```

```bash
cd ilk-oyunum
```

```bash
cargo pas çalıştır
```

| Komut | Ne yapar |
|---|---|
| `cargo pas yeni <ad>` | Yeni bir proje oluşturur (`src/ana.pas`) |
| `cargo pas çalıştır` | Derler ve çalıştırır (`-- argümanlar` programa gider) |
| `cargo pas derle` | Derler (`--yayın`: hızlı çalışan sürüm) |
| `cargo pas denetle` | Derlemeden hatalara bakar |
| `cargo pas çevir dosya.pas` | Rust karşılığını gösterir (`--ters`: Rust → Pas) |
| `cargo pas sözlük` | Bütün Türkçe sözcükleri listeler (`--json` da var) |

Komutların İngilizce (`run`, `build`, …) ve ASCII (`calistir`, `cevir`, …)
adları da çalışır.

## Hata mesajları

```
hata[E0384]: `x` değiştirilemez bir değişken; ona ikinci kez değer verilemez
 --> src/ana.pas:3:5
  |
2 |     olsun x = 5;
  |           - `x` için ilk değer burada verildi
3 |     x = 6;
  |     ^^^^^ değiştirilemez değişkene ikinci kez değer verilemez
  |
  = yardım: bu değişkeni `değişken` yapmayı düşün
  |
2 |     olsun değişken x = 5;
  = açıklama: Pas'ta değişkenler kendiliğinden değiştirilemez. …
```

Program çalışırken çökerse (panik) mesaj yine Türkçe gösterilir ve çöktüğü
satır işaretlenir.

## Dil

Pas, Rust'ın sözdizimini olduğu gibi kullanır. Değişen tek şey sözcüklerdir.
Bu sayede Pas öğrenen biri ileride gerçek Rust'a kolayca geçebilir.

- **Anahtar sözcükler:** `işlev` (fn), `olsun` (let), `değişken` (mut),
  `eğer`/`değilse` (if/else), `için … içinde` (for … in), `sürece` (while),
  `döngü` (loop), `eşle` (match), `yapı` (struct), `çeşit` (enum),
  `uygula` (impl), `nitelik` (trait), `genel` (pub), `döndür` (return),
  `doğru`/`yanlış` (true/false) …
- **Sık kullanılanlar:** `yazdır!` (println!), `yaz!` (print!), `liste!` (vec!),
  `Metin` (String), `Liste` (Vec), `Belki`/`Var`/`Yok` (Option/Some/None),
  `Sonuç`/`Tamam`/`Hata` (Result/Ok/Err), `.uzunluk()` (len), `.ekle()` (push) …

Tam liste için `cargo pas sözlük` komutunu çalıştır ya da
[`dictionary.rs`](crates/pas-cekirdek/src/dictionary.rs) dosyasına bak.

Bilinmesi gerekenler:

- İngilizce Rust sözcükleri de geçerlidir; Pas ve Rust aynı dosyada birlikte
  yazılabilir.
- Türkçe bir anahtar sözcüğü isim olarak kullanmak için başına `r#` koy:
  `olsun r#tür = 3;`
- `src/ana.pas` programın başladığı dosyadır (`main.rs`),
  `src/kütüphane.pas` ise kütüphane dosyasıdır (`lib.rs`).
- Modül adları Türkçe karakter içerebilir: `modül şekiller;` →
  `src/şekiller.pas`.
- `yaz!` yazdığını hemen ekrana basar; böylece kullanıcıdan girdi
  beklemeden önce sorulan soru ekranda görünür.

## Nasıl çalışır?

```
crates/
  pas-cekirdek/   Çevirici: sözcük çözümleyici, sözlük, Pas ⇄ Rust çevirisi,
                  kaynak eşlemesi, Türkçe derleyici ve panik mesajları
  cargo-pas/      `cargo pas` komutu
ornekler/         Örnek Pas projeleri
```

`cargo pas`, projeyi `target/pas/` altında bir "gölge" Rust projesine
çevirir (`src/ana.pas` → `src/main.rs`). Ardından cargo'yu
`--message-format=json` ile çalıştırır. Derleyicinin bildirdiği konumlar,
kaynak eşlemesi kullanılarak Pas dosyasındaki konumlara geri çevrilir.
Çeviri satır eklemediği için satır numaraları hep aynı kalır.

## Geliştirme

```bash
cargo test
```

Örnek projeyi çalıştırmak için:

```bash
cd ornekler/tanitim && cargo run --manifest-path ../../Cargo.toml -p cargo-pas -- pas çalıştır
```

## Yol haritası

- [x] Çevirici çekirdeği ve `cargo pas` komutu
- [ ] RustRover eklentisi: renklendirme, tamamlama, Çalıştır düğmesi,
      Türkçe hata gösterimi
- [ ] `pas-lsp`: rust-analyzer üzerine kurulu dil sunucusu (tanıma gitme,
      akıllı tamamlama; VS Code desteği)
- [ ] Klavyeden kolay girdi okumak için yardımcılar
- [ ] Daha fazla Türkçe hata mesajı ve açıklama
