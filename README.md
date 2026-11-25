<p align="center">
  <img src="assets/app-icon/hicolor/128x128/apps/com.localvector.app.png" width="96" height="96" alt="Local Vector icon: a white bezier curve with two solid anchor nodes on a teal tile">
</p>

<h1 align="center">Local Vector</h1>

<p align="center">
  A vector illustration app. Offline, no account, written in Rust.<br>
  <a href="https://github.com/Anon5T4R/Local-Vector/releases/latest">Download</a> ·
  <a href="ROADMAP.md">Roadmap and status</a> ·
  <a href="#where-it-comes-from">Where it comes from</a> ·
  <a href="#how-it-is-made">How it is made</a>
</p>

<!-- screenshot: a document open on the canvas, with the tool column, the control bar and the layers panel; dark and light -->

## What it does

- Draw and edit paths with the Pen, Curvature, Pencil and brush tools, down to anchors and handles;
  join, cut, erase and reshape them, and give a stroke a variable width along a path.
- Combine shapes with the Pathfinder operations, the Shape Builder and Live Paint. Draw with the
  line, arc, spiral, grid, rectangle, rounded rectangle, ellipse, polygon, star and flare tools.
- Paint with flat color, gradients and patterns, keep several fills and strokes on one object, and
  use swatches, brush and symbol libraries, opacity masks and clipping masks.
- Add live effects and raster effects, and turn tone into vector dots with Vector Halftone.
- Set type: point, area and on-path text, character and paragraph styles, fonts with shaping and
  bidirectional layout, and variable-font axes.
- Lay out with layers, groups, artboards, ruler and smart guides, saved views and slices.
- Place files as linked or embedded art, relink them, and place `.txt`, `.rtf`, `.docx` and `.doc`
  files as area text.
- Package a document with its links and the fonts its type uses.
- Trace a bitmap into paths with Image Trace, build charts from data with the graph tools, and run a
  data merge from a CSV with variables and data sets.
- Make 3D art from paths: revolve, extrude, inflate and rotate surfaces, with lights.
- Print with bleed, printer marks, tiling and separations, to a printer or to a print-ready PDF or
  PostScript file.
- Export artboards, layers, assets or the selection, at several scales at once. Save for Web writes
  an optimized web image, or one per slice with an HTML page placing them.
- Undo without a limit, and get unsaved work back after a crash with Data Recovery.
- Drive all of it from the command line, the JSON control channel or MCP
  ([docs/mcp.md](docs/mcp.md)), and extend it with resource-limited WebAssembly plug-ins.

What works well, what is weak and what is planned is in the [roadmap](ROADMAP.md).

## Formats

| | Formats |
|---|---|
| Native | `.vectorcraft` (versioned, compressed JSON with the images inside; older `.drawcraft` files still open) and `.vctemplate` templates |
| Opens | SVG, SVGZ, PDF, `.ai` and `.ait`, EPS and PostScript, DXF, EMF, WMF, PNG, JPEG, GIF, WebP, TIFF, BMP, Targa, PSD and PSB (the merged image), PCX, JPEG 2000, and Affinity `.af`/`.afdesign`/`.afpub` (recognized by its content) |
| Saves and exports | `.vectorcraft`, `.vctemplate`, SVG, SVGZ, PDF, PDF-compatible `.ai`, EPS, DXF, EMF, WMF, PNG, PNG-8, JPEG, GIF, WebP (lossless), TIFF, BMP, Targa, PSD (each layer a pixel layer, or one flat image) and `.txt` |

- A PDF opens as editable art, not as a picture.
- The `.ai` we write is a PDF by default: any PDF reader opens it, and Local Vector reopens it
  exactly because the native document travels inside. A `.ai` or PDF written by another editor opens
  as art, and the editing data it carries (layers, groups, names, artboards) is read as far as this
  app supports it; the warnings name what could not be read.
- `.ait` templates are written only by Save as Template, never by an export.
- File ▸ Open also accepts the other file names of PostScript, PNG, BMP and PSD files (`.epsf`,
  `.pns`, `.dib`, `.rle`, `.pdd`).
- An export can take one artboard, a layer, an asset or the selection, one or more scales or PPIs,
  and a preset.

## Where it comes from

Local Vector is a fork of [VectorCraft](https://github.com/storytold/vectorcraft) by
[ArtCraft](https://getartcraft.com), licensed MIT or Apache-2.0. The vector engine is their work: the
document model, the path booleans, the CPU renderer, the text stack and the core file readers and
writers. The tree started at their commit `683abdb2d` (2 November 2026).

**Why a fork.** We wanted a set of desktop apps that look and behave the same, update and sign the
same way, and follow our own priorities: Windows and Linux first, files that stay local, agents able
to drive every app, and a tree we can steer without asking the upstream to change direction.

We also wanted "written in Rust" to hold for everything the app runs, not only the engine. The
upstream tree takes its updater TLS from `ring`, which is partly C and assembly, and `aws-lc` is in
its dependency tree. Here the updater and every other network path use pure-Rust cryptography
(RustCrypto), and neither library is in the program you install.

**What changed here:**

- Name, icon and app identifiers.
- The shell shared by all Local apps: Help ▸ Search Commands (`Ctrl+Shift+/`, `Cmd+Shift+/` on
  macOS), Automatic/Light/Dark themes, and the update card.
- Our own updater and release pipeline.
- TLS moved to the pure-Rust provider (RustCrypto).
- Live automation of the open app goes through `craft-mcp`, shared by every Local app.

**What we still take from upstream:** fixes that matter to us (crashes, data loss, security, file
compatibility), reviewed one by one and adapted; new features and UI changes are not merged
automatically. Each sync is recorded in [docs/UPSTREAM-SYNC.md](docs/UPSTREAM-SYNC.md).

**How behavior is decided.** Formats are written from published specifications (PDF, PostScript,
SVG, DXF, TIFF, PSD) and from files users send us. We never run, automate or measure a commercial
editor to copy what it does.

## Download

Windows 10/11 (x64): `.msi` installer or portable `.zip`. Linux (x86_64): `.AppImage`, `.deb`,
`.rpm`, `.tar.gz` or an Arch package. All in
[Releases](https://github.com/Anon5T4R/Local-Vector/releases/latest). The scripts in
[`packaging/`](packaging) also build Windows x86 and ARM64, macOS and FreeBSD packages, which the
release workflow does not publish.

Releases carry a `SHA256SUMS.txt`, and the updater refuses a file that neither GitHub's per-file
digest nor that list vouches for. The release workflow also signs each file with
[minisign](https://jedisct1.github.io/minisign/), the same key as every Local app; when the signing
secret is missing the release ships unsigned and the run says so. The updater checks the signature
and, for now, only logs a failure. To check a file yourself:

```sh
minisign -Vm <file> -P RWTqN+hTTCFbDCGCA1cEJRLp2FSluxIUiJfqdoAYwkwNNVH5MTYH8qSN
```

The public key is also in [packaging/minisign.pub](packaging/minisign.pub).

## Network and privacy

- Your documents never leave the computer.
- At launch, in the background, the app asks GitHub whether a newer release exists; Help ▸ Check for
  Updates runs one by hand. There is no periodic check, and no setting to turn the check off yet
  (short of a launcher-managed install).
- No account, analytics or crash reporting.

Security notes, all checkable in this repository:

- No C or C++ image, font or PDF library reads the files you open: the readers, the renderer, the
  text stack and the exporters are Rust.
- The update's TLS is rustls with the pure-Rust RustCrypto provider; the binaries link no `ring`,
  `aws-lc` or OpenSSL.
- The engine crates contain no `unsafe`. In the desktop shell it appears only where the app calls
  the system (font lists, color spaces, stylus input, menus, clipboard, file dialogs).
- Releases are built by GitHub Actions from a tagged commit with locked dependencies. Every
  dependency passes `cargo-deny` (advisories, licenses, bans, sources), and none comes from a git URL.
- Documents are saved to a temporary file and then renamed over the target.

## Build from source

Rust 1.95 or newer.

```sh
git clone https://github.com/Anon5T4R/Local-Vector
cd Local-Vector
cargo run --release -p localvector   # the app
cargo xtask ci                       # fmt, clippy, tests and the app's gates
```

The Japanese and CJK fonts are vendored under `assets/craft-fonts/`, so the official builds need no
network. A build without `CRAFT_FONTS_DIR` compiles and uses the system's fonts.

## How it is made

Local Vector follows the same rules as every Local app: local files and no account, Rust only
(including the UI and the cryptography), one self-contained repository per app, forks with selective
sync, governed dependencies, signed releases built from a tag, and MCP so agents can use the app.
Most of the code is written by AI coding agents (Claude Code and OpenCode) from written plans,
reviewed by the maintainer and held to the same checks and CI. The reasons for each decision:
[How we make the Local apps](https://github.com/Styx-Studios/.github/blob/main/profile/README.md#how-we-make-the-local-apps).

## AI and agents

There is no AI in Local Vector: no chat, no provider, no key. What it has is the other direction:
every command in the app is reachable by an agent, so an assistant you run yourself (any MCP client,
or the CLI) can draw, edit and export through `localvector-cli mcp`
([docs/mcp.md](docs/mcp.md)). Nothing is sent anywhere unless your own client sends it.

## License and credits

MIT or Apache-2.0, at your option ([LICENSE-MIT](LICENSE-MIT), [LICENSE-APACHE](LICENSE-APACHE)),
third-party notices in [NOTICE](NOTICE), per-asset credits in [ASSETS.md](ASSETS.md) and
[ATTRIBUTION.md](ATTRIBUTION.md).
The upstream authors' credit is in NOTICE and in the upstream copyright line of LICENSE-MIT. The app
icon is original artwork made for the Local suite; its files are in
[`assets/app-icon/`](assets/app-icon/README.md). Release builds embed the fonts of
[craft-fonts](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md) (SIL Open Font
License 1.1). Other Local apps: [the full list](https://github.com/Styx-Studios).
