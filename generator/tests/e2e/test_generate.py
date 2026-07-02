import json
from pathlib import Path

from typer.testing import CliRunner

from mcp_gen.cli import app

runner = CliRunner()


def test_generate_openapi_crate(tmp_path: Path, fixtures_dir: Path) -> None:
    output = tmp_path / "server"
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "minimal-openapi.yaml"),
            "--output",
            str(output),
            "--base-url",
            "http://localhost:8080",
            "--name",
            "minimal-mcp",
            "--core-path",
            str(Path(__file__).resolve().parents[3] / "crates" / "mcp-factory-core"),
        ],
    )
    assert result.exit_code == 0, result.output
    assert (output / "src" / "tools.rs").exists()
    manifest = json.loads((output / "mcp-gen.manifest.json").read_text())
    assert manifest["tool_count"] == 1


def test_detects_graphql_kind(tmp_path: Path, fixtures_dir: Path) -> None:
    output = tmp_path / "gql-server"
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "minimal.graphql"),
            "--output",
            str(output),
            "--base-url",
            "http://localhost:8080/graphql",
            "--core-path",
            str(Path(__file__).resolve().parents[3] / "crates" / "mcp-factory-core"),
        ],
    )
    assert result.exit_code == 0, result.output
    tools_rs = (output / "src" / "tools.rs").read_text()
    assert "createUser" in tools_rs
