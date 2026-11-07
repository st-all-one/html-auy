#!/usr/bin/env python3
"""Gerador da tabela Rust do spec (D27).

`spec/elements.toml` é normativo; `crates/html/src/spec/table.rs` é GERADO por este script e
VERSIONADO. Não há `build.rs`: esquecer de rodar o gerador deixa o teste de paridade vermelho.

A saída é formatada na forma canônica do `rustfmt` (largura 100): arrays inline quando cabem na
linha, verticais quando não cabem. É o que permite `--check` comparar byte a byte sem chamar
ferramenta externa.

Uso:
    python3 tools/gen_spec.py            # escreve crates/html/src/spec/table.rs
    python3 tools/gen_spec.py --check    # regenera em memória e compara byte a byte
    python3 tools/gen_spec.py --stdout   # imprime sem escrever

Saída: 0 ok · 1 divergência/erro de spec. Só `tomllib` da stdlib.
"""
from __future__ import annotations

import difflib
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "spec" / "elements.toml"
TABLE = ROOT / "crates" / "html" / "src" / "spec" / "table.rs"
WIDTH = 100
# `array_width` do rustfmt: o corpo do array acima disso nunca fica inline.
ARRAY_WIDTH = 60

# Nome do elemento no spec → tag emitida. Só estes dois divergem (namespace do gerador).
TAG_OVERRIDES = {"svg_title": "title", "media_source": "source"}

# `[globals]`/`[globals.aria]`: string que descreve o tipo do valor, quando não é enum (lista).
GLOBAL_KINDS = {
    "bcp47": "Bcp47",
    "tokens": "Tokens",
    "id": "Id",
    "declared": "Declared",
    "presence": "Presence",
    "text": "Text",
}
ARIA_KINDS = {"idref": "IdRef", "idref-list": "IdRefList", "text": "Text"}

# Atributos cujo literal é lista de tokens (validados um a um, sem reordenar).
TOKEN_ATTRS = {"rel", "sandbox"}
# Atributos cujo literal não é enum nem token, e sim uma gramática.
GRAMMAR_ATTRS = {
    ("source", "type"): "Mime",
    ("media_source", "type"): "Mime",
    ("a", "hreflang"): "Bcp47",
    ("track", "srclang"): "Bcp47",
}


class SpecError(Exception):
    """Spec inválida para gerar a tabela."""


def rust_str(value: str) -> str:
    """Literal Rust para o ASCII do spec (nomes de elemento/atributo/valor)."""
    out = ['"']
    for ch in value:
        if ch == "\\":
            out.append("\\\\")
        elif ch == '"':
            out.append('\\"')
        elif ord(ch) < 0x20:
            out.append(f"\\u{{{ord(ch):x}}}")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def as_list(value) -> list[str]:
    if value is None:
        return []
    if isinstance(value, str):
        return [value]
    return list(value)


def inline_arr(items: list[str]) -> str:
    return "&[" + ", ".join(rust_str(item) for item in items) + "]"


def fits(used: int) -> bool:
    """Uma linha `used` colunas mais a vírgula final cabe no `max_width` do rustfmt."""
    return used + 1 <= WIDTH


def vertical_arr(items: list[str], indent: int) -> str:
    """Array que não cabe numa linha, no formato do rustfmt: preenche a linha quando todos os
    itens são curtos (`short_array_element_width_threshold` = 10), senão um por linha."""
    pad = " " * (indent + 4)
    if all(len(rust_str(item)) <= 10 for item in items):
        rows: list[str] = []
        row = ""
        for item in items:
            token = rust_str(item) + ","
            candidate = token if not row else row + " " + token
            if len(pad) + len(candidate) <= WIDTH:
                row = candidate
            else:
                rows.append(row)
                row = token
        rows.append(row)
        body = "".join(f"{pad}{line}\n" for line in rows)
    else:
        body = "".join(f"{pad}{rust_str(item)},\n" for item in items)
    return "&[\n" + body + " " * indent + "]"


def arr(items: list[str], indent: int, prefix: int) -> str:
    """Array de literais no formato canônico do rustfmt."""
    if not items:
        return "&[]"
    inline = inline_arr(items)
    body = inline[2:-1]
    if len(body) <= ARRAY_WIDTH and fits(indent + prefix + len(inline)):
        return inline
    return vertical_arr(items, indent)


def entry(indent: int, name: str, value) -> str:
    pad = " " * indent
    if isinstance(value, list):
        return f"{pad}{name}: {arr(value, indent, len(name) + 2)},\n"
    return f"{pad}{name}: {value},\n"


def element_literal(name: str, spec: dict) -> str:
    cats = sorted(as_list(spec.get("category")))
    if not cats:
        raise SpecError(f"element `{name}`: sem category")
    only = spec.get("only")
    if only not in (None, "shell", "component"):
        raise SpecError(f"element `{name}`: only={only!r} inválido")
    only_src = "None" if only is None else f"Some(Only::{only.capitalize()})"
    order = as_list(spec.get("order"))
    permitted = set(as_list(spec.get("required"))) | set(as_list(spec.get("allowed")))
    seen: set[str] = set()
    for attr in order:
        if attr in seen:
            raise SpecError(f"element `{name}`: order repete `{attr}`")
        seen.add(attr)
        if attr not in permitted:
            raise SpecError(f"element `{name}`: order contém `{attr}` fora de required ∪ allowed")

    parts = ["    Element {\n"]
    parts.append(entry(8, "name", rust_str(name)))
    parts.append(entry(8, "tag", rust_str(TAG_OVERRIDES.get(name, name))))
    parts.append(entry(8, "categories", cats))
    parts.append(entry(8, "required", as_list(spec.get("required"))))
    parts.append(entry(8, "allowed", as_list(spec.get("allowed"))))
    parts.append(entry(8, "order", order))
    parts.append(entry(8, "children", sorted(as_list(spec.get("children")))))
    parts.append(entry(8, "required_children", sorted(as_list(spec.get("required_children")))))
    parts.append(entry(8, "first_child", rust_str(spec.get("first_child") or "")))
    parts.append(entry(8, "forbidden_children", sorted(as_list(spec.get("forbidden_children")))))
    parts.append(entry(8, "transparent", "true" if spec.get("transparent") else "false"))
    parts.append(entry(8, "only", only_src))
    parts.append(entry(8, "void", "true" if spec.get("void") else "false"))
    parts.append(entry(8, "accessible_name", rust_str(spec.get("accessible_name") or "none")))
    parts.append(entry(8, "roles", sorted(as_list(spec.get("roles")))))
    parts.append(entry(8, "invariants", sorted(as_list(spec.get("invariants")))))
    parts.append("    },\n")
    return "".join(parts)


def attr_value_kind(element: str, attr: str) -> str:
    if attr in TOKEN_ATTRS:
        return "Tokens"
    stated = GRAMMAR_ATTRS.get((element, attr))
    if stated:
        return stated
    return "Enum"


def global_kind(value) -> tuple[str, list[str]]:
    if isinstance(value, list):
        return "Enum", value
    kind = GLOBAL_KINDS.get(value)
    if kind is None:
        raise SpecError(f"[globals] valor {value!r} desconhecido")
    return kind, []


def aria_kind(value) -> tuple[str, list[str]]:
    if isinstance(value, list):
        return "Enum", value
    kind = ARIA_KINDS.get(value)
    if kind is None:
        raise SpecError(f"[globals.aria] valor {value!r} desconhecido")
    return kind, []


def value_struct(kind_name: str, name: str, kind: str, values: list[str]) -> str:
    return (
        f"    {kind_name} {{\n"
        + entry(8, "name", rust_str(name))
        + entry(8, "kind", f"ValueKind::{kind}")
        + entry(8, "values", values)
        + "    },\n"
    )


def input_attrs_entry(input_type: str, attrs: list[str]) -> str:
    attrs = sorted(as_list(attrs))
    if not attrs:
        return f"    ({rust_str(input_type)}, &[]),\n"
    inline = f"({rust_str(input_type)}, {inline_arr(attrs)})"
    if fits(4 + len(inline)):
        return f"    {inline},\n"
    return f"    (\n        {rust_str(input_type)},\n        {arr(attrs, 8, 0)},\n    ),\n"


def generate(data: dict) -> str:
    els = data.get("element") or {}
    if not els:
        raise SpecError("nenhum [element.*] declarado")

    out: list[str] = [
        "// GERADO por tools/gen_spec.py — NÃO EDITE. Fonte: spec/elements.toml\n",
        "//\n",
        "// Dois crates incluem este arquivo por `#[path]` (o crate `html` e o `html-macros`),\n",
        "// por isso ele não tem `use` de nada: só `core` implícito.\n",
        "#![allow(\n",
        "    dead_code,\n",
        "    missing_docs,\n",
        "    unreachable_pub,\n",
        '    reason = "tabela gerada; docs em spec/elements.toml"\n',
        ")]\n",
        "\n",
        "/// Escopo de uma invariante: quem cobra a regra (docs/04 §4.0).\n",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n",
        "pub enum Scope {\n",
        "    /// Decidido dentro de um bloco `html!`: erro de compilação com span.\n",
        "    Block,\n",
        "    /// Decidido pelo construtor do crate (`Page`, `Form`): não compila sem o dado.\n",
        "    Type,\n",
        "    /// Só o HTML final responde (`audit` sobre a saída).\n",
        "    Output,\n",
        "}\n\n",
        "/// Quem emite o elemento: o shell (`Page`) ou um componente do crate (`Form`).\n",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n",
        "pub enum Only {\n",
        "    /// `html!` recusa: só o `Page` emite.\n",
        "    Shell,\n",
        "    /// `html!` recusa: só o primitivo do crate emite.\n",
        "    Component,\n",
        "}\n\n",
        "/// Tipo do valor de um atributo global ou `aria-*`.\n",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n",
        "pub enum NameKind {\n",
        "    /// Obrigatório no elemento.\n",
        "    Required,\n",
        "    /// Permitido, não obrigatório.\n",
        "    Optional,\n",
        "    /// Não se aplica ao elemento.\n",
        "    None,\n",
        "    /// Atributo global (aceito em qualquer elemento).\n",
        "    Attr,\n",
        "}\n\n",
        "/// Tipo do valor aceito: enum fechado ou gramática.\n",
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n",
        "pub enum ValueKind {\n",
        "    /// Lista fechada de literais.\n",
        "    Enum,\n",
        "    /// Identificador (`Id`).\n",
        "    Id,\n",
        "    /// Referência a um id (uma só).\n",
        "    IdRef,\n",
        "    /// Lista de referências a id.\n",
        "    IdRefList,\n",
        "    /// Texto livre, escapado.\n",
        "    Text,\n",
        "    /// Lista de tokens separados por espaço.\n",
        "    Tokens,\n",
        "    /// `data-*`: o consumidor declara.\n",
        "    Declared,\n",
        "    /// Etiqueta BCP-47 (`Lang`).\n",
        "    Bcp47,\n",
        "    /// Tipo MIME (`tipo/subtipo`).\n",
        "    Mime,\n",
        "    /// Atributo de presença (booleano).\n",
        "    Presence,\n",
        "}\n\n",
        "/// Um elemento do vocabulário.\n",
        "#[derive(Debug, PartialEq, Eq)]\n",
        "pub struct Element {\n",
        "    /// Nome no `html!`.\n",
        "    pub name: &'static str,\n",
        "    /// Tag emitida (difere do nome em `svg_title` e `media_source`).\n",
        "    pub tag: &'static str,\n",
        "    /// Categorias de conteúdo-modelo a que o elemento pertence.\n",
        "    pub categories: &'static [&'static str],\n",
        "    /// Atributos obrigatórios (implicitamente permitidos).\n",
        "    pub required: &'static [&'static str],\n",
        "    /// Allowlist adicional.\n",
        "    pub allowed: &'static [&'static str],\n",
        "    /// Ordem de emissão quando difere de `required ++ allowed ++ globals ++ aria-*`.\n",
        "    pub order: &'static [&'static str],\n",
        "    /// Categorias e/ou nomes de filho permitidos.\n",
        "    pub children: &'static [&'static str],\n",
        "    /// Filhos sem os quais o bloco não compila.\n",
        "    pub required_children: &'static [&'static str],\n",
        "    /// Filho que precisa vir primeiro (`\"\"` = nenhum).\n",
        "    pub first_child: &'static str,\n",
        "    /// Exceções sobre `children`.\n",
        "    pub forbidden_children: &'static [&'static str],\n",
        "    /// Herda o content model do pai (`<a>`): `children` só vale na raiz.\n",
        "    pub transparent: bool,\n",
        "    /// Quem pode emitir o elemento.\n",
        "    pub only: Option<Only>,\n",
        "    /// Sem filhos e sem fechamento.\n",
        "    pub void: bool,\n",
        "    /// Espécie de nome acessível exigido.\n",
        "    pub accessible_name: &'static str,\n",
        "    /// Papéis ARIA aceitos (`role=`); vazio = `role` não existe no elemento.\n",
        "    pub roles: &'static [&'static str],\n",
        "    /// Invariantes que o elemento carrega (docs/04 §4.1).\n",
        "    pub invariants: &'static [&'static str],\n",
        "}\n\n",
        "/// Os elementos do vocabulário, ordenados por nome (`element` procura aqui).\n",
        "pub static ELEMENTS: &[Element] = &[\n",
    ]
    for name in sorted(els):
        out.append(element_literal(name, els[name]))
    out.append("];\n\n")

    invariants = data.get("invariants") or {}
    for inv, scope in sorted(invariants.items()):
        if scope not in ("block", "type", "output"):
            raise SpecError(f"[invariants] `{inv}`: escopo {scope!r} inválido")
    booleans = sorted(as_list((data.get("meta") or {}).get("boolean_attributes")))
    out.append("/// Atributos cujo valor é presença (`disabled`), nunca `=\"false\"`.\n")
    out.append("pub static BOOLEAN_ATTRIBUTES: &[&str] = ")
    out.append(arr(booleans, 0, 0))
    out.append(";\n\n")

    out.append("/// Escopo declarado de cada invariante, ordenado por nome.\n")
    out.append("pub static INVARIANT_SCOPES: &[(&str, Scope)] = &[\n")
    for inv, scope in sorted(invariants.items()):
        out.append(f"    ({rust_str(inv)}, Scope::{scope.capitalize()}),\n")
    out.append("];\n\n")

    globals_tbl = data.get("globals") or {}
    aria_tbl = globals_tbl.get("aria") or {}
    out.append("/// Atributo global e o tipo do valor; ORDEM DE EMISSÃO (`order` implícito).\n")
    out.append("#[derive(Debug, PartialEq, Eq)]\n")
    out.append("pub struct GlobalValue {\n")
    out.append("    /// Nome do atributo.\n")
    out.append("    pub name: &'static str,\n")
    out.append("    /// Tipo do valor.\n")
    out.append("    pub kind: ValueKind,\n")
    out.append("    /// Valores aceitos quando o tipo é `Enum`.\n")
    out.append("    pub values: &'static [&'static str],\n")
    out.append("}\n\n")
    out.append("/// Atributos globais na ordem de emissão, `aria-*` por último.\n")
    out.append("pub static GLOBALS: &[GlobalValue] = &[\n")
    for name, value in globals_tbl.items():
        if name == "aria":
            continue
        kind, values = global_kind(value)
        out.append(value_struct("GlobalValue", name, kind, values))
    for name, value in aria_tbl.items():
        kind, values = aria_kind(value)
        out.append(value_struct("GlobalValue", name, kind, values))
    out.append("];\n\n")

    out.append("/// Atributo `aria-*` e o tipo do valor, na ordem do TOML.\n")
    out.append("#[derive(Debug, PartialEq, Eq)]\n")
    out.append("pub struct AriaValue {\n")
    out.append("    /// Nome do atributo.\n")
    out.append("    pub name: &'static str,\n")
    out.append("    /// Tipo do valor.\n")
    out.append("    pub kind: ValueKind,\n")
    out.append("    /// Valores aceitos quando o tipo é `Enum`.\n")
    out.append("    pub values: &'static [&'static str],\n")
    out.append("}\n\n")
    out.append("/// Atributos `aria-*` na ordem declarada.\n")
    out.append("pub static ARIA_VALUES: &[AriaValue] = &[\n")
    for name, value in aria_tbl.items():
        kind, values = aria_kind(value)
        out.append(value_struct("AriaValue", name, kind, values))
    out.append("];\n\n")

    attr_values = data.get("attr_values") or {}
    out.append("/// Valor aceito por `elemento.atributo` (`\"\"` = default de qualquer elemento).\n")
    out.append("#[derive(Debug, PartialEq, Eq)]\n")
    out.append("pub struct AttrValues {\n")
    out.append("    /// Elemento em que o atributo vive (`\"\"` = qualquer).\n")
    out.append("    pub element: &'static str,\n")
    out.append("    /// Nome do atributo.\n")
    out.append("    pub attr: &'static str,\n")
    out.append("    /// Tipo do valor.\n")
    out.append("    pub kind: ValueKind,\n")
    out.append("    /// Valores aceitos (`Enum`/`Tokens`).\n")
    out.append("    pub values: &'static [&'static str],\n")
    out.append("}\n\n")
    out.append("/// Valores literais por `(elemento, atributo)`, ordenados.\n")
    out.append("pub static ATTR_VALUES: &[AttrValues] = &[\n")
    parsed: list[tuple[str, str, list[str]]] = []
    for key, values in attr_values.items():
        if "." not in key:
            raise SpecError(f"[attr_values] chave `{key}` não é `elemento.atributo`")
        element, attr = key.split(".", 1)
        parsed.append((element, attr, list(values)))
    for element, attr, values in sorted(parsed):
        out.append("    AttrValues {\n")
        out.append(entry(8, "element", rust_str("" if element == "*" else element)))
        out.append(entry(8, "attr", rust_str(attr)))
        out.append(entry(8, "kind", f"ValueKind::{attr_value_kind(element, attr)}"))
        out.append(entry(8, "values", values))
        out.append("    },\n")
    out.append("];\n\n")

    input_attrs = data.get("input_attributes") or {}
    out.append("/// Atributos extras aceitos por `input type=\"…\"`, ordenados por tipo.\n")
    out.append("pub static INPUT_ATTRS: &[(&str, &[&str])] = &[\n")
    for input_type, attrs in sorted(input_attrs.items()):
        out.append(input_attrs_entry(input_type, attrs))
    out.append("];\n\n")

    out.append(
        "/// Elemento pelo nome do `html!` (`None` = desconhecido).\n"
        "#[must_use]\n"
        "pub fn element(name: &str) -> Option<&'static Element> {\n"
        "    ELEMENTS.iter().find(|candidate| candidate.name == name)\n"
        "}\n\n"
        "/// Escopo declarado da invariante.\n"
        "#[must_use]\n"
        "pub fn invariant_scope(name: &str) -> Option<Scope> {\n"
        "    INVARIANT_SCOPES\n"
        "        .iter()\n"
        "        .find(|(candidate, _)| *candidate == name)\n"
        "        .map(|(_, scope)| *scope)\n"
        "}\n\n"
        "/// Valores literais de `(elemento, atributo)`: exato primeiro, default depois.\n"
        "#[must_use]\n"
        "pub fn attr_values(element: &str, attr: &str) -> Option<&'static AttrValues> {\n"
        "    let exact = ATTR_VALUES\n"
        "        .iter()\n"
        "        .find(|entry| entry.element == element && entry.attr == attr);\n"
        "    exact.or_else(|| {\n"
        "        ATTR_VALUES\n"
        "            .iter()\n"
        "            .find(|entry| entry.element.is_empty() && entry.attr == attr)\n"
        "    })\n"
        "}\n\n"
        "/// Atributos extras aceitos pelo tipo de `input`.\n"
        "#[must_use]\n"
        "pub fn input_attrs(input_type: &str) -> Option<&'static [&'static str]> {\n"
        "    INPUT_ATTRS\n"
        "        .iter()\n"
        "        .find(|(candidate, _)| *candidate == input_type)\n"
        "        .map(|(_, attrs)| *attrs)\n"
        "}\n"
    )
    return "".join(out)


def main(argv: list[str]) -> int:
    check = "--check" in argv
    show = "--stdout" in argv
    spec_path = SPEC
    out_path = TABLE
    index = 0
    while index < len(argv):
        arg = argv[index]
        if arg in ("--spec", "--out") and index + 1 < len(argv):
            target = Path(argv[index + 1])
            if arg == "--spec":
                spec_path = target
            else:
                out_path = target
            index += 1
        index += 1
    try:
        data = tomllib.loads(spec_path.read_text(encoding="utf-8"))
        text = generate(data)
    except (SpecError, tomllib.TOMLDecodeError) as exc:
        print(f"ERROR {exc}", file=sys.stderr)
        return 1

    if show:
        sys.stdout.write(text)
        return 0
    if check:
        current = out_path.read_text(encoding="utf-8") if out_path.exists() else ""
        if current == text:
            print(f"ok {out_path.name}: tabela em dia com {spec_path.name}")
            return 0
        diff = difflib.unified_diff(
            current.splitlines(keepends=True),
            text.splitlines(keepends=True),
            fromfile=str(out_path),
            tofile="gerado por tools/gen_spec.py",
            n=2,
        )
        sys.stdout.writelines(list(diff)[:80])
        print(f"\nERROR {out_path} está velho: rode `python3 tools/gen_spec.py`")
        return 1
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(text, encoding="utf-8")
    print(f"escrito {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
