//! Bir Pas projesini, cargo'nun derleyebileceği bir "gölge" projeye çevirir.
//!
//! Gölge proje `target/pas/` altında durur ve kullanıcının projesinin
//! aynısıdır; tek farkı `.pas` dosyalarının yerinde çevrilmiş `.rs`
//! dosyalarının bulunmasıdır (`src/ana.pas` → `src/main.rs`). Dosyalar
//! yalnızca içerikleri değiştiğinde yeniden yazılır; böylece cargo'nun
//! artımlı derlemesi bozulmaz.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use pas_cekirdek::{EXTENSION, Options, SourceMap};

/// Gölge projenin, kullanıcının projesine göre yolu.
pub const SHADOW_PREFIX: &str = "target/pas/";

/// cargo'nun kendiliğinden kaynak aradığı dizinler.
const MIRRORED_DIRS: &[&str] = &["src", "examples", "tests", "benches"];

pub struct Project {
    pub root: PathBuf,
    pub shadow: PathBuf,
    pub target_dir: PathBuf,
    pub package_name: Option<String>,
    /// Diskteki yerine kullanılacak dosya içeriği (göreli yol, içerik). IDE,
    /// kaydedilmemiş bir dosyayı denetletirken kullanır.
    pub unsaved: Option<(String, String)>,
}

/// Çevrilmiş bir Pas dosyası.
pub struct GeneratedFile {
    /// Gölge projeye göre Rust dosyasının yolu, örneğin `src/main.rs`.
    pub rust_path: String,
    /// Projeye göre Pas dosyasının yolu, örneğin `src/ana.pas`.
    pub pas_path: String,
    pub pas_source: String,
    pub rust_source: String,
    pub map: SourceMap,
}

pub struct Prepared {
    pub files: Vec<GeneratedFile>,
}

impl Project {
    /// Bulunulan dizinden yukarı doğru `Cargo.toml` arar.
    pub fn find(start: &Path) -> Result<Project, String> {
        let root = start
            .ancestors()
            .find(|dir| dir.join("Cargo.toml").is_file())
            .ok_or(
                "burada bir Pas projesi yok (`Cargo.toml` bulunamadı).\n\
                    Yeni bir proje oluşturmak için: cargo pas yeni <ad>",
            )?
            .to_path_buf();
        let manifest =
            fs::read_to_string(root.join("Cargo.toml")).map_err(|e| format!("`Cargo.toml` okunamadı: {e}"))?;
        let package_name = manifest
            .parse::<toml_edit::DocumentMut>()
            .ok()
            .and_then(|doc| doc.get("package")?.get("name")?.as_str().map(str::to_owned));
        let target_dir = root.join("target");
        Ok(Project { shadow: target_dir.join("pas"), target_dir, root, package_name, unsaved: None })
    }

    pub fn shadow_manifest(&self) -> PathBuf {
        self.shadow.join("Cargo.toml")
    }

    /// Gölge projeyi oluşturur ya da günceller.
    pub fn prepare(&self) -> Result<Prepared, String> {
        let mut written = HashSet::new();
        let mut files = Vec::new();

        for dir in MIRRORED_DIRS {
            let source_dir = self.root.join(dir);
            if source_dir.is_dir() {
                self.mirror_dir(&source_dir, &mut files, &mut written)?;
            }
        }
        if self.root.join("build.rs").is_file() {
            self.copy_file(Path::new("build.rs"), &mut written)?;
        }
        self.write_manifest(&mut written)?;

        for dir in MIRRORED_DIRS {
            remove_stale(&self.shadow.join(dir), &written);
        }
        files.sort_by(|a, b| a.pas_path.cmp(&b.pas_path));
        Ok(Prepared { files })
    }

    fn mirror_dir(
        &self,
        dir: &Path,
        files: &mut Vec<GeneratedFile>,
        written: &mut HashSet<PathBuf>,
    ) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("`{}` okunamadı: {e}", dir.display()))?;
        for entry in entries {
            let path = entry.map_err(|e| e.to_string())?.path();
            let relative = path.strip_prefix(&self.root).expect("proje içinde").to_path_buf();
            if path.is_dir() {
                self.mirror_dir(&path, files, written)?;
            } else if path.extension().is_some_and(|ext| ext == EXTENSION) {
                files.push(self.translate_file(&relative, written)?);
            } else {
                self.copy_file(&relative, written)?;
            }
        }
        Ok(())
    }

    fn translate_file(&self, relative: &Path, written: &mut HashSet<PathBuf>) -> Result<GeneratedFile, String> {
        let pas_path = slash_path(relative);
        let rust_relative = rust_path_for(relative);
        if self.root.join(&rust_relative).exists() {
            return Err(format!(
                "`{pas_path}` ve `{}` aynı anda var; ikisinden birini sil",
                slash_path(&rust_relative)
            ));
        }
        let pas_source = match &self.unsaved {
            Some((path, contents)) if *path == pas_path => contents.clone(),
            _ => fs::read_to_string(self.root.join(relative)).map_err(|e| format!("`{pas_path}` okunamadı: {e}"))?,
        };

        let module_path = |name: &str| self.module_path(relative, name);
        let translation = pas_cekirdek::to_rust_with(&pas_source, &Options { module_path: Some(&module_path) });

        write_if_changed(&self.shadow.join(&rust_relative), translation.rust.as_bytes())?;
        written.insert(self.shadow.join(&rust_relative));
        Ok(GeneratedFile {
            rust_path: slash_path(&rust_relative),
            pas_path,
            pas_source,
            rust_source: translation.rust,
            map: translation.map,
        })
    }

    /// `mod ad;` bildiriminin `#[path]` yolu. rustc bu yolu bildirimi
    /// yapan dosyanın dizinine göre çözer.
    fn module_path(&self, declaring_file: &Path, name: &str) -> String {
        let dir = declaring_file.parent().unwrap_or(Path::new(""));
        let rust_name = rust_path_for(declaring_file);
        let stem = rust_name.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        // `main.rs`, `lib.rs`, `mod.rs` ve sandık kökleri alt modüllerini
        // kendi dizinlerinde arar; diğer dosyalar `dosya_adı/` altında.
        let is_root = matches!(stem, "main" | "lib" | "mod") || is_crate_root(declaring_file);
        let base = if is_root { String::new() } else { format!("{stem}/") };
        let as_dir = self.root.join(dir).join(&base).join(name);
        if as_dir.join(format!("mod.{EXTENSION}")).is_file() || as_dir.join("mod.rs").is_file() {
            format!("{base}{name}/mod.rs")
        } else {
            format!("{base}{name}.rs")
        }
    }

    fn copy_file(&self, relative: &Path, written: &mut HashSet<PathBuf>) -> Result<(), String> {
        let contents =
            fs::read(self.root.join(relative)).map_err(|e| format!("`{}` okunamadı: {e}", relative.display()))?;
        let target = self.shadow.join(relative);
        write_if_changed(&target, &contents)?;
        written.insert(target);
        Ok(())
    }

    /// Kullanıcının `Cargo.toml` dosyasını gölge projeye uyarlar.
    fn write_manifest(&self, written: &mut HashSet<PathBuf>) -> Result<(), String> {
        let text =
            fs::read_to_string(self.root.join("Cargo.toml")).map_err(|e| format!("`Cargo.toml` okunamadı: {e}"))?;
        let mut doc: toml_edit::DocumentMut = text.parse().map_err(|e| format!("`Cargo.toml` hatalı: {e}"))?;

        // Gölge proje kendi başına bir çalışma alanıdır; yoksa cargo üst
        // dizinlerdeki bir çalışma alanına ait olduğunu sanabilir.
        if !doc.contains_key("workspace") {
            doc["workspace"] = toml_edit::table();
        }
        // Göreli bağımlılık yolları artık bir başka dizinden okunuyor.
        for table in dependency_tables(&mut doc) {
            for (_, dependency) in table.iter_mut() {
                if let Some(dependency) = dependency.as_table_like_mut()
                    && let Some(path) = dependency.get_mut("path")
                {
                    absolutize(path, &self.root);
                }
            }
        }
        // `[[bin]] path = "src/oyun.pas"` gibi hedef yolları
        // ve `path = "target/pas/src/main.rs"` gibi gölge projeyi gösteren yollar.
        // İkincisi, IDE'lerin (`cargo metadata`) `.rs` dosyası olmayan bir
        // projeyi okuyabilmesi için `cargo pas yeni` tarafından yazılır.
        for key in ["lib", "bin", "example", "test", "bench"] {
            for target in targets(&mut doc, key) {
                let Some(path) = target.get_mut("path") else { continue };
                let Some(text) = path.as_str().map(|t| t.replace('\\', "/")) else { continue };
                if let Some(inside) = text.strip_prefix(SHADOW_PREFIX) {
                    *path = toml_edit::value(inside);
                } else if text.ends_with(&format!(".{EXTENSION}")) {
                    *path = toml_edit::value(slash_path(&rust_path_for(Path::new(&text))));
                }
            }
        }

        let target = self.shadow_manifest();
        write_if_changed(&target, doc.to_string().as_bytes())?;
        written.insert(target);
        Ok(())
    }
}

/// Bir Pas dosyasının gölge projedeki Rust karşılığının yolu.
fn rust_path_for(pas: &Path) -> PathBuf {
    let stem = pas.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let parent = pas.parent().unwrap_or(Path::new(""));
    let in_src = parent == Path::new("src");
    let in_bin_dir = parent.parent().is_some_and(|p| p == Path::new("src/bin"));
    let rust_stem = match stem {
        "ana" if in_src || in_bin_dir => "main",
        "kütüphane" if in_src => "lib",
        _ => stem,
    };
    parent.join(format!("{rust_stem}.rs"))
}

/// `src/bin/x.pas`, `examples/x.pas` gibi dosyalar kendi sandıklarının köküdür.
fn is_crate_root(pas: &Path) -> bool {
    let parent = pas.parent().unwrap_or(Path::new(""));
    ["src/bin", "examples", "tests", "benches"].iter().any(|dir| parent == Path::new(dir))
}

fn dependency_tables(doc: &mut toml_edit::DocumentMut) -> Vec<&mut dyn toml_edit::TableLike> {
    const KEYS: [&str; 3] = ["dependencies", "dev-dependencies", "build-dependencies"];
    let mut tables: Vec<&mut dyn toml_edit::TableLike> = Vec::new();
    for (key, item) in doc.as_table_mut().iter_mut() {
        if KEYS.contains(&key.get()) {
            tables.extend(item.as_table_like_mut());
        } else if key.get() == "target"
            && let Some(targets) = item.as_table_like_mut()
        {
            for (_, platform) in targets.iter_mut() {
                let Some(platform) = platform.as_table_like_mut() else { continue };
                for (key, item) in platform.iter_mut() {
                    if KEYS.contains(&key.get()) {
                        tables.extend(item.as_table_like_mut());
                    }
                }
            }
        }
    }
    tables
}

fn targets<'d>(doc: &'d mut toml_edit::DocumentMut, key: &str) -> Vec<&'d mut dyn toml_edit::TableLike> {
    match doc.get_mut(key) {
        Some(toml_edit::Item::Table(table)) => vec![table as &mut dyn toml_edit::TableLike],
        Some(toml_edit::Item::ArrayOfTables(array)) => {
            array.iter_mut().map(|t| t as &mut dyn toml_edit::TableLike).collect()
        }
        _ => Vec::new(),
    }
}

fn absolutize(item: &mut toml_edit::Item, root: &Path) {
    if let Some(path) = item.as_str()
        && Path::new(path).is_relative()
    {
        *item = toml_edit::value(root.join(path).to_string_lossy().into_owned());
    }
}

fn write_if_changed(path: &Path, contents: &[u8]) -> Result<(), String> {
    if fs::read(path).is_ok_and(|old| old == contents) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("`{}` oluşturulamadı: {e}", parent.display()))?;
    }
    fs::write(path, contents).map_err(|e| format!("`{}` yazılamadı: {e}", path.display()))
}

/// Kaynağı silinmiş dosyaların gölgedeki kopyalarını temizler.
fn remove_stale(dir: &Path, written: &HashSet<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            remove_stale(&path, written);
            let _ = fs::remove_dir(&path); // yalnızca boşsa silinir
        } else if !written.contains(&path) {
            let _ = fs::remove_file(&path);
        }
    }
}

pub fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_paths() {
        let p = |s: &str| slash_path(&rust_path_for(Path::new(s)));
        assert_eq!(p("src/ana.pas"), "src/main.rs");
        assert_eq!(p("src/kütüphane.pas"), "src/lib.rs");
        assert_eq!(p("src/oyun/ana.pas"), "src/oyun/ana.rs");
        assert_eq!(p("src/bin/araç/ana.pas"), "src/bin/araç/main.rs");
        assert_eq!(p("src/çalış.pas"), "src/çalış.rs");
        assert_eq!(p("examples/ana.pas"), "examples/ana.rs");
    }
}
