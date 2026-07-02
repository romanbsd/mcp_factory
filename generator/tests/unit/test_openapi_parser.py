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


def test_embeds_openapi_resources(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "minimal-openapi.yaml")
    uris = {resource.uri for resource in result.resources}
    assert uris == {"schema://openapi", "meta://tools"}
