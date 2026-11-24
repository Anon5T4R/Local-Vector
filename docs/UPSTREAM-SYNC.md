# Upstream sync

The Local suite's port/rebrand: main is ours, the upstream is a source of candidate changes and
nothing enters without review. General rules in `docs/planos/licoes-de-porta.md` (dev-notes).

## Base

- Upstream: `storytold/vectorcraft`, commit **683abdb2d** (2026-11-02, merge of #5080). The tree
  started from that commit.
- Proof (checked 2026-11-21 against the shallow clone at `C:\Dev\storytold\vectorcraft`): 5 of the
  7 testable files of `683abdb2d^..683abdb2d` match the upstream; the two remaining are
  `Cargo.toml` and `Cargo.lock`, which are ours.
- Fork: `Styx-Studios/Local-Vector`, own version on the `0.22.x` line.
- Nothing is read from `C:\Dev\storytold` in build, test or release; the folder is only a local
  reference for the `sync-upstream` skill.

## Vendored craft-update: the suite's permanent fork (outside the sync)

`libs/craft-libs/crates/update` is **out of scope** for this upstream sync. It is our crate, hosted
in the Local suite's `craft-libs`, and a **conscious permanent fork** of the upstream
`craft-update` — unified across the 10 apps on 2026-11-13, with a diff of thousands of lines
against `storytold/craft-libs/crates/update` (MSI context/migrate, launcher pins, `offers_update`,
`desktop_register`, single app catalog). No sync diff fits it.

Bugfix and feature go **straight into the single source** — repo `Styx-Studios/local-libs` (crate
`crates/update`) — and from there are replicated to the apps by the sync script (`sync.ps1 -Mode
push`, ×10 hash guard; the copy vendored here is a mirror). Never edit the local copy directly: the
next sync overwrites it. Upstream contributions only through a **manual Issue/PR**, in English,
when the problem also exists there — the same policy as local-office's `UPSTREAM-CONTRIBUTIONS.md`.
The rest of `craft-libs` follows the normal flow of this document.
