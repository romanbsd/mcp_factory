from pathlib import Path

import pytest
from typer.testing import CliRunner

from mcp_gen.cli import app, detect_kind

runner = CliRunner()


def test_detect_openapi_yaml(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "minimal-openapi.yaml") == "openapi"


def test_detect_graphql_sdl(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "minimal.graphql") == "graphql"


def test_detect_graphql_introspection(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "introspection.json") == "graphql"


def test_missing_input_file(tmp_path: Path) -> None:
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(tmp_path / "missing.yaml"),
            "--output",
            str(tmp_path / "out"),
            "--base-url",
            "http://localhost",
        ],
    )
    assert result.exit_code != 0
