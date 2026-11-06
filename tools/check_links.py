#!/usr/bin/env python3
"""Verifica que todo link relativo entre documentos existe.

O projeto é documentação primeiro: um link quebrado é um defeito de produto.
Checa `.md` de `docs/`, da raiz e de `spec/`, ignorando URLs externas, âncoras puras
e code fences (para não confundir exemplo com link real).
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCAN = [ROOT / "docs", ROOT / "spec"]
ROOT_FILES = ["README.md", "CONTRIBUTING.md"]

LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")
FENCE = re.compile(r"^(```|~~~)")


def files() -> list[Path]:
    out = [ROOT / f for f in ROOT_FILES if (ROOT / f).exists()]
    for base in SCAN:
        if base.exists():
            out.extend(sorted(base.rglob("*.md")))
    return out


def main() -> int:
    bad: list[str] = []
    checked = 0
    for f in files():
        in_fence = False
        for no, line in enumerate(f.read_text(encoding="utf-8").splitlines(), 1):
            if FENCE.match(line.strip()):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            for target in LINK.findall(line):
                if target.startswith(("http://", "https://", "mailto:", "#")):
                    continue
                path = target.split("#", 1)[0]
                if not path:
                    continue
                checked += 1
                resolved = (f.parent / path).resolve()
                if not resolved.exists():
                    bad.append(f"{f.relative_to(ROOT)}:{no} -> {target}")
    for b in bad:
        print(f"BROKEN {b}")
    print(f"\n{checked} link(s) relativos verificados, {len(bad)} quebrado(s)")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
