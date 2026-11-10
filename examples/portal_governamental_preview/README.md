# Preview do portal

Artefato de visualização do exemplo
[`portal_governamental.rs`](../portal_governamental.rs).

| Arquivo | Locale |
|---|---|
| `index.html` | `pt-BR` |
| `en/index.html` | `en` |
| `es/index.html` | `es` |
| `fr/index.html` | `fr` |

- Os quatro HTML são gerados pelo exemplo; não edite à mão.
- `assets/portal.css` é o CSS do consumidor usado apenas nesta preview.
- `assets/portal.js` existe para satisfazer a referência de asset; a página funciona sem ele.
- Cada arquivo tem `lang`, `dir`, `canonical` e os `hreflang` recíprocos dos quatro locales, além de
  `x-default` para `pt-BR`.
- O header inclui um seletor visível com links para `/`, `/en/`, `/es/` e `/fr/`; o locale atual
  recebe `aria-current="page"`.

## Regenerar

```bash
cargo run -q -p html-auy --example portal_governamental > index.html
cargo run -q -p html-auy --example portal_governamental -- en > en/index.html
cargo run -q -p html-auy --example portal_governamental -- es > es/index.html
cargo run -q -p html-auy --example portal_governamental -- fr > fr/index.html
```

## Visualizar

```bash
python3 -m http.server 8765 --bind 127.0.0.1
```

Abra `http://127.0.0.1:8765/` e use o seletor de idioma, ou navegue para `/en/`, `/es/` e `/fr/`.
O servidor precisa ser iniciado nesta pasta para que `/assets/...` e as rotas de locale resolvam.