from pathlib import Path

import pytest

from mcp_gen.models import GenerationResult
from mcp_gen.render import _rust_json_literal, _rust_string_literal, render_crate


def test_rust_string_literal_preserves_unicode_and_escapes_controls() -> None:
    assert _rust_string_literal('buyer’s "name"\n\b') == (
        '"buyer’s \\"name\\"\\n\\u{8}"'
    )


def test_rust_json_literal_uses_collision_free_raw_delimiter() -> None:
    literal = _rust_json_literal({"description": 'buyer’s "# account'})

    assert literal == 'r##"{"description":"buyer’s \\"# account"}"##'
    assert "\\u2019" not in literal


def test_render_preserves_handwritten_extensions(tmp_path: Path) -> None:
    output = tmp_path / "server"
    output.joinpath("src").mkdir(parents=True)
    extension = output / "src" / "extensions.rs"
    handwritten = "// sentinel: handwritten reporting code\n"
    extension.write_text(handwritten, encoding="utf-8")

    render_crate(
        GenerationResult(tools=[], resources=[], schema_kind="openapi", base_url=None),
        output_dir=output,
        crate_name="demo-mcp",
        base_url="https://example.test",
        core_path="../core",
        transport="stdio",
    )

    assert extension.read_text(encoding="utf-8") == handwritten
    main = (output / "src" / "main.rs").read_text(encoding="utf-8")
    assert "mod extensions;" in main
    assert ".custom_tools(&custom_tools)?" in main


def _render_empty(output: Path) -> str:
    render_crate(
        GenerationResult(tools=[], resources=[], schema_kind="openapi", base_url=None),
        output_dir=output,
        crate_name="demo-mcp",
        base_url="https://example.test",
        core_path="../core",
        transport="stdio",
    )
    return (output / "src" / "main.rs").read_text(encoding="utf-8")


def test_new_extensions_scaffold_receives_config(tmp_path: Path) -> None:
    main = _render_empty(tmp_path / "server")

    extensions = (tmp_path / "server" / "src" / "extensions.rs").read_text()
    assert "pub fn build_custom_tools(_config: &ProxyConfig)" in extensions
    assert "extensions::build_custom_tools(&config);" in main


@pytest.mark.parametrize(
    ("signature", "call"),
    [
        # Scaffold from before the config parameter existed.
        ("pub fn build_custom_tools() -> Vec<CustomToolSpec>", "build_custom_tools();"),
        ("pub fn build_custom_tools( ) -> Vec<CustomToolSpec>", "build_custom_tools();"),
        (
            "pub fn build_custom_tools(config: &ProxyConfig) -> Vec<CustomToolSpec>",
            "build_custom_tools(&config);",
        ),
        (
            "pub fn build_custom_tools(\n    cfg: &mcp_factory_core::ProxyConfig,\n) -> Vec<CustomToolSpec>",
            "build_custom_tools(&config);",
        ),
    ],
)
def test_main_matches_existing_extensions_signature(
    tmp_path: Path, signature: str, call: str
) -> None:
    output = tmp_path / "server"
    output.joinpath("src").mkdir(parents=True)
    extension = output / "src" / "extensions.rs"
    extension.write_text(f"{signature} {{\n    Vec::new()\n}}\n", encoding="utf-8")

    main = _render_empty(output)

    assert f"    let custom_tools = extensions::{call}\n" in main
    assert extension.read_text(encoding="utf-8").startswith(signature)


def test_render_preserves_workspace_member_manifest(tmp_path: Path) -> None:
    output = tmp_path / "server"
    output.mkdir()
    manifest = output / "Cargo.toml"
    member = '[package]\nname = "demo-mcp"\n\n[dependencies]\nextra.workspace = true\n'
    manifest.write_text(member, encoding="utf-8")
    result = GenerationResult(tools=[], resources=[], schema_kind="openapi", base_url=None)
    kwargs = dict(
        output_dir=output,
        crate_name="demo-mcp",
        base_url="https://example.test",
        core_path="../core",
        transport="stdio",
    )

    render_crate(result, **kwargs)
    assert manifest.read_text(encoding="utf-8") == member

    manifest.write_text("[workspace]\n", encoding="utf-8")
    render_crate(result, **kwargs)
    assert "[workspace.dependencies]" in manifest.read_text(encoding="utf-8")


def test_nullable_schema_translation_preserves_enum_constraints() -> None:
    from mcp_gen.render import _json_schema
    schema = {"type": "object", "properties": {
        "url": {"type": "string", "nullable": True},
        "state": {"type": "string", "enum": ["A", "B"], "nullable": True},
        "enabled": {"type": "boolean"},
    }}
    translated = _json_schema(schema)
    assert translated["properties"]["url"] == {"type": ["string", "null"]}
    assert translated["properties"]["state"] == {"type": ["string", "null"], "enum": ["A", "B", None]}
    assert translated["properties"]["enabled"] == {"type": "boolean"}
    assert schema["properties"]["url"]["nullable"] is True


def test_render_emits_lazy_catalog_and_preserves_resource_extension(tmp_path: Path, fixtures_dir: Path) -> None:
    from mcp_gen.openapi.parser import parse_openapi
    result = parse_openapi(fixtures_dir / "minimal-openapi.yaml")
    output = tmp_path / "server"
    output.joinpath("src").mkdir(parents=True)
    output.joinpath("src/extensions.rs").write_text("pub fn build_custom_tools() {}\npub fn build_resources() {}\n")
    render_crate(result, output_dir=output, crate_name="demo-mcp", base_url="https://example.test", core_path="../core", transport="stdio")
    tools = (output / "src/tools.rs").read_text()
    main = (output / "src/main.rs").read_text()
    catalog = tools.split("pub fn operation_catalog()", 1)[1].split("fn build_operation_", 1)[0]
    assert "OperationDescriptor" in catalog and "serde_json::from_str" not in catalog
    assert "serde_json::from_str" in tools.split("fn build_operation_", 1)[1]
    assert ".operation_catalog(tools::operation_catalog())?" in main
    assert "resources.extend(extensions::build_resources());" in main


def test_openapi_explode_false_arrays_keep_serialization_contract(tmp_path: Path) -> None:
    from mcp_gen.openapi.parser import parse_openapi
    source = tmp_path / "query.yaml"
    source.write_text("""openapi: 3.0.3
info: {title: Query encoding, version: '1'}
paths:
  /apps:
    get:
      operationId: apps
      parameters:
        - name: fields[apps]
          in: query
          style: form
          explode: false
          schema: {type: array, items: {type: string}}
        - name: tags
          in: query
          schema: {type: array, items: {type: string}}
      responses:
        '200': {description: OK}
""")
    props = parse_openapi(source).tools[0].input_schema["properties"]
    assert props["fields[apps]"]["type"] == "array"
    assert props["fields[apps]"]["x-mcp-query-explode"] is False
    assert "x-mcp-query-explode" not in props["tags"]
