from pathlib import Path
import json

from mcp_gen.google_discovery.parser import parse_google_discovery


def test_parses_methods_and_media_uploads(fixtures_dir: Path) -> None:
    result = parse_google_discovery(fixtures_dir / "minimal-google-discovery.json")

    assert result.base_url == "https://demo.googleapis.com"
    assert result.schema_kind == "google_discovery"
    assert [tool.name for tool in result.tools] == [
        "items_get",
        "items_create",
        "items_query",
        "items_upload",
    ]

    get_tool = result.tools[0]
    assert get_tool.read_only is True
    assert get_tool.title == "items.get"
    assert get_tool.rest is not None
    assert get_tool.rest.path_template == "https://demo.googleapis.com/v1/items/{name}"
    assert get_tool.input_schema["required"] == ["name"]
    assert get_tool.input_schema["properties"]["view"]["enum"] == ["BASIC", "FULL"]
    assert get_tool.output_schema["required"] == ["name"]

    create_tool = result.tools[1]
    assert create_tool.read_only is False
    assert create_tool.rest is not None and create_tool.rest.raw_body is True
    assert create_tool.input_schema["required"] == ["body"]
    assert create_tool.input_schema["properties"]["body"]["properties"]["labels"] == {
        "type": "array",
        "items": {"type": "string"},
    }
    query_tool = result.tools[2]
    assert query_tool.read_only is True
    assert query_tool.idempotent is True
    assert query_tool.rest is not None
    assert query_tool.rest.body_fields == ["body"]
    assert "required" not in query_tool.input_schema

    upload_tool = result.tools[3]
    assert upload_tool.read_only is False
    assert upload_tool.rest is not None
    assert upload_tool.rest.media_path_template == (
        "https://demo.googleapis.com/v1/items:upload"
    )
    assert upload_tool.rest.media_accept == ["application/octet-stream"]
    assert upload_tool.rest.media_max_size == 1024
    assert upload_tool.input_schema["required"] == ["mediaFile"]

    schema_resource = next(
        resource
        for resource in result.resources
        if resource.uri == "schema://google-discovery/demo/v1"
    )
    tool_resource = next(
        resource for resource in result.resources if resource.uri == "meta://tools/demo/v1"
    )
    assert schema_resource.name == "demo-v1-google-discovery"
    assert schema_resource.mime_type == "application/json"
    assert (
        schema_resource.content
        == (fixtures_dir / "minimal-google-discovery.json").read_text(encoding="utf-8")
    )
    assert tool_resource.name == "demo-v1-tools"
    assert tool_resource.mime_type == "application/json"
    assert json.loads(tool_resource.content) == [
        {"name": "items_get", "description": "Gets an item."},
        {"name": "items_create", "description": "Creates an item."},
        {"name": "items_query", "description": "Queries items without changing them."},
        {"name": "items_upload", "description": "Uploads item media."},
    ]


def test_read_only_filters_mutations(fixtures_dir: Path) -> None:
    result = parse_google_discovery(
        fixtures_dir / "minimal-google-discovery.json", read_only=True
    )
    assert [tool.name for tool in result.tools] == ["items_get", "items_query"]


def test_tags_filter_top_level_resources(fixtures_dir: Path) -> None:
    result = parse_google_discovery(
        fixtures_dir / "minimal-google-discovery.json", tags={"missing"}
    )
    assert result.tools == []


def test_parses_root_level_methods(fixtures_dir: Path) -> None:
    result = parse_google_discovery(
        fixtures_dir / "minimal-google-discovery-root-method.json"
    )

    assert [tool.name for tool in result.tools] == ["rootOp"]
    root_tool = result.tools[0]
    assert root_tool.read_only is True
    assert root_tool.rest is not None
    assert root_tool.rest.path_template == "https://demo.googleapis.com/v1/root"
    assert root_tool.output_schema["required"] == ["name"]
