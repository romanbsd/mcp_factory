from pathlib import Path

from mcp_gen.graphql.parser import parse_graphql


def test_parses_sdl_query_and_mutation(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "minimal.graphql")
    assert {tool.name for tool in result.tools} == {"user", "createUser"}
    user = next(tool for tool in result.tools if tool.name == "user")
    assert user.graphql is not None
    assert "user(id: $id)" in user.graphql.document


def test_parses_nested_input_types(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "nested-inputs.graphql")
    search = next(tool for tool in result.tools if tool.name == "search")
    props = search.input_schema["properties"]["input"]["properties"]
    assert "term" in props and "limit" in props


def test_parses_introspection_json(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "introspection.json")
    assert len(result.tools) == 1
    assert result.tools[0].name == "user"
