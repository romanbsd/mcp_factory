from mcp_gen.models import GraphQLOperation, RestOperation, ToolSpec


def test_tool_spec_roundtrip_fields() -> None:
    tool = ToolSpec(
        name="get_item",
        description="Get one item",
        input_schema={"type": "object"},
        execution_kind="rest",
        rest=RestOperation(method="GET", path_template="/items/{id}"),
    )
    assert tool.rest is not None
    assert tool.rest.method == "GET"


def test_graphql_tool_spec() -> None:
    tool = ToolSpec(
        name="user",
        description="Get user",
        input_schema={"type": "object"},
        execution_kind="graphql",
        graphql=GraphQLOperation(document="query { user { id } }"),
    )
    assert tool.graphql is not None
