import json
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


def test_detect_google_discovery(fixtures_dir: Path) -> None:
    assert (
        detect_kind(fixtures_dir / "minimal-google-discovery.json")
        == "google_discovery"
    )


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


def test_generate_with_operations_allowlist(tmp_path: Path, fixtures_dir: Path) -> None:
    output = tmp_path / "out"
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "params-openapi.yaml"),
            "--output",
            str(output),
            "--base-url",
            "http://localhost",
            "--operations",
            " createPet, ",
        ],
    )
    assert result.exit_code == 0, result.output
    assert "Generated 1 tools" in result.output
    manifest = json.loads((output / "mcp-gen.manifest.json").read_text())
    assert manifest["tool_count"] == 1


def test_generate_rejects_unknown_operation(tmp_path: Path, fixtures_dir: Path) -> None:
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "params-openapi.yaml"),
            "--output",
            str(tmp_path / "out"),
            "--base-url",
            "http://localhost",
            "--operations",
            "createPet,nope",
        ],
    )
    assert result.exit_code != 0
    assert "operationid(s)notfoundorfilteredout:nope" in _flatten(result.output)
    assert not (tmp_path / "out").exists()


def test_operations_rejected_for_non_openapi(tmp_path: Path, fixtures_dir: Path) -> None:
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "minimal.graphql"),
            "--output",
            str(tmp_path / "out"),
            "--base-url",
            "http://localhost",
            "--operations",
            "pet",
        ],
    )
    assert result.exit_code != 0
    assert "onlysupportedforopenapi" in _flatten(result.output)


@pytest.mark.parametrize(
    ("flag", "emitted"),
    [(None, True), ("--output-schema", True), ("--no-output-schema", False)],
)
def test_output_schema_flag(tmp_path: Path, flag: str | None, emitted: bool) -> None:
    import yaml

    spec = tmp_path / "s.yaml"
    pet = {"type": "object", "properties": {"name": {"type": "string"}}}
    spec.write_text(
        yaml.dump(
            {
                "openapi": "3.0.3",
                "info": {"title": "t", "version": "1.0.0"},
                "paths": {
                    "/pet": {
                        "get": {
                            "operationId": "getPet",
                            "responses": {
                                "200": {
                                    "description": "OK",
                                    "content": {"application/json": {"schema": pet}},
                                }
                            },
                        }
                    }
                },
            }
        ),
        encoding="utf-8",
    )
    output = tmp_path / "out"
    args = ["generate", "--input", str(spec), "--output", str(output),
            "--base-url", "http://localhost"]
    result = runner.invoke(app, args + ([flag] if flag else []))
    assert result.exit_code == 0, result.output
    tools_rs = (output / "src" / "tools.rs").read_text()
    assert ("output_schema: None" not in tools_rs) is emitted
    assert ("generated output schema must be valid JSON" in tools_rs) is emitted


@pytest.mark.parametrize("enabled", [False, True])
def test_compact_jsonapi_flag(tmp_path: Path, fixtures_dir: Path, enabled: bool) -> None:
    output = tmp_path / "out"
    args = ["generate", "--input", str(fixtures_dir / "minimal-openapi.yaml"),
            "--output", str(output), "--base-url", "http://localhost"]
    result = runner.invoke(app, args + (["--compact-jsonapi"] if enabled else []))
    assert result.exit_code == 0, result.output
    config = (output / "config.toml").read_text()
    main_rs = (output / "src" / "main.rs").read_text()
    assert ("\ncompact_jsonapi = true\n" in config) is enabled
    assert ("        compact_jsonapi: true,\n        ..ProxyConfig::default()" in main_rs) is enabled
    assert "compact_jsonapi" not in config + main_rs or enabled


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


def test_package_command_threads_llm_flags_to_render(
    tmp_path: Path, fixtures_dir: Path
) -> None:
    output = tmp_path / "dist"
    with patch("mcp_gen.cli.package_with_temp_crate") as package_mock:
        package_mock.return_value = (output, None, None)
        result = runner.invoke(
            app,
            [
                "package",
                "--input",
                str(fixtures_dir / "params-openapi.yaml"),
                "--output",
                str(output),
                "--base-url",
                "http://localhost:8080",
                "--operations",
                "createPet",
                "--no-output-schema",
                "--compact-jsonapi",
            ],
        )
    assert result.exit_code == 0, result.output
    assert "Packaged 1 tools" in result.output

    crate_dir = tmp_path / "crate"
    package_mock.call_args.kwargs["render_fn"](crate_dir)
    tools_rs = (crate_dir / "src" / "tools.rs").read_text()
    assert 'name: "createPet".to_string()' in tools_rs
    assert 'name: "getPet".to_string()' not in tools_rs
    assert "compact_jsonapi = true" in (crate_dir / "config.toml").read_text()
    assert "compact_jsonapi: true," in (crate_dir / "src" / "main.rs").read_text()


def test_package_command_rejects_unknown_operation(
    tmp_path: Path, fixtures_dir: Path
) -> None:
    with patch("mcp_gen.cli.package_with_temp_crate") as package_mock:
        result = runner.invoke(
            app,
            [
                "package",
                "--input",
                str(fixtures_dir / "params-openapi.yaml"),
                "--output",
                str(tmp_path / "dist"),
                "--base-url",
                "http://localhost:8080",
                "--operations",
                "nope",
            ],
        )
    assert result.exit_code != 0
    assert "nope" in _flatten(result.output)
    package_mock.assert_not_called()


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


@pytest.mark.parametrize("llm_flags", [False, True])
def test_compose_llm_flags(tmp_path: Path, fixtures_dir: Path, llm_flags: bool) -> None:
    output = tmp_path / "combined"
    result = runner.invoke(
        app,
        [
            "compose",
            "--input",
            str(fixtures_dir / "minimal-openapi.yaml"),
            "--input",
            str(fixtures_dir / "minimal-google-discovery.json"),
            "--output",
            str(output),
            "--base-url",
            "https://shared.example",
        ]
        + (["--no-output-schema", "--compact-jsonapi"] if llm_flags else []),
    )
    assert result.exit_code == 0, result.output
    tools_rs = (output / "src" / "tools.rs").read_text()
    # The Discovery fixture declares response schemas; they vanish with the flag.
    assert ("generated output schema must be valid JSON" in tools_rs) is not llm_flags
    assert ("compact_jsonapi = true" in (output / "config.toml").read_text()) is llm_flags


def test_compose_generates_one_crate_and_preserves_config(
    tmp_path: Path, fixtures_dir: Path
) -> None:
    output = tmp_path / "combined"
    config = tmp_path / "config.toml"
    config.write_text(
        'base_url = "https://configured.example"\nserver_name = "combined"\n',
        encoding="utf-8",
    )

    result = runner.invoke(
        app,
        [
            "compose",
            "--input",
            str(fixtures_dir / "minimal-openapi.yaml"),
            "--input",
            str(fixtures_dir / "minimal-google-discovery.json"),
            "--output",
            str(output),
            "--name",
            "combined-mcp",
            "--base-url",
            "https://shared.example",
            "--config",
            str(config),
        ],
    )

    assert result.exit_code == 0, result.output
    manifest = json.loads(
        (output / "mcp-gen.manifest.json").read_text(encoding="utf-8")
    )
    assert manifest == {
        "crate_name": "combined-mcp",
        "tool_count": 5,
        "resource_count": 4,
        "schema_kind": "composed",
    }
    tools = (output / "src" / "tools.rs").read_text(encoding="utf-8")
    assert 'name: "getPet".to_string()' in tools
    assert 'name: "items_query".to_string()' in tools
    assert (output / "config.toml").read_text(encoding="utf-8") == config.read_text(
        encoding="utf-8"
    )
    main_rs = (output / "src" / "main.rs").read_text(encoding="utf-8")
    assert "if config.base_url.is_empty()" in main_rs
    assert "if config.server_name.is_empty()" in main_rs
