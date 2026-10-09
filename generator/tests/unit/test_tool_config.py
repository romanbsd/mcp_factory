import json
from pathlib import Path

import pytest
from typer.testing import CliRunner

from mcp_gen.cli import app
from mcp_gen.models import glob_match
from mcp_gen.openapi.parser import UnknownOperationError, derive_description, parse_openapi
from mcp_gen.tool_config import ToolConfigError, apply_tool_config, load_tool_config

runner = CliRunner()

CONFIG = """
default_profiles = ["core"]
cursor_param = "cursor"
strip_enums = ["fields[*]"]

[profiles]
core = ["apps_*"]
testflight = ["betaGroups_*", "apps_getCollection"]

[custom_tools]
asset_upload = ["core", "testflight"]

[descriptions]
apps_getCollection = "Find apps by bundle ID."
"""


def _write(tmp_path: Path, text: str) -> Path:
    path = tmp_path / "tools.toml"
    path.write_text(text, encoding="utf-8")
    return path


def _parse(fixtures_dir: Path, tmp_path: Path, text: str = CONFIG):
    config = load_tool_config(_write(tmp_path, text))
    result = parse_openapi(
        fixtures_dir / "jsonapi-openapi.yaml", operations=config.operation_patterns
    )
    apply_tool_config(result, config)
    return result


def test_glob_match_treats_brackets_literally() -> None:
    assert glob_match("fields[apps]", "fields[*]")
    assert not glob_match("fieldsX", "fields[*]")
    assert glob_match("betaGroups_getInstance", "betaGroups_*")
    assert not glob_match("xbetaGroups_getInstance", "betaGroups_*")
    assert glob_match("v2", "v?")


def test_profiles_select_and_annotate_tools(fixtures_dir: Path, tmp_path: Path) -> None:
    result = _parse(fixtures_dir, tmp_path)
    names = {tool.name for tool in result.tools}
    # certificates_getCollection matches no profile, so it is not generated.
    assert names == {
        "apps_getCollection",
        "apps_getInstance",
        "apps_deleteInstance",
        "apps_builds_getToManyRelated",
        "betaGroups_betaTesters_createToManyRelationship",
    }
    assert result.tool_profiles["apps_getCollection"] == ["core", "testflight"]
    assert result.tool_profiles["apps_deleteInstance"] == ["core"]
    assert result.tool_profiles["betaGroups_betaTesters_createToManyRelationship"] == [
        "testflight"
    ]
    assert result.tool_profiles["asset_upload"] == ["core", "testflight"]
    assert result.default_profiles == ["core"]


def test_descriptions_cursor_and_enum_stripping(fixtures_dir: Path, tmp_path: Path) -> None:
    result = _parse(fixtures_dir, tmp_path)
    tools = {tool.name: tool for tool in result.tools}

    listing = tools["apps_getCollection"]
    assert listing.description == "Find apps by bundle ID."
    props = listing.input_schema["properties"]
    assert props["cursor"]["type"] == "string"
    assert ("cursor", "query") in {(p.name, p.location) for p in listing.rest.params}
    assert "enum" not in props["fields[apps]"]["items"]
    assert props["limit"] == {"type": "integer", "maximum": 200}

    # Only collection GETs with `limit` get the cursor, and never twice.
    assert "cursor" not in tools["apps_getInstance"].input_schema["properties"]
    builds = tools["apps_builds_getToManyRelated"]
    assert [p.name for p in builds.rest.params].count("cursor") == 1

    # The meta://tools index reflects the override.
    index = next(r for r in result.resources if r.uri == "meta://tools")
    entries = {entry["name"]: entry["description"] for entry in json.loads(index.content)}
    assert entries["apps_getCollection"] == "Find apps by bundle ID."


def test_derived_descriptions_when_spec_has_none(fixtures_dir: Path) -> None:
    tools = {tool.name: tool for tool in parse_openapi(fixtures_dir / "jsonapi-openapi.yaml").tools}
    assert tools["apps_getInstance"].description == "Get one `apps` resource by id."
    assert tools["apps_deleteInstance"].description.endswith("This cannot be undone.")
    assert tools["apps_builds_getToManyRelated"].description == (
        "List the `builds` related to one `apps` resource."
    )
    assert tools["betaGroups_betaTesters_createToManyRelationship"].description == (
        "Link additional `betaTesters` to one `betaGroups` resource (linkage only)."
    )
    # A summary still wins.
    assert tools["certificates_getCollection"].description == "List signing certificates"
    assert all("Generated from OpenAPI" not in tool.description for tool in tools.values())


@pytest.mark.parametrize(
    ("method", "path", "expected"),
    [
        ("GET", "/v1/apps", "List `apps` resources; supports the filters and pagination parameters below."),
        ("POST", "/v1/appScreenshots", "Create one `appScreenshots` resource."),
        ("PATCH", "/v1/apps/{id}", "Update attributes or relationships of one `apps` resource."),
        ("DELETE", "/v1/a/{id}/relationships/b", "Unlink `b` from one `a` resource (linkage only)."),
        ("PATCH", "/v1/a/{id}/relationships/b", "Replace the `b` linkage of one `a` resource (linkage only)."),
        ("GET", "/v1/a/{id}/metrics/usage", "Get `usage` metrics for one `a` resource."),
        ("GET", "/v1/apps/{id}/metrics", "Get metrics for one `apps` resource."),
        ("POST", "/pets/{id}/actions", "POST /pets/{id}/actions."),
        ("GET", "/", "GET /."),
    ],
)
def test_derive_description_shapes(method: str, path: str, expected: str) -> None:
    assert derive_description(method, path) == expected


def test_operation_globs_must_each_match(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "jsonapi-openapi.yaml", operations={"apps_get*"})
    assert {tool.name for tool in result.tools} == {"apps_getCollection", "apps_getInstance"}
    with pytest.raises(UnknownOperationError, match="nothing_\\*"):
        parse_openapi(fixtures_dir / "jsonapi-openapi.yaml", operations={"apps_*", "nothing_*"})


@pytest.mark.parametrize(
    ("text", "message"),
    [
        ("[profiles]\n", "at least one profile"),
        ("bogus = 1\n[profiles]\na = ['x']\n", "unknown tool config key"),
        ("[profiles]\nall = ['x']\n", "reserved"),
        ("[profiles]\na = 'x'\n", "profiles.a must be a list"),
        ("default_profiles = ['b']\n[profiles]\na = ['x']\n", "undefined profile"),
        ("[profiles]\na = ['x']\n[custom_tools]\nt = ['b']\n", "undefined profile"),
        ("[profiles]\na = ['x']\n[descriptions]\nx = ''\n", "non-empty strings"),
        ("cursor_param = 3\n[profiles]\na = ['x']\n", "cursor_param"),
        ("default_profiles = []\n[profiles]\na = ['x']\n", "must not be empty"),
        ("custom_tools = 'x'\n[profiles]\na = ['x']\n", "custom_tools\\] must be a table"),
        ("descriptions = ['x']\n[profiles]\na = ['x']\n", "descriptions\\] must be a table"),
        ("[profiles\n", "cannot read tool config"),
    ],
)
def test_invalid_configs_are_rejected(tmp_path: Path, text: str, message: str) -> None:
    with pytest.raises(ToolConfigError, match=message):
        load_tool_config(_write(tmp_path, text))


def test_apply_rejects_unknown_description_and_custom_clash(
    fixtures_dir: Path, tmp_path: Path
) -> None:
    with pytest.raises(ToolConfigError, match="unknown tool"):
        _parse(fixtures_dir, tmp_path, "[profiles]\na = ['apps_*']\n[descriptions]\nnope = 'x'\n")
    with pytest.raises(ToolConfigError, match="clash"):
        _parse(
            fixtures_dir,
            tmp_path,
            "[profiles]\na = ['apps_*']\n[custom_tools]\napps_getInstance = ['a']\n",
        )


def _generate(tmp_path: Path, fixtures_dir: Path, *extra: str):
    output = tmp_path / "out"
    result = runner.invoke(
        app,
        [
            "generate",
            "--input",
            str(fixtures_dir / "jsonapi-openapi.yaml"),
            "--output",
            str(output),
            *extra,
        ],
    )
    return result, output


def test_cli_tool_config_renders_profiles(tmp_path: Path, fixtures_dir: Path) -> None:
    result, output = _generate(
        tmp_path, fixtures_dir, "--tool-config", str(_write(tmp_path, CONFIG))
    )
    assert result.exit_code == 0, result.output
    tools_rs = (output / "src" / "tools.rs").read_text()
    main_rs = (output / "src" / "main.rs").read_text()
    config_toml = (output / "config.toml").read_text()
    assert '("apps_getCollection", &["core", "testflight"]),' in tools_rs
    assert '("asset_upload", &["core", "testflight"]),' in tools_rs
    assert 'vec!["core".to_string()]' in tools_rs
    assert ".tool_profiles(&tools::build_tool_profiles())?\n        .tools(&tools)?" in main_rs
    assert "config.profiles = Some(tools::default_profiles());" in main_rs
    assert "# Available: core, testflight" in config_toml
    assert "# Unset uses the generated default: core." in config_toml
    assert '# profiles = ["core"]' in config_toml


def test_cli_tool_config_without_defaults_says_everything_is_exposed(
    tmp_path: Path, fixtures_dir: Path
) -> None:
    result, output = _generate(
        tmp_path,
        fixtures_dir,
        "--tool-config",
        str(_write(tmp_path, "[profiles]\ncore = ['apps_*']\n")),
    )
    assert result.exit_code == 0, result.output
    config_toml = (output / "config.toml").read_text()
    assert "# Unset exposes every tool." in config_toml
    assert "# profiles = " not in config_toml
    assert "default_profiles" not in (output / "src" / "tools.rs").read_text()
    assert "config.profiles" not in (output / "src" / "main.rs").read_text()


def test_cli_without_tool_config_emits_no_profile_code(
    tmp_path: Path, fixtures_dir: Path
) -> None:
    result, output = _generate(tmp_path, fixtures_dir)
    assert result.exit_code == 0, result.output
    for generated in ("src/tools.rs", "src/main.rs", "config.toml"):
        text = (output / generated).read_text()
        assert "profiles" not in text, generated


@pytest.mark.parametrize(
    ("extra", "message"),
    [
        (["--operations", "apps_getInstance"], "either--operationsor--tool-config"),
        (["--kind", "graphql"], "onlysupportedforopenapi"),
    ],
)
def test_cli_tool_config_conflicts(
    tmp_path: Path, fixtures_dir: Path, extra: list[str], message: str
) -> None:
    import re

    result, output = _generate(
        tmp_path, fixtures_dir, "--tool-config", str(_write(tmp_path, CONFIG)), *extra
    )
    assert result.exit_code != 0
    flat = re.sub(r"\s+|│|\x1b\[[0-9;]*[a-zA-Z]", "", result.output).lower()
    assert message.lower() in flat, flat
    assert not output.exists()


def test_cli_reports_config_errors(tmp_path: Path, fixtures_dir: Path) -> None:
    result, _ = _generate(
        tmp_path,
        fixtures_dir,
        "--tool-config",
        str(_write(tmp_path, "[profiles]\na = ['missing_*']\n")),
    )
    assert result.exit_code != 0
    assert "missing_*" in result.output
