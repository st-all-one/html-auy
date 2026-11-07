#!/usr/bin/env python3
"""Gate dos atributos `data-*`: nome declarado, valor permitido e leitor no JS.

`data-*` é o único ponto do vocabulário em que o nome é decidido pelo consumidor — e por isso o
único que pode apodrecer em silêncio (nome usado no HTML e nenhum leitor, ou valor que o JS não
conhece). Três consultas, uma linha por problema, exit 1 se houver ERROR:

1. nome usado no markup ∉ tabela, ou nome declarado sem uso no markup;
2. nome declarado sem leitor no JS (`dataset.x`, `dataset["x"]`, `[data-x]`, `getAttribute`);
3. valor usado no markup fora da lista `values` declarada.

Uso:
    python3 tools/data_check.py [--spec spec/data.toml] [--markup ARQUIVO...] [--js ARQUIVO...]

Saída: uma linha por problema, prefixada por `ERROR`, e um resumo. Determinística (ordenada).
"""
from __future__ import annotations

import argparse
import re
import sys
import tomllib
from pathlib import Path

# `data-nome`, `data-nome="valor"`, `data-nome='valor'` ou `data-nome=valor` no markup.
USED = re.compile(
    r"""data-([a-z0-9]+(?:-[a-z0-9]+)*)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>"'=]+)))?"""
)
# Leitores no JS.
DATASET_DOT = re.compile(r"dataset\.([A-Za-z0-9_]+)")
DATASET_INDEX = re.compile(r"""dataset\[\s*['"]([^'"]+)['"]\s*\]""")
SELECTOR = re.compile(r"\[\s*data-([a-z0-9-]+)\s*\]")
GET_ATTRIBUTE = re.compile(r"""getAttribute\(\s*['"]data-([a-z0-9-]+)['"]\s*\)""")


def camel_to_kebab(name: str) -> str:
    """`a11yContrast` → `a11y-contrast` (o `dataset` do DOM é camelCase)."""
    return re.sub(r"([a-z0-9])([A-Z])", r"\1-\2", name).lower()


def collect(paths: list[str], suffixes: tuple[str, ...]) -> list[Path]:
    """Expande arquivos e diretórios (recursivo, ordenado)."""
    found: list[Path] = []
    for raw in paths:
        path = Path(raw)
        if path.is_dir():
            found.extend(sorted(p for p in path.rglob("*") if p.suffix in suffixes))
        elif path.exists():
            found.append(path)
        else:
            raise FileNotFoundError(raw)
    return found


def load_spec(path: Path) -> dict[str, list[str] | None]:
    """`data` do TOML → {nome: valores permitidos | None}."""
    raw = tomllib.loads(path.read_text(encoding="utf-8"))
    table = raw.get("data", {})
    declared: dict[str, list[str] | None] = {}
    for name, entry in table.items():
        if entry is None:
            declared[name] = None
            continue
        if not isinstance(entry, dict):
            raise ValueError(f"data.{name}: seção deve ser tabela")
        values = entry.get("values")
        if values is not None and not isinstance(values, list):
            raise ValueError(f"data.{name}.values: deve ser lista")
        declared[name] = values
    return declared


def scan_markup(paths: list[str]) -> list[tuple[str, str | None, str]]:
    """(nome, valor, arquivo) na ordem de leitura, deduplicado por (nome, valor)."""
    seen: set[tuple[str, str | None]] = set()
    used: list[tuple[str, str | None, str]] = []
    for path in collect(paths, (".html", ".htm")):
        text = path.read_text(encoding="utf-8")
        for match in USED.finditer(text):
            name = match.group(1)
            value = next((group for group in match.group(2, 3, 4) if group is not None), None)
            key = (name, value)
            if key in seen:
                continue
            seen.add(key)
            used.append((name, value, str(path)))
    return used


def scan_js(paths: list[str]) -> set[str]:
    """Nomes lidos pelo JS, já em kebab-case."""
    read: set[str] = set()
    for path in collect(paths, (".js", ".mjs", ".cjs", ".ts")):
        text = path.read_text(encoding="utf-8")
        read.update(camel_to_kebab(name) for name in DATASET_DOT.findall(text))
        read.update(camel_to_kebab(name) for name in DATASET_INDEX.findall(text))
        read.update(SELECTOR.findall(text))
        read.update(GET_ATTRIBUTE.findall(text))
    return read


def check(declared: dict[str, list[str] | None], used: list[tuple[str, str | None, str]],
          read: set[str]) -> list[str]:
    """Aplica as três consultas e devolve as linhas de erro ordenadas."""
    errors: list[str] = []
    used_names = {name for name, _, _ in used}

    for name, _, _ in used:
        if name not in declared:
            errors.append(f"ERROR data-{name} usado e não declarado")

    for name in declared:
        if name not in used_names:
            errors.append(f"ERROR data-{name} declarado e não usado no markup")
        if name not in read:
            errors.append(f"ERROR data-{name} declarado e não lido no JS")

    for name, value, source in used:
        allowed = declared.get(name)
        if allowed is None:
            continue
        # Atributo sem valor (`data-x`) vale `""`, como no DOM.
        value = "" if value is None else value
        if value not in allowed:
            errors.append(
                f"ERROR data-{name}=\"{value}\" fora de [{', '.join(allowed)}] ({source})"
            )

    return sorted(set(errors))


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spec", default="spec/data.toml")
    parser.add_argument("--markup", nargs="*", default=[])
    parser.add_argument("--js", nargs="*", default=[])
    args = parser.parse_args(argv)

    try:
        declared = load_spec(Path(args.spec))
        used = scan_markup(args.markup)
        read = scan_js(args.js)
    except (FileNotFoundError, ValueError) as error:
        print(f"ERROR {error}")
        return 1

    errors = check(declared, used, read)
    for line in errors:
        print(line)
    print(f"{len(errors)} erro(s), {len(declared)} data-* declarado(s)")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
