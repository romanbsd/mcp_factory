from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

import yaml
from prance import ResolvingParser

from mcp_gen.models import (
    GenerationResult,
    ParamBinding,
    ResourceSpec,
    RestOperation,
    ToolSpec,
    sanitize_tool_name,
    unique_name,
)


def load_openapi(path: Path) -> dict[str, Any]:
    parser = ResolvingParser(str(path), strict=False)
    return parser.specification


def _slugify(text: str) -> str:
    slug = re.sub(r"[^a-zA-Z0-9_]+", "_", text.strip("/").replace("/", "_"))
    slug = re.sub(r"_+", "_", slug).strip("_").lower()
    return slug or "operation"


def _merge_schemas(schemas: list[dict[str, Any]]) -> dict[str, Any]:
    if not schemas:
        return {"type": "object", "properties": {}}
    if len(schemas) == 1:
        return schemas[0]

    properties: dict[str, Any] = {}
    required: list[str] = []
    for schema in schemas:
        for key, value in schema.get("properties", {}).items():
            properties[key] = value
        for key in schema.get("required", []):
            if key not in required:
                required.append(key)
    merged: dict[str, Any] = {"type": "object", "properties": properties}
    if required:
        merged["required"] = required
    return merged


def _merged_parameters(
    path_parameters: list[dict[str, Any]],
    operation_parameters: list[dict[str, Any]],
) -> list[dict[str, Any]]:
    """Merge OpenAPI path-item and operation parameters.

    Operation-level parameters override path-level parameters with the same
    ``(name, in)`` pair.
    """
    merged: dict[tuple[str, str], dict[str, Any]] = {}
    for parameter in [*path_parameters, *operation_parameters]:
        key = (parameter["name"], parameter["in"])
        merged[key] = parameter
    return list(merged.values())


def _as_optional_object_schema(schema: dict[str, Any]) -> dict[str, Any]:
    """Flatten an optional request body without making body fields mandatory."""
    optional = dict(schema)
    optional.pop("required", None)
    return optional


def _request_body_schema(operation: dict[str, Any]) -> dict[str, Any] | None:
    request_body = operation.get("requestBody")
    if not request_body:
        return None
    content = request_body.get("content", {})
    for media_type in ("application/json", "application/*+json"):
        if media_type in content:
            return content[media_type].get("schema")
    first = next(iter(content.values()), None)
    return first.get("schema") if first else None


def _operation_description(operation: dict[str, Any]) -> str:
    parts = [operation.get("summary"), operation.get("description")]
    return "\n\n".join(part for part in parts if part) or "Generated from OpenAPI operation"


def _response_schema(operation: dict[str, Any]) -> dict[str, Any] | None:
    """JSON schema of the first 2xx response body, for the tool's outputSchema."""
    responses = operation.get("responses", {})
    for code in sorted(responses):
        if not str(code).startswith("2"):
            continue
        content = (responses[code] or {}).get("content", {})
        for media_type in ("application/json", "application/*+json"):
            if media_type in content:
                schema = content[media_type].get("schema")
                if schema:
                    return schema
    return None


def _verb_annotations(method: str) -> dict[str, bool]:
    """Map an HTTP verb to MCP behavioral hints (read-only/idempotent/etc.)."""
    m = method.upper()
    return {
        "read_only": m in {"GET", "HEAD", "OPTIONS"},
        "idempotent": m in {"GET", "HEAD", "OPTIONS", "PUT", "DELETE"},
        "destructive": m == "DELETE",
        # Proxying an external API is inherently an open-world interaction.
        "open_world": True,
    }


def parse_openapi(
    path: Path,
    *,
    include_deprecated: bool = False,
    tags: set[str] | None = None,
) -> GenerationResult:
    spec = load_openapi(path)
    tools: list[ToolSpec] = []
    seen_names: set[str] = set()

    for path_name, path_item in spec.get("paths", {}).items():
        for method in ("get", "post", "put", "patch", "delete", "head", "options"):
            if method not in path_item:
                continue
            operation = path_item[method]
            if operation.get("deprecated") and not include_deprecated:
                continue
            if tags and not tags.intersection(operation.get("tags", [])):
                continue

            operation_id = operation.get("operationId") or _slugify(f"{method}_{path_name}")
            tool_name = unique_name(sanitize_tool_name(operation_id), seen_names)
            params: list[ParamBinding] = []
            schema_parts: list[dict[str, Any]] = []

            for parameter in _merged_parameters(
                path_item.get("parameters", []),
                operation.get("parameters", []),
            ):
                name = parameter["name"]
                location = parameter["in"]
                if location not in {"path", "query", "header"}:
                    continue
                params.append(ParamBinding(name=name, location=location))
                schema = parameter.get("schema", {"type": "string"})
                schema_parts.append(
                    {
                        "type": "object",
                        "properties": {name: schema},
                        "required": [name] if parameter.get("required", False) else [],
                    }
                )

            body_schema = _request_body_schema(operation)
            body_fields: list[str] = []
            content_type: str | None = None
            raw_body = False
            if body_schema:
                request_body = operation.get("requestBody", {})
                content = request_body.get("content", {})
                if "application/json" in content:
                    content_type = "application/json"
                elif "application/x-www-form-urlencoded" in content:
                    # Runtime urlencodes the body fields instead of sending JSON.
                    content_type = "application/x-www-form-urlencoded"
                if body_schema.get("properties"):
                    body_fields = list(body_schema["properties"].keys())
                    if request_body.get("required"):
                        schema_parts.append(body_schema)
                    else:
                        schema_parts.append(_as_optional_object_schema(body_schema))
                else:
                    # Array / scalar / free-form body: expose a single `body`
                    # argument sent verbatim rather than silently dropping it.
                    raw_body = True
                    body_fields = ["body"]
                    schema_parts.append(
                        {
                            "type": "object",
                            "properties": {"body": body_schema},
                            "required": ["body"] if request_body.get("required") else [],
                        }
                    )

            annotations = _verb_annotations(method)
            tools.append(
                ToolSpec(
                    name=tool_name,
                    description=_operation_description(operation),
                    input_schema=_merge_schemas(schema_parts),
                    execution_kind="rest",
                    rest=RestOperation(
                        method=method.upper(),
                        path_template=path_name,
                        params=params,
                        body_fields=body_fields,
                        content_type=content_type,
                        raw_body=raw_body,
                    ),
                    title=operation.get("summary"),
                    output_schema=_response_schema(operation),
                    read_only=annotations["read_only"],
                    idempotent=annotations["idempotent"],
                    destructive=annotations["destructive"],
                    open_world=annotations["open_world"],
                )
            )

    schema_text = path.read_text(encoding="utf-8")
    mime_type = "application/yaml" if path.suffix in {".yaml", ".yml"} else "application/json"
    resources = [
        ResourceSpec(
            uri="schema://openapi",
            name="openapi",
            description="Embedded OpenAPI schema",
            mime_type=mime_type,
            content=schema_text,
        ),
        ResourceSpec(
            uri="meta://tools",
            name="tools",
            description="Generated tool index",
            mime_type="application/json",
            content=json.dumps(
                [{"name": tool.name, "description": tool.description} for tool in tools],
                indent=2,
            ),
        ),
    ]

    return GenerationResult(
        tools=tools,
        resources=resources,
        schema_kind="openapi",
        schema_text=schema_text,
    )
