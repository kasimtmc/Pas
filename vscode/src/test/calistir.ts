// Eklentiyi indirilmiş bir VS Code içinde test-proje ile çalıştırır.
import * as path from "node:path";
import { runTests } from "@vscode/test-electron";

async function ana(): Promise<void> {
  const eklenti = path.resolve(__dirname, "../..");
  await runTests({
    extensionDevelopmentPath: eklenti,
    extensionTestsPath: path.resolve(__dirname, "testler.js"),
    launchArgs: [path.join(eklenti, "test-proje"), "--disable-extensions"],
  });
}

ana().catch((hata) => {
  console.error(hata);
  process.exit(1);
});
