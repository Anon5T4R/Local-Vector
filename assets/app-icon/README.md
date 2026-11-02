# LocalVector app icon

<img src="localvector-small.svg" alt="LocalVector app icon: a white bezier curve with two solid anchor nodes and hollow handle tips, on a teal gradient tile" width="128">

**Figure:** a bezier S-curve with two solid anchor nodes and a handle line off each node — the
pen-tool mark that tells LocalVector apart from the other apps of the suite.

**Style:** flat shapes on a full-bleed teal gradient tile — the same language as the rest of the
Local suite's icons (flat glyph, gradient tile, sheen, the "Taylor" signature at the foot).

**Palette:**

| Colour | Hex | Used for |
|---|---|---|
| Teal | `#2dd4bf` → `#0f766e` | gradient of the tile (top-left to bottom-right) |
| White | `#ffffff` | the curve, the anchor nodes, the handle rings |
| White at 55% | `#ffffff` | the signature "Taylor" at the tile's foot |

**Tile:** `viewBox="0 0 512 512"`, a rounded square inset 32 units with `rx=100`. Windows and
Linux icons use the whole tile; the macOS renders widen the viewBox around it (824/1024 body grid),
as `packaging/icons.sh` does.

**Provenance:** original artwork made for the Local suite (no third-party material). Licence:
[LICENSE.txt](LICENSE.txt) (`MIT OR Apache-2.0`, like the repo).

## Files

| File | What it is |
|---|---|
| `localvector.svg` | the master vector; every PNG, `.ico` and `.icns` is rendered from it |
| `localvector-small.svg` | the same master under the name the READMEs link |
| `localvector-1024.png` | 1024 px render (largest raster) |
| `localvector-macos-512.png` | macOS runtime window/Dock icon (widened viewBox), embedded by `apps/localvector/src/main.rs` |
| `localvector.icns` | macOS icon (16–1024 px, PNG-packed) |
| `localvector.ico` | Windows icon, 16–256 px; embedded in `localvector.exe` by `apps/localvector/build.rs` |
| `hicolor/<n>x<n>/apps/com.localvector.app.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/com.localvector.app.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/localvector/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab,
launcher) and the Wayland app id `com.localvector.app`; `crates/ui-egui/src/brand.rs` paints the
in-app mark (the 128 px hicolor render); `packaging/linux/com.localvector.app.desktop` names the
hicolor icon; the MSI's Start Menu and desktop shortcuts and its Add/Remove Programs entry use
`localvector.ico`.

## Regenerate

Replace `localvector.svg` (and `localvector-small.svg`) with the master, then run
`packaging/icons.sh`. It needs `resvg`; the `.ico` is packed by `cargo xtask ico`. On Windows (no
`iconutil`), the committed `.icns` is packed from PNG renders directly — regenerate it the same way
when the master changes.
