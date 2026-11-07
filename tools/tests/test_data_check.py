"""Testes do gate `data-*`: três consultas, mensagens exatas e exit codes."""

from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "tools" / "data_check.py"
FIXTURES = Path(__file__).resolve().parent / "fixtures"


def run(*args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(SCRIPT), *args],
        capture_output=True,
        text=True,
        check=False,
    )


class DataCheckTest(unittest.TestCase):
    def tree(self, spec: str, markup: str, js: str) -> tuple[Path, Path, Path]:
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        base = Path(directory.name)
        spec_path = base / "data.toml"
        markup_path = base / "markup.html"
        js_path = base / "app.js"
        spec_path.write_text(spec, encoding="utf-8")
        markup_path.write_text(markup, encoding="utf-8")
        js_path.write_text(js, encoding="utf-8")
        return spec_path, markup_path, js_path

    def test_happy_path(self) -> None:
        result = run(
            "--spec", str(FIXTURES / "ok" / "data.toml"),
            "--markup", str(FIXTURES / "ok" / "markup.html"),
            "--js", str(FIXTURES / "ok" / "app.js"),
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("0 erro(s), 2 data-* declarado(s)", result.stdout)

    def test_used_but_not_declared(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf"]\n',
            '<div data-mask="cpf" data-foo="x"></div>',
            "el.dataset.mask;",
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1)
        self.assertIn("ERROR data-foo usado e não declarado", result.stdout)

    def test_declared_but_unused(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf"]\n',
            "<div></div>",
            "el.dataset.mask;",
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1)
        self.assertIn("ERROR data-mask declarado e não usado no markup", result.stdout)

    def test_declared_but_unread(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf"]\n',
            '<div data-mask="cpf"></div>',
            "console.log('nada');",
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1)
        self.assertIn("ERROR data-mask declarado e não lido no JS", result.stdout)

    def test_value_outside_declared_list(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf", "cnpj"]\n',
            '<div data-mask="rg"></div>',
            'el.dataset.mask;',
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1)
        self.assertIn('ERROR data-mask="rg" fora de [cpf, cnpj]', result.stdout)

    def test_missing_path_is_an_error(self) -> None:
        result = run("--markup", str(ROOT / "nao-existe.html"))
        self.assertEqual(result.returncode, 1)
        self.assertIn("ERROR", result.stdout)

    def test_output_is_deterministic(self) -> None:
        arguments = (
            "--spec", str(FIXTURES / "ok" / "data.toml"),
            "--markup", str(FIXTURES / "ok" / "markup.html"),
        )
        first = run(*arguments)
        second = run(*arguments)
        self.assertEqual(first.stdout, second.stdout)

    def test_single_quoted_value_is_validated(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf"]\n',
            "<div data-mask='rg'></div>",
            "el.dataset.mask;",
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn('data-mask="rg" fora de [cpf]', result.stdout)

    def test_valueness_attribute_is_the_empty_string(self) -> None:
        spec, markup, js = self.tree(
            '[data.mask]\nvalues = ["cpf"]\n',
            "<div data-mask></div>",
            "el.dataset.mask;",
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 1, result.stdout)
        self.assertIn('data-mask="" fora de [cpf]', result.stdout)

    def test_dataset_index_is_camel_case(self) -> None:
        spec, markup, js = self.tree(
            '[data."a11y-contrast"]\nvalues = ["high"]\n',
            '<div data-a11y-contrast="high"></div>',
            'el.dataset["a11yContrast"];',
        )
        result = run("--spec", str(spec), "--markup", str(markup), "--js", str(js))
        self.assertEqual(result.returncode, 0, result.stdout)

if __name__ == "__main__":
    unittest.main()
