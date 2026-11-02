//! Verify that PDF reading and writing use one immutable shared crate and patched backends.
//!
//! In this fork the shared crate and its backends are vendored under `libs/craft-libs` (a
//! snapshot of craft-libs at [`REV`], with krilla at [`KRILLA_REV`] inside `vendor/`): they
//! resolve from those paths, never from the upstream git sources. The gate still checks that
//! each resolves exactly once, at the pinned version, from the vendored tree — a registry copy
//! or a second resolution sneaking into the lock fails it.

use std::path::Path;
use std::process::Command;

use serde_json::Value;

/// The upstream revision the vendored `libs/craft-libs` tree is a snapshot of (provenance).
const REV: &str = "92d392cff132eaf53e13afff77f38b5c11b73aa0";
/// The writer's revision: craft-libs#417, CMYK, gray and spot colors with their exact inks.
const KRILLA_REV: &str = "fb08be5fa19eee8d37e624397fcdc96df6035e92";
/// (package, pinned version, directory under `libs/craft-libs/` it must resolve from).
const PACKAGES: &[(&str, &str, &str)] = &[
    ("craft-pdf", "0.1.0", "crates/pdf"),
    ("krilla", "0.8.2", "vendor/krilla"),
    ("hayro-syntax", "0.7.2", "vendor/hayro-syntax"),
    ("hayro-interpret", "0.7.0", "vendor/hayro-interpret"),
    ("hayro-jbig2", "0.3.0", "vendor/hayro-jbig2"),
    ("hayro-cmap", "0.1.0", "vendor/hayro-cmap"),
    ("hayro-postscript", "0.1.0", "vendor/hayro-postscript"),
];

/// Where the vendored craft-libs tree lives, relative to the workspace root.
const LIBS: &str = "libs/craft-libs";

pub fn run(root: &Path) -> Result<(), String> {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(root)
        .args(["metadata", "--locked", "--format-version", "1"])
        .output()
        .map_err(|e| format!("cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!("cargo metadata: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let metadata: Value = serde_json::from_slice(&output.stdout).map_err(|e| format!("cargo metadata: {e}"))?;
    validate(&metadata, root)?;
    println!("PDF: shared crate and patched backends resolve once from the vendored {LIBS} (craft-libs @ {REV}; krilla @ {KRILLA_REV})");
    Ok(())
}

fn validate(metadata: &Value, root: &Path) -> Result<(), String> {
    let packages = metadata.get("packages").and_then(Value::as_array).ok_or("cargo metadata has no packages")?;
    for &(name, version, dir) in PACKAGES {
        let expected = root.join(LIBS).join(dir).join("Cargo.toml");
        let mut found = packages.iter().filter(|p| p.get("name").and_then(Value::as_str) == Some(name));
        let package = found.next().ok_or_else(|| format!("{name}: missing resolved package"))?;
        if found.next().is_some() {
            return Err(format!("{name}: expected one resolved package"));
        }
        let manifest = package.get("manifest_path").and_then(Value::as_str).map(Path::new);
        if package.get("version").and_then(Value::as_str) != Some(version) || manifest != Some(expected.as_path()) {
            return Err(format!("{name}: expected {version} from {} (vendored {} @ {REV}); got {package}", expected.display(), dir));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn root() -> std::path::PathBuf {
        if cfg!(windows) { std::path::PathBuf::from(r"C:\repo") } else { std::path::PathBuf::from("/repo") }
    }

    fn manifest(dir: &str) -> String {
        root().join(LIBS).join(dir).join("Cargo.toml").to_str().expect("the vendored path is not valid UTF-8").to_owned()
    }

    fn metadata() -> Value {
        let packages: Vec<_> =
            PACKAGES.iter().map(|(name, version, dir)| json!({"name":name, "version":version, "manifest_path":manifest(dir)})).collect();
        json!({"packages": packages})
    }

    #[test]
    fn accepts_pinned_vendored_pdf_packages_without_changing_printer_transport() {
        let mut m = metadata();
        m["packages"].as_array_mut().unwrap().push(json!({"name":"craft-print", "version":"0.1.0", "manifest_path":manifest("crates/print")}));
        assert!(validate(&m, &root()).is_ok());
    }

    #[test]
    fn rejects_missing_or_duplicate_pdf_packages() {
        for index in 0..PACKAGES.len() {
            let mut m = metadata();
            m["packages"].as_array_mut().unwrap().remove(index);
            assert!(validate(&m, &root()).is_err());
            let mut m = metadata();
            let duplicate = m["packages"][index].clone();
            m["packages"].as_array_mut().unwrap().push(duplicate);
            assert!(validate(&m, &root()).is_err());
        }
    }

    #[test]
    fn rejects_registry_local_version_and_tree_drift() {
        for index in 0..PACKAGES.len() {
            let other_dir = if PACKAGES[index].2.starts_with("vendor/") { "crates/pdf" } else { "vendor/krilla" };
            for (field, value) in [
                ("manifest_path", json!("")),
                ("manifest_path", json!(manifest(other_dir))),
                ("manifest_path", json!(format!("{}{}", root().join(LIBS).to_str().expect("not UTF-8"), r"\elsewhere\Cargo.toml"))),
                ("version", json!("0.99.0")),
            ] {
                let shown = value.to_string();
                let mut m = metadata();
                m["packages"][index][field] = value;
                assert!(validate(&m, &root()).is_err(), "{index} {field} {shown}");
            }
        }
    }
}
