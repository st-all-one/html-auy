# Demo — `EXAMPLE.html` nas três opções

O mesmo portal do `EXAMPLE.html` implementado três vezes, em projetos Cargo independentes (cada
um tem `[workspace]`, `Cargo.toml` e `Cargo.lock` próprios):

| Pasta | Implementação | Porta |
|---|---|---|
| `html-auy/` | crate local `html` + `html!` | `3010` |
| `askama/` | Askama 0.16 + template de arquivo | `3001` |
| `maud/` | Maud 0.27 (a terceira opção) | `3002` |

```bash
cd demo/html-auy && cargo run   # http://127.0.0.1:3010
cd demo/askama   && cargo run   # http://127.0.0.1:3001
cd demo/maud     && cargo run   # http://127.0.0.1:3002
```

## O que é idêntico

As três árvores têm a mesma contagem de elementos e o mesmo DOM visual: 30 tags, 204 `div`, 56
`a`, 1 `h1`, 2 `img`, 2 `input`, 6 `section`, 5 `article`, sem `id` duplicado. As diferenças
restantes são de representação:

- `askama` usa `{% raw %}` somente para o CSS e o JSON de configuração do Tailwind; é a porta de
  escape que o projeto critica, aqui necessária para o demo estático.
- `maud` usa `PreEscaped` para esses mesmos blocos.
- `html-auy` não usa mais `Render` de consumidor: `SERVICES`/`CONTROLS` são dados e
  `service_grid()`/`header_controls()` são `html!` com `for`. `div` dentro de `a` (transparente) e
  `title` global são válidos no `spec`; `href="#"`/`href="#id"` usam `Href::Fragment`.
- O `<head>` dos três continua sendo o `head.html` do exemplo (Tailwind CDN + config inline),
  concessionária do demo, não do crate.

## Correções de fidelidade

O `EXAMPLE.html` depende do Tailwind CDN v3 e usa `pt-41`, que essa versão não gera; as três
implementações acrescentam `.pt-41{padding-top:10.25rem}` ao CSS para o hero não ficar sob o
header fixo. Também foram acrescentados `alt`/`width`/`height` às imagens, `name="q"` aos inputs e
`aria-label` nas seções, sem alteração visual.

Este demo é uma prova visual do `EXAMPLE.html`; ele mantém o Tailwind CDN e CSS inline do original.
A produção continua sujeita ao contrato de `docs/06` (sem inline, sem CDN, assets vendorizados).
