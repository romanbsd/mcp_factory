from __future__ import annotations

import json
from dataclasses import replace

from mcp_gen.models import GenerationResult, ToolSpec


class CompositionError(ValueError):
    """Generation results cannot be combined without ambiguous routing."""


def _merge_tool_index_content(first: str, second: str) -> str | None:
    """Combine two tool-index payloads, or None if either is not a JSON list."""
    try:
        first_items = json.loads(first)
        second_items = json.loads(second)
    except json.JSONDecodeError:
        return None
    if not (isinstance(first_items, list) and isinstance(second_items, list)):
        return None
    return json.dumps(first_items + second_items, indent=2)


def _uses_runtime_base_url(tool: ToolSpec) -> bool:
    if tool.execution_kind == "graphql":
        return True
    if tool.rest is None:
        return False
    path = tool.rest.path_template
    return not (path.startswith("https://") or path.startswith("http://"))


def compose_generation_results(
    results: list[GenerationResult],
    *,
    base_url_override: str | None = None,
) -> GenerationResult:
    """Combine independently parsed schemas into one collision-free server."""
    if not results:
        raise CompositionError("composition requires at least one schema")

    tools = []
    resources = []
    tool_names: set[str] = set()
    resource_uris: set[str] = set()
    runtime_base_urls: set[str] = set()

    for index, result in enumerate(results):
        for tool in result.tools:
            if tool.name in tool_names:
                raise CompositionError(f"duplicate tool name: {tool.name}")
            tool_names.add(tool.name)
            tools.append(tool)
            if _uses_runtime_base_url(tool):
                endpoint = base_url_override or result.base_url
                if not endpoint:
                    raise CompositionError(
                        f"schema {result.schema_kind} has relative operations but no base URL"
                    )
                runtime_base_urls.add(endpoint)
        for resource in result.resources:
            uri = resource.uri
            if uri in resource_uris:
                if uri == "meta://tools":
                    # The server exposes a single generated tool index; merge
                    # colliding ones instead of duplicating the URI. Per-API
                    # indexes (meta://tools/<api>/<version>) stay distinct and
                    # collide via the suffix path below.
                    target_index = next(
                        i for i, r in enumerate(resources) if r.uri == uri
                    )
                    merged = _merge_tool_index_content(
                        resources[target_index].content, resource.content
                    )
                    if merged is None:
                        raise CompositionError(
                            "cannot merge non-list meta://tools index content"
                        )
                    resources[target_index] = replace(
                        resources[target_index], content=merged
                    )
                    continue
                # Plain schema resources share fixed URIs (schema://openapi,
                # meta://tools); suffix later inputs so composing them still works.
                uri = f"{resource.uri}/{index}"
                if uri in resource_uris:
                    raise CompositionError(f"duplicate resource URI: {uri}")
                resource = replace(resource, uri=uri)
            resource_uris.add(uri)
            resources.append(resource)

    if len(runtime_base_urls) > 1:
        endpoints = ", ".join(sorted(runtime_base_urls))
        raise CompositionError(
            "relative operations require one shared base URL; found: " + endpoints
        )

    detected_urls = [result.base_url for result in results if result.base_url]
    base_url = (
        base_url_override
        or next(iter(runtime_base_urls), None)
        or (detected_urls[0] if detected_urls else None)
    )
    return GenerationResult(
        tools=tools,
        resources=resources,
        schema_kind="composed",
        base_url=base_url,
    )
