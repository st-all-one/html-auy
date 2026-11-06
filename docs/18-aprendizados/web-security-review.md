# `web-security-review` — 2.0.0

Conjunto de 21 arquivos (3.016 linhas) extraído da documentação MDN de **segurança e privacidade web**,
com `VERSION` declarando "snapshot referente ao estado da plataforma web em 2026-Q2". Cobre TLS/HSTS,
SOP/CORS/CORP/COOP, SRI, Trusted Types, CSP, cookies/CHIPS, autenticação, privacidade, threat modeling,
OpSec e três anexos operacionais (robots.txt, autocomplete, Local Network Access). Serve aqui como
**fonte de vocabulário e de valores literais de cabeçalho** — não como fonte de teoria de injeção de
markup, que ela não cobre (ver §7).

Método: leitura integral dos 21 arquivos; auditoria de `docs/08-seguranca.md` (109 linhas) e de
`spec/elements.toml` (estado observado: esquema 2, `spec_version = "2.0.0"`, 720 linhas — o arquivo foi
reescrito por revisão paralela durante esta leitura; todas as citações da spec referem-se a esse estado).
Cada achado traz citação verbatim, semântica, consequência e veredito. `[INFERENCE]` marca conclusão
que o corpus não afirma.

## 1. Inventário arquivo-a-arquivo

| arquivo | linhas | conteúdo | consequência para html-auy |
|---|---|---|---|
| `00-index.md` | 43 | índice, 7 princípios transversais, relação privacidade↔segurança | só vocabulário; nada normativo novo |
| `01-core-defenses.md` | 122 | TLS/HSTS, mixed content, Secure Contexts, SOP, CORP, COOP, SRI, CT, user activation | tabela de cabeçalhos (§5.1) e regra de SRI/CORS (A17, A20) |
| `02-authentication-modern.md` | 180 | passkeys/WebAuthn, sessão, TOTP, OIDC+PKCE, senhas | fronteira do consumidor; `autocomplete` e cookie de sessão (A25) |
| `03-cookie-security.md` | 120 | atributos de cookie, SameSite, CHIPS, state partitioning, Storage Access API | template `__Host-` literal (A23, A24, A27) |
| `04-privacy-anti-tracking.md` | 121 | bounce tracking, Referrer-Policy, Permissions-Policy, fingerprinting, Privacy Sandbox, Clear-Site-Data | Referrer-Policy + `rel=noreferrer` + `meta name=referrer` (A26, A30, A31) |
| `05-attack-defense-matrix.md` | 82 | 10 ataques × 22 defesas; defense-in-depth para XSS e CSRF | a matriz que falta em `08-seguranca.md` §1; fonte de A1, A2, A21 |
| `06-threat-modeling-framework.md` | 140 | 4 perguntas, STRIDE, LINDDUN, notação, exemplo blog, 13 ameaças PWA | processo: threat model versionado; PWA "simples e auditável" (A3, A4) |
| `07-practical-implementation.md` | 206 | 10 prioridades de cabeçalho, templates de CSP/cookie, deploy checklist | **fonte literal** de §5.1; contradição interna de CSP (A6) |
| `08-modern-web-security-model.md` | 163 | síntese em 8 pilares, arquitetura em 8 camadas, padrões × anti-padrões | lista de proibições que a crate já cumpre (A8, A19, A24) |
| `09-operational-security.md` | 116 | ambiente de dev, RBAC, secrets, lockfile, SBOM, monitoramento, SECURITY.md, backups | completa `08-seguranca.md` §5 (SBOM, commits assinados) — A36, A37 |
| `10-local-network-access.md` | 108 | address spaces, `local-network`/`loopback-network`, `targetAddressSpace`, alias legado | duas diretivas de `Permissions-Policy` que faltam em §5.1 (A32) |
| `11-robots-txt.md` | 57 | robots.txt como controle de crawl; anti-padrão de esconder paths; alternativas `meta`/header | regra de indexação para o CMS e limite do que a crate pode fazer (A34) |
| `12-form-autocompletion.md` | 57 | autocomplete vs autofill, `off`, exceção de login, WCAG 1.3.5 | valida o `autocomplete_enum` da spec e fecha o caso OTP/CVV (A38) |
| `13-input-validation.md` | 87 | allowlist × denylist, client/server, upload de arquivo, mapa ataque→defesa | regras de upload que a crate não pode expressar (A35) |
| `14-tls-deep.md` | 97 | perfis Mozilla, redirect, HSTS detalhado, checklist TLS | duplica `01` §1 com os mesmos números: confirma os valores (A28) |
| `15-firefox-tracking-protection.md` | 89 | ETP, lista de trackers, graceful degradation, código resiliente | argumento para "sem terceiros por padrão" (A3-adjacente) |
| `16-storage-access-policy-errors.md` | 85 | 5 erros `CookieBlocked*`/`CookiePartitionedForeign` do Firefox | **irrelevante** para a crate: só diagnosticar widgets embedados do consumidor |
| `17-https-by-default.md` | 63 | HTTPS como feature de privacidade, Secure Contexts, 67+ APIs | confirma `Href::External` só `https` (A33) |
| `modern-implementation-example.md` | 910 | aplicação-referência completa (Nginx→Express→Postgres→browser) com todos os conceitos aplicados | valores literais e **dois erros a não copiar**: Trusted Types `default` e Fetch Metadata com `same-site` (A15, A22) |
| `SKILL.md` | 166 | workflow de revisão em 5 passos, "non-negotiables", anti-padrões, comandos | quick-reference de cabeçalhos; contradição `unsafe-inline` (A6) |
| `VERSION` | 4 | `2.0.0` / `2026-07-21`, escopo do snapshot | ancora de versão para `18-aprendizados.md` |

## 2. Achados

### A1. XSS não tem defesa única: são seis camadas simultâneas
- Citação: `05-attack-defense-matrix.md:54` — "Para proteger contra XSS, **todas** as seguintes são necessárias:" (segue lista de 6: encoding, DOMPurify, strict CSP, Trusted Types, HttpOnly, validação).
- Semântica: a matriz trata XSS como many-to-many: nenhuma camada isolada é aceita como suficiente, e o corpus repete isso em `08:29-33` e `08:96-109`.
- Consequência: `08-seguranca.md` não pode se apresentar como "resolvido" por tipos; a §1 precisa declarar quais das seis camadas a crate entrega (encoding de saída, ausência de inline, ausência de `innerHTML`) e quais delega (CSP header, HttpOnly, validação de entrada no servidor).
- Veredito: confirma P3/D1 e **refina** `08-seguranca.md:5-6` ("nenhuma depende de disciplina").

### A2. A pré-condição do XSS é ausência de encoding — e a crate elimina a pré-condição
- Citação: `05-attack-defense-matrix.md:9` — "| **XSS** | Injeção de script pelo input do usuário | Input sem sanitização/output encoding |".
- Semântica: o corpus define XSS pela *pré-condição* (input não tratado), não pela payload.
- Consequência: reforça D2 — sem `Raw`, a pré-condição desaparece do lado do markup gerado; o que sobra é o consumidor interpolando dado em JS (`data-*` é valor enumerado) e conteúdo rich-text vindo do editor.
- Veredito: confirma P2/D1/D2.

### A3. Simplicidade e ausência de terceiros são controles de segurança de primeira classe
- Citação: `06-threat-modeling-framework.md:140` — "- Código simples e auditável = first-class security control"; `06:139` — "- Sem third-party dependencies = estratégia de segurança".
- Semântica: o corpus trata redução de superfície (menos dependências, menos código) como mitigação, não como preferência estética.
- Consequência: D9 (zero `unsafe`, sem `syn`) deixa de ser só disciplina de engenharia e vira **afirmação de segurança** citável em `08-seguranca.md` §5; o mesmo vale para "sem CDN" (`07-js-e-assets.md:118`).
- Veredito: confirma P10/D9.

### A4. Threat model é documento versionado e revalidado, não evento único
- Citação: `06-threat-modeling-framework.md:140` (lições PWA) e a metodologia descrita no mesmo arquivo [sem linha] — "processo iterativo, living document, começa cedo, revisitado frequentemente".
- Semântica: 4 perguntas (sistema → ameaças → resposta ERTA → validação), STRIDE para segurança e LINDDUN para privacidade, com notação fixa (C/A/F/E/S/T/R).
- Consequência: falta na crate um artefato de threat model; proponho `docs/08-seguranca.md` §9 com a tabela STRIDE do **gerador** (macro, parser, codegen, spec) — não da aplicação do consumidor.
- Veredito: adição (não coberta hoje).

### A5. Strict CSP recomendado é nonce-based — e a crate decide o contrário com razão
- Citação: `07-practical-implementation.md:63` — "### Strict CSP (Nonce-based) — RECOMENDADO"; valor em `07:66-69`.
- Semântica: o corpus recomenda nonce+`strict-dynamic` como padrão-ouro, porque assume aplicação dinâmica com inline legítimo.
- Consequência: a crate não tem inline e quer página cacheável (`docs/08-seguranca.md:82`); o nonce violaria P6/`ETag`. A escolha de `script-src 'self'` **é** o equivalente estrito para vocabulário fechado — mas `08-seguranca.md` §6 precisa dizer isso explicitamente, senão parece descuido.
- Veredito: refina (divergência justificada, com frase a inserir — §4).

### A6. O corpus se contradiz sobre `'unsafe-inline'` em `script-src`
- Citação: `SKILL.md:109` — "Content-Security-Policy: default-src 'none'; script-src 'nonce-{random}' 'strict-dynamic' 'unsafe-inline' https:; …" contra `SKILL.md:126` — "- ❌ NEVER `'unsafe-inline'` in `script-src`".
- Semântica: a forma longa (`nonce` + `strict-dynamic` + `'unsafe-inline'` + `https:`) é fallback histórico para navegadores antigos; o `'unsafe-inline'` só é inofensivo porque `strict-dynamic` o neutraliza.
- Consequência: **não copiar**. Em `docs/08-seguranca.md` o valor publicado deve ser o mínimo que se sustenta sem depender de neutralização por outra diretiva; qualquer `'unsafe-inline'` na tabela de §5.1 é regressão.
- Veredito: refuta a quick-reference do corpus; confirma D3.

### A7. `'strict-dynamic'` pressupõe loader dinâmico — a crate não tem nenhum
- Citação: `08-modern-web-security-model.md:30` — "- Strict CSP (nonce/hash-based) elimina XSS por injeção".
- Semântica: `strict-dynamic` existe para quem cria `<script>` em runtime; sem criação dinâmica ele só reduz compatibilidade.
- Consequência: incluí-lo no header do consumidor seria ruído e sugeriria que existe loader; a tabela de §5.1 deve proibi-lo explicitamente com motivo ("sem criação dinâmica de script; assets são `<script src defer>`").
- Veredito: irrelevante para a crate, ativamente enganoso se copiado.

### A8. Allowlist de hosts em `script-src` é anti-padrão declarado
- Citação: `08-modern-web-security-model.md:115` — "| CSP | Host-based allowlists no script-src | Muito amplo, difícil de manter |"; idem `SKILL.md:127`.
- Semântica: origem permitida em `script-src` transforma qualquer comprometimento do host em XSS.
- Consequência: valida o gate de assets (`docs/07-js-e-assets.md:127-128`: lint reprova `src`/`href` externo fora da allowlist) e proíbe `script-src https:` — inclusive como "fallback".
- Veredito: confirma D13 + o gate existente.

### A9. `object-src 'none'` e `base-uri 'none'` são obrigatórios e a crate já os garante por construção
- Citação: `07-practical-implementation.md:68-69` — "  object-src 'none';" / "  base-uri 'none';".
- Semântica: `base-uri` fecha o sequestro de URLs relativas; `object-src` fecha plugin/Flash-legacy.
- Consequência: o vocabulário não tem `<base>` nem `<object>`/`<embed>` (`spec/elements.toml:31-33` lista o que nunca entra); logo o header é cinto, não necessidade — mas continua na tabela porque o consumidor pode ter rotas não geradas pela crate.
- Veredito: confirma D4 (vocabulário fechado).

### A10. `frame-ancestors 'none'` + `X-Frame-Options` legado
- Citação: `07-practical-implementation.md:83` — "Content-Security-Policy: frame-ancestors 'none';" e `07:88` — "X-Frame-Options: DENY (ou SAMEORIGIN)".
- Semântica: `frame-ancestors` é o controle moderno; `X-Frame-Options` só existe para navegador antigo, e o corpus o mantém por compatibilidade.
- Consequência: manter os dois na tabela (o segundo marcado como legado), e não deixar a crate prometer clickjacking resolvido — ela não emite cabeçalho (`docs/08-seguranca.md:90`).
- Veredito: confirma a fronteira `08-seguranca.md` §7.

### A11. CSP em modo report-only com endpoint de relatório é etapa obrigatória do deploy
- Citação: `07-practical-implementation.md:93` — "Content-Security-Policy-Report-Only: ...; report-to csp-endpoint; report-uri /csp-reports"; `07:96` — "Reporting-Endpoints: csp-endpoint=\"https://example.com/csp-reports\"".
- Semântica: a diretiva antiga (`report-uri`) e a nova (`report-to` + `Reporting-Endpoints`) coexistem; a implantação passa por report-only antes de enforce.
- Consequência: a tabela de §5.1 precisa das duas linhas e do aviso de que relatório é **dado não confiável** (o endpoint recebe payload do browser, então: limite de tamanho, sem `eval`, sem escrever em path do usuário). O exemplo `modern:355` usa `fs.appendFileSync` com JSON do cliente — anti-exemplo.
- Veredito: adição (faltam `report-to`/`Reporting-Endpoints` em `08-seguranca.md`).

### A12. `upgrade-insecure-requests` continua sendo a rede de segurança para URL absoluta legada
- Citação: `01-core-defenses.md:27` — "Content-Security-Policy: upgrade-insecure-requests;".
- Semântica: upgrade automático de subrecurso `http:` em página `https:`.
- Consequência: com `Src`/`Href` tipados a crate não emite `http:`, então a diretiva é inócua; incluir mesmo assim (custo zero) e documentar que ela **não** substitui tipo — upgrade não vale para `<a>`/`form action`.
- Veredito: confirma P2 (tipo antes de header) e adiciona linha à tabela.

### A13. `script-src-attr` e `style-src-attr` não existem no corpus
- Citação: `07-practical-implementation.md:101` — "script-src 'unsafe-inline' data: https:;   # NUNCA" (única menção a atributo inline é a proibição genérica); `grep -c script-src-attr` = 0 nos 21 arquivos.
- Semântica: o corpus nunca separa script de *elemento* e script de *atributo*; a distinção importa porque `script-src 'self'` não impede `onclick=` em algumas configurações de compatibilidade.
- Consequência: `[INFERENCE]` a crate deve emitir `script-src-attr 'none'; style-src-attr 'none';` na tabela de §5.1 como cinto explícito do D3 — é a única forma de afirmar "nenhum handler inline" no nível do header, e não só no vocabulário.
- Veredito: lacuna do corpus; adição obrigatória à spec de cabeçalhos.

### A14. Trusted Types é a defesa declarada para DOM-XSS, não para XSS de servidor
- Citação: `05-attack-defense-matrix.md:33` — "| **Trusted Types API** | DOM-based XSS | CSP: `require-trusted-types-for 'script'` |"; `08-modern-web-security-model.md:32` — "- `require-trusted-types-for` previne DOM-based XSS".
- Semântica: TT protege *sinks* do DOM (atribuição a `innerHTML`, `script.src`), e só onde há JS.
- Consequência: a afirmação de `docs/08-seguranca.md:85-86` ("não há `innerHTML` no markup gerado") é verdadeira mas insuficiente: os bundles do consumidor (`Editor`, `HomeEditor`) manipulam DOM. A §6 deve dizer que a diretiva é **do consumidor** e recomendada nos bundles que tocam HTML — e que a crate não fornece política.
- Veredito: refina `08-seguranca.md:85-86`.

### A15. A política `default` de Trusted Types do exemplo-referência é um anti-padrão
- Citação: `modern-implementation-example.md:603-604` — "  trustedTypes.createPolicy('default', {" / "    createHTML: (input) => DOMPurify.sanitize(input),".
- Semântica: a política `default` captura **todo** sink do documento, sanitizando silenciosamente via biblioteca de terceiro.
- Consequência: `[INFERENCE]` uma política `default` (a) apaga a rastreabilidade de qual call site fez a atribuição, (b) transforma erro de programação em sanitização invisível — o oposto de P3 —, (c) reintroduz uma dependência (DOMPurify) em posição privilegiada, e (d) se o sanitizador tiver bug, o bug vale para toda a aplicação. A regra para o consumidor: política **nomeada** por módulo, `createHTML` que lança fora de allowlist explícita, zero política `default`.
- Veredito: refuta o exemplo do corpus; registrar como armadilha (§6).

### A16. CSP entregue por `<meta http-equiv>` é vetor, não solução
- Citação: `modern-implementation-example.md:596-597` — "<meta http-equiv=\"Content-Security-Policy\"" / "      content=\"require-trusted-types-for 'script'; report-to csp-endpoint\">".
- Semântica: o exemplo usa `meta` para não depender do servidor; o corpus não discute as diretivas que são ignoradas em `meta` nem o fato de que qualquer HTML injetado pode emitir seu próprio `meta`.
- Consequência: a spec já fechou isso (`spec/elements.toml:100` — `invariants = ["meta_name_enum", "no_http_equiv"]   # proíbe refresh/CSP por markup`). Manter, e acrescentar `refresh` ao motivo escrito: `http-equiv="refresh"` é redirect controlado por conteúdo e phishing de barra de endereço.
- Veredito: confirma a spec atual; o corpus serve de contraexemplo documentado.

### A17. SRI exige CORS — e a spec permite `integrity` em `link` sem `crossorigin`
- Citação: `01-core-defenses.md:96` — "- Exige CORS no CDN (`Access-Control-Allow-Origin: *`)"; spec: `spec/elements.toml:106` — "allowed = [\"as\", \"type\", \"media\", \"integrity\", \"hreflang\", \"sizes\", \"fetchpriority\", \"disabled\"]" e `spec:109` — "invariants = [\"link_rel_enum\", \"preload_requires_as_and_crossorigin\"]".
- Semântica: o browser só valida `integrity` de recurso cross-origin quando a requisição é CORS (`crossorigin`); sem ele o recurso é *opaque* e o hash não é verificável.
- Consequência: defeito interno da spec: o invariante `preload_requires_as_and_crossorigin` **exige** um atributo que a allowlist do mesmo elemento não contém. Ou se adiciona `crossorigin` a `link` (com enum `anonymous|use-credentials`), ou `integrity` sai e SRI fica impossível em `<link>` — hoje `integrity` em `link` é uma promessa que a crate não consegue cumprir.
- Veredito: conflito real; correção em §4 e §5.3.

### A18. SRI em `<script>` está correto na spec; falta o invariante de obrigatoriedade
- Citação: `01-core-defenses.md:92` — "        integrity=\"sha384-ABC123...\""; `07-practical-implementation.md:141` (mesmo padrão, hash real); spec: `spec/elements.toml:114` — "allowed = [\"type\", \"defer\", \"async\", \"integrity\", \"crossorigin\"]".
- Semântica: SRI + `crossorigin` é o par mínimo; o consumidor já exige (`docs/07-js-e-assets.md:119`: "`integrity` (SRI) obrigatório para asset **enviado pelo usuário**").
- Consequência: `script` aceita `crossorigin` mas não tem invariante que force o par `integrity`+`crossorigin` quando o `src` é externo; como a crate não tem CDN, o caso é raro — vale como invariante de lint (N4) e não como erro de macro.
- Veredito: refina; adiciona invariante `integrity_requires_crossorigin` a `script` e `link`.

### A19. CORS: refletir `Origin` com credenciais é o anti-padrão mais citado do conjunto
- Citação: `07-practical-implementation.md:131` — "NUNCA refletir `Origin` header com `Access-Control-Allow-Credentials: true`."; `08:116` — "| CORS | Refletir `Origin` header | Cria trust escalável |".
- Semântica: refletir origem qualquer com credenciais anula o SOP na prática.
- Consequência: fora do escopo da crate, mas entra na tabela de cabeçalhos como linha "CORS: allowlist estática, nunca eco" e no checklist do consumidor; a crate pode ajudar emitindo `form-action 'self'` (já presente em `docs/08-seguranca.md:78`).
- Veredito: confirma a fronteira §7 de `08-seguranca.md`.

### A20. CORP e COOP são cabeçalhos obrigatórios por resposta, inclusive para assets
- Citação: `01-core-defenses.md:72` — "Cross-Origin-Resource-Policy: same-origin"; `01:84` — "Cross-Origin-Opener-Policy: same-origin".
- Semântica: CORP impede embedding do recurso por outra origem (XS-Leak/Spectre); COOP isola `window` (`01:86`: "Protege contra XS-Leaks via `window.open()`").
- Consequência: a crate serve `/theme.css` e os assets com hash via o consumidor; CORP é o que impede que terceiro embede a fonte/Ícone do município. Linha obrigatória em §5.1 + teste de header no e2e.
- Veredito: confirma; adição à tabela.

### A21. CSRF precisa de quatro camadas, e a crate só cobre uma
- Citação: `05-attack-defense-matrix.md:65-68` — "1. `SameSite=Lax` (ou `Strict`) em cookies de sessão / 2. CSRF tokens em forms / 3. Fetch Metadata validation no server (`Sec-Fetch-Site: same-origin`) / 4. Non-simple requests (JSON content type, custom headers)".
- Semântica: token sozinho não basta; SameSite sozinho não basta.
- Consequência: a parte da crate é estrutural (form com `method` enum + `action` `LUrl` + campo token tipado — `spec/elements.toml:516-521`), o resto é consumidor. `08-seguranca.md` §7 deve listar as quatro em vez de "CSRF — do consumidor".
- Veredito: refina `08-seguranca.md:91`.

### A22. O exemplo de Fetch Metadata do corpus aceita `same-site` — e isso é buraco
- Citação: `modern-implementation-example.md:376-377` — "  // Allow same-origin and same-site" / "  if (site === 'same-origin' || site === 'same-site') return next();".
- Semântica: `Sec-Fetch-Site: same-site` inclui **qualquer subdomínio** da mesma registrable domain.
- Consequência: num portal municipal com `cms.` e `www.`, um XSS ou takeover em qualquer subdomínio passa a poder POSTar em endpoints state-changing. Regra correta: `same-origin` apenas; `same-site` só para rotas explicitamente idempotentes. Também notar `const dest = req.headers['sec-fetch-dest'];` declarado e nunca usado (código morto que sugere verificação inexistente).
- Veredito: refuta o exemplo; corrigir no §5.1/checklist.

### A23. Prefixo `__Host-` é o único bind de cookie que o browser impõe
- Citação: `03-cookie-security.md:7` — "Set-Cookie: __Host-SESSIONID=<random>; Path=/; Secure; HttpOnly; SameSite=Lax; Max-Age=2592000"; `03:18` — "| `__Host-` prefix | Recomendado | Bind ao domínio + Path=/ |".
- Semântica: `__Host-` exige `Secure`, `Path=/` e ausência de `Domain`; impede overwrite por subdomínio.
- Consequência: valor literal pronto para o consumidor; a crate não define cookie, mas o *nome* pode ser centralizado num doc para não divergir entre CMS e portal.
- Veredito: confirma (fronteira §7).

### A24. SameSite: `Lax` é o default e `None` só com `Secure`
- Citação: `03-cookie-security.md:36` — "| `Lax` | Envia em top-level navigations GET | Default recomendado |"; `03:37` — "| `None` | Envia em todos contextos | Exige `Secure` — usar apenas quando necessário |"; `08:119` — "| Cookies | SameSite=None sem Secure | Rejeitado por browsers |".
- Semântica: `Lax` permite navegação GET top-level — por isso CSRF de GET precisa ser impedido no servidor (nenhum GET state-changing).
- Consequência: fecha o requisito de que a crate não emita `method="GET"` com campos que mudam estado; `form` com `method_enum` deve restringir `GET` a formulários de busca (regra de lint, não de tipo).
- Veredito: confirma + adição de regra N3.

### A25. Sessão: cookie + timeout duplo + invalidação em evento de segurança
- Citação: `02-authentication-modern.md:100` — "Set-Cookie: session_id=<random>; HttpOnly; Secure; SameSite=Lax; Path=/; Max-Age=3600"; `07:186` — "- [ ] Idle timeout (30 min) + absolute timeout"; `07:187` — "- [ ] Invalidar sessão em eventos suspeitos".
- Semântica: o cookie é só a metade; sem idle **e** absolute timeout, e sem invalidação em troca de senha, o token vira credencial permanente.
- Consequência: três linhas no checklist do consumidor; a crate não tem sessão (não-objetivo permanente, `00-principios.md:87`).
- Veredito: confirma não-objetivo.

### A26. `Clear-Site-Data` no logout é o único controle que limpa estado de terceiros
- Citação: `07-practical-implementation.md:152` — "Clear-Site-Data: \"cache\", \"cookies\", \"storage\""; `04:112` (idêntico); `07:199` — "- [ ] Clear-Site-Data no logout".
- Semântica: limpa cache, cookies e storage daquela origem; `[INFERENCE]` não alcança storage particionado de terceiros nem dados de outra origem.
- Consequência: exigir no logout do CMS (que guarda rascunho no browser) e documentar que "storage" inclui IndexedDB do editor; a crate não pode emitir o header, mas o primitivo de logout pode declarar que ele é obrigatório.
- Veredito: adição à tabela §5.1 (linha de resposta, não de documento).

### A27. CHIPS/`Partitioned` não tem caso de uso no portal hoje
- Citação: `03-cookie-security.md:50` — "Set-Cookie: __Host-example=34d8g; SameSite=None; Secure; Path=/; Partitioned;"; `03:57` — "- `Partitioned` exige `Secure`".
- Semântica: cookie duplamente keyed por top-level site; alternativa a third-party cookie.
- Consequência: registrar como "não aplicável enquanto não houver widget embedado"; se aparecer (ex.: VLibras, mapa), a decisão muda. Evita adoção prematura de `Partitioned` em cookie de sessão próprio, que só adiciona modo de falha.
- Veredito: irrelevante hoje, com gatilho explícito.

### A28. HSTS: dois anos, `includeSubDomains`, `preload` — com dois arquivos concordando
- Citação: `01-core-defenses.md:21` — "- `max-age`: mínimo 6 meses (`15768000`), recomendado 2 anos (`63072000`)"; `14-tls-deep.md:64` — "| `max-age` | SIM | Duração em segundos. Mínimo 6 meses (`15768000`), recomendado 2 anos (`63072000`) |".
- Semântica: valores duplicados em duas fontes independentes do mesmo corpus, sem divergência.
- Consequência: valores prontos para §5.1; e a advertência de `14:65` ("pode quebrar subdomínios sem HTTPS") exige que `includeSubDomains` só entre depois de inventário de subdomínio do município.
- Veredito: confirma.

### A29. `nosniff` é o cabeçalho que sustenta "upload nunca vira HTML"
- Citação: `07-practical-implementation.md:111` — "X-Content-Type-Options: nosniff"; `07:167` (checklist).
- Semântica: sem `nosniff`, um arquivo servido com `Content-Type` errado pode ser interpretado como HTML/JS.
- Consequência: sem ele, a allowlist de extensão de upload (`13-input-validation.md:70`) é só cosmética; a regra de mídia da crate (`Src::media(ObjKey)`, `docs/08-seguranca.md:39`) precisa vir acompanhada do requisito de resposta `nosniff` + `Content-Type` derivado do sniff de conteúdo, não do nome.
- Veredito: confirma a lacuna de upload (§4) e adiciona linha à tabela.

### A30. Referrer: header + `meta` + atributo — e a spec deixa os dois últimos sem tipo
- Citação: `04-privacy-anti-tracking.md:46` — "| `strict-origin-when-cross-origin` | Full em same-origin, origin-only em cross-origin (DEFAULT) |"; `04:55` — "<meta name=\"referrer\" content=\"no-referrer\">"; `04:60` — "<a href=\"https://example.org\" rel=\"noreferrer\">Link</a>".
- Semântica: três canais, do mais forte ao mais fraco; `unsafe-url` e `origin-when-cross-origin` vazam path/query.
- Consequência: `spec/elements.toml:408` permite `rel` e `referrerpolicy` em `a` como valores livres — `referrerpolicy="unsafe-url"` e `rel="opener"` compilam hoje. Ambos precisam de enum fechado (§5.3); `meta name` precisa de enum que só admita `referrer` com política estrita.
- Veredito: conflito real (dois atributos sem tipo).

### A31. Bounce tracking e link decoration atingem analytics próprio, não só terceiros
- Citação: `04-privacy-anti-tracking.md:19` — "2. Periodicamente verifica: usuário **não interagiu** com o site nos últimos **45 dias**?"; `04:20` — "3. Se sim + third-party cookies bloqueados → **estado deletado** (cookies, localStorage, IndexedDB, Cache API)"; `04:32` — "- **Stateless mode (moderno):** TODO bounce é tratado como stateful (rede e outros estados implícitos podem ser manipulados)".
- Semântica: a heurística apaga estado de site que "só redireciona" — inclusive o do município, se o fluxo de login passar por redirect externo (SSO, portal de serviços, gov.br) e voltar sem interação registrada.
- Consequência: dois efeitos concretos para o consumidor: (a) `Asset::Analytics` opt-in (`docs/07-js-e-assets.md:126`) pode perder o first-party storage e passar a contar sessão duplicada; (b) nenhum link do portal deve sair com parâmetro de campanha (`?utm_*`) — link decoration é tráfego de tracking e é removido por Firefox/Safari. A crate não implementa nada disso, mas o `Href` interno pode recusar query string fora do contrato de rota.
- Veredito: adição (privacidade do consumidor; `Href`/`LUrl` é o único ponto de controle que a crate tem).

### A32. Permissions-Policy inclui `local-network`/`loopback-network`; a spec já tirou `allow` do `iframe`
- Citação: `10-local-network-access.md:33` — "| `local-network` | Endereços locais (`192.168.x.x`) | `navigator.permissions.query({ name: \"local-network\" })` |"; `10:60` — "Permissions-Policy: local-network=(\"https://example.com\")"; `07:135` — "Permissions-Policy: geolocation=(self), camera=(), microphone=()".
- Semântica: duas permissões novas, gated por secure context, contra CSRF a dispositivo local (roteador, impressora, intranet municipal); sem elas, requisição a espaço privado exige prompt.
- Consequência: linha de header com `local-network=()` e `loopback-network=()` por padrão (negação) e nota de que `iframe allow=""` também delega — e `allow` **saiu** da allowlist de `iframe` (`spec/elements.toml:473`), o que fecha a delegação por markup. Correto; registrar como decisão consciente.
- Veredito: confirma a spec; adição à tabela (as duas diretivas faltam).

### A33. Mixed content separa bloqueável de upgradável — e a exceção de loopback é a que interessa
- Citação: `01-core-defenses.md:33` — "- **Upgradable** (auto-HTTP→HTTPS): imagens, áudio, vídeo (`<img src>`, `<video src>`, etc.)"; `01:34` — "- **Blockable** (bloqueado totalmente): scripts, stylesheets, iframes, fetch/XHR, web fonts"; `01:36` — "**Exceção loopback:** `http://127.0.0.1`, `http://localhost`, `file://` são tratados como seguros."
- Semântica: imagem `http:` é upgrade silencioso; script `http:` é bloqueio — o que produz falha visível e difícil de diagnosticar.
- Consequência: mais um argumento para `Src`/`Href` só `https`, e um item de doc: nunca emitir `poster`/`srcset` com origem `http`; `17-https-by-default.md:25-29` repete o mesmo com "site deve quebrar" sem HTTPS como teste.
- Veredito: confirma `docs/08-seguranca.md:35-36` (recusa de esquema).

### A34. robots.txt é público: o anti-padrão é listar `/admin`
- Citação: `11-robots-txt.md:14` — "`robots.txt` é **público** — qualquer pessoa pode lê-lo. Listar paths como `/admin` ou `/secret` revela sua localização para attackers."; `11:16` — "Além disso, crawlers maliciosos (malware, email harvesters) ignoram `robots.txt`."
- Semântica: robots.txt é controle de crawl, não de acesso; e o público do CMS municipal inclui raspador.
- Consequência: no CMS, `noindex` por página via `<meta name="robots">` (`11:37`) é o único canal que a crate consegue emitir — logo precisa entrar no enum de `meta name` com valores `noindex`, `nofollow`, `noarchive`; rascunho publicado sem `noindex` é vazamento de indexação.
- Veredito: adição (nome de `meta` hoje sem enum publicado na spec).

### A35. Upload: quatro regras, nenhuma expressável em markup
- Citação: `13-input-validation.md:65` — "| XSS via download | Não servir HTML/JS de user uploads |"; `13:64` — "| Overwrite de arquivos do site | Armazenar fora do webroot |"; `13:69` — "- Gerar nome próprio para o arquivo (não confiar no filename do usuário)"; `13:70` — "- Allowlist de extensões".
- Semântica: o vetor não é "arquivo ruim", é **arquivo servido do mesmo origem com tipo interpretável**.
- Consequência: `Src::media(ObjKey)` só aceita chave opaca (bom), mas nada impede o consumidor de servir `application/octet-stream` com `Content-Type` adivinhado. Regras para o doc: rota de mídia com `nosniff` + `Content-Disposition: attachment` no que não é imagem/vídeo, domínio separado para upload, extensão derivada do sniff e não do nome.
- Veredito: confirma `docs/08-seguranca.md` §7 (fronteira) e adiciona requisito explícito.

### A36. SBOM e lockfile são item de supply chain, não de infraestrutura opcional
- Citação: `09-operational-security.md:62` — "| CycloneDX | OWASP | Supply chain security, leve |"; `09:47` — "- **Usar** `npm ci` em vez de `npm install` em CI"; `09:48` — "- **Comitar** lockfile no source control".
- Semântica: inventário de componentes + build reprodutível a partir do lockfile.
- Consequência: `docs/08-seguranca.md:54-61` cobre `cargo deny` mas não SBOM nem `cargo vet`; para uma crate vendorizada (D0) o SBOM é o artefato que o consumidor audita. Mapear `npm ci` → `--locked`/`--frozen` e CycloneDX → `cargo cyclonedx`.
- Veredito: refina `08-seguranca.md` §5 (faltam duas linhas).

### A37. Assinatura de commit, PR review obrigatório e `SECURITY.md` com SLA
- Citação: `09-operational-security.md:29` — "| Commits assinados | Verificação de autoria |"; `09:109` — "- [ ] Commits assinados + PR review + CI obrigatórios"; `09:92` — "- Arquivo na raiz do repositório".
- Semântica: integridade da cadeia de autoria + canal de reporte com prazo declarado (o exemplo em `modern-implementation-example.md:762-766` dá 24h/72h/7d/90d).
- Consequência: a crate é "interna, não publicada" (D0), mas o repositório é aberto — `SECURITY.md` e signed commits entram como itens de processo em `08-seguranca.md` §5.
- Veredito: adição.

### A38. Autocomplete: login ignora `off`; OTP/CVV pedem `off`; WCAG exige nome semântico
- Citação: `12-form-autocompletion.md:33` — "Browsers modernos **ignoram** `autocomplete=\"off\"` em campos de login (username/password) — por segurança, para incentivar uso de password managers."; `12:54` — "1. Use `autocomplete=\"off\"` para OTP, CVV, identificadores governamentais"; `12:50` — "- Pode-se usar `autocomplete` attributes mesmo com `autocomplete=\"off\"` no form".
- Semântica: `off` é pedido, não garantia; e a semântica de campo (WCAG 1.3.5) não conflita com `off`.
- Consequência: valida o invariante `autocomplete_enum` (`spec/elements.toml:558`); o enum precisa conter `username webauthn` (token múltiplo, `02:66`), `one-time-code`, `new-password`, `current-password`, `off` — e nunca string livre, porque `autocomplete` livre é como um campo de CPF vira `cc-number`.
- Veredito: confirma D1 aplicado a atributo; adiciona o caso multi-token.

### A39. DOM clobbering por `id`/`name` não é coberto pelo corpus, mas a spec expõe `id` global
- Citação: `spec/elements.toml:55` — "id = \"id\"                 # Id: gerado por primitivo ou explícito, único no bloco"; `spec:592` — "allowed = [\"class\", \"id\", \"name\", \"value\", \"disabled\", \"form\", \"popovertarget\",".
- Semântica: `[INFERENCE]` em HTML, elemento com `id`/`name` cria propriedade homônima em `window`/`document`/`document.forms`; script que faz `if (window.config)` ou `form.action` pode ser sequestrado por markup.
- Consequência: `id` global e "explícito" abre o vetor se o valor vier de dado (slug de notícia no CMS). Regra: `Id` é tipo com padrão fechado (`^[a-z][a-z0-9-]{0,63}$`), derivado do call site ou de enum de seção; `name` de controle é identificador do modelo de formulário, nunca texto do usuário. Nenhum script pode depender de nome global existir.
- Veredito: lacuna do corpus; correção na spec (§5.3) e teste (§5.4).

## 3. Confirmações do desenho

- **Escape por tipo é a única defesa de markup que o corpus valida implicitamente** (`05:9` define XSS por falta de encoding; `05:54` exige encoding entre as seis camadas) → P2/D1.
- **Ausência de porta de escape** é o que permite dizer "nenhuma das seis camadas depende de disciplina" (`08-seguranca.md:5-6`): com `Raw`, a camada 1 do `05:56` voltaria a depender de revisão → P3/D2.
- **Vocabulário fechado** é o que torna `frame-ancestors`, `object-src`, `base-uri` "cinto e suspensório": não há `<object>`/`<base>` na allowlist (`spec:31-33`) → D4.
- **Estilo como token** satisfaz o anti-padrão de `unsafe-inline` (`08:114`) sem precisar de nonce (`07:63` vs `docs/08-seguranca.md:82`) → D3/D12/P6.
- **JS como asset declarado** satisfaz "never host-based allowlists" (`08:115`) porque não há origem externa em `script src` (`docs/07-js-e-assets.md:127-128`) → D13.
- **Sem CDN** é estratégia de segurança citável (`06:139`) e elimina o caso de SRI cross-origin que a spec hoje não consegue cumprir (A17).
- **Bytes determinísticos** sustentam `ETag`/304 e, por consequência, permitem página cacheável sem nonce (`docs/09-desempenho-e-cache.md:22-30`) → P6/D14.
- **`JsonLd` como tipo** atende o caso que o ecossistema deixa aberto (`00-principios.md:34-42`); escape de `<` cobre `</SCRIPT`, `<!--` e `-->` de uma vez.
- **Zero `unsafe` + sem `syn`** é redução de superfície em posição privilegiada, exatamente o argumento de `06:139-140` e `09:20` ("Todo third-party tool … pode ser vetor de ataque") → D9.
- **`iframe` sem `allow`/`srcdoc` e com `sandbox` + `cross_origin_requires_sandbox`** (`spec:473,476`) confirma A32 e fecha o vetor de delegação de permissão por markup.

## 4. Conflitos e correções

| O que a crate diz hoje | O que o corpus diz | Correção (frase exata a inserir) |
|---|---|---|
| `spec/elements.toml:106` permite `integrity` em `link` sem `crossorigin`, e `spec:109` exige `preload_requires_as_and_crossorigin` | SRI exige CORS (`01:96`) | acrescentar `"crossorigin"` à allowlist de `link` e invariante `integrity_requires_crossorigin`; em `docs/08-seguranca.md` §5: "SRI sem `crossorigin` não é verificado pelo browser; o par é indivisível." |
| `spec:96` `required = ["content"]` em `meta` impede `<meta charset="utf-8">` | nenhum arquivo do corpus trata charset; `[INFERENCE]` a ausência de `charset` deixa o browser adivinhar a codificação | em `spec/elements.toml:96`, trocar por `required = ["content"]   # regra: content XOR charset`; em `docs/03` §3: "`meta` exige `content` **ou** `charset`, nunca ambos." |
| `spec:408` `rel`/`referrerpolicy` livres em `a` | `rel="noreferrer"` (`04:60`); `referrerpolicy` fraco vaza path | substituir por `rel = ["external","prev","next","noreferrer","noopener"]` e `referrerpolicy = ["strict-origin-when-cross-origin","no-referrer"]`; invariante `blank_target_implies_noopener` passa a emitir `rel="noopener noreferrer"` sozinho. |
| `spec:55` `id = "id"` global, "gerado por primitivo ou explícito" | clobbering por `id`/`name` não é coberto (`grep clobber` = 0) | em `spec/elements.toml`: `id = "id"   # Id: padrão ^[a-z][a-z0-9-]{0,63}$, derivado do call site; nunca de dado`; e em `docs/03` §6: "`id`/`name` nunca vêm de texto do usuário — DOM clobbering transforma nome em propriedade de `window`." |
| `docs/08-seguranca.md:90` lista cabeçalhos como "do consumidor", sem valores | `07:157-206` traz a lista completa com valores | substituir a linha por: "Cabeçalhos: ver tabela normativa em `18-aprendizados/web-security-review.md` §5.1; a crate assume `script-src 'self'` e `script-src-attr 'none'`." |
| `docs/08-seguranca.md:69-80` CSP sem `frame-src`, `script-src-attr`, `style-src-attr`, `manifest-src` | `07:66-69` só cobre parte; `script-src-attr`/`style-src-attr` ausentes do corpus (A13) | acrescentar após `frame-ancestors 'none';`: `frame-src 'none';` `script-src-attr 'none';` `style-src-attr 'none';` `worker-src 'self';` `manifest-src 'self';` |
| `docs/08-seguranca.md:54-61` supply chain sem SBOM/vet/assinatura | `09:47-79`, `09:109`, `09:92` | acrescentar duas linhas à tabela §5: "SBOM CycloneDX por release (`cargo cyclonedx`)" e "commits assinados + PR review + `SECURITY.md` com SLA" |
| `docs/03-vocabulario-de-elementos.md:107-113` lista 5 proibições, mas `spec:32-33` cita `docs/03 §5` para `formaction, ping, slot, shadowrootmode, part, exportparts, is, xlink:href` | vetores fora do escopo do corpus (§7), decisão própria | acrescentar as 8 linhas à tabela de `docs/03` §5 com o motivo de cada uma (tabela em §5.3) |
| `docs/08-seguranca.md:8-14` tabela de 5 vetores | corpus não enumera vetores de markup (§7) | substituir por §5.2 (12 vetores, com defesa e residual) |
| `docs/08-seguranca.md:99-109` testes de regressão sem header/upload/srcset | `07:157-206` (checklist) + `12:69-71` da crate | acrescentar 4 linhas: snapshot do header CSP, `nosniff` na rota de mídia, `srcset` com descritor hostil, clobbering de `id` |

## 5. Adições prontas para colar

### 5.1 Tabela de cabeçalhos (normativa, por resposta)

| Cabeçalho | Valor | Por quê | Dono |
|---|---|---|---|
| `Strict-Transport-Security` | `max-age=63072000; includeSubDomains; preload` | `01:21`, `14:64`; só após inventário de subdomínio | consumidor |
| `Content-Security-Policy` | `default-src 'none'; script-src 'self'; script-src-attr 'none'; style-src 'self'; style-src-attr 'none'; img-src 'self'; font-src 'self'; connect-src 'self'; frame-src 'none'; worker-src 'self'; manifest-src 'self'; object-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'; upgrade-insecure-requests` | `07:66-69` + A5/A13; `img-src` sem `data:` | consumidor (crate define o contrato) |
| `Content-Security-Policy-Report-Only` | `… ; report-to csp-endpoint` | `07:93`; 7 dias antes do enforce | consumidor |
| `Reporting-Endpoints` | `csp-endpoint="https://<host>/csp-report"` | `07:96`; endpoint trata payload como hostil (limite de tamanho, sem `eval`, sem path do usuário) | consumidor |
| `X-Content-Type-Options` | `nosniff` | `07:111`, A29 | consumidor |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | `04:46` (DEFAULT) | consumidor |
| `Permissions-Policy` | `geolocation=(), camera=(), microphone=(), payment=(), usb=(), local-network=(), loopback-network=()` | `07:135`, `10:60-61` | consumidor |
| `Cross-Origin-Resource-Policy` | `same-origin` | `01:72` | consumidor |
| `Cross-Origin-Opener-Policy` | `same-origin` | `01:84`, `01:86` | consumidor |
| `X-Frame-Options` | `DENY` | legado de `frame-ancestors` (`07:88`) | consumidor |
| `Clear-Site-Data` | `"cache", "cookies", "storage"` | `07:152`; resposta de logout | consumidor |
| `Cache-Control` | `no-store` em CMS autenticado; `public, no-cache` em HTML com formulário | `09-desempenho-e-cache.md:29-30` da crate | consumidor |
| `X-Robots-Tag` | `noindex, nofollow` em rascunho/preview | `11:36,41` | consumidor |
| CORS | allowlist estática; **nunca** eco de `Origin` com credenciais | `07:131`, `08:116` | consumidor |

### 5.2 Vetores de injeção (o que a crate impede, o que sobra)

| Vetor | Por que é injeção | Defesa na crate | Residual / regra que falta |
|---|---|---|---|
| texto/atributo | `<script>`, `" onmouseover=` | `Text`/`AttrVal` + nome de atributo estático (`08:10-11`) | nenhum |
| URL/esquema | `javascript:`, `data:`, `vbscript:`, `file:`, `java\tscript:` | `Href`/`Src` tipados, esquema normalizado e **recusado**, não sanitizado (`08:35-36`) | nunca percent-decodificar antes de validar; rejeitar `\t\n\r` e espaço |
| `srcset` diferencial | parser de browser aceita descritor e URL dentro de token | `SrcSet::from_variants` gera descritor (`docs/08:39`) | descritor **gerado**, nunca vindo do chamador |
| JSON-LD | `</SCRIPT>`, `<!--`, `-->` encerram o bloco | `JsonLd` escapa `<` `>` `&` `'` (`08:13`) | fixtures com `<!--` e `\u2028` |
| SVG inline | namespace SVG tem `<script>`, `<use href>`, `<animate>`, `<foreignObject>` | `<svg>` com filhos fechados (`spec:478-482`); sem `xlink:href` (`spec:33`) | `use`/`foreignObject` nunca entram; `phrasing` dentro de `<svg>` gera elemento SVG desconhecido — restringir a `svg_title`/`desc` |
| `meta http-equiv` | `refresh` (redirect/phishing), CSP por markup (downgrade) | `no_http_equiv` (`spec:100`) | manter; `name` com enum |
| `meta name="referrer"` | enfraquece a política da página | não tipado hoje | enum: só `strict-origin-when-cross-origin`/`no-referrer` |
| DOM clobbering | `id`/`name` viram `window.X`/`form.X` | `id` "gerado por primitivo" (`spec:55`) | tipo `Id` com padrão; `name` de controle = identificador de modelo |
| `target="_blank"` | reverse tabnabbing; nome de janela reutilizado | `blank_target_implies_noopener` (`spec:412`) | `target` como enum `only _blank`, e emitir `rel="noopener noreferrer"` |
| `formaction`/`ping` | sobrescreve destino do form; beacon cross-site | ausentes (impossível emitir) | manter proibidos (A39, §5.3) |
| `srcdoc` | HTML como string aninhada | ausente (`docs/03-vocabulario-de-elementos.md:111`) | manter |
| upload servido como HTML | mesma origem + tipo interpretável | `Src::media(ObjKey)` opaco | `nosniff` + `Content-Disposition` + host separado (A35) |

### 5.3 TOML pronto para `spec/elements.toml`

```toml
# [globals] — restringir o que hoje é livre
id = "id"                 # Id: ^[a-z][a-z0-9-]{0,63}$; derivado do call site ou de enum; nunca de dado
# hidden = ["", "until-found"]  # manter, mas `until-found` NUNCA esconde segredo:
#                               # conteúdo continua no DOM e aparece em Ctrl+F
```

```toml
# link — fechar SRI
allowed = ["as", "type", "media", "integrity", "crossorigin", "hreflang", "sizes", "fetchpriority", "disabled"]
invariants = ["link_rel_enum", "preload_requires_as_and_crossorigin", "integrity_requires_crossorigin"]
```

```toml
# a — target/rel/referrerpolicy como enum
referrerpolicy = ["strict-origin-when-cross-origin", "no-referrer"]
rel = ["external", "prev", "next", "noopener", "noreferrer"]
# invariantes: blank_target_implies_noopener emite rel="noopener noreferrer";
#              rel="opener" e referrerpolicy fraca não existem no vocabulário
```

```toml
# meta — charset sem content; name fechado
required = ["content"]         # regra: content XOR charset
name = ["description", "robots", "referrer", "theme-color", "color-scheme"]
# robots: ["noindex", "nofollow", "noarchive"]; referrer: só políticas estritas
```

Proibições a acrescentar em `docs/03-vocabulario-de-elementos.md` §5 (com o motivo):

| Atributo | Motivo |
|---|---|
| `formaction` | sobrescreve o destino do `<form>` — o `action` tipado deixa de valer |
| `ping` | envia beacon cross-site sem interação; vazamento de navegação |
| `slot=`, `part=`, `exportparts`, `is=` | custom element/shadow DOM: nome e comportamento fora do vocabulário |
| `shadowrootmode` | declarative shadow DOM: documento dentro do documento, fora do escape |
| `xlink:href` | URL de SVG sem passar por `Href` |
| `nonce` | só faz sentido com inline; a crate não tem |

### 5.4 Checklist de regressão a acrescentar em `08-seguranca.md` §8

- snapshot do header CSP completo (falha se qualquer diretiva esperada faltar);
- `nosniff` presente na rota que serve `Src::media`;
- fixture de `srcset` com descritor hostil → erro de tipo, não parse divergente;
- fixture com `<!--` e `-->` dentro de `JsonLd` → JSON parseável no N4;
- fixture com `id`/`name` vindos de dado → erro de macro (padrão de `Id`);
- `trybuild`: `a(target: "_blank")` emite `rel="noopener noreferrer"` no golden.

## 6. Armadilhas de complexidade

1. **Nonce/hash CSP.** Exige gerador por request, mata `ETag` (P6) e obriga `no-store` — troca cache por uma garantia que `'self'` já dá sem inline. Custo: ~150 LOC + um modo de falha novo.
2. **`'strict-dynamic'`.** Só tem função com loader dinâmico; sem ele, é compatibilidade reduzida e a sugestão falsa de que existe criação de script em runtime.
3. **Entregar Trusted Types pela crate.** Política e sinks são do JS do consumidor; qualquer API aqui obriga dependência em `html-core` e quebra P5.
4. **Política `default` de Trusted Types.** Sanitização invisível: erro de programação vira comportamento silencioso, e o sanitizador (terceiro) herda o privilégio de toda a página (`modern:603-604`).
5. **Sanitizador de HTML dentro da crate.** DOMPurify-equivalente em Rust é projeto próprio (`ammonia` puxa `html5ever`), e existence de sanitizador reabre a porta que D2 fechou. Conteúdo rich-text é renderizado nó-a-nó no vocabulário.
6. **Endpoint de CSP report na crate.** Recebe payload arbitrário do browser por request: precisa de quota, rotação, storage e proteção de path — infraestrutura, não markup.
7. **Parser/avaliador de CSP no lint.** A crate não emite header; validar política é trabalho do `csp-evaluator`/proxy, e um parser de CSP é superfície nova para um ganho zero.
8. **Allowlist de origem externa configurável em runtime.** Configuração que aceita string de origem é `attrs()` com outro nome; a allowlist é dado versionado (D5).
9. **SRI calculado em build para asset local.** Asset já tem hash no nome (`docs/07-js-e-assets.md:110-117`); somar `integrity` local é byte extra e mais um lugar para divergir. SRI só para asset de terceiro.
10. **`target` livre e `rel` livre.** Duas strings que parecem inofensivas e reabrem tabnabbing e vazamento de referrer (A30).
11. **`id` como `&str`.** DOM clobbering entra pela porta do slug de conteúdo e não aparece em nenhum teste de escape.
12. **`hidden="until-found"` como controle de acesso.** Conteúdo permanece no DOM, legível e buscável; autorização é servidor.
13. **CSRF/sessão/upload na crate.** Repetiria o erro de framework; `00-principios.md:87` já fecha.

## 7. Lacunas do corpus

1. **Enumeração de vetores de markup.** SVG inline, `meta http-equiv`, `formaction`, `ping`, `srcdoc`, `xlink:href`, `use`, clobbering por `id`/`name`, `srcset` diferencial e terminadores de JSON-LD: `grep` nos 21 arquivos devolve **zero** ocorrências para `clobber`, `formaction`, `ping`, `srcset`, `svg`, `srcdoc`. A §1 de `08-seguranca.md` não pode se apoiar neste conjunto.
2. **Diretivas CSP por tipo de sink.** `script-src-attr`, `style-src-attr`, `frame-src`, `worker-src`, `manifest-src` e o próprio `trusted-types` não aparecem; a lista de §5.1 é `[INFERENCE]` a partir do vocabulário fechado.
3. **CSP entregue por `meta`.** O corpus usa (`modern:596`) sem discutir quais diretivas são ignoradas nesse canal — a crate proíbe, mas o motivo escrito precisa ser dela.
4. **Servir upload com segurança.** Uma linha (`13:65`); falta `Content-Disposition`, `Content-Type` derivado de sniff, host separado e o caso "imagem com polyglot".
5. **`enctype` de formulário.** `spec:517` permite `enctype`; o corpus trata upload (`13:57-72`) sem dizer que `text/plain` em endpoint JSON é vetor de confusão de tipo.
6. **`Robots` em CMS multitenant.** `11` cobre site único; o CMS do município publica rascunho com URL previsível — falta a regra "rascunho nunca responde 200 indexável".
7. **Cadeia de suprimentos Rust.** `cargo vet`, `advisory-db`, yanked, `unmaintained` e verificação de vendor não existem no corpus (que é npm/`npm ci`); `08-seguranca.md` §5 precisa decidir sozinho.
8. **Assinatura de commit com chave SSH/sigstore.** `09:29` só cita "commits assinados".
9. **`Integrity-Policy`/`integrity` para fetch de JS de terceiro** e `Subresource Integrity` para `modulepreload`: ausentes.
10. **Cache de resposta autenticada.** `09-desempenho-e-cache.md` da crate decide; o corpus só fala de `Clear-Site-Data`.
11. **Superfície de fingerprinting da própria crate.** `04` §5 lista defesas do browser; nenhuma palavra sobre `sizes`/`srcset`/fontes do município como fingerprint. `[INFERENCE]` risco baixo, mas a decisão é nossa.
12. **E-mail/AMP.** O corpus é só web; `18-aprendizados.md` já registra perfil de e-mail próprio — segurança de e-mail (cliente que não honra CSP) fica fora.
