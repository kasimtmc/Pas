# Pas kurulumu (Windows): `cargo-pas.exe` programını GitHub'daki son
# yayından indirip %USERPROFILE%\.cargo\bin dizinine koyar.
#
#   irm https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.ps1 | iex
$ErrorActionPreference = "Stop"

$depo = "kasimtmc/Pas"
$cargoEvi = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE ".cargo" }
$hedefDizin = Join-Path $cargoEvi "bin"

if (-not [Environment]::Is64BitOperatingSystem) {
    Write-Error "Pas yalnızca 64 bit Windows'ta çalışır."
}
$dosya = "cargo-pas-x86_64-pc-windows-msvc.exe"
$adres = "https://github.com/$depo/releases/latest/download"
$gecici = Join-Path ([IO.Path]::GetTempPath()) ([Guid]::NewGuid())
New-Item -ItemType Directory -Path $gecici | Out-Null

try {
    Write-Host "İndiriliyor: $dosya"
    $program = Join-Path $gecici "cargo-pas.exe"
    $ozetler = Join-Path $gecici "SHA256SUMS"
    Invoke-WebRequest -UseBasicParsing "$adres/$dosya" -OutFile $program
    Invoke-WebRequest -UseBasicParsing "$adres/SHA256SUMS" -OutFile $ozetler

    $beklenen = (Get-Content $ozetler | Where-Object { $_ -match " $([regex]::Escape($dosya))$" }) -split " " | Select-Object -First 1
    $bulunan = (Get-FileHash -Algorithm SHA256 $program).Hash.ToLower()
    if (-not $beklenen -or $beklenen -ne $bulunan) {
        Write-Error "İndirilen dosyanın özeti tutmuyor; dosya bozuk olabilir."
    }

    New-Item -ItemType Directory -Force -Path $hedefDizin | Out-Null
    Move-Item -Force $program (Join-Path $hedefDizin "cargo-pas.exe")
    Write-Host "Kuruldu: $(Join-Path $hedefDizin 'cargo-pas.exe')"
}
finally {
    Remove-Item -Recurse -Force $gecici -ErrorAction SilentlyContinue
}

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host ""
    Write-Host "Pas programlarını derlemek için Rust da gerekli: https://rustup.rs adresinden kurabilirsin."
}
elseif (-not (Get-Command rust-analyzer -ErrorAction SilentlyContinue)) {
    Write-Host ""
    Write-Host "Editörde tanıma gitme ve akıllı tamamlama için: rustup component add rust-analyzer"
}
Write-Host ""
Write-Host "Başlamak için: cargo pas yeni ilk-projem"
