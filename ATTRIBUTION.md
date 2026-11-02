# Attribution

Every bundled asset, with its author, source and license, is listed per file in
[`ASSETS.md`](ASSETS.md); that table is the authoritative one, and `cargo xtask assets` (part of
`cargo xtask ci`) fails on any asset missing from it. This file records the material that is not
an app asset: documentation and screenshots, the vendored forks, test data and the licences kept
next to them.

LocalVector's own code and original artwork are MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). The code derives from VectorCraft
(ArtCraft), also MIT OR Apache-2.0; that credit is in [`NOTICE`](NOTICE) and in the upstream
copyright line of [`LICENSE-MIT`](LICENSE-MIT).

## Documentation and screenshots

| Path | Author | Source | License |
|---|---|---|---|
| `docs/images/shot-*.png` | LocalVector contributors | Screenshots of LocalVector itself (its own UI icons and Lucide), taken from the example files in `examples/` | MIT OR Apache-2.0 |
| `docs/images/art-*.png`, `docs/images/dusk-poster.png` | LocalVector contributors | Rendered by LocalVector from the example files in `examples/` | MIT OR Apache-2.0 |
| `examples/*.vectorcraft`, `examples/*.svg` | LocalVector contributors | Original artwork drawn through the LocalVector command API | MIT OR Apache-2.0 |
| `packaging/macos/dmg/background.svg`, `.tiff`, `DS_Store` | Xus Badia (contributed in #493) | Original artwork for the DMG window, rebuilt by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 |

## Vendored forks

`libs/` holds forks of the crates the desktop app builds against. Each checkout keeps its upstream
licence files and its patch ledger; all of them are MIT OR Apache-2.0, and the exact revisions are
pinned in `Cargo.toml` and recorded in the `Anon5T4R/local-deps` inventory.

| Path | Upstream | License |
|---|---|---|
| `libs/egui` | <https://github.com/emilk/egui>, fork at <https://github.com/storytold/egui> | MIT OR Apache-2.0 |
| `libs/winit` | <https://github.com/rust-windowing/winit>, fork at <https://github.com/storytold/winit> | Apache-2.0 |
| `libs/winprint.rs` | <https://github.com/storytold/winprint> | MIT OR Apache-2.0 |
| `libs/craft-libs` | <https://github.com/storytold/craft-libs> | MIT OR Apache-2.0 |
| `libs/craft-libs/vendor/hayro-*` | <https://github.com/LaurenzV/hayro> (patched copies) | MIT OR Apache-2.0, licence files in `licenses/pdf-backend/` |

## Fonts

The fonts embedded by the app come from
[craft-fonts](https://github.com/storytold/craft-fonts), vendored at `assets/craft-fonts/`
(committed, so releases and CI need no network). Each family's `OFL.txt` sits beside it and ships
with the packages as `OFL-<family>.txt`; the families and their licences are listed in
[`NOTICE`](NOTICE) and in `assets/craft-fonts/ATTRIBUTION.md`. The Latin UI and document fonts in
`assets/fonts/` keep their own `OFL-*.txt` files and are listed in [`ASSETS.md`](ASSETS.md).

## Test data

| Path | Origin | License |
|---|---|---|
| `crates/engine/tests/fixtures/jpeg2000/*` | Authored for LocalVector's tests with FFmpeg's JPEG 2000 encoder; hashes in `ASSETS.md` | MIT OR Apache-2.0 |
| `corpus/` (never committed) | Fetched at pinned commits by `cargo xtask corpus`; real-file oracles live in <https://github.com/storytold/photocraft-corpus> | per its own notices |
