#!/usr/bin/env python3
"""Validador de referência de spec/elements.toml (esquema 2).

Este é o protótipo do comando `cq spec` descrito em docs/01-arquitetura.md.
Enquanto o crate `html-spec` não existe, ele é a única barreira contra a spec apodrecer.

Uso:  python3 tools/validate_spec.py [caminho]
Saída: uma linha por problema, prefixada por ERROR/WARN, e um resumo final.
Código de saída: 1 se houver ERROR.
"""
from __future__ import annotations

import difflib
import re
import sys
import tomllib
from pathlib import Path

CATEGORIES = {
    "flow", "phrasing", "sectioning", "heading", "form",
    "table", "media", "metadata", "interactive", "text", "none", "json", "svg_shape",
}
ACCESSIBLE_NAME_KINDS = {
    "required", "optional", "none",
    "alt", "title", "label", "legend", "caption", "summary",
}
GLOBALS = {"class", "id", "data-*", "aria-*", "lang", "dir", "hidden",
           "translate", "spellcheck", "inert", "style", "tabindex"}
# atributos cujo NOME coincide com nome de elemento (o validador não deve confundir)
KNOWN_ATTRS = {
    "title", "label", "cite", "datetime", "for", "scope", "href", "src", "alt", "value", "name",
    "id", "type", "content", "rel", "kind", "srclang", "method", "action", "closedby", "open",
    "span", "start", "reversed", "as", "media", "sizes", "srcset", "width", "height", "loading",
    "decoding", "fetchpriority", "poster", "controls", "preload", "muted", "loop", "playsinline",
    "referrerpolicy", "sandbox", "viewbox", "focusable", "colspan", "rowspan", "headers", "rows",
    "cols", "wrap", "dirname", "minlength", "maxlength", "pattern", "min", "max", "step", "size",
    "checked", "multiple", "accept", "list", "selected", "disabled", "readonly", "required",
    "placeholder", "autocomplete", "inputmode", "enterkeyhint", "spellcheck", "translate",
    "hidden", "inert", "lang", "dir", "low", "high", "optimum", "default", "form",
    "popovertarget", "popovertargetaction", "enctype", "novalidate", "integrity", "crossorigin",
    "defer", "async", "hreflang", "target", "download", "property", "charset", "tabindex",
    "viewBox", "preserveAspectRatio", "fill", "stroke", "stroke-width",
    "stroke-linecap", "stroke-linejoin", "d", "cx", "cy", "r", "x", "y", "x1", "y1", "x2", "y2",
    "rx", "ry", "points", "transform",
}
FORBIDDEN_ATTRS = {
    "style", "srcdoc", "formaction", "ping", "slot", "part", "exportparts",
    "shadowrootmode", "is", "xlink:href", "accesskey", "draggable",
}
# elementos que o vocabulário nunca deve conter (docs/03 §5, docs/08 §2)
FORBIDDEN_ELEMENTS = {
    "style", "template", "slot", "iframe-srcdoc", "object", "embed", "map",
    "area", "fencedframe", "canvas", "base",
}

problems: list[str] = []
warns: list[str] = []


def err(msg: str) -> None:
    problems.append(f"ERROR {msg}")


def warn(msg: str) -> None:
    warns.append(f"WARN  {msg}")


def check_doc_counts(data: dict) -> None:
    """Números declarados nos docs têm que bater com o spec (o drift clássico: "63 elementos")."""
    root = Path(__file__).resolve().parent.parent
    t3 = (root / "docs" / "03-vocabulario-de-elementos.md").read_text(encoding="utf-8")
    m = re.search(r"declara \*\*(\d+)\*\*", t3)
    if not m:
        err("docs/03: contagem declarada de elementos não encontrada")
    elif int(m.group(1)) != len(data["element"]):
        err(f"docs/03: declara **{m.group(1)}** elementos; o spec tem {len(data['element'])}")
    t4 = (root / "docs" / "04-validacao.md").read_text(encoding="utf-8")
    rows = {line.split("|")[1].strip().strip("`"): line
            for line in t4.splitlines() if line.startswith("| `") and "**" in line}
    for scope in ("block", "type", "output"):
        row = rows.get(scope, "")
        nums = re.findall(r"\*\*(\d+)\*\*", row)
        if not nums:
            err(f"docs/04 §4.0: linha do escopo `{scope}` sem contagem")
            continue
        real = sum(1 for v in data["invariants"].values() if v == scope)
        if int(nums[-1]) != real:
            err(f"docs/04 §4.0: declara {nums[-1]} `{scope}`; o spec tem {real}")
    if not any(v in ("block", "type", "output") for v in data["invariants"].values()):
        err("[invariants] vazio: nenhum escopo declarado")


def main(path: str) -> int:
    data = tomllib.loads(Path(path).read_text(encoding="utf-8"))
    els = data.get("element", {})
    schema = data.get("schema")
    meta = data.get("meta", {})

    if schema != 2:
        err(f"schema={schema!r}; este validador implementa o esquema 2")
    if not meta.get("spec_version"):
        err("[meta] sem spec_version (obrigatório: congelamento por minor)")
    if not els:
        err("nenhum [element.*] declarado")
        return report()

    names = set(els)

    # `aria-*` é tabela de enums, não lista de nomes
    aria = data.get("globals", {}).get("aria")
    if not isinstance(aria, dict):
        err("[globals.aria] deve ser tabela (atributo = enum de valores), não lista")
    else:
        for k, v in aria.items():
            if not k.startswith("aria-"):
                err(f"[globals.aria] chave `{k}` não é atributo aria-*")
            if not (isinstance(v, (list, str))):
                err(f"[globals.aria] `{k}`: valor deve ser enum (lista) ou tipo de referência")

    # toda invariante usada declara escopo, com dono explícito:
    # block = macro (erro de compilação) · type = construtor do crate (Page, Form) · output = N4
    declared_inv = data.get("invariants", {})
    spec_inv_names: set[str] = set()
    for name, spec in els.items():
        for i in spec.get("invariants", []):
            spec_inv_names.add(i)
    for i in sorted(spec_inv_names - set(declared_inv)):
        err(f"invariante `{i}` usada mas sem escopo declarado em [invariants]")
    for i, sc in sorted(declared_inv.items()):
        if sc not in ("block", "type", "output"):
            err(f"[invariants] `{i}`: escopo `{sc}` inválido (block|document|page)")
        if i not in spec_inv_names:
            warn(f"[invariants] `{i}` declarada e não usada por nenhum elemento")

    # categorias vêm do [meta], não da imaginação do autor
    decl_cats = set(meta.get("categories", []))
    if not decl_cats:
        err("[meta] categories ausente")
    for cat in sorted(decl_cats - CATEGORIES):
        err(f"[meta] categoria desconhecida `{cat}`")
    for cat in sorted(CATEGORIES - decl_cats - {"none"}):
        warn(f"categoria `{cat}` conhecida pelo validador mas não declarada em [meta]")

    # atributos booleanos: declarados em [meta] e usados com tipo bool (D19)
    booleans = set(meta.get("boolean_attributes", []))
    if not booleans:
        err("[meta] boolean_attributes ausente")
    used_bool: set[str] = set()
    for name, spec in els.items():
        for attr in list(spec.get("required", [])) + list(spec.get("allowed", [])):
            if attr in booleans:
                used_bool.add(attr)
    for attr in sorted(used_bool - booleans):
        err(f"atributo booleano `{attr}` usado sem estar em [meta] boolean_attributes")
    for attr in sorted(booleans - used_bool):
        warn(f"[meta] declara `{attr}` como booleano, mas nenhum elemento o admite")

    # nome de elemento: sem hífen (custom element), minúsculo, sem espaço
    for name in names:
        if "-" in name:
            err(f"element `{name}`: nome com hífen (custom element é proibido, docs/14 §lit)")
        if name != name.lower() or " " in name:
            err(f"element `{name}`: nome inválido")
        if name in FORBIDDEN_ELEMENTS:
            err(f"element `{name}`: elemento proibido pelo vocabulário")

    for name, spec in els.items():
        cats = spec.get("category")
        cats = [cats] if isinstance(cats, str) else cats
        if not cats:
            err(f"element `{name}`: sem category")
        for c in cats or []:
            if c not in CATEGORIES:
                err(f"element `{name}`: category desconhecida `{c}`")

        void = bool(spec.get("void"))
        required = spec.get("required", [])
        allowed = spec.get("allowed", [])
        children = spec.get("children")
        children = [children] if isinstance(children, str) else (children or [])
        req_children = spec.get("required_children", [])
        first_child = spec.get("first_child")
        accessible = spec.get("accessible_name")
        forbidden_children = spec.get("forbidden_children", [])

        # 1. required são atributos, nunca filhos nem nomes de elemento
        for attr in required:
            if attr in names and attr not in KNOWN_ATTRS:
                err(f"element `{name}`: `required` contém o ELEMENTO `{attr}` "
                    f"(use required_children)")
            if attr in GLOBALS and attr not in ("lang", "id"):
                warn(f"element `{name}`: `{attr}` é global; não precisa estar em required")
            if attr in FORBIDDEN_ATTRS:
                err(f"element `{name}`: required contém atributo proibido `{attr}`")

        # 2. allowlist não repete required (implicitamente permitido) e não aceita lixo
        for attr in list(required) + list(allowed):
            if attr in ("data-*", "aria-*") or attr.startswith(("data-", "aria-")) :
                continue          # chave declarada pelo consumidor (docs/03 §6)
            if attr in GLOBALS or attr in KNOWN_ATTRS:
                continue
            if "-" in attr or attr != attr.lower():
                continue          # atributo com hífen/camelCase (SVG) é declarado, não validado aqui
            near = difflib.get_close_matches(attr, KNOWN_ATTRS | GLOBALS, n=2)
            hint = f" — você quis {', '.join(near)}?" if near else ""
            err(f"element `{name}`: atributo `{attr}` não existe no vocabulário HTML{hint}")

        for attr in allowed:
            if attr in required:
                warn(f"element `{name}`: `{attr}` está em required e em allowed (redundante)")
            if attr in FORBIDDEN_ATTRS:
                err(f"element `{name}`: allowed contém atributo proibido `{attr}`")
            if attr.startswith("on"):
                err(f"element `{name}`: allowed contém handler `{attr}`")
            base = attr.split("-")[0]
            svg_ctx = "svg_shape" in (cats or []) or name in {"svg", "svg_title", "desc", "use"}
            if "-" in attr and attr not in GLOBALS and base not in ("aria", "data") and not svg_ctx:
                warn(f"element `{name}`: `{attr}` tem hífen e não é aria-*/data-*")

        if "role" in allowed:
            err(f"element `{name}`: `role` livre (use `roles = [...]` — D4)")

        roles = spec.get("roles", [])
        for r in roles:
            if not re.fullmatch(r"[a-z][a-z-]*", r):
                err(f"element `{name}`: papel ARIA inválido `{r}`")

        # 3. accessible_name
        if accessible is None:
            warn(f"element `{name}`: sem accessible_name declarado")
        elif accessible not in ACCESSIBLE_NAME_KINDS:
            err(f"element `{name}`: accessible_name inválido `{accessible}`")

        # 4. conteúdo-modelo
        for c in children:
            if c not in CATEGORIES and c not in names:
                err(f"element `{name}`: children contém `{c}` (nem categoria nem elemento)")
            elif c in names and c in FORBIDDEN_ELEMENTS:
                err(f"element `{name}`: children inclui elemento proibido `{c}`")
        def child_ok(child: str) -> bool:
            """Um filho casa se estiver listado por nome OU por qualquer categoria dele."""
            if child in children:
                return True
            child_cats = els.get(child, {}).get("category")
            child_cats = [child_cats] if isinstance(child_cats, str) else (child_cats or [])
            return any(c in children for c in child_cats)

        for c in req_children:
            if c not in names:
                err(f"element `{name}`: required_children contém `{c}`, que não é elemento")
            elif not child_ok(c):
                err(f"element `{name}`: required_children `{c}` não casa com children {children}")
        if first_child and not child_ok(first_child):
            err(f"element `{name}`: first_child `{first_child}` não casa com children {children}")
        for c in forbidden_children:
            if c not in CATEGORIES and c not in names:
                err(f"element `{name}`: forbidden_children contém `{c}` inválido")

        # 5. void não tem filhos
        if void and (children and children != ["none"]):
            err(f"element `{name}`: void não pode ter children ({children})")
        if void and (req_children or first_child):
            err(f"element `{name}`: void não pode ter required_children/first_child")
        if not void and "none" in children:
            err(f"element `{name}`: children=['none'] em elemento não-void")

        # 6. invariantes declaradas precisam existir em docs/04 (checagem leve: nome kebab/snake)
        for inv in spec.get("invariants", []):
            if not inv.replace("_", "").isalnum():
                err(f"element `{name}`: invariante `{inv}` fora do padrão snake_case")

    # 7. rastreabilidade: toda invariante compilada está documentada em docs/04
    spec_inv: dict[str, list[str]] = {}
    for name, spec in els.items():
        for inv in spec.get("invariants", []):
            spec_inv.setdefault(inv, []).append(name)

    doc = Path(__file__).resolve().parent.parent / "docs" / "04-validacao.md"
    if not doc.exists():
        err("docs/04-validacao.md ausente: invariantes sem registro normativo")
    else:
        text = doc.read_text(encoding="utf-8")
        documented = set(re.findall(r"`([a-z][a-z0-9]*(?:_[a-z0-9]+)+)`", text)) - names
        for inv, owners in sorted(spec_inv.items()):
            if inv not in documented:
                err(f"invariante `{inv}` (usada em {', '.join(owners)}) não está documentada "
                    f"em docs/04-validacao.md §4.1")
        for inv in sorted(documented - set(spec_inv)):
            if inv.count("_") >= 1 and inv.split("_")[0] in {
                "alt", "href", "label", "no", "main", "single", "banner", "shell", "li", "type",
                "method", "scope", "caption", "tbody", "details", "closedby", "dialog", "popover",
                "media", "srclang", "source", "cross", "preload", "link", "meta", "script",
                "document", "datetime", "preserve", "svg", "section", "landmark", "for", "pattern",
                "attribute", "autocomplete", "srcset", "fold",
            }:
                warn(f"docs/04 documenta `{inv}`, mas nenhum elemento a declara em invariants")

    # `only`: elemento que não se escreve à mão (documento → Page; formulário → Form::open)
    for name, e in els.items():
        only = e.get("only")
        if only is None:
            continue
        if only not in ("shell", "component"):
            err(f"element `{name}`: `only = \"{only}\"` inválido (shell|component)")
            continue
        need = "shell_only_elements" if only == "shell" else "form_requires_csrf"
        invs = e.get("invariants", [])
        invs = [invs] if isinstance(invs, str) else invs
        if need not in invs:
            err(f"element `{name}`: `only = \"{only}\"` sem a invariante `{need}` em invariants")

    # 8. coerências globais entre elementos
    if "html" in els:
        ch = els["html"].get("children", [])
        ch = [ch] if isinstance(ch, str) else ch
        for need in ("head", "body"):
            cats = els.get(need, {}).get("category")
            cats = [cats] if isinstance(cats, str) else (cats or [])
            if need not in ch and not any(c in ch for c in cats):
                err(f"element `html`: children {ch} não alcança `{need}`")
    for name in ("head", "body", "main", "html"):
        if name not in els:
            err(f"elemento estrutural `{name}` ausente")

    check_doc_counts(data)
    return report()


def report() -> int:
    for line in problems:
        print(line)
    for line in warns:
        print(line)
    print(f"\n{len(problems)} erro(s), {len(warns)} aviso(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    default = Path(__file__).resolve().parent.parent / "spec" / "elements.toml"
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else str(default)))
