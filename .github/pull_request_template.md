<!--
Thanks for contributing to Local Vector.

Base the PR on `main`, the only integration branch. Delete the sections that do not apply.
-->

## What and why

<!-- What behaviour is different afterwards, and why. Link the issue: `Fixes #123`. -->

## How you verified it

<!-- The commands you ran and what they said. Per crate is enough and preferred:
     `cargo test -p <crate>`. CI runs the workspace on every PR. -->

- [ ] `cargo clippy --locked --all-targets -- -D warnings` on the crates I changed
- [ ] tests pass on the crates I changed
- [ ] `cargo xtask brands`, `cargo xtask assets`, `cargo xtask layers` (and `cargo xtask cursors`
      if I added a cursor)
- [ ] formatting: the CI fmt step covers the same files you did (nightly `rustfmt`, vendored
      `libs/` and `assets/craft-fonts/` excluded)

**Tests.** New behaviour needs a test at the right scale: unit, crate integration or end to end.
For a bug fix, confirm the test fails before the fix.

## Never-crash

Every file, command, MCP call, preferences file and key is untrusted input (`AGENTS.md`).

- [ ] No new `unwrap()`/`expect()`/`panic!`/`unreachable!`/`todo!`/`unimplemented!` outside tests
- [ ] No indexing or slicing with a position that comes from input (`get`, strings only at a
      character boundary); checked or saturating arithmetic; input-sized allocations bounded;
      recursion bounded
- [ ] No new `unsafe`
- [ ] Every crash fixed comes with a regression test

## Provenance

- [ ] Clean-room: nothing copied from Adobe products or GPL/AGPL code; behaviour from public
      documentation only
- [ ] No asset copied from a proprietary product; every new asset has a row in `ASSETS.md`
- [ ] No font file committed here (they live in `assets/craft-fonts` and in `assets/fonts`)
- [ ] No new dependency without `Cargo.lock` and its entry in the `local-deps` inventory
- [ ] No `git+` dependency (CI checks it)

## Documentation

- [ ] Docs updated if documented behaviour changed (`docs/`, `README.md`, the crate's README)

## What the reviewer should look at first

<!-- The decision you are least sure of, a deliberate constraint you want pinned, or a behaviour
     change that existing users will notice. -->
