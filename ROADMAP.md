# Local Vector Roadmap

**Stage: alpha** · the six central Illustrator flows (drawing and path editing, painting and style,
type, layers/artboards/export, printing, file exchange) work end to end with save and reopen; only
file exchange is partial.

Independent fork of vectorcraft (ArtCraft) in João's Local suite — clean-room: behavior comes only
from public documentation, never from running or measuring Illustrator. This is the state page;
the architecture is in `docs/architecture.md` and the history lives in git and the release notes.

## Numbers (measured 2026-10-10)

| | Value |
|---|---|
| Ready for real work | ~62% |
| Feature depth | ~75% |
| Essential practitioner | ~75% |

Beta asks for ~75% ready and reliable two-way `.ai` exchange.

## Why still alpha

The flows exist, but a professional still cannot switch: real `.ai` files open with the wrong layer
structure in real cases and do not save back with Illustrator's editing data; the interaction
details have never been checked side by side; and some machines cannot start the app.

## Queue (what is missing, in order)

1. **Interaction fidelity pass**, tool by tool: 60–90 h.
2. **`.ai` from real users** (layer structure, guides, speed): 10–20 h.
3. **Startup and stability on real machines**: 20–35 h.
4. **Fixes of the central tools** (Shape Builder regions, Asset Export borders): 8–15 h.
5. **Raster effects**: thumbnails of the Effect Gallery filters and a preview area in the dialog.
6. **Type:** tab leaders and spell-checking pending; vertical type, Hebrew/Arabic bidirectional,
   paragraph direction and per-line/per-object compositors done.
7. **3D:** Revolve/Extrude/Inflate/Rotate with lights done; 3D materials and shadows, SVG Filters
   and the remaining raster effects pending.
8. **Brushes and symbols** in progress (libraries done); **graphs:** column designs pending.
9. **Automation:** Actions record/play and persist; variables, scripting and batch remain.
10. **Packaging/broad QA** (accessibility, RPM on a real distro, a corpus of large files,
    dirty-region rendering and the GPU backend's peak).
11. **Formats:** DWG (via DXF), PSD placement as layers, reading PDF overprint and Affinity export.

## Languages

Interface: catalogs in `crates/ui-egui/src/i18n/` - every registered catalog (en, es, fr, pt-br, ja,
ko, vi, id, de, zh-hans, zh-hant and the rest) is marked complete in the registry; coverage is not
measured (`cargo xtask i18n-report` measures it). Missing: hi, ar (no RTL interface). Engine messages
in English. Only Georgian has had native review.

## Suite

Since 2026-11 the app follows the craft standard (kittest guards, dialogs at the minimum width, AA
contrast on the accent, craft-palette/update/icons); what changes in the shell comes from
`C:\Dev\docs\planos\padrao-craft.md` (dev-notes repo).
