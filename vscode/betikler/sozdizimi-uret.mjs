// Pas sözlüğünden (`cargo pas sözlük --json`) VS Code için TextMate
// dilbilgisi üretir. Sözlüğün tek doğruluk kaynağı
// crates/pas-cekirdek/src/sozluk.rs dosyasıdır.
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const kok = join(dirname(fileURLToPath(import.meta.url)), "..", "..");
const json = execFileSync("cargo", ["run", "-q", "-p", "cargo-pas", "--", "pas", "sozluk", "--json"], {
  cwd: kok,
  encoding: "utf8",
});
const sozluk = JSON.parse(json);
const rustAnahtarSozcukleri = [
  "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
  "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
  "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "macro_rules", "union",
];

const kacir = (s) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
// Türkçe harfler de isme dahil olduğu için \b yerine Unicode sınırları kullanılır.
const ISIM_DISI = "(?<![\\p{L}\\p{N}_])";
const ISIM_SONU = "(?![\\p{L}\\p{N}_])";
const sozcukler = (liste) => `${ISIM_DISI}(?:${liste.sort((a, b) => b.length - a.length).map(kacir).join("|")})${ISIM_SONU}`;
const sinif = (tur) => sozluk.filter((g) => g.tur === tur).map((g) => g.pas);

const dilbilgisi = {
  $schema: "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
  name: "Pas",
  scopeName: "source.pas",
  patterns: [
    { include: "#yorumlar" },
    { include: "#dizgiler" },
    { name: "constant.character.pas", match: "b?'(?:\\\\.|\\\\u\\{[0-9a-fA-F]+\\}|[^'\\\\])'" },
    { name: "storage.modifier.lifetime.pas", match: "'[\\p{L}_][\\p{L}\\p{N}_]*" },
    { name: "constant.numeric.pas", match: "\\b\\d[\\d_]*(?:\\.\\d[\\d_]*)?(?:[eE][+-]?\\d+)?(?:[iuf](?:8|16|32|64|128|size))?\\b|\\b0x[\\da-fA-F_]+\\b" },
    { name: "meta.attribute.pas", begin: "#!?\\[", end: "\\]" },
    {
      match: `${ISIM_DISI}(işlev|fn)\\s+([\\p{L}_][\\p{L}\\p{N}_]*)`,
      captures: { 1: { name: "keyword.other.pas" }, 2: { name: "entity.name.function.pas" } },
    },
    { name: "entity.name.function.macro.pas", match: "[\\p{L}_][\\p{L}\\p{N}_]*!(?!=)" },
    { name: "keyword.other.pas", match: sozcukler([...sinif("anahtar_sozcuk"), ...rustAnahtarSozcukleri]) },
    { name: "support.type.pas", match: sozcukler(sinif("tur")) },
    { name: "support.constant.pas", match: sozcukler(sinif("deger")) },
    { name: "entity.name.type.pas", match: `${ISIM_DISI}\\p{Lu}[\\p{L}\\p{N}_]*${ISIM_SONU}` },
    { name: "keyword.operator.pas", match: "[-+*/%=!<>&|^]=?|::|\\.\\.=?|=>|->" },
  ],
  repository: {
    yorumlar: {
      patterns: [
        { name: "comment.line.documentation.pas", match: "//[/!].*$" },
        { name: "comment.line.double-slash.pas", match: "//.*$" },
        { name: "comment.block.pas", begin: "/\\*", end: "\\*/", patterns: [{ include: "#yorumlar" }] },
      ],
    },
    dizgiler: {
      patterns: [
        { name: "string.quoted.double.pas", begin: "[bc]?\"", end: "\"", patterns: [{ name: "constant.character.escape.pas", match: "\\\\." }, { name: "variable.other.placeholder.pas", match: "\\{[^{}]*\\}" }] },
        { name: "string.quoted.raw.pas", begin: "[bc]?r(#*)\"", end: "\"\\1" },
      ],
    },
  },
};

writeFileSync(join(kok, "vscode", "sozdizimi", "pas.tmLanguage.json"), JSON.stringify(dilbilgisi, null, 2) + "\n");
console.log(`Dilbilgisi üretildi: ${sozluk.length} sözlük girdisi`);
