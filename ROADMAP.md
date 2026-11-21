# Local Vector Roadmap

**Stage: alpha** · os seis fluxos centrais do Illustrator (desenho e edição de path, pintura e
estilo, tipo, camadas/artboards/export, impressão, troca de arquivos) funcionam ponta a ponta com
salvamento e reabertura; só a troca de arquivos é parcial.

Fork independente do vectorcraft (ArtCraft) na suíte Local do João — clean-room: comportamento
vem só de documentação pública, nunca de rodar ou medir o Illustrator. Esta é a página de estado;
a arquitetura está em `docs/architecture.md` e o histórico mora no git e nas release notes.

## Números (medição de 2026-10-10)

| | Valor |
|---|---|
| Pronto para trabalho real | ~62% |
| Profundidade de features | ~75% |
| Praticante essencial | ~75% |

Beta pede ~75% pronto e a troca `.ai` confiável nos dois sentidos.

## Por que ainda alpha

Os fluxos existem, mas um profissional ainda não troca: arquivos `.ai` reais abrem com estrutura
de camadas errada em casos reais e não salvam de volta com os dados de edição do Illustrator; os
detalhes de interação nunca foram conferidos lado a lado; e algumas máquinas não conseguem
iniciar o app.

## Fila (o que falta, em ordem)

1. **Passada de fidelidade de interação**, ferramenta por ferramenta: 60–90 h.
2. **`.ai` de usuários reais** (estrutura de camadas, guias, velocidade): 10–20 h.
3. **Inicialização e estabilidade em máquinas reais**: 20–35 h.
4. **Correção das ferramentas centrais** (regiões do Shape Builder, bordas do Asset Export): 8–15 h.
5. **Efeitos de raster**: miniaturas dos filtros da Effect Gallery e área de prévia no diálogo.
6. **Type:** tab leaders, verificação ortográfica e tipo vertical pendentes; Hebrew/Arabic
   bidirecional, direção de parágrafo e compositores por linha/objeto feitos.
7. **3D:** Revolve/Extrude/Inflate/Rotate com luzes feitos; materiais e sombras 3D, SVG Filters e
   os efeitos de raster restantes pendentes.
8. **Brushes e symbols** em progresso (bibliotecas feitas); **graphs:** designs de coluna pendentes.
9. **Automação:** Actions gravam/tocam e persistem; variables, scripting e batch restam.
10. **Embalagem/QA amplo** (acessibilidade, RPM em distro real, corpus de arquivos grandes,
    dirty-region rendering e o pico do backend GPU).
11. **Formatos:** DWG (via DXF), placement de PSD como camadas, leitura de overprint de PDF e
    export Affinity.

## Idiomas

Interface: catálogos em `crates/ui-egui/src/i18n/` — completos: en, es, fr, pt-br, ja, ko, vi e
mais (de, 98%); parciais: zh-hans (85%), zh-hant (80%); nenhum: hi, ar (sem interface RTL), id.
Mensagens da engine em inglês. Somente o georgiano teve revisão nativa.

## Suíte

Desde 2026-11 o app segue o padrão craft (guardas do kittest, diálogos na largura mínima,
contraste AA no accent, craft-palette/update/icons); o que muda na casca vem do
`C:\Dev\docs\planos\padrao-craft.md` (repo dev-notes).
