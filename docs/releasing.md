# Releasing LocalVector

Every tag `vX.Y.Z` runs `.github/workflows/release.yml`. The workflow builds the installers for
macOS, Windows, Linux (AppImage, deb, rpm, tarball, Flatpak bundles and a RISC-V tarball), FreeBSD
and the web build, then publishes them in the **GitHub Release** of that tag with generated notes.
The pipeline is ours: it calls no shared workflow of another organisation and needs no secret to
run.

The other packagers — the Flatpak from-source manifest, FreeBSD and the web build — stay in
`packaging/` and are run by hand when needed.

User-facing names say **LocalVector**. Files, binaries and ids stay lowercase
(`localvector-<version>-<platform>-<arch>.<ext>`, `com.localvector.app`).

## In-app update checks

The native desktop app reads the public GitHub latest-release endpoint of
`Anon5T4R/Local-Vector` in a background thread once, at launch, with no periodic checks or automatic
retries; Help ▸ Check for Updates runs one by hand. A check needs no GitHub token and sends no
document or personal data, and a network failure is quiet. Downloads are verified against the
SHA-256 the release asset carries. The web build has no updater (its window says so), and
package-manager installs update through their package manager.

Tags must be semantic versions (`v0.20.0` or `0.20.0`); only published stable versions newer than
the installed workspace version trigger a notice. Prereleases do not trigger it. Flatpak builds also
check for releases; repository-managed installations are updated through their package manager.
Web/headless frontends have no update transport.

## Cutting a release

1. **Bump the version** on `main`. It lives in exactly one place, `[workspace.package] version`
   in the root `Cargo.toml`; every crate inherits it and the packaging scripts read it from there:

   ```sh
   cargo xtask version                 # prints the current version, e.g. 0.20.0
   cargo xtask version set 0.21.0      # or 0.21.0-rc.1; updates Cargo.toml and Cargo.lock
   ```

   Commit the change (`Cargo.toml` + `Cargo.lock`) through the normal review flow, and merge it.
2. **Tag the commit** and push the tag: `git tag v0.21.0 && git push origin v0.21.0`. The workflow
   starts by itself.
3. **Wait for the release.** The platform jobs build and package; the `publish` job then creates the
   GitHub Release for the tag with every artifact attached and `SHA256SUMS.txt`.
4. **Check it.** Download an installer or two and read the job summaries. A `::warning::` there means
   a signing secret was missing and that artifact is unsigned.
5. Publish the release if it was created as a draft. A version with a pre-release suffix
   (`-rc.1`) is marked as a pre-release.

A pre-release tag publishes the same way; mark it as a pre-release in the GitHub UI if it shouldn't
be offered as "latest".

**Test runs:** *Actions › Release › Run workflow* runs the same jobs on a branch, without the
`publish` job. The artifacts are on the run's page.

## What gets built

| Platform | Artifacts | Built on |
|---|---|---|
| macOS 11+ (universal: Apple silicon and Intel) | `localvector-<v>-macos-universal.dmg`, `localvector-cli-<v>-macos-universal.zip` | `macos-15` |
| Windows x64 | `localvector-<v>-windows-x64.msi`, `localvector-<v>-windows-x64-portable.zip` | `windows-latest` |
| Windows x86 (32-bit) | `localvector-<v>-windows-x86.msi`, `localvector-<v>-windows-x86-portable.zip` | `windows-latest` |
| Windows on ARM64 | `localvector-<v>-windows-arm64.msi`, `localvector-<v>-windows-arm64-portable.zip` | `windows-latest` (cross-compiled) |
| Linux x86_64 | `localvector-<v>-linux-x86_64.{AppImage,deb,rpm,tar.gz}` | `ubuntu-22.04` |
| Linux aarch64 | `localvector-<v>-linux-aarch64.{AppImage,deb,rpm,tar.gz}` | `ubuntu-22.04-arm` |
| Linux riscv64 | `localvector-<v>-linux-riscv64.tar.gz` (cross-compiled, glibc >= 2.39; CLI smoke-tested under QEMU) | `ubuntu-24.04` |
| Linux AppImage updates | `localvector-<v>-linux-{x86_64,aarch64}.AppImage.zsync` | with the AppImages |
| Flatpak x86_64, aarch64 | `localvector-<v>-linux-{x86_64,aarch64}.flatpak` | `ubuntu-24.04`, `ubuntu-24.04-arm` (repackages the Linux tarball) |
| FreeBSD 14 x86_64 | `localvector-<v>-freebsd-x86_64.tar.gz` | a FreeBSD 14.3 VM on `ubuntu-latest` |
| Web | `localvector-web-<v>.zip`, a static site (see [`packaging/web/README.md`](../packaging/web/README.md)) | `ubuntu-latest` |

`localvector --version` and `localvector-cli --version` print the version from `Cargo.toml`, the
commit and the build date. The packaging scripts fill `LOCALVECTOR_BUILD_SHA` and
`LOCALVECTOR_BUILD_DATE` when the caller leaves them empty (`packaging/env.sh` for local builds), and
`crates/engine/src/build_info.rs` reads them at compile time.

**Fonts.** The fonts are vendored in this repository: every build job sets
`CRAFT_FONTS_DIR=$GITHUB_WORKSPACE/assets/craft-fonts` and `CRAFT_FONTS_REQUIRED=1`, so releases
embed its Japanese fonts and fail rather than ship without them (the web build embeds only the UI
font, BIZ UDPGothic Regular, to stay small). The packages carry each embedded font's licence as
`OFL-<family>.txt` (`copy_font_licences` in `packaging/env.sh`). A build without `CRAFT_FONTS_DIR`
still compiles and falls back to the system's fonts. The build option is in
[`docs/development.md`](development.md) › Fonts.

### macOS

`packaging/macos/package.sh` builds `aarch64-apple-darwin` and `x86_64-apple-darwin` with
`MACOSX_DEPLOYMENT_TARGET=11.0`, joins them with `lipo` and assembles `LocalVector.app`:

- `Info.plist` is generated from `Info.plist.in` (bundle id `com.localvector.app`,
  `LSMinimumSystemVersion` 11.0, the version and the build commit). The icon is
  `assets/app-icon/localvector.icns`. `cargo xtask bundle` fills in the same template for its
  development app.
- **Document types:** `Info.plist.in` declares every format File › Open reads, so Finder opens
  them with LocalVector (double-click, Open With, a drop on the Dock icon). LocalVector owns its own
  formats and is an alternate for the others. A test keeps the list in step with the engine's
  `OPEN_EXTS`. AppKit hands the files to the app as open-document events, which
  `apps/localvector/src/open_documents.rs` opens like files named on the command line.
- **Signing** goes inside-out with the hardened runtime and a secure timestamp: the executable
  first, then the bundle, with no `--deep` on the final signature. The entitlements
  (`entitlements.plist`) are deliberately empty.
- **Notarization:** the app is zipped and sent with `xcrun notarytool submit --wait`, then the
  ticket is stapled and checked with `stapler validate` and `spctl`. The app goes on a DMG
  (`hdiutil create -srcfolder`, retried when a CI runner's device is busy, with an `Applications` link
  to drag onto), which is signed, notarized and stapled too; the app mounted from it must pass
  `codesign --verify --strict --deep` (#1085). Its Finder window (background, icon size and positions)
  comes from [`packaging/macos/dmg/`](../packaging/macos/dmg/README.md), and its volume is named
  `LocalVector` without the version, which the window's background needs; the DMG file name keeps
  the version.
- **CLI:** the universal `localvector-cli` is signed the same way, zipped, and the zip is
  notarized. A bare executable can't hold a stapled ticket, so Gatekeeper looks the CLI's ticket up
  online the first time a downloaded copy runs.

Locally, without certificates, the script signs ad hoc (`codesign -s -`) and skips notarization,
which is enough to check the bundle and the DMG on your own Mac:

```sh
packaging/macos/package.sh --arch universal   # needs both rustup targets
packaging/macos/package.sh --arch aarch64     # quicker, host only
```

The DMG background and `DS_Store` are committed; `packaging/macos/dmg/generate.py` rebuilds both
from `background.svg` when the icon or the layout changes.

### Windows

`packaging/windows/package.ps1 -Arch x64|x86|arm64` builds with `-C target-feature=+crt-static`,
so neither the MSI nor the portable zip needs the Visual C++ redistributable. The flag is scoped
to the target triple, so host build scripts aren't affected.

- Before packaging, the script checks each binary's PE machine type against `-Arch` (an x64 build
  can never ship labelled arm64), that `localvector.exe` is a GUI program (no console window) and
  that `localvector-cli.exe` is a console program.
- `localvector.wxs` (WiX v5) is a **per-machine** package (`Scope="perMachine"`): it installs into
  `Program Files\LocalVector` (`Program Files (x86)\LocalVector` for the 32-bit build), needs an
  elevated setup, and refuses a per-user install (`<Launch Condition="ALLUSERS = 1 OR (Installed
  AND REMOVE = "ALL")">`, so an older per-user copy can still be uninstalled). It creates a plain
  Start Menu shortcut and a desktop shortcut — not advertised MSI shortcuts, whose descriptor
  Windows can't use as a file handler — each in its own component with an HKLM key path, so
  Windows Installer removes them on uninstall. LocalVector becomes the default app for
  `.vectorcraft` (and the older `.drawcraft`), is listed under "Open with" for SVG, PDF,
  `.ai`/`.ait`, EPS, Affinity, DXF, EMF/WMF and the image formats the app opens, it registers App
  Paths (Win+R `localvector`), and it writes `HKLM\Software\Anon5T4R\LocalVector\InstallDir` so the
  updater knows this is an MSI install and updates with the next MSI rather than the portable zip.
  The MSI version is the numeric `X.Y.Z`, because MSI has no pre-release field. Same-version
  upgrades are allowed so that release candidates replace each other; a downgrade is refused. The
  details are in [`packaging/windows/INSTALLATION.md`](../packaging/windows/INSTALLATION.md).
- Double-clicking the MSI opens the `WixUI_Minimal` wizard: welcome with the licence (the project's
  `LICENSE-MIT`/`LICENSE-APACHE` text), a progress page, and a Finish page that says where to find
  the app, with a ticked "Launch LocalVector" box. The pages name the full version
  (`localvector.en-us.wxl`). `package.ps1` draws the wizard's banner and side bitmaps from the app
  icon, so no WiX stock art ships. `msiexec /qn ALLUSERS=1` installs silently per-machine, as the
  launch condition requires. Building needs the WiX UI and Util extensions:
  `wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2`.
- Shortcut icon identifiers keep the executable's `.exe` extension: MSI uses the identifier as the
  cached icon filename, and an extensionless filename can render as a blank document icon.
  `packaging/windows/check-icons.ps1` checks the references and extensions; `package.ps1` runs it
  before `wix build`.
- **Portable zip:** it ships `packaging/windows/portable.txt` beside `localvector.exe`. That
  marker (or a `LocalVector.portable` file) switches on portable mode: `ui.json` and the folders
  next to it (Swatches, Graphic Styles, Fonts, Data Recovery, logs…) go to `LocalVectorData\` next
  to the exe instead of `%APPDATA%\LocalVector`. The first time `LocalVectorData` is created, the
  settings already in `%APPDATA%\LocalVector` are copied into it (the originals stay), so someone
  updating a portable zip from before the marker keeps their settings. If that folder isn't
  writable the app logs a warning and uses `%APPDATA%`. Deleting `portable.txt` returns settings to
  the user profile (the `LocalVectorData` folder is left as it is). The MSI has no marker.
  `LOCALVECTOR_CONFIG_DIR` still wins, `LOCALVECTOR_NO_PREFS` never looks for the marker, and a
  portable copy skips the one-time read of the old DrawCraft settings. The logic is in
  `apps/localvector/src/profile.rs`. On the other platforms the marker goes where the user sees the
  app, never inside it: on macOS next to `LocalVector.app` (nothing is written inside the signed
  bundle), for an AppImage next to the `.AppImage` file (`$APPIMAGE`; the mounted executable's own
  folder is read-only). Flatpak isn't supported: `/app` is read-only and the sandbox has no folder
  "beside the app", so a Flatpak install always uses its own config folder.
- The ARM64 build is cross-compiled on the x64 runner, which can't run it: install that MSI on a
  Windows 11 ARM64 machine, check that both installed programs are ARM64 and run
  `localvector-cli --version` there.
- **Signing:** `packaging/windows/sign.ps1` signs both `.exe` files and then the `.msi` with
  `signtool` (SHA-256, RFC 3161 timestamp), using whichever material is present:
  1. a `.pfx` certificate (`WINDOWS_CERTIFICATE`, base64, and `WINDOWS_CERTIFICATE_PASSWORD`), or
  2. Azure Trusted Signing (`AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET`,
     `AZURE_SIGNING_ENDPOINT`, `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE`).

  With neither, it warns and leaves the files unsigned, so test builds still produce installers.
  It is the one place to adapt when the signing setup changes; `WINDOWS_TIMESTAMP_URL` and
  `SIGNTOOL` override the timestamp server and the `signtool.exe` path.

Locally on Windows: `dotnet tool install -g wix --version 5.0.2`,
`wix extension add -g WixToolset.UI.wixext/5.0.2 WixToolset.Util.wixext/5.0.2`, then
`pwsh packaging/windows/package.ps1 -Arch x64`.

### Linux

`packaging/linux/package.sh` builds the release binaries and packages them as an AppImage, a
`.deb` and an `.rpm` (with [nfpm](https://nfpm.goreleaser.com), from `nfpm.yaml`) and a plain
`.tar.gz` tree (`bin/`, `share/`). The packages install both programs, the desktop entry, the
AppStream metainfo, the MIME type for LocalVector documents, the icons and the licence files.

**Document types.** The desktop entry's `MimeType` lists every format File › Open reads but PSD,
PSB, `.ps` PostScript, Targa, PCX and JPEG 2000, so file managers list LocalVector under Open With
for them. `packaging/linux/com.localvector.app.mime.xml` declares the types that shared-mime-info
lacks: LocalVector documents and templates, `.ait` templates and Affinity files
(`application/x-affinity`). An `.af` file gets the Affinity type by its content, and `.afdesign`
and `.afpub` files by their extension. The MIME type names of LocalVector's own formats
(`application/x-vectorcraft`, `application/x-vectorcraft-template`, `*.vectorcraft`) are the
format's identifiers and stay as they are. A test keeps both files in step with the engine's
`OPEN_EXTS`.

The jobs run on `ubuntu-22.04`, the oldest GitHub-hosted image, so the binaries only need
glibc 2.35 or newer: Ubuntu 22.04+, Debian 12+, Fedora 36+ and RHEL 10. Moving the job to a newer
image raises that floor, so do it deliberately. After packaging, the job prints the `.deb`'s
metadata and contents, runs `ldd` on the binary and runs each AppImage with `--version`.

**RPM dependencies.** The x86_64 and aarch64 RPMs require 64-bit shared-library
capabilities rather than distribution-specific package names: `libc`, libm's `GLIBC_2.35`,
`libgcc_s`, `libxkbcommon`, and either Vulkan or EGL. Windowing and GPU libraries are loaded at
runtime, so `ldd` alone does not list every requirement. Weak recommendations still use package
names and do not block installation.

**RISC-V.** The `linux-riscv64` job cross-compiles `riscv64gc-unknown-linux-gnu` on `ubuntu-24.04`
against riscv64 libraries from ports.ubuntu.com (so the floor there is glibc 2.39) and runs
`package.sh --formats tar` with `CROSS_ARCH`, `CROSS_TARGET`, `CROSS_COMPILE` and
`EMULATOR=qemu-riscv64` set: a `.tar.gz` only, with `localvector-cli --version` run under QEMU.

**AppImage updates.** Each AppImage embeds the update information
`gh-releases-zsync|Anon5T4R|Local-Vector|latest|localvector-*-linux-<arch>.AppImage.zsync`
(appimagetool `-u`), and `package.sh` writes the matching `.zsync` beside it when `zsyncmake` (the
`zsync` package) is installed. A build of a fork points at its own releases through
`GITHUB_REPOSITORY`. [AppImageUpdate](https://github.com/AppImageCommunity/AppImageUpdate) and
AppImageLauncher then download only the changed blocks from the newest published, non-pre-release
release. The job checks both are there.

**Flatpak.** The `flatpak` jobs (x86_64 on `ubuntu-24.04`, aarch64 on `ubuntu-24.04-arm`) take the
Linux job's tarball and repackage it with `packaging/linux/flatpak-bundle.sh` and
`packaging/linux/flatpak/com.localvector.app.bundle.yml` into a single-file bundle
(`flatpak install --user localvector-<v>-linux-<arch>.flatpak`), then install it and run
`localvector-cli --version` in the sandbox. No Rust build happens there, so the bundle carries the
same binaries as the AppImage. `packaging/linux/flatpak/com.localvector.app.yml` is the
from-source manifest for a Flathub submission (its header says how to build it);
`packaging/check-app-specific.sh` keeps the runtime, the sandbox permissions and the command of the
two manifests identical.

### FreeBSD

GitHub has no FreeBSD runners, so the build runs in a FreeBSD 14.3 VM on `ubuntu-latest` with
`pkg install libxkbcommon wayland libX11 libXcursor libXrandr libXi libxcb mesa-libs vulkan-loader
gtk3 fontconfig freetype2 alsa-lib`. `packaging/freebsd/package.sh` writes a
`/usr/local`-style tree; users install it with
`tar -xzf localvector-*-freebsd-x86_64.tar.gz --strip-components 1 -C /usr/local`.

### Web

The web job builds `apps/localvector-web` with trunk and zips the static site
(`packaging/web/package.sh`). [`packaging/web/README.md`](../packaging/web/README.md) covers
hosting: any path works, `.wasm` must be served as `application/wasm`, and the sample header files
for common hosts.

## Signing

This repository holds **no signing secrets**: with none in the environment the packaging scripts
warn once and publish unsigned artifacts, so a release never fails for a missing certificate. They
still sign when the material is present, so a signed build is a local run with the variables below
set (or a CI run with those secrets configured).

| Variable | Used for |
|---|---|
| `WINDOWS_CERTIFICATE` | base64 `.pfx` code-signing certificate |
| `WINDOWS_CERTIFICATE_PASSWORD` | password for that `.pfx` |
| `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, `AZURE_CLIENT_SECRET` | service principal for Azure Trusted Signing (an alternative to the `.pfx`) |
| `AZURE_SIGNING_ENDPOINT` | e.g. `https://eus.codesigning.azure.net` |
| `AZURE_SIGNING_ACCOUNT`, `AZURE_CERT_PROFILE` | Trusted Signing account and certificate profile names |
| `APPLE_CERTIFICATE` | base64 `.p12` with a "Developer ID Application" identity |
| `APPLE_CERTIFICATE_PASSWORD` | password for that `.p12` |
| `KEYCHAIN_PASSWORD` | password for the temporary keychain (random if unset) |
| `APPLE_ID` | Apple ID used by `notarytool` |
| `APPLE_PASSWORD` | app-specific password for that Apple ID |
| `APPLE_TEAM_ID` | the Developer ID team |

`packaging/macos/import-cert.sh` decodes the `.p12` into a temporary keychain
(`security create-keychain`, then `security import`, then `set-key-partition-list` so codesign can
use the key without a prompt). It exports the identity's SHA-1 as `MACOS_SIGN_IDENTITY`. The
keychain is deleted at the end.

## Icons

`assets/app-icon/localvector.svg` is the canonical icon: a bezier curve with two anchor nodes on a
teal gradient tile, drawn for the Local suite (MIT OR Apache-2.0, see `LICENSE.txt` there; palette
and geometry in `README.md`). `packaging/icons.sh` regenerates the 1024 px PNG, the macOS 512 px
render, the `.icns`, the `.ico` (packed by `cargo xtask ico`) and the hicolor PNGs from it. It needs
`resvg`, plus `iconutil` on macOS; on Windows the committed `.icns` is packed from the PNG renders
directly. The outputs are committed, so packaging never needs those tools.

## Checks

`cargo xtask ci` runs fmt, clippy, build and test, the layering and wasm checks, the no-panic lints
and the asset attribution gate. The packaging scripts carry their own checks, which you run by hand
on a change to `packaging/`: `packaging/windows/check-icons.ps1`,
`packaging/check-app-specific.sh`, and `bash -n` on every script.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-02 | major | Rebranded from VectorCraft to LocalVector: names, ids, MSI package (per-machine, new UpgradeCode), GitHub URLs and the vendored fonts. |
| 2026-10-14 | minor | RPM glibc floor names libm's `GLIBC_2.35`: RHEL 9's glibc 2.34 provides libc's through backports |
| 2026-10-12 | minor | RPM dependencies use 64-bit runtime capabilities and enforce glibc 2.35 (#1275); real distro install/start-up checks remain pending |
| 2026-10-12 | minor | Portable mode (#1366): the Windows portable zip ships `portable.txt`; where the marker goes on macOS and Linux, and what Flatpak can't do |
