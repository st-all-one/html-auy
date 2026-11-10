# Preview do portal

Artefato de visualização do exemplo
[`portal_governamental.rs`](../portal_governamental.rs).

- `index.html` é o documento gerado pelo exemplo; não edite à mão.
- `assets/portal.css` é o CSS do consumidor usado apenas nesta preview.
- `assets/portal.js` existe para satisfazer a referência de asset; a página funciona sem ele.

## Regenerar

```bash
cargo run -q -p html-auy --example portal_governamental > index.html
```

## Visualizar

```bash
python3 -m http.server 8765 --bind 127.0.0.1
```

Abra `http://127.0.0.1:8765/`. O servidor precisa ser iniciado nesta pasta para que os caminhos
absolutos `/assets/...` resolvam.
