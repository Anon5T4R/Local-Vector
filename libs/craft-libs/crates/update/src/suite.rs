//! The Craft App Suite as releases know it: each app's id, name and repository.
//!
//! An app that updates itself takes its own entry (`suite::LOCALPAINT`); the launcher takes all
//! of them. Installers use the list to tell a Craft App's package from anyone else's.

use crate::app::App;

pub const PHOTOCRAFT: App = App::new("photocraft", "PhotoCraft", "Image editing", "storytold/photocraft");
pub const LOCALPAINT: App = App {
    former_names: &["LocalPaint"],
    publisher: "Local Paint contributors",
    ..App::new("localpaint", "Local Paint", "Image editing", "Anon5T4R/Local-Paint")
};
pub const LOCALVECTOR: App = App {
    former_names: &["LocalVector"],
    publisher: "Local Vector contributors",
    ..App::new("localvector", "Local Vector", "Vector illustration", "Anon5T4R/Local-Vector")
};
pub const VECTORCRAFT: App = App::new("vectorcraft", "VectorCraft", "Vector illustration", "storytold/vectorcraft");
pub const FILMCRAFT: App = App::new("filmcraft", "FilmCraft", "Video editing, color and sound", "storytold/filmcraft");
pub const LIGHTCRAFT: App = App::new("lightcraft", "LightCraft", "Photo library and raw development", "storytold/lightcraft");
pub const PDFCRAFT: App = App {
    former_ids: &["printcraft"],
    former_names: &["PrintCraft"],
    ..App::new("pdfcraft", "PdfCraft", "Reading, organizing and protecting PDFs", "storytold/pdfcraft")
};
pub const EFFECTCRAFT: App = App::new("effectcraft", "EffectCraft", "Motion graphics and visual effects", "storytold/effectcraft");
pub const DESIGNCRAFT: App = App::new("designcraft", "DesignCraft", "Page layout and publishing", "storytold/designcraft");
pub const FIGCRAFT: App = App::new("figcraft", "FigCraft", "Interface design and prototyping", "storytold/figcraft");
pub const WORDCRAFT: App = App::new("wordcraft", "WordCraft", "Writing and document design", "storytold/wordcraft");
pub const SOUNDCRAFT: App = App::new("soundcraft", "SoundCraft", "Recording, editing and mixing audio", "storytold/soundcraft");
pub const DECKCRAFT: App = App::new("deckcraft", "DeckCraft", "Presentations and slide shows", "storytold/deckcraft");
pub const CADCRAFT: App = App::new("cadcraft", "CADCraft", "Computer-aided design and drafting", "storytold/cadcraft");
pub const GRIDCRAFT: App = App::new("gridcraft", "GridCraft", "Spreadsheets and data", "storytold/gridcraft");

/// ArtCraft Launcher, described like a Craft App so it updates itself with the same machinery
/// (asset names `artcraft-launcher-<ver>-…`, bundle id `ai.storyteller.artcraft-launcher`,
/// `ArtCraft Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: App = App::new("artcraft-launcher", "ArtCraft Launcher", "Installs and updates the Craft App Suite", "storytold/craft-launcher");

/// The Craft App Suite, in getartcraft.com's display order.
pub const SUITE: [App; 15] = [
    PHOTOCRAFT,
    LOCALPAINT,
    LOCALVECTOR,
    VECTORCRAFT,
    FILMCRAFT,
    LIGHTCRAFT,
    PDFCRAFT,
    EFFECTCRAFT,
    DESIGNCRAFT,
    FIGCRAFT,
    WORDCRAFT,
    SOUNDCRAFT,
    DECKCRAFT,
    CADCRAFT,
    GRIDCRAFT,
];

/// The suite's app with this id.
pub fn by_id(id: &str) -> Option<&'static App> {
    SUITE.iter().find(|a| a.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thirteen_apps_with_unique_ids_in_their_own_repos() {
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            if a.id == LOCALPAINT.id {
                assert_eq!(a.repo, "Anon5T4R/Local-Paint");
                continue;
            }
            if a.id == LOCALVECTOR.id {
                assert_eq!(a.repo, "Anon5T4R/Local-Vector");
                continue;
            }
            assert_eq!(a.repo, format!("storytold/{}", a.id));
            assert_eq!(a.name.to_lowercase(), a.id);
        }
        // The suite's own apps spell their display name in two words, whatever their id says.
        for a in [LOCALPAINT, LOCALVECTOR] {
            assert!(a.name.starts_with("Local "), "{a:?}");
            assert_eq!(a.names().count(), 2, "{a:?}: the name it had before the rename counts");
        }
        assert!(by_id(LAUNCHER.id).is_none());
        assert_eq!(by_id("localpaint").map(|a| a.repo), Some("Anon5T4R/Local-Paint"));
        assert_eq!(by_id("pdfcraft").map(|a| a.former_ids), Some(&["printcraft"][..]));
    }
}
