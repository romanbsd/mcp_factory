from __future__ import annotations

import json
from pathlib import Path

from jinja2 import Environment, PackageLoader, select_autoescape

from mcp_gen.models import GenerationResult


def _env() -> Environment:
    return Environment(
        loader=PackageLoader("mcp_gen", "templates"),
        autoescape=select_autoescape(enabled_extensions=()),
        trim_blocks=True,
        lstrip_blocks=True,
    )


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
