# Sincronização com o upstream

Porta/rebrand da suíte Local: o main é nosso, o upstream é fonte de mudanças candidatas e nada entra sem revisão. Regras gerais em `docs/planos/licoes-de-porta.md` (dev-notes).

## Base

- Upstream: `storytold/vectorcraft`, commit **683abdb2d** (2026-11-02, merge do #5080). A árvore
  partiu desse commit.
- Prova (conferida em 2026-11-21 no clone raso de `C:\Dev\storytold\vectorcraft`): 5 dos 7 arquivos
  testáveis de `683abdb2d^..683abdb2d` batem com o upstream; os dois restantes são `Cargo.toml` e
  `Cargo.lock`, que são nossos.
- Fork: `Anon5T4R/Local-Vector`, versão própria na linha `0.22.x`.
- Nada é lido de `C:\Dev\storytold` em build, teste ou release; a pasta é só referência local para a
  skill `sync-upstream`.

## craft-update vendored: fork permanente da suíte (fora do sync)

`libs/craft-libs/crates/update` está **fora do escopo** deste sync com o upstream. É um crate nosso, hospedado no `craft-libs` da suíte Local, e um **fork permanente consciente** do `craft-update` upstream — unificado nos 10 apps em 2026-11-13, com um diff de milhares de linhas contra `storytold/craft-libs/crates/update` (MSI context/migrate, pins do launcher, `offers_update`, `desktop_register`, catálogo único de apps). Nenhum diff de sync encaixa nele.

Bugfix e feature vão **direto na fonte única** — repo `Anon5T4R/local-libs` (crate `crates/update`) — e daí são replicados para os apps pelo script de sincronização (`sync.ps1 -Mode push`, guarda de hash ×10; a cópia vendored aqui é espelho). Nunca edite a cópia local diretamente: o próximo sync sobrescreve. Contribuições ao upstream só via **Issue/PR manual**, em inglês, quando o problema também existir lá - mesma política do `UPSTREAM-CONTRIBUTIONS.md` do local-office. O restante do `craft-libs` segue o fluxo normal deste documento.
