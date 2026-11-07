# html-auy

Implementação do `EXAMPLE.html` com o crate local `html` (`html!` + valores tipados).

```bash
cargo run
# http://127.0.0.1:3010
```

O `<head>` do exemplo usa Tailwind CDN e CSS inline; o corpo inteiro é gerado por `html!`, sem
`Render` de consumidor. O `<h1>` do hero é markup comum; links `href="#"`/`href="#id"` usam
`Href::Fragment`. Os testes do binário (`cargo test`) verificam um `h1`, um `main`, 204 `div`, 56
`a`, 8 `button`, os seis cartões e a ausência de `href="/#"`.
