from __future__ import annotations

from pathlib import Path

import typer


def default_core_path() -> Path:
    """Path to mcp-factory-core relative to the mcp-gen package (repo checkout)."""
    return Path(__file__).resolve().parents[2] / "crates" / "mcp-factory-core"


def resolve_core_path(explicit: str | None = None) -> str:
    path = Path(explicit).resolve() if explicit else default_core_path().resolve()
    if not path.is_dir():
        raise typer.BadParameter(
            f"mcp-factory-core not found at {path}. "
            "Pass --core-path if the runtime crate lives elsewhere."
        )
    return str(path)
