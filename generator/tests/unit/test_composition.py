import json

import pytest

from mcp_gen.composition import CompositionError, compose_generation_results
from mcp_gen.models import (
    GenerationResult,
    ResourceSpec,
    RestOperation,
    ToolSpec,
)


def _result(name: str, uri: str, path: str, base_url: str | None) -> GenerationResult:
    return GenerationResult(
        tools=[
            ToolSpec(
                name=name,
                description=name,
                input_schema={"type": "object"},
                execution_kind="rest",
                rest=RestOperation(method="GET", path_template=path),
            )
        ],
        resources=[ResourceSpec(uri, uri, uri, "text/plain", uri)],
        schema_kind="openapi",
        base_url=base_url,
    )


def test_composes_absolute_operations_with_distinct_upstreams() -> None:
    result = compose_generation_results(
        [
            _result("one", "schema://one", "https://one.example/items", "https://one.example"),
            _result("two", "schema://two", "https://two.example/items", "https://two.example"),
        ]
    )

    assert [tool.name for tool in result.tools] == ["one", "two"]
    assert [resource.uri for resource in result.resources] == ["schema://one", "schema://two"]
    assert result.schema_kind == "composed"


def test_rejects_duplicate_tool_names() -> None:
    with pytest.raises(CompositionError, match="duplicate tool name: same"):
        compose_generation_results(
            [
                _result("same", "schema://one", "https://one.example/items", None),
                _result("same", "schema://two", "https://two.example/items", None),
            ]
        )


def test_disambiguates_duplicate_resource_uris() -> None:
    result = compose_generation_results(
        [
            _result("one", "schema://same", "https://one.example/items", None),
            _result("two", "schema://same", "https://two.example/items", None),
        ]
    )

    assert [resource.uri for resource in result.resources] == [
        "schema://same",
        "schema://same/1",
    ]


def test_merges_colliding_tool_index_resources() -> None:
    def with_tool_index(name: str, tools: list[dict]) -> GenerationResult:
        return GenerationResult(
            tools=[],
            resources=[
                ResourceSpec(
                    "meta://tools",
                    "tools",
                    "Generated tool index",
                    "application/json",
                    json.dumps(tools),
                )
            ],
            schema_kind="openapi",
            base_url=None,
        )

    result = compose_generation_results(
        [
            with_tool_index("a", [{"name": "a_tool", "description": "A"}]),
            with_tool_index("b", [{"name": "b_tool", "description": "B"}]),
        ]
    )

    assert [resource.uri for resource in result.resources] == ["meta://tools"]
    assert json.loads(result.resources[0].content) == [
        {"name": "a_tool", "description": "A"},
        {"name": "b_tool", "description": "B"},
    ]


def test_non_index_meta_resources_follow_suffix_path() -> None:
    def with_meta(uri: str, content: str) -> GenerationResult:
        return GenerationResult(
            tools=[],
            resources=[
                ResourceSpec(uri, uri, uri, "application/json", content)
            ],
            schema_kind="openapi",
            base_url=None,
        )

    result = compose_generation_results(
        [
            with_meta("meta://tools-reference", '[{"name": "a"}]'),
            with_meta("meta://tools-reference", '[{"name": "b"}]'),
        ]
    )

    assert [resource.uri for resource in result.resources] == [
        "meta://tools-reference",
        "meta://tools-reference/1",
    ]


def test_rejects_relative_operations_with_distinct_upstreams() -> None:
    with pytest.raises(CompositionError, match="one shared base URL"):
        compose_generation_results(
            [
                _result("one", "schema://one", "/items", "https://one.example"),
                _result("two", "schema://two", "/items", "https://two.example"),
            ]
        )
