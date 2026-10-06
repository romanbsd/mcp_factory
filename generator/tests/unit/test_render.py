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
