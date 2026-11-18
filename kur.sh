#!/bin/sh
# Pas kurulumu (Linux ve macOS): `cargo-pas` programını GitHub'daki son
# yayından indirip ~/.cargo/bin dizinine koyar.
#
#   curl -fsSL https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.sh | sh
set -eu

DEPO="kasimtmc/Pas"
HEDEF_DIZIN="${CARGO_HOME:-$HOME/.cargo}/bin"

hata() {
    echo "hata: $1" >&2
    exit 1
}

case "$(uname -s)" in
    Linux) sistem="unknown-linux-musl" ;;
    Darwin) sistem="apple-darwin" ;;
    *) hata "bu sistem desteklenmiyor: $(uname -s). Windows için kur.ps1 dosyasını kullan." ;;
esac
case "$(uname -m)" in
    x86_64 | amd64) islemci="x86_64" ;;
    arm64 | aarch64) islemci="aarch64" ;;
    *) hata "bu işlemci desteklenmiyor: $(uname -m)" ;;
esac
dosya="cargo-pas-$islemci-$sistem"
adres="https://github.com/$DEPO/releases/latest/download"

if command -v curl >/dev/null 2>&1; then
    indir() { curl -fsSL "$1" -o "$2"; }
elif command -v wget >/dev/null 2>&1; then
    indir() { wget -q "$1" -O "$2"; }
else
    hata "curl ya da wget gerekli"
fi

gecici="$(mktemp -d)"
trap 'rm -rf "$gecici"' EXIT

echo "İndiriliyor: $dosya"
indir "$adres/$dosya" "$gecici/cargo-pas" || hata "indirilemedi: $adres/$dosya"
indir "$adres/SHA256SUMS" "$gecici/SHA256SUMS" || hata "özet dosyası indirilemedi"

beklenen="$(grep " $dosya\$" "$gecici/SHA256SUMS" | cut -d ' ' -f 1)"
if command -v sha256sum >/dev/null 2>&1; then
    bulunan="$(sha256sum "$gecici/cargo-pas" | cut -d ' ' -f 1)"
else
    bulunan="$(shasum -a 256 "$gecici/cargo-pas" | cut -d ' ' -f 1)"
fi
[ -n "$beklenen" ] && [ "$beklenen" = "$bulunan" ] || hata "indirilen dosyanın özeti tutmuyor; dosya bozuk olabilir"

mkdir -p "$HEDEF_DIZIN"
mv "$gecici/cargo-pas" "$HEDEF_DIZIN/cargo-pas"
chmod +x "$HEDEF_DIZIN/cargo-pas"
echo "Kuruldu: $HEDEF_DIZIN/cargo-pas"

if ! command -v cargo >/dev/null 2>&1; then
    echo
    echo "Pas programlarını derlemek için Rust da gerekli. Kurmak için:"
    echo "    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
elif ! rust-analyzer --version >/dev/null 2>&1; then
    echo
    echo "Editörde tanıma gitme ve akıllı tamamlama için:"
    echo "    rustup component add rust-analyzer"
fi
case ":$PATH:" in
    *":$HEDEF_DIZIN:"*) ;;
    *) echo; echo "Not: $HEDEF_DIZIN dizini PATH içinde değil; yeni bir terminal açman gerekebilir." ;;
esac
echo
echo "Başlamak için: cargo pas yeni ilk-projem"
