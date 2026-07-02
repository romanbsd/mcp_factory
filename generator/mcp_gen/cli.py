from __future__ import annotations

import json
from pathlib import Path
from typing import Literal

import typer
import yaml

from mcp_gen.graphql.parser import parse_graphql
from mcp_gen.openapi.parser import parse_openapi
from mcp_gen.packaging import package_with_temp_crate
from mcp_gen.paths import resolve_core_path
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


def _parse_schema(
    input: Path,
    *,
    kind: str | None,
    include_deprecated: bool,
    tags: str | None,
):
    schema_kind = kind or detect_kind(input)
    tag_set = {tag.strip() for tag in tags.split(",") if tag.strip()} if tags else None

    if schema_kind == "openapi":
        return parse_openapi(
            input,
            include_deprecated=include_deprecated,
            tags=tag_set,
        )
    if schema_kind == "graphql":
        return parse_graphql(input)
    raise typer.BadParameter(f"unsupported kind: {schema_kind}")


@app.command("generate")
def generate(
    input: Path = typer.Option(..., "--input", "-i", exists=True, dir_okay=False, readable=True),
    output: Path = typer.Option(..., "--output", "-o", file_okay=False),
    kind: str | None = typer.Option(None, "--kind", help="openapi or graphql"),
    base_url: str = typer.Option(..., "--base-url"),
    name: str = typer.Option("generated-mcp", "--name"),
    transport: str = typer.Option("stdio", "--transport", help="stdio, http, or both"),
    core_path: str | None = typer.Option(
        None,
        "--core-path",
        help="Override path to mcp-factory-core (default: auto-detected from mcp-gen install)",
    ),
    include_deprecated: bool = typer.Option(False, "--include-deprecated"),
    tags: str | None = typer.Option(None, "--tags", help="Comma-separated OpenAPI tags filter"),
) -> None:
    """Generate a Rust MCP proxy crate from an OpenAPI or GraphQL schema."""
    result = _parse_schema(
        input,
        kind=kind,
        include_deprecated=include_deprecated,
        tags=tags,
    )

    render_crate(
        result,
        output_dir=output,
        crate_name=name,
        base_url=base_url,
        core_path=resolve_core_path(core_path),
        transport=transport,
    )
    typer.echo(f"Generated {len(result.tools)} tools into {output}")


@app.command("package")
def package(
    input: Path = typer.Option(..., "--input", "-i", exists=True, dir_okay=False, readable=True),
    output: Path = typer.Option(..., "--output", "-o", file_okay=False, help="Directory for the portable dist"),
    kind: str | None = typer.Option(None, "--kind", help="openapi or graphql"),
    base_url: str = typer.Option(..., "--base-url"),
    name: str = typer.Option("generated-mcp", "--name"),
    transport: str = typer.Option("stdio", "--transport", help="stdio, http, or both"),
    core_path: str | None = typer.Option(
        None,
        "--core-path",
        help="Override path to mcp-factory-core (default: auto-detected from mcp-gen install)",
    ),
    include_deprecated: bool = typer.Option(False, "--include-deprecated"),
    tags: str | None = typer.Option(None, "--tags", help="Comma-separated OpenAPI tags filter"),
    target: str | None = typer.Option(
        None,
        "--target",
        help="Rust target triple for cross-compilation (e.g. x86_64-unknown-linux-gnu)",
    ),
    archive: bool = typer.Option(False, "--archive", help="Also create a .tar.gz next to the dist directory"),
    keep_source: bool = typer.Option(False, "--keep-source", help="Keep generated Rust source beside the dist"),
) -> None:
    """Generate, build, and package a portable MCP server binary."""
    result = _parse_schema(
        input,
        kind=kind,
        include_deprecated=include_deprecated,
        tags=tags,
    )
    def render(crate_dir: Path) -> None:
        render_crate(
            result,
            output_dir=crate_dir,
            crate_name=name,
            base_url=base_url,
            core_path=resolve_core_path(core_path),
            transport=transport,
        )

    dist_dir, archive_path, source_dir = package_with_temp_crate(
        render_fn=render,
        dist_dir=output,
        crate_name=name,
        base_url=base_url,
        transport=transport,
        target=target,
        archive=archive,
        keep_source=keep_source,
    )
    typer.echo(f"Packaged {len(result.tools)} tools into {dist_dir}")
    if archive_path:
        typer.echo(f"Archive: {archive_path}")
    if source_dir:
        typer.echo(f"Source: {source_dir}")


@app.command("version")
def version() -> None:
    """Print the generator version."""
    typer.echo(__version__)


if __name__ == "__main__":
    app()
