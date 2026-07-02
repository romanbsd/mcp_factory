from __future__ import annotations

import os
from pathlib import Path
from unittest.mock import patch

import pytest

from mcp_gen.packaging import (
    assemble_dist,
    create_archive,
    package_crate,
    package_with_temp_crate,
    release_binary_path,
    target_triple,
)


def test_target_triple_explicit() -> None:
    assert target_triple("aarch64-unknown-linux-gnu") == "aarch64-unknown-linux-gnu"


def test_target_triple_host() -> None:
    triple = target_triple(None)
    assert "-" in triple


def test_release_binary_path_unix(tmp_path: Path) -> None:
    path = release_binary_path(tmp_path, "my-mcp", None)
    if os.name == "nt":
        assert path.name == "my-mcp.exe"
    else:
        assert path.name == "my-mcp"
    assert path.parent.name == "release"


def test_release_binary_path_cross_target(tmp_path: Path) -> None:
    path = release_binary_path(tmp_path, "my-mcp", "x86_64-unknown-linux-gnu")
    assert "x86_64-unknown-linux-gnu" in path.parts


def test_assemble_dist(tmp_path: Path) -> None:
    binary = tmp_path / "build" / "my-mcp"
    binary.parent.mkdir()
    binary.write_bytes(b"\x7fELF")
    config = tmp_path / "build" / "config.toml"
    config.write_text("base_url = \"http://localhost\"\n", encoding="utf-8")

    dist = tmp_path / "dist" / "my-mcp"
    assemble_dist(
        binary_path=binary,
        dist_dir=dist,
        crate_name="my-mcp",
        base_url="http://localhost:8080",
        transport="stdio",
        config_path=config,
    )

    assert (dist / "my-mcp").is_file()
    assert (dist / "config.toml").is_file()
    assert "Cursor MCP configuration" in (dist / "README.txt").read_text()
    assert os.access(dist / "my-mcp", os.X_OK)


def test_create_archive(tmp_path: Path) -> None:
    dist = tmp_path / "bundle"
    dist.mkdir()
    (dist / "my-mcp").write_text("bin", encoding="utf-8")

    archive = create_archive(dist)
    assert archive.suffix == ".gz"
    assert archive.is_file()


def test_package_crate_invokes_cargo(tmp_path: Path) -> None:
    crate_dir = tmp_path / "crate"
    crate_dir.mkdir()
    binary = crate_dir / "target" / "release" / "demo-mcp"
    binary.parent.mkdir(parents=True)
    binary.write_bytes(b"\x7fELF")
    (crate_dir / "config.toml").write_text("base_url = \"http://x\"\n", encoding="utf-8")

    with patch("mcp_gen.packaging.build_release") as build_mock:
        dist, archive = package_crate(
            crate_dir,
            dist_dir=tmp_path / "out",
            crate_name="demo-mcp",
            base_url="http://x",
            transport="stdio",
        )

    build_mock.assert_called_once_with(crate_dir, target=None)
    assert (dist / "demo-mcp").is_file()
    assert archive is None


def test_package_crate_missing_binary(tmp_path: Path) -> None:
    crate_dir = tmp_path / "crate"
    crate_dir.mkdir()

    with patch("mcp_gen.packaging.build_release"):
        with pytest.raises(FileNotFoundError, match="expected release binary"):
            package_crate(
                crate_dir,
                dist_dir=tmp_path / "out",
                crate_name="demo-mcp",
                base_url="http://x",
                transport="stdio",
            )


def test_package_crate_with_archive(tmp_path: Path) -> None:
    crate_dir = tmp_path / "crate"
    crate_dir.mkdir()
    binary = crate_dir / "target" / "release" / "demo-mcp"
    binary.parent.mkdir(parents=True)
    binary.write_bytes(b"\x7fELF")

    with patch("mcp_gen.packaging.build_release"):
        dist, archive = package_crate(
            crate_dir,
            dist_dir=tmp_path / "out",
            crate_name="demo-mcp",
            base_url="http://x",
            transport="stdio",
            archive=True,
        )

    assert archive is not None
    assert archive.is_file()


def test_package_with_temp_crate(tmp_path: Path) -> None:
    dist_dir = tmp_path / "dist" / "demo-mcp"
    rendered: list[Path] = []

    def render(crate_dir: Path) -> None:
        crate_dir.mkdir(parents=True)
        rendered.append(crate_dir)
        binary = crate_dir / "target" / "release" / "demo-mcp"
        binary.parent.mkdir(parents=True)
        binary.write_bytes(b"\x7fELF")
        (crate_dir / "config.toml").write_text("base_url = \"http://x\"\n", encoding="utf-8")

    with patch("mcp_gen.packaging.build_release"):
        assembled, archive, source = package_with_temp_crate(
            render_fn=render,
            dist_dir=dist_dir,
            crate_name="demo-mcp",
            base_url="http://x",
            transport="stdio",
            keep_source=True,
        )

    assert len(rendered) == 1
    assert assembled == dist_dir
    assert archive is None
    assert source is not None
    assert source.name == "demo-mcp-source"
    assert not (source / "target").exists()
