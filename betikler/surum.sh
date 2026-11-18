#!/bin/sh
# Yeni bir Pas sürümü hazırlar: sürüm numarasını her yerde günceller,
# testleri çalıştırır, "Sürüm X" commit'ini ve `vX` etiketini oluşturur.
#
#   betikler/surum.sh 0.1.2
#   betikler/surum.sh 0.1.2 "Yeni özellik / New feature"   # eklenti sürüm notuyla
#   betikler/surum.sh 0.1.2 --gonder                        # sonunda GitHub'a da gönder
#
# Etiket GitHub'a gönderilince yayın iş akışı (.github/workflows/yayin.yml)
# cargo-pas'ı her platform için derleyip eklentilerle birlikte yayınlar.
set -eu

kok="$(cd "$(dirname "$0")/.." && pwd)"
cd "$kok"

hata() {
    echo "hata: $1" >&2
    exit 1
}

surum=""
not=""
gonder=0
for arguman in "$@"; do
    case "$arguman" in
        --gonder) gonder=1 ;;
        -*) hata "bilinmeyen seçenek: $arguman" ;;
        *) if [ -z "$surum" ]; then surum="$arguman"; else not="$arguman"; fi ;;
    esac
done
[ -n "$surum" ] || hata "kullanım: betikler/surum.sh <sürüm> [sürüm notu] [--gonder]"
echo "$surum" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' || hata "sürüm 0.1.2 biçiminde olmalı: $surum"

eski="$(sed -n 's/^version = "\(.*\)"$/\1/p' Cargo.toml | head -n 1)"
[ "$eski" != "$surum" ] || hata "sürüm zaten $surum"
[ -z "$(git status --porcelain)" ] || hata "çalışma dizininde commit edilmemiş değişiklikler var"
[ "$(git rev-parse --abbrev-ref HEAD)" = "main" ] || hata "yeni sürüm yalnızca main dalından çıkarılır"
! git rev-parse -q --verify "refs/tags/v$surum" >/dev/null || hata "v$surum etiketi zaten var"
command -v npm >/dev/null 2>&1 || hata "npm gerekli (vscode/package-lock.json için)"

echo "Sürüm: $eski → $surum"

# GNU ve BSD (macOS) sed'inde aynı çalışan yerinde düzenleme
duzenle() {
    sed -i.yedek "$1" "$2"
    rm -f "$2.yedek"
}

duzenle "s/^version = \"$eski\"\$/version = \"$surum\"/" Cargo.toml
duzenle "s/^version = \"$eski\"\$/version = \"$surum\"/" eklenti/build.gradle.kts
(cd vscode && npm version "$surum" --no-git-tag-version --allow-same-version >/dev/null)

if [ -n "$not" ]; then
    # Yeni not, sürüm notlarının en üstüne eklenir.
    satir="    <p>$surum: $not</p>"
    awk -v satir="$satir" '{ print } /<change-notes><!\[CDATA\[/ { print satir }' \
        eklenti/src/main/resources/META-INF/plugin.xml >eklenti/plugin.xml.yeni
    mv eklenti/plugin.xml.yeni eklenti/src/main/resources/META-INF/plugin.xml
fi

echo "Cargo.lock güncelleniyor ve testler çalışıyor…"
cargo test --quiet >/dev/null
[ "$(cargo run -q -p cargo-pas -- pas sürüm)" = "cargo-pas $surum" ] || hata "cargo-pas sürümü güncellenmedi"

git add Cargo.toml Cargo.lock eklenti/build.gradle.kts eklenti/src/main/resources/META-INF/plugin.xml \
    vscode/package.json vscode/package-lock.json
git commit -q -m "Sürüm $surum"
git tag -a "v$surum" -m "Pas $surum"
echo "Commit ve v$surum etiketi oluşturuldu."

if [ "$gonder" = 1 ]; then
    git push origin main "v$surum"
    echo "Gönderildi; yayın: https://github.com/kasimtmc/Pas/actions/workflows/yayin.yml"
else
    echo
    echo "Yayınlamak için:"
    echo "    git push origin main v$surum"
fi
