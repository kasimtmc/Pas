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

# Örnekler: tek dosyalık dersler ve örnek projeler.
python3 - "$hedef/ornekler.json" <<'PY'
import json, pathlib, sys

ornekler = []
for dizin in sorted(pathlib.Path("dersler").iterdir()) + sorted(pathlib.Path("ornekler").iterdir()):
    pas_dosyalari = list((dizin / "src").glob("*.pas")) if (dizin / "src").is_dir() else []
    if [d.name for d in pas_dosyalari] != ["ana.pas"]:
        continue
    benioku = dizin / "README.md"
    if benioku.is_file():
        ad = benioku.read_text(encoding="utf-8").splitlines()[0].lstrip("# ").strip()
        ders = benioku.as_posix()
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
