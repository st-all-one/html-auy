# askama

Implementação do `EXAMPLE.html` com Askama 0.16 (`templates/index.html` + `#[derive(Template)]`).

```bash
cargo run
# http://127.0.0.1:3001
```

`title` e `heading` são interpolados; o restante do template é o HTML do exemplo, com as correções
de `alt`, `name` e `aria-label` aplicadas para comparar com as outras duas implementações.
