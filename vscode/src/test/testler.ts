// VS Code içinde çalışan testler: Türkçe tanılama, hızlı düzeltme ve tanıma gitme.
import * as assert from "node:assert";
import * as path from "node:path";
import * as vscode from "vscode";

async function bekle<T>(aciklama: string, deneme: () => Promise<T | undefined> | T | undefined): Promise<T> {
  const son = Date.now() + 180_000;
  while (Date.now() < son) {
    const sonuc = await deneme();
    if (sonuc !== undefined) {
      return sonuc;
    }
    await new Promise((r) => setTimeout(r, 500));
  }
  throw new Error(`Zaman aşımı: ${aciklama}`);
}

export async function run(): Promise<void> {
  const kok = vscode.workspace.workspaceFolders![0].uri.fsPath;
  const ana = vscode.Uri.file(path.join(kok, "src", "ana.pas"));
  const belge = await vscode.workspace.openTextDocument(ana);
  await vscode.window.showTextDocument(belge);
  assert.strictEqual(belge.languageId, "pas");

  // Türkçe tanılama: `ş` değiştirilemez bir değişken.
  const hata = await bekle("E0384 tanılaması", () =>
    vscode.languages.getDiagnostics(ana).find((t) => String(typeof t.code === "object" ? t.code.value : t.code) === "E0384"),
  );
  assert.match(hata.message, /`ş` değiştirilemez/);
  console.log(`tanılama: ${hata.message.split("\n")[0]}`);

  // Hızlı düzeltme: `değişken` ekle.
  const eylemler = await vscode.commands.executeCommand<vscode.CodeAction[]>(
    "vscode.executeCodeActionProvider",
    ana,
    hata.range,
  );
  const duzeltme = eylemler.find((e) => e.title === "Ekle: değişken");
  assert.ok(duzeltme, `düzeltme yok: ${eylemler.map((e) => e.title).join(", ")}`);
  console.log(`hızlı düzeltme: ${duzeltme.title}`);

  // Tanıma gitme: `topla` → hesap.pas
  const konum = belge.positionAt(belge.getText().indexOf("topla(1"));
  const tanimlar = await bekle("tanım", async () => {
    const sonuc = await vscode.commands.executeCommand<(vscode.Location | vscode.LocationLink)[]>(
      "vscode.executeDefinitionProvider",
      ana,
      konum,
    );
    return sonuc && sonuc.length > 0 ? sonuc : undefined;
  });
  const hedef = "targetUri" in tanimlar[0] ? tanimlar[0].targetUri : tanimlar[0].uri;
  assert.strictEqual(path.basename(hedef.fsPath), "hesap.pas");
  console.log(`tanım: ${path.relative(kok, hedef.fsPath)}`);
}
