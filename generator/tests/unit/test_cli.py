from pathlib import Path
from unittest.mock import patch

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


def test_package_command(tmp_path: Path, fixtures_dir: Path) -> None:
    output = tmp_path / "dist"
    with patch("mcp_gen.cli.package_with_temp_crate") as package_mock:
        package_mock.return_value = (output, None, None)
        result = runner.invoke(
            app,
            [
                "package",
                "--input",
                str(fixtures_dir / "minimal-openapi.yaml"),
                "--output",
                str(output),
                "--base-url",
                "http://localhost:8080",
                "--name",
                "demo-mcp",
            ],
        )
    assert result.exit_code == 0, result.output
    assert "Packaged" in result.output
    package_mock.assert_called_once()
    kwargs = package_mock.call_args.kwargs
    assert kwargs["crate_name"] == "demo-mcp"
    assert kwargs["dist_dir"] == output


def test_package_command_with_archive(tmp_path: Path, fixtures_dir: Path) -> None:
    output = tmp_path / "dist"
    archive = tmp_path / "dist.tar.gz"
    with patch("mcp_gen.cli.package_with_temp_crate") as package_mock:
        package_mock.return_value = (output, archive, None)
        result = runner.invoke(
            app,
            [
                "package",
                "--input",
                str(fixtures_dir / "minimal.graphql"),
                "--output",
                str(output),
                "--base-url",
                "http://localhost/graphql",
                "--name",
                "gql-mcp",
                "--archive",
            ],
        )
    assert result.exit_code == 0, result.output
    assert "Archive:" in result.output
