// Pas oyun alanı: Pas kodunu tarayıcıda Rust'a çevirir (pas.wasm), Rust oyun
// alanında (play.rust-lang.org) derletip çalıştırır ve sonucu Türkçe gösterir.

const SURUM = new URL(import.meta.url).searchParams.get("s") ?? "";
const OYUN_ALANI = "https://play.rust-lang.org/execute";
const DEPO = "https://github.com/kasimtmc/Pas";
const KAYIT_ANAHTARI = "pas-oyun-alani-kod";
const DERS_ANAHTARI = "pas-oyun-alani-ders";
const GOREV_ANAHTARI = "pas-oyun-alani-gorevler";

const $ = (kimlik) => document.getElementById(kimlik);
const kod = $("kod");
const girdi = $("girdi");
const cikti = $("cikti");
const durum = $("durum");
const calistirDugmesi = $("calistir");

// ---- WebAssembly ----

let pas = null;
const kodlayici = new TextEncoder();
const cozucu = new TextDecoder();

async function pasYukle() {
  const yanit = await fetch(`pas.wasm?s=${SURUM}`);
  if (!yanit.ok) throw new Error(`pas.wasm: HTTP ${yanit.status}`);
  const { instance } = await WebAssembly.instantiate(await yanit.arrayBuffer(), {});
  const x = instance.exports;
  // Her işlev JSON alır, JSON döndürür (bkz. crates/pas-web/src/wasm.rs).
  const cagir = (islev, deger) => {
    const baytlar = kodlayici.encode(JSON.stringify(deger));
    const adres = x.bellek_ayir(baytlar.length);
    new Uint8Array(x.memory.buffer, adres, baytlar.length).set(baytlar);
    const uzunluk = x[islev](adres, baytlar.length);
    x.bellek_birak(adres, baytlar.length);
    return JSON.parse(cozucu.decode(new Uint8Array(x.memory.buffer, x.sonuc_adresi(), uzunluk)));
  };
  pas = {
    derle: (kaynak, girdi, tohum) => cagir("pas_derle", { kaynak, girdi, tohum }),
    coz: (kaynak, girdi, tohum, hata_ciktisi) => cagir("pas_coz", { kaynak, girdi, tohum, hata_ciktisi }),
    rust: (kaynak) => cagir("pas_rust", { kaynak }),
    renklendir: (kaynak) => cagir("pas_renklendir", { kaynak }),
    sozluk: () => cagir("pas_sozluk", {}),
  };
}

// ---- Düzenleyici ----

const kacis = (metin) => metin.replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" })[c]);

/** Hata işaretleri: [{bas, son, tur}] (UTF-16 konumları) */
let isaretler = [];
let hataliSatirlar = new Map();

function renklendir() {
  const metin = kod.value;
  let html = "";
  let konum = 0;
  if (pas) {
    for (const [bas, son, sinif] of pas.renklendir(metin)) {
      html += kacis(metin.slice(konum, bas));
      html += `<span class="r-${sinif}">${kacis(metin.slice(bas, son))}</span>`;
      konum = son;
    }
  }
  html += kacis(metin.slice(konum));
  // Son satır boşsa yükseklik kaymasın diye bir boşluk eklenir.
  $("renkli").innerHTML = html + "\n ";

  let isaretHtml = "";
  konum = 0;
  for (const { bas, son, tur } of [...isaretler].sort((a, b) => a.bas - b.bas)) {
    if (bas < konum) continue;
    const bitis = Math.max(son, bas + 1);
    isaretHtml += kacis(metin.slice(konum, bas));
    // Boş aralık ya da satır sonu: tek bir boşluk işaretlenir.
    const parca = metin.slice(bas, bitis).replace(/\n/g, " ") || " ";
    isaretHtml += `<mark class="${tur}">${kacis(parca)}</mark>`;
    konum = bitis;
  }
  isaretHtml += kacis(metin.slice(konum));
  $("isaretler").innerHTML = isaretHtml + "\n ";

  const satirSayisi = metin.split("\n").length;
  let numaralar = "";
  for (let i = 1; i <= satirSayisi; i++) {
    const tur = hataliSatirlar.get(i);
    numaralar += tur ? `<span class="${tur}">${i}</span>\n` : `${i}\n`;
  }
  $("satir-numaralari").innerHTML = numaralar;

  boyutlandir();
}

function boyutlandir() {
  kod.style.height = "auto";
  kod.style.width = "100%";
  kod.style.height = `${kod.scrollHeight}px`;
  if (kod.scrollWidth > kod.clientWidth) kod.style.width = `${kod.scrollWidth}px`;
}

let renklendirmeBekliyor = false;
function degisti() {
  if (isaretler.length) {
    // Kod değişince eski hata işaretleri yanlış yerleri gösterir.
    isaretler = [];
    hataliSatirlar = new Map();
  }
  if (!renklendirmeBekliyor) {
    renklendirmeBekliyor = true;
    requestAnimationFrame(() => {
      renklendirmeBekliyor = false;
      renklendir();
      rustuGuncelle();
    });
  }
  kaydet();
}

/** Seçimi bir metinle değiştirir; geri alma (Ctrl+Z) çalışmaya devam eder. */
function yaz(metin) {
  if (!document.execCommand("insertText", false, metin)) {
    kod.setRangeText(metin, kod.selectionStart, kod.selectionEnd, "end");
    degisti();
  }
}

const GIRINTI = "    ";

kod.addEventListener("input", degisti);
kod.addEventListener("keydown", (olay) => {
  if (olay.key === "Enter" && (olay.ctrlKey || olay.metaKey)) {
    olay.preventDefault();
    calistir();
    return;
  }
  const { selectionStart: bas, selectionEnd: son, value: metin } = kod;
  const satirBasi = metin.lastIndexOf("\n", bas - 1) + 1;
  if (olay.key === "Tab") {
    olay.preventDefault();
    if (olay.shiftKey) {
      // Seçili satırların girintisini azalt.
      const secimSonu = metin.indexOf("\n", son);
      const blok = metin.slice(satirBasi, secimSonu === -1 ? metin.length : secimSonu);
      const yeni = blok.replace(/^ {1,4}/gm, "");
      kod.setSelectionRange(satirBasi, satirBasi + blok.length);
      yaz(yeni);
      kod.setSelectionRange(satirBasi, satirBasi + yeni.length);
    } else if (bas !== son && metin.slice(bas, son).includes("\n")) {
      const blok = metin.slice(satirBasi, son);
      const yeni = blok.replace(/^/gm, GIRINTI);
      kod.setSelectionRange(satirBasi, son);
      yaz(yeni);
      kod.setSelectionRange(satirBasi, satirBasi + yeni.length);
    } else {
      yaz(GIRINTI);
    }
  } else if (olay.key === "Enter" && !olay.shiftKey) {
    // Yeni satır bir öncekinin girintisiyle başlar; `{` sonrasında bir kademe içeri.
    olay.preventDefault();
    const satir = metin.slice(satirBasi, bas);
    let girinti = satir.match(/^\s*/)[0];
    const oncesi = satir.trimEnd();
    const sonrasi = metin.slice(son).match(/^[^\n]*/)[0].trimStart();
    if (/[{([]$/.test(oncesi)) {
      if (/^[})\]]/.test(sonrasi)) {
        yaz(`\n${girinti}${GIRINTI}\n${girinti}`);
        const imlec = bas + 1 + girinti.length + GIRINTI.length;
        kod.setSelectionRange(imlec, imlec);
        return;
      }
      girinti += GIRINTI;
    }
    yaz(`\n${girinti}`);
  } else if (olay.key === "}" && bas === son && /^\s+$/.test(metin.slice(satirBasi, bas))) {
    // Boş bir satıra `}` yazılınca bir kademe dışarı alınır.
    olay.preventDefault();
    const girinti = metin.slice(satirBasi, bas);
    kod.setSelectionRange(satirBasi, bas);
    yaz(girinti.slice(0, Math.max(0, girinti.length - GIRINTI.length)) + "}");
  }
});

function kaydet() {
  try {
    localStorage.setItem(KAYIT_ANAHTARI, kod.value);
  } catch {
    // Tarayıcı kayda izin vermiyorsa sorun değil.
  }
}

function kodaGit(aralik) {
  if (!aralik) return;
  kod.focus();
  kod.setSelectionRange(aralik.bas, Math.max(aralik.son, aralik.bas));
  const satirYuksekligi = parseFloat(getComputedStyle($("duzenleyici")).lineHeight) || 22;
  $("duzenleyici").scrollTop = Math.max(0, (aralik.satir - 4) * satirYuksekligi);
}

// ---- Çalıştırma ----

/** Çıktı konsoluna yazılan cevaplarla doldurulan girdi; kullanıcı kutuya kendisi yazarsa korunur. */
let girdiKonsoldan = false;
girdi.addEventListener("input", () => (girdiKonsoldan = false));

let calisiyor = false;

/** `rastgele!` için tohum; cevaplarla yeniden çalıştırırken aynı kalır ki tutulan sayı değişmesin. */
let tohum = 1;
const yeniTohum = () => 1 + Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);

async function calistir({ yeniden = false } = {}) {
  if (!pas || calisiyor) return;
  if (!yeniden) {
    tohum = yeniTohum();
    if (girdiKonsoldan) {
      girdi.value = "";
      girdiKonsoldan = false;
    }
  }
  const kaynak = kod.value;
  const hazirGirdi = girdi.value;
  await mesgulken(yeniden ? "Çalışıyor…" : "Derleniyor…", async () => {
    sonucuGoster(kaynak, hazirGirdi, tohum, await oyunAlanindaCalistir(kaynak, hazirGirdi, tohum));
  });
}

/** Pas kaynağını Rust oyun alanında derleyip çalıştırır; oyun alanının yanıtını döndürür. */
async function oyunAlanindaCalistir(kaynak, hazirGirdi, tohum) {
  const yanit = await fetch(OYUN_ALANI, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      channel: "stable",
      mode: "debug",
      edition: "2024",
      crateType: "bin",
      tests: false,
      backtrace: false,
      code: pas.derle(kaynak, hazirGirdi, tohum),
    }),
  });
  if (!yanit.ok) throw new Error(`HTTP ${yanit.status}`);
  return yanit.json();
}

/** Çalıştırma ve denetleme sırasında düğmeleri kapatır, bağlantı hatasını gösterir. */
async function mesgulken(durumMetni, is) {
  calisiyor = true;
  calistirDugmesi.disabled = true;
  $("denetle").disabled = true;
  durumuYaz(durumMetni, "calisiyor");
  try {
    await is();
  } catch (hata) {
    console.error(hata);
    cikti.innerHTML = "";
    cikti.append(bilgi("Rust oyun alanına ulaşılamadı. İnternet bağlantını kontrol edip yeniden dene."));
    durumuYaz("Bağlantı hatası");
  } finally {
    calisiyor = false;
    calistirDugmesi.disabled = false;
    $("denetle").disabled = false;
  }
}

function durumuYaz(metin, sinif = "") {
  durum.textContent = metin;
  durum.className = `durum ${sinif}`;
}

function bilgi(metin) {
  const p = document.createElement("p");
  p.className = "bilgi";
  p.textContent = metin;
  return p;
}

function sonucuGoster(kaynak, hazirGirdi, tohum, sonuc) {
  const cozum = pas.coz(kaynak, hazirGirdi, tohum, sonuc.stderr ?? "");
  cikti.innerHTML = "";

  isaretler = [];
  hataliSatirlar = new Map();
  for (const tani of cozum.tanilamalar) {
    cikti.append(taniGoster(tani));
    const tur = tani.seviye === "hata" ? "hata" : "uyari";
    if (tani.aralik) {
      isaretler.push({ ...tani.aralik, tur: tur === "hata" ? "" : "uyari" });
      if (hataliSatirlar.get(tani.aralik.satir) !== "hatali") {
        hataliSatirlar.set(tani.aralik.satir, tur === "hata" ? "hatali" : "uyarili");
      }
    }
  }

  const hataSayisi = cozum.tanilamalar.filter((t) => t.seviye === "hata").length;
  if (!cozum.derlendi) {
    if (!hataSayisi) cikti.append(bilgi("Program derlenemedi.\n" + (sonuc.stderr ?? "")));
    durumuYaz(hataSayisi === 1 ? "1 hata" : `${hataSayisi} hata`);
    renklendir();
    return;
  }

  for (const [i, svg] of (cozum.cizimler ?? []).entries()) cikti.append(cizimGoster(svg, i));

  const pre = document.createElement("pre");
  pre.className = "program-ciktisi";
  pre.append(ciktiyiBicimle(sonuc.stdout ?? "", hazirGirdi));
  if (cozum.hata_ciktisi) {
    const span = document.createElement("span");
    span.className = "hata-ciktisi";
    span.textContent = cozum.hata_ciktisi;
    pre.append(span);
  }
  cikti.append(pre);

  if (cozum.panik) {
    cikti.append(panikGoster(cozum.panik));
    if (cozum.panik.aralik) {
      const { bas } = cozum.panik.aralik;
      const satirSonu = kaynak.indexOf("\n", bas);
      isaretler.push({ ...cozum.panik.aralik, son: satirSonu === -1 ? kaynak.length : satirSonu, tur: "panik" });
      hataliSatirlar.set(cozum.panik.aralik.satir, "hatali");
    }
    durumuYaz("Program çöktü");
  } else if (cozum.girdi_bitti) {
    pre.append(konsolGirdisi());
    durumuYaz("Cevabını bekliyor");
  } else if (sonuc.success) {
    const bos = !sonuc.stdout && !cozum.hata_ciktisi && !cozum.cizimler?.length;
    if (bos) cikti.append(bilgi("Program hiçbir şey yazdırmadı."));
    durumuYaz("Bitti ✓");
  } else {
    cikti.append(bilgi(cikisAciklamasi(sonuc.exitDetail ?? "")));
    durumuYaz("Program durduruldu");
  }
  renklendir();
}

/** Hazır girdiden okunan satırlar (yankı) farklı renkte gösterilir. */
function ciktiyiBicimle(metin, hazirGirdi) {
  const parca = document.createDocumentFragment();
  const cevaplar = hazirGirdi.split("\n");
  if (cevaplar.at(-1) === "") cevaplar.pop();
  let kalan = metin;
  // Program her soruyu sorup cevabı satırın sonuna yazar: "Adın ne? Ayşe\n"
  for (const cevap of cevaplar) {
    const aranan = cevap.replace(/\r$/, "") + "\n";
    const yer = kalan.indexOf(aranan);
    if (yer === -1) break;
    parca.append(kalan.slice(0, yer));
    const span = document.createElement("span");
    span.className = "yanki";
    span.textContent = aranan.slice(0, -1);
    parca.append(span, "\n");
    kalan = kalan.slice(yer + aranan.length);
  }
  parca.append(kalan);
  return parca;
}

/**
 * Kaplumbağanın çizimi. SVG bir resim olarak gösterilir; böylece içindeki
 * hiçbir şey sayfada çalıştırılmaz.
 */
function cizimGoster(svg, sira) {
  const adres = `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`;
  const kutu = eleman("figure", "cizim");
  const resim = document.createElement("img");
  resim.src = adres;
  resim.alt = "Kaplumbağanın çizimi";
  const indir = document.createElement("a");
  indir.href = adres;
  indir.download = sira ? `çizim-${sira + 1}.svg` : "çizim.svg";
  indir.textContent = "Çizimi indir";
  const alt = eleman("figcaption");
  alt.append(indir);
  kutu.append(resim, alt);
  return kutu;
}

function konsolGirdisi() {
  const alan = document.createElement("input");
  alan.className = "konsol-girdisi";
  alan.setAttribute("aria-label", "Programa cevabın");
  alan.autocomplete = "off";
  alan.spellcheck = false;
  alan.addEventListener("keydown", (olay) => {
    if (olay.key !== "Enter") return;
    olay.preventDefault();
    const mevcut = girdi.value;
    girdi.value = (mevcut && !mevcut.endsWith("\n") ? mevcut + "\n" : mevcut) + alan.value + "\n";
    girdiKonsoldan = true;
    alan.disabled = true;
    calistir({ yeniden: true });
  });
  requestAnimationFrame(() => {
    alan.focus({ preventScroll: true });
    cikti.scrollTop = cikti.scrollHeight;
  });
  return alan;
}

function cikisAciklamasi(ayrinti) {
  const kod = ayrinti.match(/status (\d+)/)?.[1];
  if (/timed out|killed|timeout/i.test(ayrinti)) {
    return "Program çok uzun sürdü ve durduruldu. Bitmeyen bir döngü olabilir mi?";
  }
  if (kod) return `Program ${kod} çıkış koduyla bitti.`;
  return `Program durduruldu: ${ayrinti}`;
}

function eleman(etiket, sinif, icerik) {
  const e = document.createElement(etiket);
  if (sinif) e.className = sinif;
  if (icerik !== undefined) e.innerHTML = icerik;
  return e;
}

/** Ters tırnak içindeki parçalar kod olarak gösterilir. */
const mesajHtml = (metin) => kacis(metin).replace(/`([^`]+)`/g, "<code>$1</code>").replace(/\n/g, "<br>");

function taniGoster(tani) {
  const uyari = tani.seviye !== "hata";
  const kutu = eleman("div", `tani${uyari ? " uyari" : ""}`);
  const baslik = tani.kod ? `${tani.seviye}[${tani.kod}]` : tani.seviye;
  kutu.append(eleman("div", "tani-basligi", `<b>${kacis(baslik)}</b>: ${mesajHtml(tani.mesaj)}`));
  if (tani.aralik) kutu.append(kesit(tani));
  for (const not of tani.notlar) {
    kutu.append(eleman("p", "tani-notu", `<b>${kacis(not.seviye)}:</b> ${mesajHtml(not.mesaj)}`));
    if (not.oneri) {
      const isaret = not.oneri.ekle ? "+" : "|";
      kutu.append(eleman("pre", "oneri-satiri", `${not.oneri.satir} ${isaret} ${kacis(not.oneri.metin)}`));
    }
  }
  if (tani.aciklama) kutu.append(eleman("p", "tani-aciklamasi", `💡 ${mesajHtml(tani.aciklama)}`));
  kutu.title = "Hatanın yerine gitmek için tıkla";
  kutu.addEventListener("click", (olay) => {
    if (window.getSelection()?.toString()) return;
    olay.preventDefault();
    kodaGit(tani.aralik);
  });
  return kutu;
}

/** rustc'nin gösterdiği gibi: kod satırları ve altlarında işaretler. */
function kesit(tani) {
  const satirlar = kod.value.split("\n");
  const etiketler = tani.etiketler.length ? tani.etiketler : [{ ...tani.aralik, birincil: true, mesaj: null }];
  const satirNolari = [...new Set(etiketler.map((e) => e.satir))].sort((a, b) => a - b);
  const genislik = String(satirNolari.at(-1)).length;
  const bosluk = " ".repeat(genislik);
  const kenar = (metin) => `<span class="kenar">${metin}</span>`;
  let html = `${bosluk}${kenar("-->")} ana.pas:${tani.aralik.satir}:${tani.aralik.sutun}\n${bosluk} ${kenar("|")}\n`;
  let onceki = null;
  for (const no of satirNolari) {
    if (onceki !== null && no > onceki + 1) html += `${kenar("...")}\n`;
    onceki = no;
    const satir = (satirlar[no - 1] ?? "").replace(/\t/g, "    ");
    html += `${kenar(`${String(no).padStart(genislik)} |`)} ${kacis(satir)}\n`;
    const buSatirdakiler = etiketler.filter((e) => e.satir === no).sort((a, b) => a.sutun - b.sutun);
    for (const e of buSatirdakiler) {
      const uzunluk = e.bitis_satiri === no ? Math.max(1, e.bitis_sutunu - e.sutun) : Math.max(1, [...satir].length - e.sutun + 1);
      const isaret = (e.birincil ? "^" : "-").repeat(uzunluk);
      const mesaj = e.mesaj ? ` ${mesajHtml(e.mesaj)}` : "";
      html += `${bosluk} ${kenar("|")} ${" ".repeat(e.sutun - 1)}<span class="${e.birincil ? "birincil" : "ikincil"}">${isaret}${mesaj}</span>\n`;
    }
  }
  return eleman("pre", "kesit", html.trimEnd());
}

function panikGoster(panik) {
  const kutu = eleman("div", "tani");
  kutu.append(eleman("div", "tani-basligi", `<b>hata</b>: program çöktü (panik, <code>${kacis(panik.is_parcacigi)}</code> iş parçacığında)`));
  if (panik.aralik) {
    kutu.append(kesit({ aralik: panik.aralik, etiketler: [{ ...panik.aralik, bitis_sutunu: panik.aralik.sutun + 1, birincil: true, mesaj: null }] }));
  }
  if (panik.mesaj) kutu.append(eleman("p", "tani-notu", mesajHtml(panik.mesaj)));
  if (/girdi bitti/.test(panik.mesaj)) {
    kutu.append(eleman("p", "tani-aciklamasi", "💡 Program senden bir şey okumak istedi. Cevabını “Hazır girdi” kutusuna yaz."));
  }
  kutu.addEventListener("click", () => kodaGit(panik.aralik));
  return kutu;
}

calistirDugmesi.addEventListener("click", () => calistir());

// ---- Örnekler, paylaşma, yan paneller ----

let ornekler = [];

async function ornekleriYukle() {
  try {
    ornekler = await (await fetch(`ornekler.json?s=${SURUM}`)).json();
  } catch {
    ornekler = [];
  }
  const secim = $("ornekler");
  for (const [i, ornek] of ornekler.entries()) {
    const secenek = document.createElement("option");
    secenek.value = String(i);
    secenek.textContent = ornek.ad;
    secim.append(secenek);
  }
  secim.addEventListener("change", () => {
    const ornek = ornekler[Number(secim.value)];
    secim.value = "";
    if (ornek) ornegiAc(ornek);
  });
}

/** Son yüklenen örneğin kodu; değiştirilmediyse sormadan başka örnek açılır. */
let kodBasi = "";

/** Örneğin (ya da dersin) kodunu düzenleyiciye yükler; ders ise anlatımını da açar. */
function ornegiAc(ornek, { sor = true } = {}) {
  const degismis = kod.value.trim() && kod.value !== kodBasi;
  if (sor && degismis && !confirm("Yazdığın kodun yerine örnek yüklensin mi?")) return;
  gorevdenCik();
  kod.value = ornek.kaynak;
  kodBasi = ornek.kaynak;
  girdi.value = "";
  girdiKonsoldan = false;
  history.replaceState(null, "", ornek.ders ? `#ders=${ornek.ders.kimlik}` : location.pathname);
  if (ornek.ders) dersiGoster(ornek);
  degisti();
  cikti.innerHTML = "";
  cikti.append(eleman("p", "karsilama", `<b>${kacis(ornek.ad)}</b> yüklendi. <b>▶ Çalıştır</b>'a bas.`));
  durumuYaz("");
}

// ---- Dersler ----

/** Panelde gösterilen ders (bir örnek). */
let acikDers = null;
const dersler = () => ornekler.filter((ornek) => ornek.ders);
const dersBul = (kimlik) => ornekler.find((ornek) => ornek.ders?.kimlik === kimlik);

function dersiKaydet(kimlik) {
  try {
    localStorage.setItem(DERS_ANAHTARI, kimlik);
  } catch {
    // Kayıt yapılamıyorsa sorun değil.
  }
}

function dersiGoster(ornek) {
  acikDers = ornek;
  const liste = dersler();
  const sira = liste.indexOf(ornek);
  $("ders-listesi-dugmesi").textContent = `Ders ${sira + 1} / ${liste.length}`;
  $("onceki-ders").disabled = sira <= 0;
  $("sonraki-ders").disabled = sira >= liste.length - 1;
  $("ders-metni").innerHTML = dersHtml(ornek.ders.metin, ornek.ders.yol) + gorevlerHtml(ornek);
  $("ders-metni").scrollTop = 0;
  paneliAc("ders-paneli", "dersler-dugmesi", true);
  dersiKaydet(ornek.ders.kimlik);
}

function dersListesiniGoster() {
  $("ders-listesi-dugmesi").textContent = "Bütün dersler";
  $("onceki-ders").disabled = true;
  $("sonraki-ders").disabled = true;
  const ogeler = dersler()
    .map((ornek) => {
      const kimlik = kacis(ornek.ders.kimlik);
      const secili = ornek === acikDers ? ' aria-current="true"' : "";
      const toplam = ornek.ders.gorevler?.length ?? 0;
      const biten = (ornek.ders.gorevler ?? []).filter((_, i) => tamamlananlar.has(gorevKimligi(ornek, i))).length;
      const rozet = toplam ? `<span class="ilerleme${biten === toplam ? " tamam" : ""}">${biten === toplam ? "✓ " : ""}${biten}/${toplam}</span>` : "";
      return `<li><a href="#ders=${kimlik}" data-ders="${kimlik}"${secili}>${kacis(ornek.ad)}${rozet}</a></li>`;
    })
    .join("");
  $("ders-metni").innerHTML =
    "<h2>Dersler</h2><p>Programlamayı adım adım öğren. Her derste önce kodu çalıştır, sonra anlatımı oku " +
    "ve en sondaki <b>Kendin dene</b> alıştırmalarını yap.</p>" +
    `<ol class="ders-listesi">${ogeler}</ol>`;
  $("ders-metni").scrollTop = 0;
  paneliAc("ders-paneli", "dersler-dugmesi", true);
}

function komsuDers(yon) {
  const liste = dersler();
  const ders = liste[liste.indexOf(acikDers) + yon];
  if (ders) ornegiAc(ders);
}

$("onceki-ders").addEventListener("click", () => komsuDers(-1));
$("sonraki-ders").addEventListener("click", () => komsuDers(1));
$("ders-listesi-dugmesi").addEventListener("click", dersListesiniGoster);
$("dersler-dugmesi").addEventListener("click", () => {
  if (!$("ders-paneli").hidden) {
    paneliAc("ders-paneli", "dersler-dugmesi", false);
    dersiKaydet("");
  } else if (acikDers) {
    dersiGoster(acikDers);
  } else {
    dersListesiniGoster();
  }
});
$("ders-metni").addEventListener("click", (olay) => {
  const basla = olay.target.closest("[data-gorev-basla]");
  if (basla) {
    goreveBasla(acikDers, Number(basla.dataset.gorevBasla));
    return;
  }
  const baglanti = olay.target.closest("a[data-ders]");
  if (!baglanti) return;
  olay.preventDefault();
  const ders = dersBul(baglanti.dataset.ders);
  if (ders) ornegiAc(ders);
});

// ---- Görevler ----

/** Tamamlanan görevler: "03-soru-sor#0" gibi kimlikler. */
const tamamlananlar = new Set();
try {
  for (const kimlik of JSON.parse(localStorage.getItem(GOREV_ANAHTARI) ?? "[]")) tamamlananlar.add(kimlik);
} catch {
  // Kayıt okunamıyorsa ilerleme sıfırdan başlar.
}
const gorevKimligi = (ornek, sira) => `${ornek.ders.kimlik}#${sira}`;

/** Üzerinde çalışılan görev: { ornek, sira, gorev } */
let acikGorev = null;

function gorevlerHtml(ornek) {
  const gorevler = ornek.ders.gorevler ?? [];
  if (!gorevler.length) return "";
  const kartlar = gorevler.map((gorev, sira) => {
    const tamam = tamamlananlar.has(gorevKimligi(ornek, sira));
    const acik = acikGorev?.ornek === ornek && acikGorev.sira === sira;
    const durum = tamam ? "✓" : String(sira + 1);
    const ipucu = gorev.ipucu ? `<details><summary>İpucu</summary><p>${satirIci(gorev.ipucu, ornek.ders.yol)}</p></details>` : "";
    const dugme = acik
      ? `<p class="gorev-acik">Kodunu yaz, sonra <b>✓ Denetle</b>'ye bas.</p>`
      : `<button class="gorev-basla" data-gorev-basla="${sira}">${tamam ? "Yeniden yap" : "Göreve başla"}</button>`;
    return `<div class="gorev${tamam ? " tamam" : ""}${acik ? " acik" : ""}">
      <h4><span class="gorev-durumu">${durum}</span>${kacis(gorev.baslik)}</h4>
      ${dersHtml(gorev.aciklama, ornek.ders.yol)}${ipucu}${dugme}
      <details class="cozum"><summary>Çözümü göster</summary>${kodBlogu(gorev.cozum, "pas")}</details>
    </div>`;
  });
  return `<h3 class="gorevler-basligi">Görevler</h3>${kartlar.join("")}`;
}

function goreveBasla(ornek, sira) {
  const gorev = ornek?.ders.gorevler?.[sira];
  if (!gorev) return;
  const degismis = kod.value.trim() && kod.value !== kodBasi;
  if (degismis && !confirm("Yazdığın kodun yerine görevin başlangıç kodu yüklensin mi?")) return;
  acikGorev = { ornek, sira, gorev };
  kod.value = gorev.baslangic + "\n";
  kodBasi = kod.value;
  girdi.value = "";
  girdiKonsoldan = false;
  degisti();
  $("denetle").hidden = false;
  $("gorev-etiketi").hidden = false;
  $("gorev-adi").textContent = `Görev: ${gorev.baslik}`;
  const kaydirma = $("ders-metni").scrollTop;
  dersiGoster(ornek);
  $("ders-metni").scrollTop = kaydirma;
  cikti.innerHTML = "";
  cikti.append(eleman("p", "karsilama", `<b>${kacis(gorev.baslik)}</b>: kodunu yaz, ▶ Çalıştır ile dene, hazır olunca <b>✓ Denetle</b>'ye bas.`));
  durumuYaz("");
  kod.focus();
}

function gorevdenCik() {
  if (!acikGorev) return;
  acikGorev = null;
  $("denetle").hidden = true;
  $("gorev-etiketi").hidden = true;
}

$("gorevi-birak").addEventListener("click", () => {
  const ornek = acikGorev?.ornek;
  gorevdenCik();
  if (ornek && !$("ders-paneli").hidden) {
    const kaydirma = $("ders-metni").scrollTop;
    dersiGoster(ornek);
    $("ders-metni").scrollTop = kaydirma;
  }
});

$("denetle").addEventListener("click", denetle);

/** Görevi her örnek girdiyle çalıştırıp çıktıyı (ya da çizimi) beklenenle karşılaştırır. */
async function denetle() {
  if (!pas || calisiyor || !acikGorev) return;
  const { ornek, sira, gorev } = acikGorev;
  const kaynak = kod.value;
  const TOHUM = 1;
  await mesgulken("Denetleniyor…", async () => {
    const ornekler = gorev.ornekler.length ? gorev.ornekler : [{ girdi: "", cikti: null }];
    for (const { girdi: ornekGirdi, cikti: beklenen } of ornekler) {
      const hazirGirdi = ornekGirdi ? `${ornekGirdi}\n` : "";
      const sonuc = await oyunAlanindaCalistir(kaynak, hazirGirdi, TOHUM);
      const cozum = pas.coz(kaynak, hazirGirdi, TOHUM, sonuc.stderr ?? "");
      sonucuGoster(kaynak, hazirGirdi, TOHUM, sonuc);
      const kosul = ornekler.length > 1 && ornekGirdi ? ` (girdi: ${ornekGirdi.split("\n").join(", ")})` : "";
      if (!cozum.derlendi) return gorevSonucu(false, "Önce derleyicinin gösterdiği hataları düzelt.");
      if (cozum.panik) return gorevSonucu(false, `Program çöktü${kosul}. Önce bunu düzelt.`);
      if (cozum.girdi_bitti) {
        return gorevSonucu(false, `Program, görevin verdiği girdilerden daha fazlasını okumaya çalıştı${kosul}. Görevi yeniden oku.`);
      }
      if (beklenen === null) {
        const cozumSonucu = await oyunAlanindaCalistir(gorev.cozum, "", TOHUM);
        const beklenenCizim = pas.coz(gorev.cozum, "", TOHUM, cozumSonucu.stderr ?? "").cizimler;
        if (!cizimlerAyni(cozum.cizimler ?? [], beklenenCizim ?? [])) {
          return gorevSonucu(false, "Çizim istenen şekle benzemiyor. Görevi yeniden oku: kaç adım yürüyor, kaç derece dönüyorsun?");
        }
        continue;
      }
      const fark = ciktiFarki(sonuc.stdout ?? "", beklenen);
      if (fark) return gorevSonucu(false, fark + kosul);
    }
    tamamlananlar.add(gorevKimligi(ornek, sira));
    try {
      localStorage.setItem(GOREV_ANAHTARI, JSON.stringify([...tamamlananlar]));
    } catch {
      // Kayıt yapılamıyorsa sorun değil.
    }
    gorevSonucu(true);
    if (!$("ders-paneli").hidden && acikDers === ornek) {
      const kaydirma = $("ders-metni").scrollTop;
      dersiGoster(ornek);
      $("ders-metni").scrollTop = kaydirma;
    }
  });
}

/** Denetimin sonucu çıktının en üstünde gösterilir. */
function gorevSonucu(basarili, aciklama = "") {
  const kutu = eleman("div", `gorev-sonucu${basarili ? " basarili" : ""}`);
  if (basarili) {
    const { ornek, sira } = acikGorev;
    kutu.innerHTML = "<b>🎉 Görev tamamlandı!</b> Harika iş çıkardın.";
    const sonraki = ornek.ders.gorevler[sira + 1];
    const liste = dersler();
    const sonrakiDers = liste[liste.indexOf(ornek) + 1];
    if (sonraki) {
      const dugme = eleman("button", "", `Sonraki görev: ${kacis(sonraki.baslik)} →`);
      dugme.addEventListener("click", () => goreveBasla(ornek, sira + 1));
      kutu.append(dugme);
    } else if (sonrakiDers) {
      const dugme = eleman("button", "", `Sonraki ders: ${kacis(sonrakiDers.ad)} →`);
      dugme.addEventListener("click", () => ornegiAc(sonrakiDers));
      kutu.append(dugme);
    }
    durumuYaz("Görev tamamlandı ✓");
  } else {
    kutu.innerHTML = `<b>Henüz olmadı.</b> ${mesajHtml(aciklama)}`;
    durumuYaz("Henüz olmadı");
  }
  cikti.prepend(kutu);
  cikti.scrollTop = 0;
}

/** Programın çıktısıyla beklenen çıktı arasındaki ilk fark, Türkçe. */
function ciktiFarki(gercek, beklenen) {
  const satirlar = (metin) => {
    const s = metin.split("\n").map((satir) => satir.trimEnd());
    while (s.length && s.at(-1) === "") s.pop();
    return s;
  };
  const g = satirlar(gercek);
  const b = satirlar(beklenen);
  for (let i = 0; i < Math.max(g.length, b.length); i++) {
    if (i >= g.length) return `Çıktı erken bitti: ${i + 1}. satırda \`${b[i]}\` bekleniyordu.`;
    if (i >= b.length) return `Fazladan bir satır var: \`${g[i]}\``;
    if (g[i] !== b[i]) return `${i + 1}. satırda \`${b[i]}\` bekleniyordu ama \`${g[i]}\` çıktı.`;
  }
  return null;
}

/** İki çizimdeki çizgi ve dairelerin (renk ve sıra önemsiz) aynı olup olmadığı. */
function cizimlerAyni(a, b) {
  const sekiller = (cizimler) => {
    const liste = [];
    for (const svg of cizimler) {
      for (const [, x1, y1, x2, y2] of svg.matchAll(/<line x1="([-\d.]+)" y1="([-\d.]+)" x2="([-\d.]+)" y2="([-\d.]+)"/g)) {
        const uclar = [[+x1, +y1], [+x2, +y2]].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
        // Sıfır uzunluklu çizgiler sayılmaz.
        if (Math.hypot(uclar[0][0] - uclar[1][0], uclar[0][1] - uclar[1][1]) > 0.5) liste.push(["c", ...uclar.flat()]);
      }
      for (const [, x, y, r] of svg.matchAll(/<circle cx="([-\d.]+)" cy="([-\d.]+)" r="([-\d.]+)"/g)) liste.push(["d", +x, +y, +r]);
    }
    // Her iki çizim de aynı anahtarla sıralanır; karşılaştırma ise küçük farklara izin verir.
    const anahtar = (sekil) => `${sekil[0]}${sekil.slice(1).map(Math.round).join(",")}`;
    return liste.sort((p, q) => (anahtar(p) < anahtar(q) ? -1 : anahtar(p) > anahtar(q) ? 1 : 0));
  };
  const x = sekiller(a);
  const y = sekiller(b);
  return x.length === y.length && x.every((p, i) => p[0] === y[i][0] && p.slice(1).every((d, j) => Math.abs(d - y[i][j + 1]) < 1));
}

/**
 * Ders anlatımlarındaki Markdown'u HTML'e çevirir. Derslerde kullanılanlar
 * yeter: başlıklar, paragraflar, listeler, tablolar, kod blokları, bağlantılar.
 * Komut satırını anlatan "Çalıştır" bölümü oyun alanında gerekmediği için atlanır.
 */
function dersHtml(markdown, yol) {
  const satirlar = markdown.split("\n");
  let html = "";
  const paragraf = [];
  const paragrafiBitir = () => {
    if (!paragraf.length) return;
    const metin = paragraf.join(" ");
    paragraf.length = 0;
    const sonraki = metin.match(/^Sonraki ders: \[([^\]]+)\]\(\.\.\/([^/]+)\/README\.md\)$/);
    html += sonraki
      ? `<p><a class="sonraki" href="#ders=${kacis(sonraki[2])}" data-ders="${kacis(sonraki[2])}">Sonraki ders: ${kacis(sonraki[1])} →</a></p>`
      : `<p>${satirIci(metin, yol)}</p>`;
  };
  let i = 0;
  while (i < satirlar.length) {
    const satir = satirlar[i];
    if (satir.startsWith("```")) {
      paragrafiBitir();
      const dil = satir.slice(3).trim();
      const kodSatirlari = [];
      for (i++; i < satirlar.length && !satirlar[i].startsWith("```"); i++) kodSatirlari.push(satirlar[i]);
      i++;
      html += kodBlogu(kodSatirlari.join("\n"), dil);
      continue;
    }
    const baslik = satir.match(/^(#{1,4}) (.*)$/);
    if (baslik) {
      paragrafiBitir();
      if (baslik[1] === "##" && baslik[2].trim() === "Çalıştır") {
        for (i++; i < satirlar.length && !satirlar[i].startsWith("## "); i++);
        continue;
      }
      const duzey = baslik[1].length + 1;
      html += `<h${duzey}>${satirIci(baslik[2], yol)}</h${duzey}>`;
      i++;
      continue;
    }
    if (/^-{3,}\s*$/.test(satir)) {
      paragrafiBitir();
      html += "<hr>";
      i++;
      continue;
    }
    if (satir.startsWith("|")) {
      paragrafiBitir();
      const tablo = [];
      for (; i < satirlar.length && satirlar[i].startsWith("|"); i++) tablo.push(satirlar[i]);
      html += tabloHtml(tablo, yol);
      continue;
    }
    const madde = satir.match(/^([-*]|\d+\.) (.*)$/);
    if (madde) {
      paragrafiBitir();
      const sirali = /\d/.test(madde[1]);
      const ogeler = [];
      while (i < satirlar.length) {
        const oge = satirlar[i].match(/^([-*]|\d+\.) (.*)$/);
        if (oge && /\d/.test(oge[1]) === sirali) {
          ogeler.push(oge[2]);
        } else if (/^\s{2,}\S/.test(satirlar[i]) && ogeler.length) {
          ogeler[ogeler.length - 1] += " " + satirlar[i].trim();
        } else {
          break;
        }
        i++;
      }
      const etiket = sirali ? "ol" : "ul";
      html += `<${etiket}>${ogeler.map((oge) => `<li>${satirIci(oge, yol)}</li>`).join("")}</${etiket}>`;
      continue;
    }
    if (satir.trim()) paragraf.push(satir.trim());
    else paragrafiBitir();
    i++;
  }
  paragrafiBitir();
  return html;
}

/** Satır içi Markdown: `kod`, **kalın**, *eğik*, [bağlantı](adres). */
function satirIci(metin, yol) {
  // Kod parçaları önce ayrılır; içlerindeki `*` ya da `[` biçimlendirme sayılmaz.
  const kodlar = [];
  const yer = (i) => `\u0000${i}\u0000`;
  const html = kacis(metin.replace(/`([^`]+)`/g, (_, kod) => yer(kodlar.push(kod) - 1)))
    .replace(/\*\*(.+?)\*\*/g, "<b>$1</b>")
    .replace(/(^|[^*\w])\*([^*\s][^*]*?)\*(?![*\w])/g, "$1<i>$2</i>")
    .replace(/\[([^\]]+)\]\(([^)\s]+)\)/g, (_, yazi, adres) => baglanti(yazi, adres, yol));
  return html.replace(/\u0000(\d+)\u0000/g, (_, i) => `<code>${kacis(kodlar[Number(i)])}</code>`);
}

function baglanti(yazi, adres, yol) {
  const ders = adres.match(/^\.\.\/(\d\d-[^/]+)\/README\.md$/);
  if (ders && dersBul(ders[1])) return `<a href="#ders=${ders[1]}" data-ders="${ders[1]}">${yazi}</a>`;
  // Diğer göreli bağlantılar GitHub'daki dosyaya gider.
  const tam = /^https?:/.test(adres) ? adres : new URL(adres.replace(/&amp;/g, "&"), `${DEPO}/blob/main/${yol}`).href;
  return `<a href="${kacis(tam)}" target="_blank" rel="noopener">${yazi}</a>`;
}

function tabloHtml(satirlar, yol) {
  const hucreler = (satir) =>
    satir
      .replace(/^\||\|$/g, "")
      .split(/(?<!\\)\|/)
      .map((hucre) => satirIci(hucre.trim().replace(/\\\|/g, "|"), yol));
  const [baslik, , ...govde] = satirlar;
  const ust = hucreler(baslik).map((h) => `<th>${h}</th>`).join("");
  const alt = govde.map((satir) => `<tr>${hucreler(satir).map((h) => `<td>${h}</td>`).join("")}</tr>`).join("");
  return `<table><thead><tr>${ust}</tr></thead><tbody>${alt}</tbody></table>`;
}

/** Pas kod blokları düzenleyicideki gibi renklendirilir. */
function kodBlogu(metin, dil) {
  if (dil && dil !== "pas") return `<pre><code>${kacis(metin)}</code></pre>`;
  let html = "";
  let konum = 0;
  for (const [bas, son, sinif] of pas.renklendir(metin)) {
    html += kacis(metin.slice(konum, bas)) + `<span class="r-${sinif}">${kacis(metin.slice(bas, son))}</span>`;
    konum = son;
  }
  return `<pre><code>${html}${kacis(metin.slice(konum))}</code></pre>`;
}

// Paylaşılan bağlantı: #kod=<sıkıştırılmış, base64url>
async function sikistir(metin) {
  const akim = new Blob([kodlayici.encode(metin)]).stream().pipeThrough(new CompressionStream("deflate-raw"));
  const baytlar = new Uint8Array(await new Response(akim).arrayBuffer());
  let ikili = "";
  for (const b of baytlar) ikili += String.fromCharCode(b);
  return btoa(ikili).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

async function ac(metin) {
  const ikili = atob(metin.replace(/-/g, "+").replace(/_/g, "/"));
  const baytlar = Uint8Array.from(ikili, (c) => c.charCodeAt(0));
  const akim = new Blob([baytlar]).stream().pipeThrough(new DecompressionStream("deflate-raw"));
  return cozucu.decode(await new Response(akim).arrayBuffer());
}

$("paylas").addEventListener("click", async () => {
  try {
    const adres = `${location.origin}${location.pathname}#kod=${await sikistir(kod.value)}`;
    history.replaceState(null, "", adres);
    await navigator.clipboard.writeText(adres);
    bildir("Bağlantı kopyalandı. Arkadaşına ya da öğretmenine gönderebilirsin.");
  } catch {
    bildir("Bağlantı adres çubuğunda; oradan kopyalayabilirsin.");
  }
});

// Geri bildirim formu, açık ders ve (kısaysa) kodla doldurulmuş olarak açılır.
$("geri-bildirim").addEventListener("click", () => {
  const adres = new URL(`${DEPO}/issues/new`);
  adres.searchParams.set("template", "geri-bildirim.yml");
  adres.searchParams.set("nerede", acikDers && !$("ders-paneli").hidden ? `Oyun alanı, ${acikDers.ad}` : "Oyun alanı");
  if (kod.value.length <= 2000) adres.searchParams.set("kod", kod.value);
  $("geri-bildirim").href = adres.href;
});

let bildirimZamanlayicisi;
function bildir(metin) {
  const kutu = $("bildirim");
  kutu.textContent = metin;
  kutu.hidden = false;
  clearTimeout(bildirimZamanlayicisi);
  bildirimZamanlayicisi = setTimeout(() => (kutu.hidden = true), 3500);
}

function paneliAc(panel, dugme, acik) {
  $(panel).hidden = !acik;
  $(dugme).setAttribute("aria-pressed", String(acik));
}

$("rust-dugmesi").addEventListener("click", () => {
  const acik = $("rust-paneli").hidden;
  paneliAc("sozluk-paneli", "sozluk-dugmesi", false);
  paneliAc("rust-paneli", "rust-dugmesi", acik);
  rustuGuncelle();
});

$("sozluk-dugmesi").addEventListener("click", () => {
  const acik = $("sozluk-paneli").hidden;
  paneliAc("rust-paneli", "rust-dugmesi", false);
  paneliAc("sozluk-paneli", "sozluk-dugmesi", acik);
  if (acik) $("sozluk-ara").focus();
});

const PANEL_DUGMELERI = { "rust-paneli": "rust-dugmesi", "sozluk-paneli": "sozluk-dugmesi", "ders-paneli": "dersler-dugmesi" };
for (const dugme of document.querySelectorAll("[data-kapat]")) {
  dugme.addEventListener("click", () => {
    const panel = dugme.dataset.kapat;
    paneliAc(panel, PANEL_DUGMELERI[panel], false);
    if (panel === "ders-paneli") dersiKaydet("");
  });
}

function rustuGuncelle() {
  if (pas && !$("rust-paneli").hidden) $("rust-kodu").textContent = pas.rust(kod.value);
}

let sozluk = [];
const SINIF_ADLARI = {
  anahtar_sozcuk: "Anahtar sözcükler",
  makro: "Makrolar",
  tur: "Türler",
  deger: "Değerler",
  islev: "İşlevler",
  yontem: "Yöntemler",
};

function sozluguGoster() {
  const aranan = $("sozluk-ara").value.trim().toLocaleLowerCase("tr");
  const liste = $("sozluk-listesi");
  liste.innerHTML = "";
  let tur = null;
  for (const g of sozluk) {
    const metin = `${g.pas} ${g.rust} ${g.aciklama}`.toLocaleLowerCase("tr");
    if (aranan && !metin.includes(aranan)) continue;
    if (g.tur !== tur) {
      tur = g.tur;
      liste.append(eleman("h3", "", kacis(SINIF_ADLARI[tur] ?? g.tur_adi)));
    }
    const girdiKutusu = eleman("div", "sozluk-girdisi");
    girdiKutusu.innerHTML = `<code>${kacis(g.pas)}</code><span class="rust">${kacis(g.rust)}</span><p>${mesajHtml(g.aciklama)}</p>`;
    liste.append(girdiKutusu);
  }
  if (!liste.childElementCount) liste.append(eleman("p", "aciklama-metni", "Bulunamadı."));
}

$("sozluk-ara").addEventListener("input", sozluguGoster);

// ---- Başlangıç ----

async function basla() {
  // Önce kayıtlı ya da paylaşılan kod; örnekler sonra gelir.
  let baslangic = null;
  const bagliDers = location.hash.match(/^#ders=([\w-]+)$/)?.[1];
  if (location.hash.startsWith("#kod=")) {
    try {
      baslangic = await ac(location.hash.slice(5));
    } catch {
      bildir("Bu bağlantıdaki kod açılamadı.");
    }
  }
  let kayitliDers = null;
  if (baslangic === null) {
    try {
      baslangic = localStorage.getItem(KAYIT_ANAHTARI);
      kayitliDers = localStorage.getItem(DERS_ANAHTARI);
    } catch {
      baslangic = null;
    }
  }
  if (baslangic !== null) {
    kod.value = baslangic;
    renklendir();
  }

  try {
    await pasYukle();
  } catch (hata) {
    console.error(hata);
    cikti.innerHTML = "";
    cikti.append(bilgi("Pas yüklenemedi. Sayfayı yenilemeyi dene; olmazsa tarayıcın WebAssembly desteklemiyor olabilir."));
    return;
  }
  sozluk = pas.sozluk();
  sozluguGoster();
  await ornekleriYukle();
  if (bagliDers && dersBul(bagliDers)) {
    // Bir dersin bağlantısı (`#ders=03-soru-sor`) o dersi kodu ile açar.
    ornegiAc(dersBul(bagliDers), { sor: false });
  } else if (baslangic === null && ornekler.length) {
    // İlk kez gelen biri birinci dersle başlar.
    ornegiAc(dersler()[0] ?? ornekler[0]);
  } else {
    // Kayıtlı kod değiştirilmemiş bir örnekse başka örnek açarken sorulmaz.
    if (ornekler.some((ornek) => ornek.kaynak === kod.value)) kodBasi = kod.value;
    if (kayitliDers && dersBul(kayitliDers)) dersiGoster(dersBul(kayitliDers));
    renklendir();
  }
  calistirDugmesi.disabled = false;
  window.addEventListener("resize", boyutlandir);
  // Sayfa açıkken bir ders bağlantısına gidilirse o ders açılır.
  window.addEventListener("hashchange", () => {
    const ders = dersBul(location.hash.match(/^#ders=([\w-]+)$/)?.[1]);
    if (ders && ders !== acikDers) ornegiAc(ders);
  });
}

basla();
