# maud

Implementação do `EXAMPLE.html` com Maud 0.27 (a terceira opção: macro + `Markup`, sem tabela de
atributos do crate).

```bash
cargo run
# http://127.0.0.1:3002
```

O corpo inteiro é `maud::html!`; o `<head>` também é Maud, com `PreEscaped` apenas para o CSS e o
script de configuração do Tailwind.
