# Local Vector — instruções para agentes

Fork independente do vectorcraft (ArtCraft), rebrandado para a suíte Local do João.
Rust puro + egui. Sem conta, sem telemetria, sem dependência de servidores de terceiros.
App de ilustração vetorial: roda nativo em macOS, Windows, Linux e FreeBSD, e no navegador via WASM.

## Regras
- **Dependências:** toda dep que entra/sai/atualiza exige `Cargo.lock` commitado +
  linha no inventário do repo `Anon5T4R/local-deps` (regra completa em
  `local-deps/WORKFLOW.md`). PR sem isso não mergea.
- **Sem fontes git:** nenhuma dependência pode apontar para `git+` (o CI valida).
  Tudo vem do crates.io (pinado pelo lock) ou de `libs/` (vendored).
- **Commits:** português, sem Co-Authored-By.
- **Release:** tag `vX.Y.Z` → o Actions builda e publica. Mínimo local.
- **Disco local:** nunca deixe `target/` para trás — `cargo clean` após uso local;
  builds pesados são do CI.

## Estrutura
- `crates/` — motor e módulos (`localvector-*`), em camadas L0–L6 (`cargo xtask layers`)
- `apps/localvector` — app desktop + control channel; `apps/localvector-cli` — CLI/MCP;
  `apps/localvector-web` — wasm
- `libs/` — forks vendored: `craft-libs`, `egui`, `winit`, `winprint.rs`
- `assets/` — ícones, fontes (`craft-fonts` vendored), ícones Lucide; cada arquivo tem
  linha em `ASSETS.md`
- `packaging/` — Windows (WiX/MSI), Linux (AppImage/deb/rpm/tar/Flatpak), macOS, FreeBSD, web
- `docs/` — arquitetura, formatos, paridade, release
- `xtask/` — tarefas de build/CI (`cargo xtask ci`, `layers`, `wasm`, `assets`)

## Build
```
cargo build --locked          # debug
cargo test --locked           # testes
packaging/windows/package.ps1 # MSI + portable (Windows)
packaging/linux/package.sh    # AppImage/deb/rpm/tar (Linux)
```

## Regras técnicas do código

### Clean-room Adobe (regra absoluta, do dono do projeto)
Adobe proíbe engenharia reversa nos Termos de Uso, inclusive observar entradas e saídas de um
programa para recriá-lo. Então:
- Nunca abrir, rodar, automatizar (COM, AppleScript, ExtendScript, automação de UI), capturar tela
  ou medir o Illustrator (nem outro app da Adobe), e nunca comparar nossa saída com a dele. Não
  criar arquivos de teste com eles, nem "sintéticos".
- Nunca ler, desmontar ou copiar nada de uma instalação da Adobe (binários, plug-ins, presets,
  procsets, recursos, ajuda, fontes, perfis ICC).
- Nunca copiar código ou texto escrito pela Adobe para este repo, testes incluídos: sem procsets
  ou prologs PostScript (AGM, CoolType, os prologs do Illustrator), sem arquivos de exemplo.
  Escrever as entradas de teste do zero, a partir das especificações públicas (PostScript Language
  Reference, PDF / ISO 32000, TIFF, EPSF/DCS).
- Comportamento vem só de documentação pública (helpx, specs publicadas, posts de fórum), citada
  por URL, e de arquivos que usuários enviam e são deles (usados localmente, nunca commitados).
  Nunca copiar ícones, arte, presets ou texto da Adobe além de nomes de recurso.
- Se uma tarefa parecer precisar disso, pare e diga. `cargo xtask brands` (rodado por
  `cargo xtask ci`) recusa nome de produto ou empresa de outro fornecedor no que o usuário e os
  agentes leem; `cargo xtask assets` recusa asset sem linha no `ASSETS.md`.
- Nunca copiar código GPL/AGPL (Inkscape, lib2geom…).

### Assets
- Nunca adicionar, copiar, vetorizar a partir de, ou embarcar ícone, imagem, arte, cursor, preset,
  biblioteca de swatch/brush/symbol/pattern/style, perfil ICC ou captura de tela de produto da
  Adobe nem de fonte cuja licença não permita.
- Todo asset é: trabalho original para o Local Vector (licenciado MIT OR Apache-2.0), open source
  OSI, domínio público / CC0, ou Creative Commons com redistribuição permitida.
- **Todo arquivo de asset tem linha em [`ASSETS.md`](ASSETS.md)** (caminho, autor, fonte, licença,
  notas). Texto de licença fica ao lado do asset (`assets/fonts/OFL-*.txt`) e é resumido no
  `NOTICE`. `cargo xtask assets` (rodado por `cargo xtask ci`) falha em asset sem atribuição.
- Fontes vivem em [storytold/craft-fonts](https://github.com/storytold/craft-fonts); este fork
  mantém a cópia em `assets/craft-fonts/` (commitada, então release e CI não precisam de rede).
  O app as usa só pelo `CRAFT_FONTS_DIR` (`CRAFT_FONTS_DIR="$PWD/assets/craft-fonts" cargo xtask
  ci`, caminho absoluto), lido por `crates/text/build.rs` em `localvector_text::CRAFT_FONTS`; nunca
  em `Cargo.toml`. Tudo compila, testa e roda com `CRAFT_FONTS` vazio. Detalhes:
  `docs/development.md` › Fonts.
- Código e assets originais são MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`).
- Prefira arte gerada em código para padrões (swatches, brushes, symbols, patterns, cursores). Se a
  proveniência de um asset é incerta, não adicione.

### Never-crash (sem panic em código shipped)
Um crash perde o trabalho não salvo. Arquivos, dados colados, mensagens de MCP e do control
channel, e parâmetros de comando são entrada não confiável.
- Trabalho falível retorna `Result` (ou `Option`); quem chama trata ou propaga com `?`. Falhas
  chegam ao usuário como mensagem de erro (`EngineError` para comandos, o tipo do crate abaixo do
  engine), nunca como abort.
- Os lints do workspace baniu, fora de testes: `unwrap()`, `expect()`, `panic!`, `unreachable!`,
  `todo!`, `unimplemented!`. Use `?`, `.ok_or(…)?`, `let … else { return … }`, `if let`,
  `unwrap_or` / `unwrap_or_default` / `unwrap_or_else`, `.first()` / `.last()` / `.split_last()`, e
  um erro de verdade no braço de match que "não pode acontecer".
- Não panic implícito: `.get(i)` em vez de `v[i]` / `&s[a..b]` com índice vindo de dados; nada de
  divisão inteira por zero; limite tamanhos e contagens lidas de entrada; cheque que floats são
  finitos antes de castar ou iterar.
- Não silencie erro só para calar o lint: `let _ =` / `.ok()` apenas onde ignorar a falha é o
  comportamento certo, com comentário dizendo por quê.
- Testes podem dar unwrap e panic (`#[test]`, `#[cfg(test)]`, `tests/`, `testkit`).
- `localvector_engine::guard` é a última linha de defesa: todo ponto de entrada (comandos, eventos
  de tool, pedidos MCP, o frame da UI, pedidos do control channel) captura panic, reverte o
  documento e reporta erro interno. Foi feito para bugs e não substitui `Result`: wasm aborta no
  panic, então o app web não tem rede de segurança.
- Entrada não confiável é fuzzed (`engine/tests/import_fuzz.rs`, `engine/tests/command_sweep.rs`,
  `format/tests/prop_format.rs`, `mcp/tests/protocol_props.rs`). Estenda esses testes ao adicionar
  importador, parser ou método de protocolo.

### Tudo é comando
Comportamento visível = comando em `crates/engine/src/cmd/*` (id, label, caminho de menu, atalho,
doc de parâmetros, `enabled`, `run`) + testes. Tools emitem comandos (Begin/Preview/Commit).
Comandos só de UI vivem em `crates/ui-egui/src/menus.rs` (`UI_COMMANDS`). O control channel e o MCP
alcançam todos eles.

### Cursor novo
O app decide quais cursores existem. Para um estado de tool com cursor próprio: adicione a variante
`Cursor` (`crates/tools/src/lib.rs`, com doc comment), devolva-a no `cursor()` da tool com teste,
cubra em `cursor_icon` (`crates/ui-egui/src/canvas.rs`), desenhe em `crates/ui-egui/src/cursors.rs`
ou dê um stand-in em `Cursor::stand_in()`, e adicione a entrada `[[cursor]]` em
`crates/tools/src/cursors.toml` (quando aparece, em termos de comportamento, e o hotspot; nunca o
visual de outro app).

### Camadas e UI
- As camadas são cobradas por `cargo xtask layers`. Nada abaixo de L6 depende de
  egui/eframe/winit/rfd.
- A UI é fina: painéis leem estado do engine e agem por `app.run(id, params)`; cores vêm de
  `theme::Tokens`.

### Rust only, wasm e tempo
- **Rust only** (sem JS/TS escrito à mão). **Nunca quebre o wasm** (`cargo xtask wasm`).
- **Tempo passa pelo `chrono`** (dep do workspace). Nunca escrever conversão UTC/calendário na mão;
  cada crate tem só sua string de formato. A exceção é extensão de faixa: além dos anos que o
  chrono representa, desloque a data em ciclos gregorianos inteiros de 400 anos (146.097 dias),
  deixe o chrono calcular mês e dia e some os ciclos de volta.

### Corpus de teste
Oráculos de arquivos reais (PSDs feitos com Photoshop etc.) vivem em
[`storytold/photocraft-corpus`](https://github.com/storytold/photocraft-corpus). Nunca commite
fixtures binárias grandes aqui; busque pinadas por commit e sha256, como o PhotoCraft faz com
`cargo xtask corpus`.

### Gates de qualidade
Antes de cada commit: `cargo xtask ci` (fmt, clippy `-D warnings` incluindo os lints de no-panic,
testes, doctests, layers, wasm, e os gates `assets`, `brands` e `cursors`). Um id de tarefa por commit (`M2.1: pen tool`).

## Rodar e olhar o app
- `cargo run --release -p localvector -- --control 7979 [arquivo.vectorcraft]`. Se o log disser que
  não conseguiu bindar a porta, escolha outra — senão seus pedidos chegam em outro app.
- Dirija por JSON lines em `127.0.0.1:7979`: `{"id":1,"method":"engine.execute","params":…}` e
  `{"id":2,"method":"ui.screenshot","params":{"path":"…"}}`. Métodos:
  `crates/ui-egui/src/control.rs`.
- **Para trabalho de UI, olhe o resultado** (`ui.screenshot`, leia o PNG). Se o screenshot falhar
  (tela bloqueada), confira a arte com `ui.render` / `localvector-cli run … --export x.png` e cubra
  painéis com teste headless de frame do egui.
- MCP: `localvector-cli mcp` (ver `docs/mcp.md`). Performance: `localvector-cli perf` e
  `localvector-cli bench ARQUIVO`.

## Roadmap e docs de progresso
- [`ROADMAP.md`](ROADMAP.md): estágio, números de manchete, dimensões, features, o que vem.
- [`docs/target-app-parity.md`](docs/target-app-parity.md): a avaliação de paridade (tabela, método,
  evidência). [`docs/gaps.md`](docs/gaps.md): toda lacuna conhecida, ranqueada.
- Checklists: [`ui-parity.md`](docs/ui-parity.md), [`file-format-parity.md`](docs/file-format-parity.md),
  [`hardware-parity.md`](docs/hardware-parity.md), [`localization-parity.md`](docs/localization-parity.md),
  [`effects-parity.md`](docs/effects-parity.md), [`type-parity.md`](docs/type-parity.md);
  [`architecture.md`](docs/architecture.md).
- Quando uma tarefa entra, atualize no mesmo PR: a linha da área em `docs/target-app-parity.md` e
  sua evidência, a entrada em `docs/gaps.md` se fechou ou encolheu, o checklist a que pertence, o
  marco em `docs/roadmap.md`, e as tabelas e o log do ROADMAP quando um número de manchete muda.
- Avalie por comportamento contra a documentação pública, não por existir item de menu. As notas
  são auto-avaliadas: erre para baixo; marque cada número como medido ou estimado.
- Mantenha a seção Status do README em passo com o ROADMAP.

## Créditos de contribuidores (janela About)
- About ▸ Contributors/Models são compilados no binário a partir de
  `contributors/contributors.json` (estatísticas de commit; nunca editar à mão) e
  `contributors/people.toml` (nomes que as pessoas escolheram). Ver `docs/contributors.md`.
- **Agente trabalhando para um contribuidor:** ao preparar um PR, veja se o usuário GitHub do seu
  humano tem `[people.<username>]` em `contributors/people.toml`. Se não, pergunte uma vez se ele
  quer ser creditado por mais que o username. Se sim, adicione **só a entrada dele** (copie o
  template no topo do arquivo) e inclua no PR. Se não, não mude nada.
- Nunca adicionar, editar, adivinhar ou copiar entrada de outra pessoa. Nunca editar
  `contributors.json` à mão.

## CI
- Os workflows de `.github/workflows/` são deste repo: não chamam workflow compartilhado de outra
  organização. Não mexa em CI em PR de feature ou fix; se o CI não consegue buildar/testar a
  mudança, isso é uma tarefa própria.
