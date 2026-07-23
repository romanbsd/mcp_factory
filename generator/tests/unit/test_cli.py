import re
from pathlib import Path
from unittest.mock import patch

import pytest
from typer.testing import CliRunner

from mcp_gen.cli import app, detect_kind

runner = CliRunner()


def _flatten(output: str) -> str:
    """Strip ANSI codes, box-drawing chars and whitespace so substring checks
    don't depend on how the CLI framework wraps error panels (varies by
    typer/click/rich version and terminal width)."""
    output = re.sub(r"\x1b\[[0-9;]*[a-zA-Z]", "", output)
    output = output.replace("│", " ")
    return re.sub(r"\s+", "", output).lower()


def test_detect_openapi_yaml(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "minimal-openapi.yaml") == "openapi"


def test_detect_graphql_sdl(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "minimal.graphql") == "graphql"


def test_detect_graphql_introspection(fixtures_dir: Path) -> None:
    assert detect_kind(fixtures_dir / "introspection.json") == "graphql"


def test_detect_rejects_non_object_json(tmp_path: Path) -> None:
    bad = tmp_path / "bad.json"
    bad.write_text("[1, 2, 3]")
    with pytest.raises(Exception):
        detect_kind(bad)


def _spec_with_servers(path: Path, servers: list | None) -> Path:
    import yaml

    spec = {
        "openapi": "3.0.3",
        "info": {"title": "t", "version": "1.0.0"},
        "paths": {"/ping": {"get": {"operationId": "ping",
                                    "responses": {"200": {"description": "OK"}}}}},
    }
    if servers is not None:
        spec["servers"] = servers
    path.write_text(yaml.dump(spec), encoding="utf-8")
    return path


def test_base_url_defaults_from_servers(tmp_path: Path) -> None:
    spec = _spec_with_servers(tmp_path / "s.yaml", [{"url": "https://api.example.com"}])
    output = tmp_path / "out"
    result = runner.invoke(
        app,
        ["generate", "--input", str(spec), "--output", str(output)],
    )
    assert result.exit_code == 0, result.output
    assert 'base_url = "https://api.example.com"' in (output / "config.toml").read_text()


def test_missing_base_url_without_servers_errors(tmp_path: Path) -> None:
    spec = _spec_with_servers(tmp_path / "s.yaml", None)
    result = runner.invoke(
        app,
        ["generate", "--input", str(spec), "--output", str(tmp_path / "out")],
    )
    assert result.exit_code != 0
    assert "base-url" in _flatten(result.output)


def test_invalid_transport_rejected(tmp_path: Path, fixtures_dir: Path) -> None:
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "minimal-openapi.yaml"),
            "--output",
            str(tmp_path / "out"),
            "--base-url",
            "http://localhost",
            "--transport",
            "bogus",
        ],
    )
    assert result.exit_code != 0
    assert "invalid transport" in result.output.lower()


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
