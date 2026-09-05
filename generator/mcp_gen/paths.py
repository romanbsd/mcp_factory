from __future__ import annotations

import os
from pathlib import Path

import typer


def default_core_path() -> Path:
    """Path to mcp-factory-core relative to the mcp-gen package (repo checkout)."""
    return Path(__file__).resolve().parents[2] / "crates" / "mcp-factory-core"


def resolve_core_path(
    explicit: str | None = None, *, relative_to: Path | None = None
) -> str:
    path = Path(explicit).resolve() if explicit else default_core_path().resolve()
    if not path.is_dir():
        raise typer.BadParameter(
            f"mcp-factory-core not found at {path}. "
            "Pass --core-path if the runtime crate lives elsewhere."
        )
    if relative_to is not None:
        return Path(os.path.relpath(path, relative_to.resolve())).as_posix()
    return str(path)
