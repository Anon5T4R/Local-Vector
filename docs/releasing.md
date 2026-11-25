# Releasing Local Vector

Every tag matching `v*` runs `.github/workflows/release.yml`. The workflow builds Windows x64 and
Linux x86_64 packages, turns the Linux tarball into an Arch package, and the `publish` job attaches
everything to the **GitHub Release** of the tag with generated notes. The pipeline is ours: it calls
no shared workflow of another organisation.

What the release contains:

| Job | Runner | Artifacts |
|---|---|---|
| `windows` | `windows-latest` | `localvector-<v>-windows-x64.msi`, `localvector-<v>-windows-x64-portable.zip` |
| `linux` | `ubuntu-22.04` | `localvector-<v>-linux-x86_64.{AppImage,deb,rpm,tar.gz}`, `localvector-cli-<v>-linux-x86_64.tar.gz` |
| `arch` | `ubuntu-latest`, in the `archlinux:base` container | `localvector-<v>-linux-x86_64.pkg.tar.zst` |
| `publish` | `ubuntu-latest` | `SHA256SUMS.txt`, plus a `<asset>.minisig` beside each file of the three jobs when the minisign key is configured |

macOS, FreeBSD, ARM64, 32-bit Windows, Linux aarch64/riscv64 and Flatpak are **not** part of the
release: `packaging/macos/` and `packaging/freebsd/` still hold hand-run packagers, and
`packaging/windows/package.ps1` and `packaging/linux/package.sh` accept other architectures for
local builds, but no workflow job builds them.

User-facing names say **Local Vector**. Files, binaries and ids stay lowercase
(`localvector-<version>-<platform>-<arch>.<ext>`, `com.localvector.app`).

## Cutting a release

1. **Bump the version** on `main`. It lives in exactly one place, `[workspace.package] version` in
   the root `Cargo.toml`; every crate inherits it and the packaging scripts read it from there:

   ```sh
   cargo xtask version                 # prints the current version
   cargo xtask version set X.Y.Z       # or X.Y.Z-rc.1; also refreshes Cargo.lock
   ```

   Commit the change through the normal review flow and merge it.
2. **Tag the commit** and push the tag: `git tag vX.Y.Z && git push origin vX.Y.Z`. Only `v*` tags
   start the workflow, and `publish` runs only when the ref is a `refs/tags/v*` tag.
3. **Wait for the release.** The three build jobs package their platform; `publish` then writes
   `SHA256SUMS.txt`, signs the assets when it can, and creates the GitHub Release titled
   `Local Vector vX.Y.Z` with generated notes. Re-running it against an existing release uploads
   the files again with `--clobber`.
4. **Check it.** Download an installer or two and read the publish job's log. A `::warning::` in
   `publish` means the release was published unsigned.

The workflow creates no draft and sets no pre-release flag: a `-rc.1` tag publishes as an ordinary
GitHub Release. The in-app update check ignores pre-release versions, so it does not offer one.

**Test runs:** *Actions › Release › Run workflow* on a non-tag ref runs the three build jobs without
`publish`, whose `if` only admits `v*` tags. The artifacts are on the run's page.

## Windows

The `windows` job runs `packaging/windows/package.ps1` with no `-Arch`, and its default is `x64`.
The script builds both programs with `-C target-feature=+crt-static`, so neither package needs the
Visual C++ redistributable, and checks each binary's PE header before packaging: the machine type
must match the target and `localvector.exe` must be a GUI program while `localvector-cli.exe` stays
a console program.

- **MSI.** WiX v5.0.2 (the job installs the tool with its UI and Util extensions pinned to 5.0.2)
  builds a **per-machine** package (`Scope="perMachine"`) and refuses a per-user install through a
  launch condition requiring `ALLUSERS=1`; the wizard is
  `WixUI_Minimal`, and its licence page points at the MIT/Apache texts while its banner and side
  bitmap are drawn from the app icon (no WiX stock art). The MSI's `ProductVersion` is numeric, so a
  pre-release suffix is dropped there while the display version keeps it. A `MajorUpgrade` allows
  same-version upgrades so release candidates replace each other, and refuses a downgrade. The
  package registers the Program Files folder, Start Menu and desktop shortcuts, an App Paths entry
  for `localvector.exe`, its own file types and the formats it opens under "Open with", and
  `HKLM\Software\Anon5T4R\LocalVector\InstallDir` for the updater. The details are in
  [`packaging/windows/INSTALLATION.md`](../packaging/windows/INSTALLATION.md).
- **Portable zip.** Both executables, the documentation and `portable.txt` sit at the archive root,
  so the marker lands beside `localvector.exe` wherever the zip is extracted; the script re-extracts
  the archive and checks those three files. The marker switches settings and user folders to
  `LocalVectorData\` next to the executable instead of `%APPDATA%\LocalVector`.
- **Signing.** `sign.ps1` is called on both executables and then on the MSI, but the job passes it no
  signing variables; see [Signing](#signing).
- **Smoke test.** The x64 build runs on the x64 runner, so the script ends by running
  `localvector-cli --version`.
- The job turns off Defender's real-time scanning for the build: it writes thousands of `.rlib` and
  `.o` files.

## Linux

The `linux` job runs on `ubuntu-22.04` and calls
`packaging/linux/package.sh --formats "appimage deb rpm tar cli"` after installing the X11, Wayland
and xkbcommon development packages and nfpm 2.48.0.

The build host sets the floor: `ubuntu-22.04` gives glibc 2.35, so the binaries need glibc 2.35 or
newer. Moving the job to a newer image raises that floor, so do it deliberately.

All formats share one staged FHS tree: both binaries (stripped when the toolchain's `strip` is
present), the desktop entry, the MIME XML, the AppStream metainfo, the hicolor icons, the
documentation and the embedded fonts' licences.

- **`.tar.gz`.** The FHS tree plus `install.sh`, which defaults to a per-user install under
  `~/.local` and also takes `--uninstall`.
- **`.deb` and `.rpm`.** Built by nfpm from `packaging/linux/nfpm.yaml`. The `.deb` requires `libc6
  (>= 2.35)`, `libgcc-s1`, `libxkbcommon0` and either `libvulkan1` or `libegl1`. The RPM requires
  64-bit shared-library capabilities instead of package names: `libc.so.6()(64bit)`, libm's
  `libm.so.6(GLIBC_2.35)(64bit)`, `libgcc_s.so.1()(64bit)`, `libxkbcommon.so.0()(64bit)` and either
  `libvulkan.so.1()(64bit)` or `libEGL.so.1()(64bit)`. The X11 and Wayland libraries are loaded at
  runtime and are recommendations; the GPU libraries are hard dependencies, as listed above.
- **AppImage.** Built with a pinned appimagetool 1.9.1 and a pinned type2 runtime, both verified by
  SHA-256. The file embeds the update information
  `gh-releases-zsync|Anon5T4R|Local-Vector|latest|localvector-*-linux-x86_64.AppImage.zsync`
  (`GITHUB_REPOSITORY` overrides the repository for a fork). `package.sh` writes the matching
  `.zsync` only when `zsyncmake` (the `zsync` package) is installed; the job installs no zsync
  package, so whether a release carries a `.zsync` depends on the runner image providing the tool.
- **CLI tarball.** `localvector-cli-<v>-linux-x86_64.tar.gz` holds the stripped CLI and the
  licences, for machines that never open a window.
- **Smoke test.** The script runs the staged `localvector-cli --version`.

## Arch

The `arch` job needs `linux`, has a 15-minute timeout and runs `ubuntu-latest` in an
`archlinux:base` container. It downloads the Linux tarball, reads the version from the root
`Cargo.toml`, copies `packaging/arch/PKGBUILD` and replaces its `pkgver`, `source` and
`sha256sums` with the local tarball and its real SHA-256 (the release URL does not exist yet at that
point). It then installs `fakeroot`, builds with `makepkg` as an unprivileged `builder` user —
`makepkg` refuses to run as root — and renames the result
`ai.storyteller.localvector-<v>-1-x86_64.pkg.tar.zst` to
`localvector-<v>-linux-x86_64.pkg.tar.zst`. The package name stays
`ai.storyteller.localvector` with `provides=('localvector')`; the file name is the suite's asset
contract. It installs the tarball's `bin/` and `share/` trees under `/usr`.

## Fonts and version strings

Both build jobs set `CRAFT_FONTS_DIR=$GITHUB_WORKSPACE/assets/craft-fonts`, so the packages embed
the vendored craft fonts. They do not set `CRAFT_FONTS_REQUIRED`: if that checkout is bad or
missing, `crates/text/build.rs` only warns and the build falls back to the app's own and the
installed fonts. Every package copies each embedded font's `OFL.txt` as `OFL-<family>.txt`.

`localvector --version` and `localvector-cli --version` print only the package version.

## Signing

### Release assets (minisign)

The `publish` job installs `minisign` with apt, writes `MINISIGN_KEY` to a file in the runner's temp
directory and signs every file under the downloaded `windows/`, `linux/` and `arch/` folders with
`MINISIGN_PASSPHRASE`, writing `<asset>.minisig` beside each. After every signature it verifies the
result against `packaging/minisign.pub`, the public half committed to this repository, so a secret
that does not match the key the updater trusts never publishes a useless signature.
`SHA256SUMS.txt` itself is not signed, and because it is written before the signing step it lists no
`.minisig` file.

The release never fails for signing. When `MINISIGN_KEY` or `MINISIGN_PASSPHRASE` is absent, when
apt cannot install minisign, when a signature fails, or when the secret key does not match
`packaging/minisign.pub`, the step prints a `::warning::`, deletes any `.minisig` written so far and
exits successfully: the release is published unsigned.

### Windows (Authenticode)

`packaging/windows/sign.ps1` signs the two executables and the MSI with `signtool` (SHA-256 and an
RFC 3161 timestamp) using the first material present:

1. a `.pfx` certificate: `WINDOWS_CERTIFICATE` (base64) and `WINDOWS_CERTIFICATE_PASSWORD`, or
2. Azure Trusted Signing: `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`,
   `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT` and `AZURE_CERT_PROFILE`.

With neither it warns and leaves the files unsigned. The release workflow passes no signing
variables to the `windows` job, so CI artifacts are unsigned even when those secrets exist in the
repository; a signed Windows build is a local run with the variables set. `WINDOWS_TIMESTAMP_URL`
and `SIGNTOOL` override the timestamp server and the `signtool.exe` path.

## In-app update checks

The app uses `craft-update` (`crates/ui-egui/src/updates_ui.rs`) against `Styx-Studios/Local-Vector`.
The desktop enables one background check at launch; it runs once, sends no GitHub token and stays
quiet when it fails. Help ▸ Check for Updates runs another and reports the failure. Only published,
non-draft, non-pre-release releases whose version is newer than the installed one are offered.

A download is installed only with a published checksum: the feed's or GitHub's per-file digest,
else the release's `SHA256SUMS.txt`; a release with neither is refused before anything is fetched.
The `.minisig` sidecar is checked but advisory: a missing, invalid or unreachable signature is
logged and never blocks the install. Copies owned by a package manager are updated through their
package manager.

## Icons

`assets/app-icon/localvector.svg` is the canonical artwork. `packaging/icons.sh` regenerates the
1024 px PNG, the macOS 512 px render, the hicolor set and the Windows `.ico` (packed by
`cargo xtask ico`); the `.icns` comes from `iconutil` on macOS and is left as committed everywhere
else. The outputs are committed, so building and packaging never need `resvg` or `iconutil`.

## Checks

`cargo xtask ci` is the gate: fmt, clippy with `-D warnings`, tests and doctests, the layer check,
the asset, brand and cursor gates, `cargo-deny`, the `libs/` mirror check, and the remaining
CI-only steps (the bundled-font-free builds, the PDF dependency check and the i18n report).
`package.ps1` runs `packaging/windows/check-icons.ps1` before `wix build`. The packaging scripts
carry their own checks; run them by hand on a change to `packaging/`.
