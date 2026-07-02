from __future__ import annotations

import json
from pathlib import Path
from typing import Literal

import typer
import yaml

from mcp_gen.graphql.parser import parse_graphql
from mcp_gen.openapi.parser import parse_openapi
from mcp_gen.render import render_crate

from . import __version__

app = typer.Typer(no_args_is_help=True, add_completion=False, name="mcp-gen")


def detect_kind(path: Path) -> Literal["openapi", "graphql"]:
    if path.suffix in {".graphql", ".gql"}:
        return "graphql"
    if path.suffix == ".json":
        text = path.read_text(encoding="utf-8")
        try:
            data = json.loads(text)
        except json.JSONDecodeError as exc:
            raise typer.BadParameter(f"invalid JSON schema file: {exc}") from exc
        if "openapi" in data:
            return "openapi"
        if data.get("__schema") or data.get("data", {}).get("__schema"):
            return "graphql"
        raise typer.BadParameter("unsupported JSON input: expected OpenAPI or GraphQL introspection")
    if path.suffix in {".yaml", ".yml"}:
        data = yaml.safe_load(path.read_text(encoding="utf-8"))
        if isinstance(data, dict) and "openapi" in data:
            return "openapi"
    raise typer.BadParameter(f"could not detect schema kind for {path}")


@app.command("generate")
def generate(
    input: Path = typer.Option(..., "--input", "-i", exists=True, dir_okay=False, readable=True),
    output: Path = typer.Option(..., "--output", "-o", file_okay=False),
    kind: str | None = typer.Option(None, "--kind", help="openapi or graphql"),
    base_url: str = typer.Option(..., "--base-url"),
    name: str = typer.Option("generated-mcp", "--name"),
    transport: str = typer.Option("stdio", "--transport", help="stdio, http, or both"),
    core_path: str = typer.Option(
        "../../crates/mcp-factory-core",
        "--core-path",
        help="Path to mcp-factory-core for generated Cargo.toml",
    ),
    include_deprecated: bool = typer.Option(False, "--include-deprecated"),
    tags: str | None = typer.Option(None, "--tags", help="Comma-separated OpenAPI tags filter"),
) -> None:
    """Generate a Rust MCP proxy crate from an OpenAPI or GraphQL schema."""
    schema_kind = kind or detect_kind(input)
    tag_set = {tag.strip() for tag in tags.split(",") if tag.strip()} if tags else None

    if schema_kind == "openapi":
        result = parse_openapi(
            input,
            include_deprecated=include_deprecated,
            tags=tag_set,
        )
    elif schema_kind == "graphql":
        result = parse_graphql(input)
    else:
        raise typer.BadParameter(f"unsupported kind: {schema_kind}")

    render_crate(
        result,
        output_dir=output,
        crate_name=name,
        base_url=base_url,
        core_path=core_path,
        transport=transport,
    )
    typer.echo(f"Generated {len(result.tools)} tools into {output}")


@app.command("version")
def version() -> None:
    """Print the generator version."""
    typer.echo(__version__)


if __name__ == "__main__":
    app()
