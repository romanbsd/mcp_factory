from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from jinja2 import Environment, PackageLoader, select_autoescape

from mcp_gen.models import GenerationResult


def _rust_string_literal(value: str) -> str:
    """Return a Rust string literal without JSON-only escape sequences."""
    escaped: list[str] = []
    replacements = {
        '"': r'\"',
        "\\": r"\\",
        "\n": r"\n",
        "\r": r"\r",
        "\t": r"\t",
    }
    for char in value:
        if char in replacements:
            escaped.append(replacements[char])
        elif ord(char) < 0x20 or ord(char) == 0x7F:
            escaped.append(f"\\u{{{ord(char):x}}}")
        else:
            escaped.append(char)
    return f'"{"".join(escaped)}"'


def _rust_json_literal(value: Any) -> str:
    """Serialize JSON into a Rust raw string with a collision-free delimiter."""
    serialized = json.dumps(
        value,
        ensure_ascii=False,
        separators=(",", ":"),
        sort_keys=True,
    )
    hashes = ""
    while f'"{hashes}' in serialized:
        hashes += "#"
    return f'r{hashes}"{serialized}"{hashes}'


def _env() -> Environment:
    env = Environment(
        loader=PackageLoader("mcp_gen", "templates"),
        autoescape=select_autoescape(enabled_extensions=()),
        trim_blocks=True,
        lstrip_blocks=True,
    )
    env.filters["rust_string"] = _rust_string_literal
    env.filters["rust_json"] = _rust_json_literal
    return env


def render_crate(
    result: GenerationResult,
    *,
    output_dir: Path,
    crate_name: str,
    base_url: str,
    core_path: str,
    transport: str,
) -> None:
    env = _env()
    context = {
        "crate_name": crate_name,
        "base_url": base_url,
        "core_path": core_path,
        "transport": transport,
        "tools": result.tools,
        "resources": result.resources,
    }

    output_dir.mkdir(parents=True, exist_ok=True)
    (output_dir / "src").mkdir(exist_ok=True)

    templates = {
        "Cargo.toml.j2": output_dir / "Cargo.toml",
        "main.rs.j2": output_dir / "src" / "main.rs",
        "tools.rs.j2": output_dir / "src" / "tools.rs",
        "resources.rs.j2": output_dir / "src" / "resources.rs",
        "config.toml.j2": output_dir / "config.toml",
    }

    for template_name, target in templates.items():
        target.write_text(env.get_template(template_name).render(**context), encoding="utf-8")

    manifest = {
        "crate_name": crate_name,
        "tool_count": len(result.tools),
        "resource_count": len(result.resources),
        "schema_kind": result.schema_kind,
    }
    (output_dir / "mcp-gen.manifest.json").write_text(
        json.dumps(manifest, indent=2),
        encoding="utf-8",
    )
