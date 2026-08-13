from pathlib import Path

import pytest
from prance import ValidationError

from mcp_gen.openapi.parser import (
    OpenAPICompatibilityWarning,
    load_openapi,
    parse_openapi,
)


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


def test_includes_path_level_parameters_with_operation_override(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "path-params.yaml",
        {
            "/pets/{petId}": {
                "parameters": [
                    {
                        "name": "petId",
                        "in": "path",
                        "required": True,
                        "schema": {"type": "string"},
                    },
                    {
                        "name": "trace",
                        "in": "header",
                        "schema": {"type": "string"},
                    },
                ],
                "get": {
                    "operationId": "getPet",
                    "parameters": [
                        {
                            "name": "trace",
                            "in": "header",
                            "required": True,
                            "schema": {"type": "string", "minLength": 1},
                        }
                    ],
                    "responses": {"200": {"description": "OK"}},
                },
            }
        },
    )
    tool = parse_openapi(path).tools[0]
    assert {(p.name, p.location) for p in tool.rest.params} == {
        ("petId", "path"),
        ("trace", "header"),
    }
    assert tool.input_schema["required"] == ["petId", "trace"]
    assert tool.input_schema["properties"]["trace"]["minLength"] == 1


def test_optional_request_body_does_not_require_body_fields(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "optional-body.yaml",
        {
            "/pets": {
                "post": {
                    "operationId": "createPet",
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {"name": {"type": "string"}},
                                    "required": ["name"],
                                }
                            }
                        }
                    },
                    "responses": {"200": {"description": "OK"}},
                }
            }
        },
    )
    tool = parse_openapi(path).tools[0]
    assert tool.rest.body_fields == ["name"]
    assert "required" not in tool.input_schema


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


def test_read_only_keeps_only_non_mutating_methods(tmp_path: Path) -> None:
    import json

    path = _write_spec(
        tmp_path / "rw.yaml",
        {
            "/items": {
                "get": {
                    "operationId": "listItems",
                    "responses": {"200": {"description": "OK"}},
                },
                "head": {
                    "operationId": "headItems",
                    "responses": {"200": {"description": "OK"}},
                },
                "options": {
                    "operationId": "optionsItems",
                    "responses": {"200": {"description": "OK"}},
                },
                "post": {
                    "operationId": "createItem",
                    "responses": {"201": {"description": "Created"}},
                },
                "put": {
                    "operationId": "putItem",
                    "responses": {"200": {"description": "OK"}},
                },
                "patch": {
                    "operationId": "patchItem",
                    "responses": {"200": {"description": "OK"}},
                },
                "delete": {
                    "operationId": "deleteItem",
                    "responses": {"204": {"description": "Gone"}},
                },
            }
        },
    )
    result = parse_openapi(path, read_only=True)
    assert {tool.name for tool in result.tools} == {
        "listItems",
        "headItems",
        "optionsItems",
    }
    meta = next(r for r in result.resources if r.uri == "meta://tools")
    assert {entry["name"] for entry in json.loads(meta.content)} == {
        "listItems",
        "headItems",
        "optionsItems",
    }


def _write_spec(path: Path, paths: dict, **extra) -> Path:
    import yaml

    spec = {"openapi": "3.0.3", "info": {"title": "t", "version": "1.0.0"}, "paths": paths}
    spec.update(extra)
    path.write_text(yaml.dump(spec), encoding="utf-8")
    return path


def test_coerces_single_key_description_mapping_before_validation(
    tmp_path: Path,
) -> None:
    path = _write_spec(
        tmp_path / "description-placeholder.yaml",
        {},
        components={
            "schemas": {
                "Invoice": {
                    "type": "object",
                    "properties": {
                        "price": {
                            "type": "number",
                            "description": {"tls_object_price": None},
                        }
                    },
                }
            }
        },
    )

    with pytest.warns(
        OpenAPICompatibilityWarning,
        match=r"components\.schemas\.Invoice\.properties\.price\.description",
    ):
        spec = load_openapi(path)

    assert spec["components"]["schemas"]["Invoice"]["properties"]["price"][
        "description"
    ] == "{tls_object_price}"


def test_does_not_coerce_arbitrary_description_mappings(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "invalid-description.yaml",
        {},
        components={
            "schemas": {
                "Invoice": {
                    "type": "object",
                    "description": {"text": "not a placeholder"},
                }
            }
        },
    )

    with pytest.raises(ValidationError):
        load_openapi(path)


def test_detects_base_url_from_servers(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "servers.yaml",
        {"/ping": {"get": {"operationId": "ping", "responses": {"200": {"description": "OK"}}}}},
        servers=[{"url": "https://api.example.com/v1"}],
    )
    assert parse_openapi(path).base_url == "https://api.example.com/v1"


def test_no_servers_means_no_base_url(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "noserver.yaml",
        {"/ping": {"get": {"operationId": "ping", "responses": {"200": {"description": "OK"}}}}},
    )
    assert parse_openapi(path).base_url is None


def test_deprecated_operation_is_flagged(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "dep.yaml",
        {
            "/old": {
                "get": {
                    "operationId": "old",
                    "summary": "Old thing",
                    "deprecated": True,
                    "responses": {"200": {"description": "OK"}},
                }
            }
        },
    )
    tool = parse_openapi(path, include_deprecated=True).tools[0]
    assert tool.description.startswith("[DEPRECATED]")


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


def test_verb_annotations_and_output_schema(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "ann.yaml",
        {
            "/pets/{id}": {
                "get": {
                    "operationId": "getPet",
                    "summary": "Fetch a pet",
                    "parameters": [
                        {"name": "id", "in": "path", "required": True,
                         "schema": {"type": "string"}}
                    ],
                    "responses": {
                        "200": {
                            "description": "OK",
                            "content": {
                                "application/json": {
                                    "schema": {"type": "object",
                                               "properties": {"id": {"type": "string"}}}
                                }
                            },
                        }
                    },
                },
                "delete": {
                    "operationId": "deletePet",
                    "parameters": [
                        {"name": "id", "in": "path", "required": True,
                         "schema": {"type": "string"}}
                    ],
                    "responses": {"204": {"description": "gone"}},
                },
            }
        },
    )
    tools = {t.name: t for t in parse_openapi(path).tools}

    get = tools["getPet"]
    assert get.read_only is True
    assert get.idempotent is True
    assert get.destructive is False
    assert get.open_world is True
    assert get.title == "Fetch a pet"
    assert get.output_schema == {"type": "object", "properties": {"id": {"type": "string"}}}

    delete = tools["deletePet"]
    assert delete.read_only is False
    assert delete.destructive is True
    assert delete.idempotent is True
    assert delete.output_schema is None


def test_array_response_has_no_output_schema(tmp_path: Path) -> None:
    # MCP structuredContent must be an object, so array responses declare none.
    path = _write_spec(
        tmp_path / "list.yaml",
        {
            "/pets": {
                "get": {
                    "operationId": "listPets",
                    "responses": {
                        "200": {
                            "description": "OK",
                            "content": {
                                "application/json": {
                                    "schema": {"type": "array", "items": {"type": "object"}}
                                }
                            },
                        }
                    },
                }
            }
        },
    )
    assert parse_openapi(path).tools[0].output_schema is None


def test_form_urlencoded_body_sets_content_type(tmp_path: Path) -> None:
    path = _write_spec(
        tmp_path / "form.yaml",
        {
            "/login": {
                "post": {
                    "operationId": "login",
                    "requestBody": {
                        "required": True,
                        "content": {
                            "application/x-www-form-urlencoded": {
                                "schema": {
                                    "type": "object",
                                    "properties": {"user": {"type": "string"}},
                                }
                            }
                        },
                    },
                    "responses": {"200": {"description": "OK"}},
                }
            }
        },
    )
    tool = parse_openapi(path).tools[0]
    assert tool.rest.content_type == "application/x-www-form-urlencoded"
    assert tool.rest.body_fields == ["user"]
    assert tool.rest.raw_body is False


def test_embeds_openapi_resources(fixtures_dir: Path) -> None:
    result = parse_openapi(fixtures_dir / "minimal-openapi.yaml")
    uris = {resource.uri for resource in result.resources}
    assert uris == {"schema://openapi", "meta://tools"}
