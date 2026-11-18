## Kurulum

Önce [Rust](https://rustup.rs) kurulu olmalı. Sonra `cargo pas` komutunu kur:

**Linux ve macOS:**

```bash
curl -fsSL https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.ps1 | iex
```

Ya da aşağıdaki dosyalardan sistemine uygun olanı indirip adını `cargo-pas`
(Windows'ta `cargo-pas.exe`) yap ve `~/.cargo/bin` dizinine koy.

**VS Code eklentisi:** `pas-vscode-*.vsix` dosyasını VS Code'da
Uzantılar → ⋯ → VSIX'ten yükle (Install from VSIX) ile kurabilirsin.

**RustRover eklentisi:** `pas-eklenti-*.zip` dosyasını
Ayarlar → Eklentiler → ⚙ → Diskten eklenti yükle ile kurabilirsin.

Editörde tanıma gitme ve akıllı tamamlama için: `rustup component add rust-analyzer`
