# Security Policy

Local Vector processes complex document and image formats and exposes a CLI, a JSON control channel and an MCP server. Treat files, paths, metadata, automation requests and command parameters as potentially malicious.

## Reporting a vulnerability

Please give the repository maintainers a reasonable opportunity to investigate and coordinate a fix before publishing exploitable details, proof-of-concept code or weaponized files.

This repository does not document a dedicated private security address or reporting channel. Contact the repository maintainers through the owner/maintainer channels already listed in the project, provide only a minimal non-exploitable summary in public, and ask for an appropriate private transfer method before sharing sensitive details. Do not open a public issue containing a working exploit, a secret, a sensitive crash dump or a malicious attachment.

If no private channel can be established, withhold weaponized material and report the smallest safe description that allows maintainers to make contact.

## What to include

A useful report includes:

- affected Local Vector version, commit or release artifact;
- operating system, architecture, build profile and relevant feature flags;
- the affected component or path;
- security impact and required attacker access;
- exact reproduction steps;
- expected behaviour and actual behaviour;
- crash logs, backtraces, sanitizer output or resource measurements when applicable;
- whether the problem reproduces from a clean checkout;
- any known workaround or containment.

Remove credentials, personal paths, private document content and unrelated system information from logs.

## Malicious test files

Do not attach a potentially harmful or non-redistributable file to a public issue. Initially provide its format, size, cryptographic hash, observed effect, provenance category (for example synthetic or fuzz-generated) and the command needed to reproduce the problem. Coordinate a private transfer method with the maintainers before sending the sample.

Test files accepted into the repository must be minimized, free of personal or proprietary data, legally redistributable and documented with the expected failure behaviour. A fixed parser defect should get a permanent regression test whenever safe and practical. Files that came from Adobe or another vendor's product, and anything made with one, are never accepted: see the clean-room rules in [`AGENTS.md`](AGENTS.md).

## Scope

Security reports may cover:

- SVG/SVGZ, PDF, PDF-compatible `.ai`, EPS and PostScript, Affinity, DXF, EMF/WMF, PSD/PSB (read as a merged image), raster formats, `.vectorcraft` (and the legacy `.drawcraft`), colour profiles and embedded images;
- decompression bombs, oversized dimensions, integer overflow, memory exhaustion, recursion or hangs;
- import/export and filesystem path handling, including traversal, symlinks and unsafe overwrite behaviour;
- engine command validation and serialization/deserialization;
- the desktop control server, the headless JSON server, the MCP server and the MCP-to-GUI bridge;
- authentication, authorization/capabilities, request limits, batch limits and connection exhaustion;
- GPU resource validation or driver-facing input handling;
- dependencies, CI, packaging, signing, checksums and release artifact integrity.

General bugs without a security impact should use the project's normal issue and contribution process.

## Current limitations

The control channel requires a bearer token before method dispatch and enforces request-line, active-connection and batch-step limits. Authentication does not provide per-client or per-tool capabilities, so an authenticated client still receives the exposed control surface. Remote filesystem methods use separately configured read and write roots (`--automation-read-root`, `--automation-write-root`), and engine commands that still use ambient filesystem paths are denied at the automation boundary. JSON-depth, render, document-memory, command-duration, client-identity, encryption and audit controls are not yet implemented. The protocols are described in [`docs/control-protocol.md`](docs/control-protocol.md) and [`docs/mcp.md`](docs/mcp.md).
