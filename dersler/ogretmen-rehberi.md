# Öğretmen rehberi

Pas, Rust programlama dilini Türkçe anahtar sözcüklerle yazmayı sağlar.
Programlamaya yeni başlayanlar, özellikle İngilizceyi henüz iyi bilmeyen
çocuklar ve gençler için hazırlandı. Yazılan kod gerçek Rust'a çevrilip
gerçek Rust derleyicisiyle çalıştırılır; hata mesajları Türkçedir ve Pas
kodundaki doğru satırı gösterir.

Bu rehber, Pas'ı sınıfta kullanmak isteyen öğretmenler içindir.

## Kurulum gerektirmeden: Pas Oyun Alanı

En kolay yol **[Pas Oyun Alanı](https://kasimtmc.github.io/Pas/)**dır.
Hiçbir şey kurmak, hesap açmak ya da ad vermek gerekmez.

- **Gerekenler:** Güncel bir tarayıcı (Chrome, Edge, Firefox ya da Safari)
  ve internet. Tablet ve telefonda da açılır; ama kod yazmak için klavyeli
  bir bilgisayar çok daha rahattır.
- **Okul ağı:** `kasimtmc.github.io` ve `play.rust-lang.org` adreslerine
  erişim açık olmalı. Kodlar derlenip çalıştırılmak için Rust'ın resmî
  oyun alanına (play.rust-lang.org) gönderilir.
- **İlk açılış:** Oyun alanını ilk kez açan öğrenci doğrudan 1. derse başlar.
  Sol tarafta dersin anlatımı, ortada kod, sağda çıktı vardır.

### Öğrenci ne yapar?

1. Dersin anlatımını okur ve **▶ Çalıştır**'a basarak örnek kodu çalıştırır.
2. Kodda küçük değişiklikler yapıp sonucunu görür.
3. Anlatımın sonundaki **Görevler** bölümünden bir görev seçer
   (**Göreve başla**), kodunu yazar ve **✓ Denetle**'ye basar.
4. Pas çıktının doğru olup olmadığını söyler. Yanlışsa ilk farkı gösterir:
   "3. satırda `6` bekleniyordu ama `7` çıktı."
5. Görev tamamlanınca bir sonraki göreve ya da derse geçer.

Takılan öğrenci için her görevde bir **İpucu** ve bir **Çözümü göster**
bölümü vardır. Öğrencilerden önce kendileri denemelerini, ipucuna bakmalarını,
çözümü en son açmalarını isteyebilirsiniz.

### Bağlantılar

- Bir dersi doğrudan açmak için: `https://kasimtmc.github.io/Pas/#ders=03-soru-sor`
  (dersin adını aşağıdaki tablodan alın). Bu bağlantıyı tahtaya yazabilir ya
  da öğrencilere gönderebilirsiniz.
- **Paylaş** düğmesi, öğrencinin kodunu içeren bir bağlantı oluşturur. Kod
  bağlantının içindedir; hiçbir sunucuda saklanmaz. Öğrenciler ödevlerini bu
  bağlantıyla size gönderebilir; bağlantıyı açtığınızda kodlarını görür ve
  çalıştırabilirsiniz.

### Kendi görevini hazırlamak

Derslerdeki görevlerin dışında kendi görevlerinizi de hazırlayabilirsiniz.
Oyun alanının altındaki **görev hazırla** bağlantısına (ya da
`https://kasimtmc.github.io/Pas/#gorev-hazirla` adresine) gidin:

1. Başlığı, açıklamayı ve isterseniz bir ipucunu yazın.
2. Öğrencinin başlayacağı kodu ve görevin bir çözümünü yazın; düzenleyicide
   yazdığınız kodu düğmeyle alabilirsiniz.
3. Görev bir şey yazdırıyorsa örnekler ekleyin: her örnekte programın
   okuyacağı girdi (isteğe bağlı) ve beklenen çıktı vardır.
   **Beklenen çıktıları çözümden üret** düğmesi çözümü çalıştırıp çıktıları
   sizin yerinize doldurur. Kaplumbağa görevlerinde "çizimi denetle"yi
   seçin; öğrencinin çizimi çözümünkiyle karşılaştırılır.
4. **Önizle** ile görevi deneyin, **Bağlantı oluştur** ile bağlantıyı alıp
   öğrencilere gönderin.

Görev, bağlantının içinde saklanır; hiçbir sunucuya kaydedilmez. Çözümün
öğrencilere görünmesini istemiyorsanız ilgili kutuyu boş bırakın; ama
çözüm bağlantının içinde durduğu için meraklı bir öğrencinin onu bulabileceğini
unutmayın. Görevi sonradan değiştirmek için bağlantıyı açıp
**Bu görevi düzenle**'ye basın ve yeni bir bağlantı oluşturun.

### Programlarım

Öğrenciler yazdıkları programları **Programlarım** düğmesiyle adlandırıp
kaydedebilir; açık bir programdaki her değişiklik kendiliğinden kaydedilir.
Ders ya da görev açılınca program kapanır, böylece dersin kodu öğrencinin
programının üzerine yazılmaz. Programlar da yalnızca o tarayıcıda saklanır.

### İlerleme ve gizlilik

- Tamamlanan görevler ve son açılan ders yalnızca **o tarayıcıda** saklanır.
  Ortak kullanılan bilgisayarlarda öğrenciler birbirinin ilerlemesini görür;
  tarayıcı verileri silinince ilerleme sıfırlanır.
- Oyun alanı ad, e-posta ya da başka bir kişisel bilgi istemez ve toplamaz.
  Çalıştırılan kod derlenmek için play.rust-lang.org'a gönderilir.
- **Geri bildirim gönder** bağlantısı GitHub'da herkese açık bir form açar
  (GitHub hesabı gerekir). Öğrencilerin oraya kişisel bilgi yazmamasını
  hatırlatın.

## Ders planı

Her ders tek bir kavramı ele alır. Süreler 40 dakikalık bir ders saati için
öneridir; görevleri bitiren öğrenciler dersin "Kendin dene" alıştırmalarına
geçebilir.

| Ders | Bağlantı | Konular | Görev | Süre |
|---|---|---|---|---|
| 1. Merhaba dünya! | `#ders=01-merhaba` | `işlev ana`, `yazdır!`, `yaz!`, yorumlar | 2 | 1 ders saati |
| 2. Değişkenler | `#ders=02-degiskenler` | `tanımla`, `değişken`, sayılar, dört işlem | 2 | 1 |
| 3. Soru sormak | `#ders=03-soru-sor` | `oku!`, sayı ve metin okuma | 2 | 1 |
| 4. Kararlar | `#ders=04-kararlar` | `eğer`, `değilse`, karşılaştırma, `&&`, `\|\|` | 2 | 1–2 |
| 5. Döngüler | `#ders=05-donguler` | `her … için`, `sürece`, `döngü`, `kır` | 3 | 2 |
| 6. İşlevler | `#ders=06-islevler` | işlev yazmak, parametre, değer döndürmek | 2 | 1–2 |
| 7. Listeler | `#ders=07-listeler` | `liste!`, `ekle`, `uzunluk`, listede dolaşmak | 2 | 1–2 |
| 8. Yapılar | `#ders=08-yapilar` | `yapı`, `uygula`, yöntemler, `kendi` | 2 | 2 |
| 9. Çeşitler ve eşle | `#ders=09-cesitler-ve-esle` | `çeşit`, `eşle`, `Belki` | 2 | 2 |
| 10. Proje: Taş, kâğıt, makas | `#ders=10-tas-kagit-makas` | önceki derslerin hepsi, `rastgele!` | 1 | 2 |
| 11. Listelerle sihir | `#ders=11-yineleyiciler` | kapanışlar, `dönüştür`, `süz`, `biriktir` | 2 | 2 |
| 12. Sözlükler | `#ders=12-sozlukler` | `Sözlük`, `yerleştir`, `al`, sayaçlar | 2 | 1–2 |
| 13. Hatalarla başa çıkmak | `#ders=13-sonuc-ve-hatalar` | `Sonuç`, `Tamam`/`Hata`, `?` | 2 | 2 |
| 14. Nitelikler | `#ders=14-nitelikler` | `nitelik`, `uygula … için …`, `dinamik` | 1 | 2 |
| 15. Kaplumbağa ile çizim | `#ders=15-kaplumbaga` | `Kaplumbağa`, döngülerle şekiller | 2 | 1–2 |

Kaplumbağa dersi (15) yalnızca döngüleri ve işlevleri gerektirir. Görsel
sonuç veren çalışmaları seven sınıflarda 6. dersten hemen sonra da işlenebilir.

### Bir ders saati için örnek akış

1. **Hatırlatma (5 dk):** Önceki derste ne öğrendik? Bir öğrencinin
   paylaştığı kodu birlikte çalıştırın.
2. **Anlatım (10 dk):** Dersin örnek kodunu tahtada (ya da ekranda)
   çalıştırın. Kodu birlikte değiştirin: "Burayı 5 yerine 50 yaparsak ne olur?"
3. **Görevler (20 dk):** Öğrenciler görevleri kendi başlarına ya da ikişer
   kişilik gruplar halinde yapar. Bitirenler "Kendin dene" alıştırmalarına
   geçer.
4. **Paylaşım (5 dk):** Birkaç öğrenci çözümünü gösterir. Farklı çözümler
   olduğunu görmek öğreticidir.

## Hata mesajlarını okumayı öğretmek

Pas'ın en önemli özelliklerinden biri Türkçe hata mesajlarıdır. Öğrencileri
hata yapmaktan korkmamaya ve mesajı okumaya teşvik edin. Kırmızı dalgalı
çizginin üzerine gelindiğinde hata mesajı orada da görünür; mesajın altındaki
💡 açıklaması hatanın ne anlama geldiğini çocukların anlayacağı dille anlatır.

Yeni başlayanların en sık yaptığı hatalar:

| Hata | Pas ne der? | Ne yapmalı? |
|---|---|---|
| Satır sonunda `;` unutmak | `;` bekleniyordu | Satırın sonuna `;` koy. |
| Değişeceği halde `değişken` yazmamak | `x` değiştirilemez bir değişken; ona ikinci kez değer verilemez | `tanımla değişken x = …;` |
| Karşılaştırmada `=` kullanmak | türler uyuşmuyor (yardım: karşılaştırmak istediysen `=` yerine `==` yaz) | `eğer x == 5` |
| Tırnağı kapatmamak | kapatılmamış dizgi | Metnin sonuna `"` ekle. |
| Sayı yerine metin okumak | `oku!` metin okur; sayı okumak için türünü de yaz | `oku!("Yaşın? ", u32)` |
| Listeyi `{}` ile yazdırmak | `Liste<i32>` doğrudan `{}` ile yazdırılamaz | `yazdır!("{:?}", liste);` |
| Bir adı yanlış yazmak | `sayi` adında bir değer bulunamadı (yardım: benzer adlı bir yerel değişken var) | Önerilen adı kullan. |

Pas'ın önerdiği düzeltmeler (yeşil satırlar) çoğu zaman doğrudur;
öğrencilerden öneriyi körü körüne kopyalamak yerine neden önerildiğini
düşünmelerini isteyin.

## Sorun giderme

- **"Rust oyun alanına ulaşılamadı":** İnternet bağlantısı yok ya da okul
  ağı `play.rust-lang.org` adresini engelliyor.
- **"Program çok uzun sürdü ve durduruldu":** Büyük ihtimalle bitmeyen bir
  döngü vardır (örneğin `sürece` koşulundaki değişken hiç değişmiyordur).
  Programlar en fazla birkaç saniye çalışabilir.
- **Program soru soruyor:** `oku!` kullanan programlarda çıktının altında
  bir yazma alanı açılır; cevabı yazıp Enter'a basmak yeterlidir.
- **Çizim görünmüyor:** Kaplumbağa programı bitince çizer; programın hata
  vermeden bittiğinden emin olun.

## Bilgisayara kurulum

Oyun alanı derslerin hepsi için yeterlidir. Daha büyük projeler, birden çok
dosya ya da internetsiz çalışma için Pas bilgisayara kurulabilir ve RustRover
ya da VS Code ile kullanılabilir. Kurulum için ana
[README](../README.md#kurulum) dosyasına bakın.

## Geri bildirim

Pas'ı sınıfta kullandıysanız deneyimlerinizi duymak isteriz: hangi dersler
iyi gitti, öğrenciler nerede zorlandı, hangi hata mesajı anlaşılmadı?
Oyun alanındaki **Geri bildirim gönder** bağlantısını ya da
[GitHub'daki formu](https://github.com/kasimtmc/Pas/issues/new?template=geri-bildirim.yml)
kullanabilirsiniz.
