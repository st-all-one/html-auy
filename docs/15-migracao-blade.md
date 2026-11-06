# 15 — Migração do Blade (estudo de caso medido)

Este documento registra a migração que motivou o projeto. Os números foram **medidos** no código de
origem, não estimados.

## 1. Superfície medida

| Métrica | Valor |
|---|---|
| views totais | **222** (154 de módulo, 61 de layout, 7 de service) |
| linhas de Blade | 23.612 |
| tags HTML | **6.765** |
| elementos distintos | **63** (≈ 30 cobrem 95%) |
| chamadas `__()` em view | **3.787** |
| chaves de tradução distintas | **2.702** (4 locales) |
| nomes de rota em view | **362** |
| `<x-slot:…>` nomeado | **62** |
| arquivos com `@php` | **130** |
| views com `app()`/container | **~30** |
| `@push`/`@stack` | 2 arquivos |
| `<x-dynamic-component>` | 1 |
| JS de aplicação | **2.804 LOC** em 6 grupos |

## 2. Mapa Blade → html-auy

| Recurso em uso | Ocorrências | Destino | Esforço |
|---|---|---|---|
| marcação + variáveis | 222 views | `html!` com `@if`/`for` | mecânico |
| `__('chave')` | 3.787 / 2.702 chaves | `enum Msg` **gerado** dos 4 catálogos | mecânico se gerado |
| `route('nome')` | 362 nomes | `enum Route` gerado → `LUrl` | mecânico (gerado) |
| `@php` com defaults | 130 arquivos | `Default` nos props + builder | mecânico |
| `app()` / `Blade::render()` | ~30 arquivos | `ShellCtx`/`WidgetCtx` (assembler) | **julgamento** |
| `<x-slot:…>` | 62 | parâmetro do componente | mecânico |
| `@push`/`@stack` | 2 | `Ctx::require(Asset)` → `BTreeSet` | trivial |
| `<x-dynamic-component>` | 1 | `match` | trivial |
| `@vite` | 2 | registro de asset com hash | mecânico |
| `@csrf` | 69 arquivos | campo tipado do primitivo `Form` | mecânico |

## 3. Onde está o julgamento (e por quê)

`portal.blade.php` faz, dentro da view:

```php
$currentYear   = app(Clock::class)->now()->format('Y');
$supportAssets = app(SupportAssets::class)->groups();
$searchTerm    = app(SearchInput::class)->sanitize(request()->query('q'));
$leftHtml      = Blade::render('<x-layout::slot name="main.left" />');   // renderiza componente p/ string
@if (app(ThemeResolver::class)->hasConfiguredOverrides())
```

Isso não é template: é **assembler**. No desenho novo vira `ShellCtx` montado no handler, e o
compilador proíbe a view de tocar `Pool`/`Clock`/registry. Os ~30 arquivos nessa situação são:

- 2 shells (`portal`, `cms`);
- 12 blocos/widgets (`blocks/*`, `widgets/*`);
- `slot`, `widget`, `blocks`, `ui/icon`, `portal` (componentes de layout);
- 3 componentes de service (`media::picture`, `richtext::editor`, `seo::tags`).

## 4. Três defeitos latentes encontrados no estudo

Encontrados no componente de imagem (o exemplo canônico de parcial autocontido):

| Defeito | Consequência | Como o crate resolve |
|---|---|---|
| `'alt' => ''` como default | imagem sem alternativa **em silêncio**; axe só pega a página visitada | `alt` obrigatório; decorativa = `Alt::decorative()` explícito |
| `config('services.media.stream_route')` **por render** | leitura de config em view; acoplamento | rota injetada no primitivo |
| `usort` + `ksort` do `srcset` no componente | cada parcial reimplementa a ordenação; esquecer gera `srcset` não determinístico | `SrcSet::from_variants` (único lugar, testado) |

Padrão geral: o trabalho de "garantir invariante" estava espalhado em N componentes. No desenho novo
ele existe uma vez, no vocabulário.

## 5. Estimativa

| Categoria | Views | Ritmo | Horas |
|---|---|---|---|
| mecânicas (lista/form/show) | ~150 | 30–40 min | ~85 |
| médias (slots, tabela rica, filtros) | ~60 | 2 h | ~120 |
| julgamento (widgets, blocos, shells, services) | ~12 | 3–4 h | ~45 |
| geradores (`Msg` 2.702 + `Route` 362) | — | script + revisão | ~16 |
| **total** | **222** | | **~265 h ≈ 7 semanas** (1 pessoa + agentes) |

## 6. Ferramental de conversão

1. **`Lang/*.php` → catálogo** — script lê os 4 arrays e emite o catálogo de tradução + o `enum Msg`
   com paridade verificada. Nenhuma variante entra sem os 4 locales.
2. **Tabela de rotas → `enum Route`** — gerado do registro do roteador; `LUrl::variante(params)`
   tipado; o locale é injetado pelo roteador, não pelo chamador.
3. **Conversor de view (assistido)** — transforma a estrutura Blade (`@if/@foreach/x-slot`) em
   esqueleto `html!`; nomes de atributo são validados contra a spec no primeiro `cargo check`.
4. **Snapshot por view** — cada view convertida gera um `.snap`; o diff Blade→crate é revisado uma vez,
   e a partir daí é o contrato.

## 7. O que se ganha (verificável)

| Ganho | Como se verifica |
|---|---|
| `href` sem locale deixa de compilar | `trybuild` |
| `img` sem `alt` deixa de compilar | `trybuild` |
| `style`/`on*` deixam de existir | spec + `trybuild` |
| `<h1>` único e ordem de heading | `html-lint` |
| `id` duplicado / `for` órfão | erro de macro |
| JSON-LD nunca quebra a página | property test |
| nenhuma página sem shell | tipo (`Page`) |
| `srcset` determinístico | property test |

Gates que **saem** do runtime: `LayoutContractTest` (varredura de `<html>` nas views),
`ModuleI18nTest` (paridade virou compilação), `LocaleUrlInvariantTest` (varredura de `href` virou
tipo). Gates que **ficam**: axe, `sniffCSS`, teclado, e2e.

## 8. O que se perde

| Perda | Mitigação |
|---|---|
| markup fora de arquivo `.html` | snapshot revisável (`.snap` é o HTML legível) |
| `@php` livre na view | assembler explícito (é o objetivo) |
| edição de copy no template | catálogo de tradução (arquivo, editável por não-Rust) |
| velocidade inicial de escrever tela nova | primitivos + defaults do gerador; medido em `M2` |

## 9. Ordem recomendada (não inverter)

1. `Msg` + `Route` gerados (destrava tudo).
2. `ShellCtx` + `Page` (destrava toda view).
3. Um módulo completo ponta a ponta (Notícias: lista, show, CMS, busca, sitemap, auditoria).
4. Os primitivos usados por todos (`Field`, `Table`, `Dialog`, `Picture`, `Pagination`, `Breadcrumb`).
5. Os ~30 arquivos de julgamento.
6. O resto mecânico, em lote.

**Não converter 1:1.** O objetivo da etapa 3 é provar que dá para escrever a mesma tela com menos
linhas e sem `app()` na view. Se não der, o projeto parou no lugar certo (ver
[`16-roadmap.md`](16-roadmap.md), M1).
