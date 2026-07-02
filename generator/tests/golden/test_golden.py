import os
import re
import subprocess
from pathlib import Path
from shutil import which

import pytest

from mcp_gen.cli import app
from typer.testing import CliRunner

runner = CliRunner()
FIXTURES = Path(__file__).resolve().parents[1] / "fixtures"
CORE_PATH_RE = re.compile(r'mcp-factory-core = \{ path = "[^"]+" \}')


def _normalize_cargo_toml(text: str) -> str:
    return CORE_PATH_RE.sub('mcp-factory-core = { path = "<CORE>" }', text)


def _generate(tmp_path: Path, fixture: str, name: str) -> Path:
    output = tmp_path / name
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(FIXTURES / fixture),
            "--output",
            str(output),
            "--base-url",
            "http://127.0.0.1:9",
            "--name",
            name,
        ],
    )
    assert result.exit_code == 0, result.output
    return output


@pytest.mark.skipif(which("cargo") is None, reason="cargo not installed")
def test_generated_openapi_crate_checks(tmp_path: Path) -> None:
    output = _generate(tmp_path, "minimal-openapi.yaml", "minimal-mcp")
    proc = subprocess.run(
        ["cargo", "check"],
        cwd=output,
        capture_output=True,
        text=True,
        check=False,
    )
    assert proc.returncode == 0, proc.stdout + proc.stderr


def test_golden_minimal_openapi(tmp_path: Path) -> None:
    output = _generate(tmp_path, "minimal-openapi.yaml", "minimal-mcp")
    golden_dir = Path(__file__).resolve().parents[1] / "golden" / "minimal-openapi"
    if os.environ.get("UPDATE_GOLDEN") == "1":
        golden_dir.mkdir(parents=True, exist_ok=True)
        for name in ("tools.rs", "resources.rs", "Cargo.toml"):
            (golden_dir / name).write_text((output / "src" / name).read_text() if name != "Cargo.toml" else (output / name).read_text())
        return

    for name in ("tools.rs", "resources.rs", "Cargo.toml"):
        generated = (
            (output / "src" / name).read_text()
            if name != "Cargo.toml"
            else (output / name).read_text()
        )
        expected = (golden_dir / name).read_text()
        if name == "Cargo.toml":
            generated = _normalize_cargo_toml(generated)
            expected = _normalize_cargo_toml(expected)
        assert generated == expected
