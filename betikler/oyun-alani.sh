#!/bin/sh
# Pas oyun alanını (tarayıcıda çalışan Pas) derler ve yayınlanacak siteyi
# hazırlar. Site GitHub Pages'e .github/workflows/oyun-alani.yml ile konur.
#
#   betikler/oyun-alani.sh                 # target/oyun-alani/ altına
#   betikler/oyun-alani.sh _site           # başka bir dizine
#
# Yerelde denemek için:
#   python3 -m http.server -d target/oyun-alani 8000
set -eu

kok="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kok"
hedef="${1:-target/oyun-alani}"

cargo build --release --target wasm32-unknown-unknown -p pas-web

rm -rf "$hedef"
mkdir -p "$hedef"
cp oyun-alani/oyun-alani.js oyun-alani/stil.css "$hedef/"
cp target/wasm32-unknown-unknown/release/pas_web.wasm "$hedef/pas.wasm"

# Tarayıcı eski dosyaları önbellekten getirmesin diye adreslere sürüm eklenir.
surum="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)-$(git rev-parse --short HEAD 2>/dev/null || echo yerel)"
sed "s/__SURUM__/$surum/g" oyun-alani/index.html >"$hedef/index.html"

# Örnekler: tek dosyalık dersler (anlatımlarıyla birlikte) ve örnek projeler.
python3 - "$hedef/ornekler.json" <<'PY'
import json, pathlib, sys

def gorevleri_oku(metin):
    """`gorevler.md`: `## Başlık`, açıklama, `İpucu: …`, ```pas başlangıç```,
    ```girdi```/```çıktı``` çiftleri ve ```pas çözüm``` blokları.
    Biçim `crates/pas-web/src/gorev_testleri.rs` içindeki okuyucuyla aynıdır."""
    gorevler = []
    satirlar = iter(metin.splitlines())
    girdi = ""
    for satir in satirlar:
        if satir.startswith("## "):
            gorevler.append({"baslik": satir[3:].strip(), "aciklama": [], "ipucu": None,
                             "baslangic": "", "cozum": "", "ornekler": []})
            continue
        if not gorevler:
            continue
        gorev = gorevler[-1]
        if satir.startswith("```"):
            bilgi = satir[3:].strip()
            blok = []
            for ic in satirlar:
                if ic.startswith("```"):
                    break
                blok.append(ic)
            blok = "\n".join(blok)
            if bilgi == "pas başlangıç":
                gorev["baslangic"] = blok
            elif bilgi == "pas çözüm":
                gorev["cozum"] = blok
            elif bilgi == "girdi":
                girdi = blok
            elif bilgi == "çıktı":
                gorev["ornekler"].append({"girdi": girdi, "cikti": blok})
                girdi = ""
            else:
                gorev["aciklama"] += [satir, *blok.split("\n"), "```"]
            continue
        if satir.startswith("İpucu: "):
            gorev["ipucu"] = satir[len("İpucu: "):]
        else:
            gorev["aciklama"].append(satir)
    for gorev in gorevler:
        gorev["aciklama"] = "\n".join(gorev["aciklama"]).strip()
    return gorevler

ornekler = []
for dizin in sorted(pathlib.Path("dersler").iterdir()) + sorted(pathlib.Path("ornekler").iterdir()):
    pas_dosyalari = list((dizin / "src").glob("*.pas")) if (dizin / "src").is_dir() else []
    if [d.name for d in pas_dosyalari] != ["ana.pas"]:
        continue
    benioku = dizin / "README.md"
    if benioku.is_file():
        metin = benioku.read_text(encoding="utf-8")
        ad = metin.splitlines()[0].lstrip("# ").strip()
        ders = {"kimlik": dizin.name, "yol": benioku.as_posix(), "metin": metin, "gorevler": []}
        if (dizin / "gorevler.md").is_file():
            ders["gorevler"] = gorevleri_oku((dizin / "gorevler.md").read_text(encoding="utf-8"))
    kaynak = (dizin / "src" / "ana.pas").read_text(encoding="utf-8")
    if not benioku.is_file():
        # Örneklerin adı ilk yorum satırından gelir: "// Sayı tahmin oyunu: …"
        ilk = kaynak.splitlines()[0].removeprefix("//").strip() if kaynak.startswith("//") else dizin.name
        ad = "Örnek: " + ilk.split(":")[0].rstrip(".")
        ders = None
    ornekler.append({"ad": ad, "kaynak": kaynak, "ders": ders})

with open(sys.argv[1], "w", encoding="utf-8") as dosya:
    json.dump(ornekler, dosya, ensure_ascii=False)
print(f"{len(ornekler)} örnek eklendi")
PY

echo "Oyun alanı hazır: $hedef/"
