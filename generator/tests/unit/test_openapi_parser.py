from pathlib import Path

from mcp_gen.openapi.parser import parse_openapi


def test_extracts_single_operation(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "minimal-openapi.yaml")
    assert len(result.tools) == 1
    assert result.tools[0].name == "getPet"
    assert result.tools[0].rest is not None
    assert result.tools[0].rest.path_template == "/pets/{petId}"


def test_merges_path_query_header_and_body(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "params-openapi.yaml")
    names = {tool.name for tool in result.tools}
    assert names == {"getPet", "createPet"}

    get_pet = next(tool for tool in result.tools if tool.name == "getPet")
    assert {param.location for param in get_pet.rest.params} == {"path", "header", "query"}

    create_pet = next(tool for tool in result.tools if tool.name == "createPet")
    assert create_pet.rest.body_fields == ["name", "tag"]


def test_resolves_refs(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "refs-openapi.yaml")
    assert len(result.tools) == 1
    assert "name" in result.tools[0].input_schema.get("properties", {})


def test_skips_deprecated_by_default(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "deprecated-openapi.yaml")
    assert [tool.name for tool in result.tools] == ["currentGet"]


def test_includes_deprecated_when_requested(fixtures_dir: Path) -> None:
    result = parse_openapi(
        fixtures_dir / "deprecated-openapi.yaml",
        include_deprecated=True,
    )
    assert {tool.name for tool in result.tools} == {"legacyGet", "currentGet"}


def test_filters_by_tags(fixtures_dir: Path) -> None:
    spec = {
        "openapi": "3.0.3",
        "info": {"title": "Tagged", "version": "1.0.0"},
        "paths": {
            "/a": {
                "get": {
                    "operationId": "a",
                    "tags": ["alpha"],
                    "responses": {"200": {"description": "OK"}},
                }
            },
            "/b": {
                "get": {
                    "operationId": "b",
                    "tags": ["beta"],
                    "responses": {"200": {"description": "OK"}},
                }
            },
        },
    }
    path = fixtures_dir / "tagged-openapi.yaml"
    import yaml

    path.write_text(yaml.dump(spec), encoding="utf-8")
    result = parse_openapi(path, tags={"alpha"})
    assert [tool.name for tool in result.tools] == ["a"]


def _write_spec(path: Path, paths: dict) -> Path:
    import yaml

    spec = {"openapi": "3.0.3", "info": {"title": "t", "version": "1.0.0"}, "paths": paths}
    path.write_text(yaml.dump(spec), encoding="utf-8")
    return path


def test_sanitizes_invalid_tool_name(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "dot.yaml",
        {"/thing": {"get": {"operationId": "store.get.thing", "responses": {"200": {"description": "OK"}}}}},
    )
    assert parse_openapi(path).tools[0].name == "store_get_thing"


def test_dedups_colliding_slug_names(tmp_path: Path) -> None:
    # No operationId, so names come from the path slug; these two distinct paths
    # slugify to the same base and must be disambiguated (else the generated
    # server aborts on a duplicate tool name).
    op = {"get": {"responses": {"200": {"description": "OK"}}}}
    path = _write_spec(tmp_path / "collide.yaml", {"/a-b": op, "/a_b": op})
    names = sorted(tool.name for tool in parse_openapi(path).tools)
    assert names == ["get_a_b", "get_a_b_2"]


def test_wraps_non_object_request_body(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "arr.yaml",
        {
            "/bulk": {
                "post": {
                    "operationId": "bulkCreate",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/json": {
                                "schema": {"type": "array", "items": {"type": "string"}}
                            }
                        },
                    },
                    "responses": {"200": {"description": "OK"}},
                }
            }
        },
    )
    tool = parse_openapi(path).tools[0]
    assert tool.rest.raw_body is True
    assert tool.rest.body_fields == ["body"]
    body_schema = tool.input_schema["properties"]["body"]
    assert body_schema["type"] == "array"
    assert tool.input_schema.get("required") == ["body"]


def test_embeds_openapi_resources(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "minimal-openapi.yaml")
    uris = {resource.uri for resource in result.resources}
    assert uris == {"schema://openapi", "meta://tools"}
