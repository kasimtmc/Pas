# Pas: Türkçe Rust

[![CI](https://github.com/kasimtmc/Pas/actions/workflows/ci.yml/badge.svg)](https://github.com/kasimtmc/Pas/actions/workflows/ci.yml)

Pas, Rust programlama dilini Türkçe anahtar sözcüklerle yazmayı sağlar.
Programlamaya yeni başlayan, İngilizce terimlere tam hakim olmayan kişiler
için hazırlandı. Odak aldığı grup çocuklar ve gençlerdir. Yazılan kod
gerçek Rust'a çevrilip Rust derleyicisiyle derlenir. Hata mesajları da
Türkçe gösterilir ve Pas kodundaki doğru satırı işaret eder.

```
işlev ana() {
    yap değişken sayılar = liste![3, 1, 4];
    sayılar.ekle(1);

    her sayı içinde sayılar.yinele() {
        eğer sayı % 2 == 0 {
            yazdır!("{sayı} çift");
        } değilse {
            yazdır!("{sayı} tek");
        }
    }
}
```

Hiçbir şey kurmadan denemek için [**Pas Oyun Alanı**](https://kasimtmc.github.io/Pas/)nı
aç: kodunu tarayıcıda yazıp çalıştırabilir, dersleri oradan açabilir ve
programının bağlantısını paylaşabilirsin.

Programlamaya yeni başlıyorsan [**dersler**](dersler/README.md) dizinindeki
adım adım derslerle başlayabilirsin: yazdırmaktan başlayıp bir taş, kâğıt,
makas oyunu yazmaya, oradan da sözlükler ve nitelikler gibi konulara uzanan
on dört ders.

## Kurulum

Önce [Rust](https://rustup.rs) kurulu olmalı. Sonra `cargo pas` komutunu kur.

Linux ve macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.sh | sh
```

Windows (PowerShell):

```powershell
irm https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.ps1 | iex
```

Bu betikler hazır derlenmiş `cargo-pas` dosyasını
[son yayından](https://github.com/kasimtmc/Pas/releases/latest) indirip
`~/.cargo/bin` dizinine koyar. RustRover eklentisi de `cargo-pas`'ı
bulamazsa onu indirip kurmayı önerir. Kendin derlemek istersen:

```bash
cargo install --git https://github.com/kasimtmc/Pas cargo-pas
```

Editörde tanıma gitme, belge gösterme ve akıllı tamamlama için
rust-analyzer da gerekir:

```bash
rustup component add rust-analyzer
```

Depoyu indirdiysen `cargo install --path crates/cargo-pas` da olur.

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
| `cargo pas lsp` | Editörler için dil sunucusu (LSP); rust-analyzer gerekir |

Komutların İngilizce (`run`, `build`, …) ve ASCII (`calistir`, `cevir`, …)
adları da çalışır.

## RustRover eklentisi

`eklenti/` dizinindeki eklenti RustRover'a (ve diğer JetBrains IDE'lerine)
Pas desteği ekler:

- `.pas` dosyaları için renklendirme
- Türkçe sözcükleri ve Rust karşılıklarını gösteren kod tamamlama
- `işlev ana()` satırının yanında ▶ Çalıştır düğmesi
- Derleyici hatalarının, dosya kaydedilmeden de editörde Türkçe gösterilmesi
- Derleyicinin önerdiği düzeltmeleri Alt+Enter ile uygulama (örneğin
  `yap x` → `yap değişken x`, `isim` → `isim.kopyala()`)
- Kod katlama, otomatik girinti (Ctrl+Alt+L girintiyi düzeltir) ve
  işlevleri, yapıları, `uygula` bloklarını listeleyen yapı görünümü
- Tanıma gitme (Ctrl+tık), kullanıldığı yerleri bulma (Alt+F7), üzerine
  gelince belge gösterme ve rust-analyzer destekli akıllı tamamlama; bunlar
  için rust-analyzer gerekir: `rustup component add rust-analyzer`
- Ctrl+/ ile yorum satırı, parantez eşleme
- Çalıştırma çıktısındaki `--> src/ana.pas:3:5` konumlarına tıklayınca o satıra gitme

Eklenti `cargo-pas` aracını kullanır, önce onu kur (yukarıya bak).

Eklentinin hazır paketi her değişiklikte otomatik olarak üretilir:
[GitHub Actions](https://github.com/kasimtmc/Pas/actions/workflows/ci.yml)
sayfasında son başarılı çalışmayı aç ve **pas-eklenti** dosyasını indir.
İçindeki `pas-eklenti-*.zip` dosyasını RustRover'da
**Ayarlar → Eklentiler → ⚙ → Diskten eklenti yükle**
(Settings → Plugins → ⚙ → Install Plugin from Disk) ile kurabilirsin.

Eklentiyi kendin derlemek istersen JDK 25 gerekir:

```bash
cd eklenti && ./gradlew buildPlugin
```
Eklentiyi kurmadan, ayrı bir deneme RustRover'ında açmak için:

```bash
cd eklenti && ./gradlew runIde
```

## VS Code eklentisi

`vscode/` dizinindeki eklenti aynı dil sunucusunu kullanır: renklendirme,
Türkçe hatalar ve hızlı düzeltmeler (Ctrl+.), `işlev ana()` üstünde
**▶ Çalıştır**, tanıma gitme (F12), belge gösterme ve tamamlama.
`cargo-pas` bulunamazsa onu kuran komutu terminalde çalıştırmayı önerir.

Hazır paket (`pas-vscode-*.vsix`) [son yayında](https://github.com/kasimtmc/Pas/releases/latest);
VS Code'da **Uzantılar → ⋯ → VSIX'ten yükle** (Install from VSIX) ile
kurulur. Kendin derlemek için:

```bash
cd vscode && npm ci && npm run paketle
```

## Hata mesajları

```
hata[E0384]: `x` değiştirilemez bir değişken; ona ikinci kez değer verilemez
 --> src/ana.pas:3:5
  |
2 |     yap x = 5;
  |         - `x` için ilk değer burada verildi
3 |     x = 6;
  |     ^^^^^ değiştirilemez değişkene ikinci kez değer verilemez
  |
  = yardım: değerini sonradan değiştirebilmek için başına `değişken` ekle
  |
2 |     yap değişken x = 5;
  = açıklama: Pas'ta değişkenler kendiliğinden değiştirilemez. …
```

Program çalışırken çökerse (panik) mesaj yine Türkçe gösterilir ve çöktüğü
satır işaretlenir.

## Dil

Pas, Rust'ın sözdizimini olduğu gibi kullanır. Değişen tek şey sözcüklerdir.
Bu sayede Pas öğrenen biri ileride gerçek Rust'a kolayca geçebilir.

- **Anahtar sözcükler:** `işlev` (fn), `yap` (let), `değişken` (mut),
  `eğer`/`değilse` (if/else), `her … içinde` (for … in), `sürece` (while),
  `döngü` (loop), `eşle` (match), `yapı` (struct), `çeşit` (enum),
  `uygula` (impl), `nitelik` (trait), `genel` (pub), `döndür` (return),
  `doğru`/`yanlış` (true/false) …
- **Sık kullanılanlar:** `yazdır!` (println!), `yaz!` (print!), `liste!` (vec!),
  `Metin` (String), `Liste` (Vec), `Belki`/`Var`/`Yok` (Option/Some/None),
  `Sonuç`/`Tamam`/`Hata` (Result/Ok/Err), `Liste::yeni()` (new), `.uzunluk()` (len),
  `.ekle()` (push), `.al()` (get), `.yoksa()` (unwrap_or), `.sırala()` (sort),
  `.toplam()` (sum), `.süz()` (filter), `.dönüştür()` (map) …

Pas programları değerleri de Türkçe yazdırır: `yazdır!("{:?}", Var(doğru))`
ekrana `Var(doğru)` yazar, `Some(true)` değil.

### Klavyeden okuma

`oku!` makrosu kullanıcıdan bir satır okur:

```
yap ad = oku!("Adın ne? ");                // Metin
yap yaş = oku!("Kaç yaşındasın? ", u32);   // sayı
yap boy = oku!("Boyun kaç metre? ", f64);  // 1,45 da 1.45 de olur
yap devam = oku!("Devam mı? ", mantıksal); // evet/hayır, doğru/yanlış
```

Sayı beklenirken başka bir şey yazılırsa `oku!` "`on iki` geçerli bir
tam sayı değil, tekrar dene." diyerek yeniden sorar.

### Rastgele sayılar

`rastgele!` iki sayı arasında (ikisi de dahil) rastgele bir sayı seçer:

```
yap zar = rastgele!(1, 6);
yap tutulan: u32 = rastgele!(1, 100);
yap oran = rastgele!(0.0, 1.0);            // ondalıklı sayılar da olur
```

Örnek bir oyun için [`ornekler/sayi-tahmin`](ornekler/sayi-tahmin/src/ana.pas)
dizinine bak.

Tam liste için `cargo pas sözlük` komutunu çalıştır ya da
[`sozluk.rs`](crates/pas-cekirdek/src/sozluk.rs) dosyasına bak.

Bilinmesi gerekenler:

- İngilizce Rust sözcükleri de geçerlidir; Pas ve Rust aynı dosyada birlikte
  yazılabilir.
- Türkçe bir anahtar sözcüğü isim olarak kullanmak için başına `r#` koy:
  `yap r#tür = 3;`
- `Cargo.toml` içindeki `[[bin]] path = "target/pas/src/main.rs"` satırı
  IDE'lerin projeyi tanıması içindir; `cargo pas yeni` bunu kendisi yazar.
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
  pas-web/        Oyun alanının tarayıcıda (WebAssembly) çalışan parçası
oyun-alani/       Oyun alanı sayfası (HTML, CSS, JavaScript)
eklenti/          RustRover eklentisi (Kotlin)
vscode/           VS Code eklentisi (TypeScript)
dersler/          Yeni başlayanlar için adım adım dersler
ornekler/         Örnek Pas projeleri
```

`cargo pas lsp` ise editörle rust-analyzer arasında bir köprüdür: açık
`.pas` dosyalarını çevirip rust-analyzer'a Rust olarak gösterir, cevaplardaki
konumları `.pas` dosyalarına, isimleri ve kod parçalarını Pas'a çevirir
(`push` → `ekle`). LSP destekleyen her editör onu kullanabilir.

`cargo pas`, projeyi `target/pas/` altında bir "gölge" Rust projesine
çevirir (`src/ana.pas` → `src/main.rs`). Ardından cargo'yu
`--message-format=json` ile çalıştırır. Derleyicinin bildirdiği konumlar,
kaynak eşlemesi kullanılarak Pas dosyasındaki konumlara geri çevrilir.
Çeviri satır eklemediği için satır numaraları hep aynı kalır.

## Geliştirme

```bash
cargo test
```

```bash
cd eklenti && ./gradlew test
```

Yeni bir sürüm çıkarmak için:

```bash
betikler/surum.sh 0.2.0 "Yenilikler / What's new"
```

Betik sürüm numarasını her yerde günceller, testleri çalıştırır ve
`v0.2.0` etiketini oluşturur. Etiket gönderilince (`git push origin main v0.2.0`)
yayın iş akışı hazır dosyaları ve eklentileri GitHub Releases'a koyar.
Depo ayarlarında `OVSX_PAT` gizli değişkeni tanımlıysa VS Code eklentisi
[Open VSX](https://open-vsx.org)'e de gönderilir.

`crates/cargo-pas/tests/hata_derlemi/` dizini, çocukların sık yapacağı
hataları içeren küçük Pas programlarından oluşur. Testler bunların hepsini
derleyip mesajların Türkçeye çevrildiğini denetler. Çevirisi olmayan yeni
bir hata gördüğünde buraya bir örnek ekleyip
[`mesajlar.rs`](crates/pas-cekirdek/src/mesajlar.rs) dosyasına çevirisini yaz.

Oyun alanı, çeviriyi tarayıcıda WebAssembly ile yapar; derleyip çalıştırmak
için çevrilen Rust kodunu [Rust oyun alanına](https://play.rust-lang.org)
gönderir. Gelen derleyici çıktısı yine tarayıcıda Pas'a eşlenip
Türkçeleştirilir. Yerelde denemek için:

```bash
betikler/oyun-alani.sh && python3 -m http.server -d target/oyun-alani 8000
```

`main` dalına gönderilen değişiklikler oyun alanına kendiliğinden yansır.

Örnek projeyi çalıştırmak için:

```bash
cd ornekler/tanitim && cargo run --manifest-path ../../Cargo.toml -p cargo-pas -- pas çalıştır
```

## Yol haritası

- [x] Çevirici çekirdeği ve `cargo pas` komutu
- [x] RustRover eklentisi: renklendirme, tamamlama, Çalıştır düğmesi,
      Türkçe hata gösterimi
- [x] Eklenti: Türkçe hata önerilerini tek tıkla uygulama (hızlı düzeltme)
- [x] Eklenti: blok ve öğe yapısı (girinti, katlama, yapı görünümü)
- [x] Yeni başlayanlar için Türkçe dersler
- [x] `cargo pas lsp`: rust-analyzer üzerine kurulu dil sunucusu (tanıma
      gitme, kullanımlar, hover, akıllı tamamlama)
- [x] VS Code eklentisi (aynı dil sunucusunu kullanır)
- [x] Klavyeden kolay girdi okumak için `oku!` makrosu
- [x] Kurulum gerektirmeyen, tarayıcıda çalışan oyun alanı
- [ ] Daha fazla Türkçe hata mesajı ve açıklama

## Lisans

Pas, [MIT](LICENSE-MIT) ya da [Apache-2.0](LICENSE-APACHE) lisanslarından
biriyle (hangisini istersen) kullanılabilir. Rust'ın kendisi de aynı ikili
lisansı kullanır.

Aksi açıkça belirtilmedikçe, bu projeye gönderdiğin katkılar Apache-2.0
lisansında tanımlandığı biçimde, ek koşul olmadan yukarıdaki iki lisansla
lisanslanmış sayılır.
