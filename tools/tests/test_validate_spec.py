"""Regressões do validador de `spec/elements.toml`: cada defeito clássico tem um caso."""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "tools" / "validate_spec.py"
SPEC = ROOT / "spec" / "elements.toml"


def validate(text: str) -> tuple[int, str]:
    directory = tempfile.TemporaryDirectory()
    path = Path(directory.name) / "elements.toml"
    path.write_text(text, encoding="utf-8")
    result = subprocess.run(
        [sys.executable, str(SCRIPT), str(path)],
        capture_output=True,
        text=True,
        check=False,
    )
    directory.cleanup()
    return result.returncode, result.stdout


class ValidateSpecTest(unittest.TestCase):
    def setUp(self) -> None:
        self.source = SPEC.read_text(encoding="utf-8")

    def assert_error(self, mutated: str, needle: str) -> None:
        code, output = validate(mutated)
        self.assertEqual(code, 1, output)
        self.assertIn(needle, output)

    def test_real_spec_is_clean(self) -> None:
        code, output = validate(self.source)
        self.assertEqual(code, 0, output)
        self.assertIn("0 erro(s)", output)

    def test_schema_version(self) -> None:
        self.assert_error(
            self.source.replace("schema = 2", "schema = 3", 1),
            "schema=3; este validador implementa o esquema 2",
        )

    def test_invariant_without_scope(self) -> None:
        self.assert_error(
            self.source.replace('"a_rel_enum"', '"a_rel_enum", "ghost_rule"', 1),
            "invariante `ghost_rule` usada mas sem escopo declarado",
        )

    def test_element_with_hyphen(self) -> None:
        self.assert_error(
            self.source + '\n[element."foo-bar"]\ncategory = ["flow"]\n',
            'element `foo-bar`: nome com hífen',
        )

    def test_undeclared_boolean(self) -> None:
        mutated = self.source.replace('"controls", ', "", 1)
        self.assert_error(
            mutated,
            "atributo booleano `controls` usado sem estar em [meta] boolean_attributes",
        )

    def test_required_children_mismatch(self) -> None:
        self.assert_error(
            self.source + '\n[element.sandbox]\ncategory = ["flow"]\nchildren = ["none"]\n'
            'required_children = ["span"]\n',
            "required_children `span`",
        )

    def test_free_role(self) -> None:
        self.assert_error(
            self.source + '\n[element.oembed]\ncategory = ["flow"]\nallowed = ["role"]\n',
            "element `oembed`: `role` livre",
        )

    def test_only_without_invariant(self) -> None:
        self.assert_error(
            self.source + '\n[element.shellish]\ncategory = ["flow"]\nonly = "shell"\n',
            "`only = \"shell\"` sem a invariante `shell_only_elements`",
        )


if __name__ == "__main__":
    unittest.main()
