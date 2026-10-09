from __future__ import annotations

import re
import warnings
from pathlib import Path
from typing import Any

import yaml
from prance import ResolvingParser

from mcp_gen.models import (
    GenerationResult,
    glob_match,
    ParamBinding,
    RestOperation,
    ToolSpec,
    sanitize_tool_name,
    unique_name,
)
from mcp_gen.resources import build_embedded_resources


class OpenAPICompatibilityWarning(UserWarning):
    """An invalid but unambiguous OpenAPI construct was repaired."""


class UnknownOperationError(ValueError):
    """A requested operationId is absent from the schema or filtered out."""


def _coerce_description_placeholders(
    node: Any,
    path: tuple[str, ...] = (),
) -> list[str]:
    """Coerce YAML ``description: {key}`` mappings to literal strings."""
    repaired: list[str] = []
    if isinstance(node, dict):
        for key, value in list(node.items()):
            current_path = (*path, str(key))
            if key == "description" and isinstance(value, dict) and len(value) == 1:
                placeholder, marker = next(iter(value.items()))
                if isinstance(placeholder, str) and marker is None:
                    node[key] = f"{{{placeholder}}}"
                    repaired.append(".".join(current_path))
                    continue
            repaired.extend(_coerce_description_placeholders(value, current_path))
    elif isinstance(node, list):
        for index, value in enumerate(node):
            repaired.extend(
                _coerce_description_placeholders(value, (*path, str(index)))
            )
    return repaired


def _drop_empty_enums(node: Any, path: tuple[str, ...] = ()) -> list[str]:
    """Remove ``enum: []`` (invalid per spec; emitted by App Store Connect)."""
    repaired: list[str] = []
    if isinstance(node, dict):
        if node.get("enum") == []:
            del node["enum"]
            repaired.append(".".join((*path, "enum")))
        for key, value in node.items():
            repaired.extend(_drop_empty_enums(value, (*path, str(key))))
    elif isinstance(node, list):
        for index, value in enumerate(node):
            repaired.extend(_drop_empty_enums(value, (*path, str(index))))
    return repaired


class _CompatibleResolvingParser(ResolvingParser):
    def _validate(self) -> None:
        for repair, what in (
            (_coerce_description_placeholders, "Coerced YAML description placeholder mapping(s) to strings"),
            (_drop_empty_enums, "Dropped empty enum(s)"),
        ):
            repaired = repair(self.specification)
            if repaired:
                warnings.warn(
                    f"{what} before OpenAPI validation: {', '.join(repaired)}",
                    OpenAPICompatibilityWarning,
                    stacklevel=4,
                )
        super()._validate()


def load_openapi(path: Path) -> dict[str, Any]:
    # Recursive schemas (e.g. tree nodes) are cut at the cycle with an
    # unconstrained `{}` schema instead of failing resolution.
    parser = _CompatibleResolvingParser(
        str(path), strict=False, recursion_limit_handler=lambda *_: {}
    )
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


def _operation_description(operation: dict[str, Any], method: str, path_name: str) -> str:
    parts = [operation.get("summary"), operation.get("description")]
    text = "\n\n".join(part for part in parts if part) or derive_description(method, path_name)
    if operation.get("deprecated"):
        text = f"[DEPRECATED] {text}"
    return text


def derive_description(method: str, path_name: str) -> str:
    """Describe an operation from its REST shape when the spec gives no
    summary/description, e.g. ``GET /v1/apps/{id}/builds`` -> "List the
    `builds` related to one `apps` resource.". Resource names are kept
    verbatim (they are usually the JSON:API `type`)."""
    method = method.upper()
    segments = [segment for segment in path_name.strip("/").split("/") if segment]
    # Drop a leading version segment such as "v1".
    if segments and re.fullmatch(r"v\d+", segments[0]):
        segments = segments[1:]
    fallback = f"{method} {path_name}."
    if not segments:
        return fallback

    def is_param(segment: str) -> bool:
        return segment.startswith("{") and segment.endswith("}")

    def owner_of(index: int) -> str:
        """Nearest non-parameter segment before `index`."""
        name = next((s for s in reversed(segments[:index]) if not is_param(s)), "resource")
        return f"`{name}`"

    if "relationships" in segments:
        index = segments.index("relationships")
        owner = owner_of(index)
        related = f"`{segments[index + 1]}`" if index + 1 < len(segments) else "relationship"
        actions = {
            "GET": f"Get the IDs of the {related} linked to one {owner} resource",
            "POST": f"Link additional {related} to one {owner} resource",
            "PATCH": f"Replace the {related} linkage of one {owner} resource",
            "DELETE": f"Unlink {related} from one {owner} resource",
        }
        return f"{actions[method]} (linkage only)." if method in actions else fallback

    if "metrics" in segments and method == "GET":
        index = segments.index("metrics")
        owner = owner_of(index)
        if index + 1 < len(segments):
            return f"Get `{segments[index + 1]}` metrics for one {owner} resource."
        return f"Get metrics for one {owner} resource."

    last = segments[-1]
    if is_param(last):
        resource = owner_of(len(segments) - 1)
        actions = {
            "GET": f"Get one {resource} resource by {last[1:-1]}.",
            "PATCH": f"Update attributes or relationships of one {resource} resource.",
            "PUT": f"Replace one {resource} resource.",
            "DELETE": f"Delete one {resource} resource. This cannot be undone.",
        }
        return actions.get(method, fallback)

    resource = f"`{last}`"
    if len(segments) >= 3 and is_param(segments[-2]):
        owner = owner_of(len(segments) - 2)
        if method == "GET":
            return f"List the {resource} related to one {owner} resource."
        return fallback
    actions = {
        "GET": f"List {resource} resources; supports the filters and pagination parameters below.",
        "POST": f"Create one {resource} resource.",
        "PUT": f"Replace {resource}.",
        "PATCH": f"Update {resource}.",
        "DELETE": f"Delete {resource}. This cannot be undone.",
    }
    return actions.get(method, fallback)


def _detect_base_url(spec: dict[str, Any]) -> str | None:
    """First non-empty `servers[].url`, used as the default upstream base URL."""
    for server in spec.get("servers", []):
        url = (server or {}).get("url")
        if url:
            return url
    return None


def _response_schema(operation: dict[str, Any]) -> dict[str, Any] | None:
    """JSON schema of the first 2xx response body, for the tool's outputSchema.

    Restricted to object-typed responses: MCP ``structuredContent`` must be an
    object, and the runtime only attaches it for object bodies, so declaring an
    array/scalar outputSchema would never be satisfied."""
    responses = operation.get("responses", {})
    for code in sorted(responses):
        if not str(code).startswith("2"):
            continue
        content = (responses[code] or {}).get("content", {})
        for media_type in ("application/json", "application/*+json"):
            if media_type in content:
                schema = content[media_type].get("schema")
                if schema and (schema.get("type") == "object" or "properties" in schema):
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


_READ_ONLY_METHODS = frozenset({"get", "head", "options"})


def _build_param_inputs(
    path_parameters: list[dict[str, Any]],
    operation_parameters: list[dict[str, Any]],
) -> tuple[list[ParamBinding], list[dict[str, Any]]]:
    params: list[ParamBinding] = []
    schema_parts: list[dict[str, Any]] = []

    for parameter in _merged_parameters(path_parameters, operation_parameters):
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

    return params, schema_parts


def _build_body_inputs(
    operation: dict[str, Any],
) -> tuple[list[str], str | None, bool, list[dict[str, Any]]]:
    body_fields: list[str] = []
    content_type: str | None = None
    raw_body = False
    schema_parts: list[dict[str, Any]] = []

    body_schema = _request_body_schema(operation)
    if body_schema is None:
        return body_fields, content_type, raw_body, schema_parts

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
        return body_fields, content_type, raw_body, schema_parts

    # Array / scalar / free-form body: expose a single `body` argument sent
    # verbatim rather than silently dropping it.
    raw_body = True
    body_fields = ["body"]
    schema_parts.append(
        {
            "type": "object",
            "properties": {"body": body_schema},
            "required": ["body"] if request_body.get("required") else [],
        }
    )
    return body_fields, content_type, raw_body, schema_parts


def _build_tool_spec(
    method: str,
    path_name: str,
    operation: dict[str, Any],
    path_parameters: list[dict[str, Any]],
    seen_names: set[str],
) -> ToolSpec:
    operation_id = operation.get("operationId") or _slugify(f"{method}_{path_name}")
    tool_name = unique_name(sanitize_tool_name(operation_id), seen_names)
    params, schema_parts = _build_param_inputs(
        path_parameters=path_parameters,
        operation_parameters=operation.get("parameters", []),
    )
    (
        body_fields,
        content_type,
        raw_body,
        body_schema_parts,
    ) = _build_body_inputs(operation=operation)
    schema_parts.extend(body_schema_parts)

    annotations = _verb_annotations(method)
    return ToolSpec(
        name=tool_name,
        description=_operation_description(operation, method, path_name),
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
        operation_id=operation.get("operationId"),
    )


def parse_openapi(
    path: Path,
    *,
    include_deprecated: bool = False,
    tags: set[str] | None = None,
    operations: set[str] | None = None,
    read_only: bool = False,
) -> GenerationResult:
    spec = load_openapi(path)
    # Entries are exact operationIds or globs (e.g. "betaGroups_*");
    # each must select at least one operation.
    matched_patterns: set[str] = set()
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
            if read_only and method not in _READ_ONLY_METHODS:
                continue
            if operations is not None:
                operation_id = operation.get("operationId") or ""
                hits = {p for p in operations if glob_match(operation_id, p)}
                if not hits:
                    continue
                matched_patterns |= hits
            tools.append(
                _build_tool_spec(
                    method=method,
                    path_name=path_name,
                    operation=operation,
                    path_parameters=path_item.get("parameters", []),
                    seen_names=seen_names,
                )
            )

    if operations is not None and operations - matched_patterns:
        missing = ", ".join(sorted(operations - matched_patterns))
        raise UnknownOperationError(f"operationId(s) not found or filtered out: {missing}")

    schema_text = path.read_text(encoding="utf-8")
    mime_type = (
        "application/yaml"
        if path.suffix in {".yaml", ".yml"}
        else "application/json"
    )
    resources = build_embedded_resources(
        tools,
        schema_uri="schema://openapi",
        schema_name="openapi",
        schema_description="Embedded OpenAPI schema",
        schema_mime_type=mime_type,
        schema_text=schema_text,
    )

    return GenerationResult(
        tools=tools,
        resources=resources,
        schema_kind="openapi",
        base_url=_detect_base_url(spec),
    )
