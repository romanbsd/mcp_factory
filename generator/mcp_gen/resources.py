from __future__ import annotations

import json

from mcp_gen.models import ResourceSpec, ToolSpec


def build_embedded_resources(
    tools: list[ToolSpec],
    *,
    schema_uri: str,
    schema_name: str,
    schema_description: str,
    schema_mime_type: str,
    schema_text: str,
    tool_index_uri: str = "meta://tools",
    tool_index_name: str = "tools",
) -> list[ResourceSpec]:
    """Build the schema and generated-tool-index resources for one input."""
    return [
        ResourceSpec(
            uri=schema_uri,
            name=schema_name,
            description=schema_description,
            mime_type=schema_mime_type,
            content=schema_text,
        ),
        ResourceSpec(
            uri=tool_index_uri,
            name=tool_index_name,
            description="Generated tool index",
            mime_type="application/json",
            content=json.dumps(
                [{"name": tool.name, "description": tool.description} for tool in tools],
                indent=2,
            ),
        ),
    ]
