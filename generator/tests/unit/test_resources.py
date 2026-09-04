import json

from mcp_gen.models import ToolSpec
from mcp_gen.resources import build_embedded_resources


def test_builds_schema_and_tool_index_resources() -> None:
    tools = [
        ToolSpec(
            name="get_item",
            description="Get one item",
            input_schema={"type": "object"},
            execution_kind="rest",
        )
    ]

    resources = build_embedded_resources(
        tools,
        schema_uri="schema://demo",
        schema_name="demo",
        schema_description="Demo schema",
        schema_mime_type="application/json",
        schema_text='{"demo": true}',
    )

    assert resources[0].content == '{"demo": true}'
    assert resources[0].uri == "schema://demo"
    assert json.loads(resources[1].content) == [
        {"name": "get_item", "description": "Get one item"}
    ]
