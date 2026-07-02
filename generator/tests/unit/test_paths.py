from pathlib import Path

import pytest
import typer

from mcp_gen.paths import default_core_path, resolve_core_path


def test_default_core_path_points_at_runtime_crate() -> None:
    core = default_core_path()
    assert core.name == "mcp-factory-core"
    assert core.is_dir()
    assert (core / "Cargo.toml").is_file()


def test_resolve_core_path_uses_default() -> None:
    resolved = resolve_core_path(None)
    assert Path(resolved) == default_core_path().resolve()


def test_resolve_core_path_honors_override(tmp_path: Path) -> None:
    custom = tmp_path / "mcp-factory-core"
    custom.mkdir()
    (custom / "Cargo.toml").write_text("[package]\nname = \"mcp-factory-core\"\n", encoding="utf-8")
    assert resolve_core_path(str(custom)) == str(custom.resolve())


def test_resolve_core_path_missing_dir(tmp_path: Path) -> None:
    with pytest.raises(typer.BadParameter, match="mcp-factory-core not found"):
        resolve_core_path(str(tmp_path / "missing"))
