# Windows installation

Local Vector installs for all users, into `Program Files\Local Vector` (`Program Files (x86)` for
the 32-bit build). The package is `Scope="perMachine"`: it needs an elevated setup and refuses a
per-user install, because its registrations (app paths, file types, the shortcuts' key paths) live
under `HKLM`. An older per-user copy can still be uninstalled: the launch condition allows
`Installed AND REMOVE="ALL"`.

Silent installs pass `ALLUSERS=1` (`msiexec /i localvector-<version>-windows-<arch>.msi /qn
ALLUSERS=1`).

The package registers:

- `HKLM\Software\Anon5T4R\LocalVector\InstallDir`, which the app's updater reads to tell an MSI
  install (it updates with the next MSI) from a portable copy (it updates with the portable zip);
- App Paths for `localvector.exe`, so Win+R `localvector` works;
- Local Vector as the default app for `.vectorcraft` (and the older `.drawcraft`), and as an
  "Open with" choice for SVG, PDF, `.ai`/`.ait`, EPS, Affinity, DXF, EMF/WMF and the image formats
  the app opens;
- a Start Menu shortcut and a desktop shortcut, both plain links to `localvector.exe` (not
  advertised MSI shortcuts), each in its own component with an `HKLM` key path so Windows
  Installer removes them on uninstall.

`packaging/windows/check-icons.ps1` checks the icon references and their extensions (ICE50) without
building the package; `package.ps1` runs it before `wix build`, and `wix msi validate` is the
slower check to run by hand on a packaging change:

```powershell
./packaging/windows/check-icons.ps1
wix msi validate dist/release/localvector-<version>-windows-x64.msi -ice ICE50
```

Passing these checks does not prove a real installation, elevation, upgrades, registry writes,
shortcut behaviour, cancellation, rollback or uninstall: those still need the real payload in a
clean Windows test environment.
