# 11 — Estabilidade e versionamento

## 1. O que "estável" significa aqui

O projeto **não** é publicado como crate (D0), mas precisa ser estável para o consumidor que o
vendoriza. Estabilidade tem quatro contratos:

| Contrato | Garantia |
|---|---|
| **sintaxe de `html!`** | congelada por minor; mudança só em major interno |
| **spec de elementos** | `spec_version` própria (`2.0.0`), congelada por minor do projeto |
| **MSRV** | declarada, verificada no CI; subir MSRV é major |
| **semântica de escape** | nunca muda sem major; tem propriedade testada |

## 2. Política de versão (interna)

`0.x.y` com semver aplicado ao **consumidor** (que é único e conhecido):

| Mudança | Bump |
|---|---|
| nova entrada em `spec/elements.toml` | minor |
| nova regra de `N4` | minor **se** zero ocorrência nos arquivos de referência; senão major |
| novo primitivo | minor |
| atributo passa a ser obrigatório em um elemento | **major** |
| elemento removido da spec | **major** |
| mudança na tabela de escape | **major** |
| novo erro de compilação numa marcação que antes compilava | **major** (é quebra) |
| melhoria de mensagem de erro | patch |

Regra prática: se um `cargo check` que passava para de passar, é major — mesmo que o código esteja
"mais correto". O consumidor precisa de um ponto único de atualização.

## 3. MSRV e churn de proc-macro

O ponto frágil de qualquer macro é a dependência do compilador. Mitigações:

- MSRV declarada e testada (`cargo build --locked` com o toolchain mínimo no CI) — **D32**: MSRV `1.92` em três lugares, pin `1.99` é outro número.
- Matriz de CI: MSRV, stable, beta. (nightly não é suportado.)
- Parser sem dependência de detalhes internos: só `proc-macro2`/`quote`, ambos com API estável.
- Nenhum `unsafe`, nenhum feature gate noturno.
- Teste de fumaça de macro (`trybuild` de sucesso) roda na matriz inteira.

## 4. Congelamento da spec

- `spec/elements.toml` tem `spec_version`.
- Congelamento por minor: dentro de uma minor, só entram **adições** (elemento, atributo permitido,
  invariante em elemento não usado).
- Cada mudança registra entrada em `docs/13-decisoes.md` com: motivo, caso de uso, custo estimado de
  migração.
- Revisão periódica: a cada minor, um teste compara a spec com os elementos realmente emitidos no
  consumidor e lista os sem uso — candidatos a remoção no próximo major.

## 5. Deprecação

```
deprecated = "use `Tone::Primary` em vez de cor explícita; remoção em 1.0"
```

- Elemento/atributo deprecado continua compilando por uma minor inteira.
- A mensagem de erro/aviso de compilação carrega o motivo (não existe advisory de lint: D28/A4).
- Remoção só em major, com nota de migração (`docs/15-migracao-blade.md` mantido como exemplo).

## 6. Atualização no consumidor

```
1. atualizar versão vendorizada
2. cargo check          → erros de major aparecem aqui, com mensagem e sugestão
3. defaults do gerador   → mecânicos já vêm aplicados (D28/A3)
4. insta review         → snapshots mostram exatamente o que mudou no HTML
5. gates (axe/sniff)    → sem mudança comportamental esperada
```

O passo 4 é o que torna atualização segura: qualquer diferença de bytes aparece como diff de snapshot.

## 7. Compatibilidade com o consumidor (o que o crate assume)

- Rust ≥ 1.92, edição 2024.
- Tipos do consumidor que o crate referencia por trait: `Msg` (texto), `Locale`, `Asset`, `LUrl`.
  Todos por **trait**, nunca por tipo concreto — o crate não conhece o domínio.
- Nenhum acesso a I/O, tempo, aleatoriedade ou variável de ambiente.

### 7.1 Lints, publicação e lockfile

```toml
# workspace: os lints são declarados UMA vez, na forma correta ([workspace.lints.clippy]).
[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "warn", priority = -1 }
# cada crate herda com `[lints] workspace = true`
```

- `publish = false` em todos os crates (D0) e `Cargo.lock` **versionado**: build reprodutível não
  depende de resolução de versão em CI.
- `rust-version = "1.92"` em cada manifesto, e um job de CI compila **em 1.92**, não só na stable —
  sem isso a MSRV declarada é ficção e API nova entra sem ninguém notar.

## 8. O que pode mudar sem aviso

- Mensagens de erro (texto), desde que mantenham as três partes.
- Ordem de regras de lint no relatório.
- Implementação interna do parser.
- Números de benchmark (documentados, não contratuais).
