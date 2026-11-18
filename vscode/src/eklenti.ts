// Pas için VS Code eklentisi: `cargo pas lsp` dil sunucusunu başlatır
// (tanıma gitme, hover, tamamlama, Türkçe hatalar ve hızlı düzeltmeler) ve
// programı çalıştırmak için bir komut ile "▶ Çalıştır" bağlantısı ekler.
import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import * as vscode from "vscode";
import { LanguageClient, LanguageClientOptions, ServerOptions } from "vscode-languageclient/node";

let istemci: LanguageClient | undefined;

const KURULUM_KOMUTU =
  process.platform === "win32"
    ? "irm https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.ps1 | iex"
    : "curl -fsSL https://raw.githubusercontent.com/kasimtmc/Pas/main/kur.sh | sh";

export async function activate(baglam: vscode.ExtensionContext): Promise<void> {
  baglam.subscriptions.push(
    vscode.commands.registerCommand("pas.calistir", calistir),
    vscode.commands.registerCommand("pas.kur", kur),
    vscode.languages.registerCodeLensProvider({ language: "pas" }, new CalistirBaglantisi()),
  );

  const cargoPas = cargoPasBul();
  if (!cargoPas) {
    const secim = await vscode.window.showWarningMessage(
      "Pas için cargo-pas bulunamadı. Pas dosyalarını çalıştırmak ve Türkçe hataları görmek için kurman gerekiyor.",
      "Kur",
    );
    if (secim === "Kur") {
      kur();
    }
    return;
  }

  const sunucu: ServerOptions = { command: cargoPas, args: ["lsp"] };
  const secenekler: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "pas" }],
    // Türkçe hatalar ve hızlı düzeltmeler de dil sunucusundan gelsin.
    initializationOptions: { tanilamalar: true },
  };
  istemci = new LanguageClient("pas", "Pas", sunucu, secenekler);
  await istemci.start();
}

export async function deactivate(): Promise<void> {
  await istemci?.stop();
}

/** Ayarlardaki yol, PATH ya da ~/.cargo/bin içinde `cargo-pas`. */
function cargoPasBul(): string | undefined {
  const ayar = vscode.workspace.getConfiguration("pas").get<string>("cargoPasYolu");
  if (ayar) {
    return fs.existsSync(ayar) ? ayar : undefined;
  }
  const ad = process.platform === "win32" ? "cargo-pas.exe" : "cargo-pas";
  const cargoEvi = process.env.CARGO_HOME ?? path.join(os.homedir(), ".cargo");
  const dizinler = [...(process.env.PATH ?? "").split(path.delimiter), path.join(cargoEvi, "bin")];
  return dizinler.map((dizin) => path.join(dizin, ad)).find((yol) => yol && fs.existsSync(yol));
}

/** Dosyayı içeren Pas projesinin kökü (`Cargo.toml` dosyasının dizini). */
function projeKoku(dosya: string): string | undefined {
  let dizin = path.dirname(dosya);
  while (true) {
    if (fs.existsSync(path.join(dizin, "Cargo.toml"))) {
      return dizin;
    }
    const ust = path.dirname(dizin);
    if (ust === dizin) {
      return undefined;
    }
    dizin = ust;
  }
}

async function calistir(): Promise<void> {
  const belge = vscode.window.activeTextEditor?.document;
  if (!belge || belge.languageId !== "pas") {
    vscode.window.showWarningMessage("Çalıştırmak için bir Pas dosyası aç.");
    return;
  }
  const kok = projeKoku(belge.uri.fsPath);
  if (!kok) {
    vscode.window.showWarningMessage("Bu dosya bir Pas projesinde değil (Cargo.toml bulunamadı).");
    return;
  }
  const cargoPas = cargoPasBul();
  if (!cargoPas) {
    kur();
    return;
  }
  await vscode.workspace.saveAll(false);
  const terminal = terminalBul(`Pas: ${path.basename(kok)}`, kok);
  terminal.show();
  // Terminalin kodlamasından etkilenmesin diye komutun ASCII adı kullanılır.
  terminal.sendText(process.platform === "win32" ? `& "${cargoPas}" calistir` : `"${cargoPas}" calistir`);
}

function kur(): void {
  const terminal = terminalBul("Pas kurulumu");
  terminal.show();
  terminal.sendText(KURULUM_KOMUTU);
  vscode.window.showInformationMessage("Kurulum bitince VS Code penceresini yeniden yükle (Developer: Reload Window).");
}

function terminalBul(ad: string, dizin?: string): vscode.Terminal {
  return vscode.window.terminals.find((t) => t.name === ad) ?? vscode.window.createTerminal({ name: ad, cwd: dizin });
}

/** `işlev ana()` satırının üstünde "▶ Çalıştır" bağlantısı. */
class CalistirBaglantisi implements vscode.CodeLensProvider {
  provideCodeLenses(belge: vscode.TextDocument): vscode.CodeLens[] {
    const desen = /^\s*(?:genel\s+)?(?:işlev|fn)\s+ana\s*\(/mu;
    const eslesme = desen.exec(belge.getText());
    if (!eslesme) {
      return [];
    }
    const satir = belge.positionAt(eslesme.index + eslesme[0].length - 1).line;
    return [new vscode.CodeLens(new vscode.Range(satir, 0, satir, 0), { title: "▶ Çalıştır", command: "pas.calistir" })];
  }
}
