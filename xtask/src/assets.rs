//! `cargo xtask assets`: every non-code asset must be attributed in `ASSETS.md`.
//!
//! Scans all git-tracked (and untracked, not ignored) files with an asset extension, plus
//! everything under `assets/`, `docs/images/` and `examples/`, and fails if a path is not listed
//! as `` `path` `` in ASSETS.md. See the asset policy in AGENTS.md.
//!
//! The vendored trees keep their own attribution (`libs/` ships its forks' licences,
//! `assets/craft-fonts/ATTRIBUTION.md` covers the fonts repo's files); build output never ships.
//! Those paths are out of the scan, so a fork's assets never need a row here.

use std::path::Path;

const ASSET_EXT: &[&str] = &[
    "png",
    "jpg",
    "jpeg",
    "gif",
    "webp",
    "bmp",
    "tif",
    "tiff",
    "ico",
    "icns",
    "svg",
    "pdf",
    "ai",
    "eps",
    "psd",
    "icc",
    "icm",
    "ttf",
    "otf",
    "woff",
    "woff2",
    "ase",
    "aco",
    "abr",
    "vectorcraft",
    "drawcraft",
    "mp4",
    "wav",
    "mp3",
];
const ASSET_DIRS: &[&str] = &["assets/", "docs/images/", "examples/"];

/// Paths whose files this repo does not attribute: vendored forks with their own licence and
/// attribution files, and build output (git-ignored today; listed so a change to the ignore rules
/// cannot make the scan demand rows for it).
const EXCLUDE: &[&str] = &["libs/", "assets/craft-fonts/", "target/", "dist/"];

pub fn is_asset(path: &str) -> bool {
    let ext = Path::new(path).extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).unwrap_or_default();
    ASSET_DIRS.iter().any(|d| path.starts_with(d)) || ASSET_EXT.contains(&ext.as_str())
}

/// Whether `path` is out of the scan (vendored tree or build output).
pub fn excluded(path: &str) -> bool {
    EXCLUDE.iter().any(|dir| path.starts_with(dir))
}

/// Paths that need an ASSETS.md entry but lack one.
pub fn missing(files: &[String], assets_md: &str) -> Vec<String> {
    files.iter().filter(|f| !excluded(f) && is_asset(f) && !assets_md.contains(&format!("`{f}`"))).cloned().collect()
}

pub fn run(root: &Path) -> Result<(), String> {
    let files = crate::repo_files(root)?;
    let md = std::fs::read_to_string(root.join("ASSETS.md")).map_err(|e| format!("ASSETS.md: {e}"))?;
    let miss = missing(&files, &md);
    if miss.is_empty() {
        let counted = files.iter().filter(|f| !excluded(f) && is_asset(f)).count();
        println!("assets: all {counted} asset files attributed in ASSETS.md");
        Ok(())
    } else {
        Err(format!("{} asset file(s) lack an ASSETS.md entry (author, source, licence):\n  {}", miss.len(), miss.join("\n  ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unattributed_assets() {
        let files = vec!["assets/icons/a.svg".to_string(), "crates/x/src/lib.rs".into(), "docs/images/b.png".into(), "tests/fixture.png".into()];
        let md = "| `assets/icons/a.svg` | me | here | MIT |";
        assert_eq!(missing(&files, md), vec!["docs/images/b.png".to_string(), "tests/fixture.png".into()]);
        assert!(!is_asset("crates/x/src/lib.rs"));
        assert!(is_asset("assets/fonts/OFL.txt"));
    }

    #[test]
    fn vendored_trees_and_build_output_are_out_of_the_scan() {
        let files = vec![
            "libs/egui/web_demo/favicon.ico".to_string(),
            "libs/winprint.rs/test_data/test_document.pdf".into(),
            "assets/craft-fonts/fonts/BIZ-UDGothic.ttf".into(),
            "target/release/icon.png".into(),
            "dist/AppImage/icon.svg".into(),
            "assets/icons/mine.svg".into(),
        ];
        assert_eq!(missing(&files, ""), vec!["assets/icons/mine.svg".to_string()], "only our own assets need a row");
    }
}
