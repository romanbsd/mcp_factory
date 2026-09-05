from __future__ import annotations

import json
import warnings
from pathlib import Path
from typing import Any

from mcp_gen.resources import build_embedded_resources
from mcp_gen.models import (
    GenerationResult,
    ParamBinding,
    RestOperation,
    ToolSpec,
    sanitize_tool_name,
    unique_name,
)


class GoogleDiscoveryCompatibilityWarning(UserWarning):
    """A Discovery operation could not be represented faithfully and was skipped."""


def load_google_discovery(path: Path) -> dict[str, Any]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, dict) or data.get("kind") != "discovery#restDescription":
        raise ValueError("expected a Google API Discovery REST document")
    return data


def _schema(
    value: dict[str, Any],
    definitions: dict[str, Any],
    seen_refs: tuple[str, ...] = (),
) -> dict[str, Any]:
    if "$ref" in value:
        name = value["$ref"]
        if name in seen_refs:
            return {
                "type": "object",
                "description": f"Recursive reference to Google schema {name}",
            }
        target = definitions.get(name)
        if not isinstance(target, dict):
            return {"type": "object", "description": f"Google schema {name}"}
        return _schema(target, definitions, (*seen_refs, name))

    result: dict[str, Any] = {}
    raw_type = value.get("type")
    if raw_type == "any":
        result["type"] = "object"
    elif raw_type in {"string", "number", "integer", "boolean", "object", "array"}:
        result["type"] = raw_type

    if "description" in value:
        result["description"] = value["description"]
    if "format" in value:
        format_name = value["format"]
        format_map = {
            "date-time": "date-time",
            "google-datetime": "date-time",
            "int32": "int32",
            "uint32": "int64",
            "int64": "int64",
            "uint64": "int64",
            "double": "double",
            "float": "float",
            "byte": "byte",
        }
        if format_name in format_map:
            result["format"] = format_map[format_name]
    if "enum" in value:
        result["enum"] = value["enum"]
    if "default" in value:
        result["default"] = value["default"]
    if value.get("readOnly"):
        result["readOnly"] = True

    properties = value.get("properties")
    if isinstance(properties, dict):
        result["type"] = "object"
        result["properties"] = {
            name: _schema(prop, definitions, seen_refs)
            for name, prop in properties.items()
            if isinstance(prop, dict)
        }
        required = [
            name
            for name, prop in properties.items()
            if isinstance(prop, dict) and prop.get("required")
        ]
        if required:
            result["required"] = required

    additional = value.get("additionalProperties")
    if isinstance(additional, dict):
        result["type"] = "object"
        result["additionalProperties"] = _schema(additional, definitions, seen_refs)

    items = value.get("items")
    if isinstance(items, dict):
        result["items"] = _schema(items, definitions, seen_refs)

    if value.get("repeated"):
        item = dict(result)
        item.pop("description", None)
        result = {"type": "array", "items": item}
        if "description" in value:
            result["description"] = value["description"]

    return result or {}


def _walk_methods(spec: dict[str, Any], prefix: tuple[str, ...] = ()):
    """Yield (resource_path, method_name, method) for every operation.

    Accepts either a full Discovery document (whose ``methods`` hold top-level
    operations and ``resources`` the nested tree) or a resource subtree wrapper
    for recursion.
    """
    methods = spec.get("methods", {})
    if isinstance(methods, dict):
        for method_name, method in methods.items():
            if isinstance(method, dict):
                yield prefix, method_name, method
    resources = spec.get("resources", {})
    if not isinstance(resources, dict):
        return
    for resource_name, resource in resources.items():
        if not isinstance(resource, dict):
            continue
        resource_path = (*prefix, resource_name)
        for method_name, method in resource.get("methods", {}).items():
            if isinstance(method, dict):
                yield resource_path, method_name, method
        nested = resource.get("resources", {})
        if isinstance(nested, dict):
            yield from _walk_methods({"resources": nested}, resource_path)


def _base_url(spec: dict[str, Any]) -> str | None:
    root = spec.get("rootUrl")
    if not isinstance(root, str) or not root:
        return None
    service_path = spec.get("servicePath")
    if isinstance(service_path, str) and service_path:
        return f"{root.rstrip('/')}/{service_path.strip('/')}"
    return root.rstrip("/")


def _is_read_only(method_id: str, http_method: str) -> bool:
    """Classify retrieval-style Google methods, including POST-backed queries."""
    if http_method in {"GET", "HEAD", "OPTIONS"}:
        return True
    action = method_id.rsplit(".", 1)[-1].lower()
    return action in {"query", "search"}


def _collect_tool_parameters(
    method: dict[str, Any],
    definitions: dict[str, Any],
    *,
    body_required: bool = True,
) -> tuple[dict[str, Any], list[ParamBinding], list[str], bool]:
    input_properties: dict[str, Any] = {}
    required: list[str] = []
    params: list[ParamBinding] = []

    parameters = method.get("parameters", {})
    if isinstance(parameters, dict):
        for name, parameter in parameters.items():
            if not isinstance(parameter, dict):
                continue
            location = parameter.get("location")
            if location not in {"path", "query"}:
                continue
            input_properties[name] = _schema(parameter, definitions)
            params.append(ParamBinding(name=name, location=location))
            if parameter.get("required"):
                required.append(name)

    raw_body = False
    request_schema = method.get("request")
    if isinstance(request_schema, dict):
        input_properties["body"] = _schema(request_schema, definitions)
        # Discovery has no optional-body marker. Query/search methods accept an
        # empty request, so only mutations force a body.
        if body_required:
            required.append("body")
        raw_body = True

    input_schema: dict[str, Any] = {
        "type": "object",
        "properties": input_properties,
    }
    if required:
        input_schema["required"] = required
    return input_schema, params, ["body"] if raw_body else [], raw_body


def _method_output_schema(
    method: dict[str, Any], definitions: dict[str, Any]
) -> dict[str, Any] | None:
    response_schema = method.get("response")
    if not isinstance(response_schema, dict):
        return None
    return _schema(response_schema, definitions)


def _to_rest_path_template(
    upstream_base_url: str | None, method: dict[str, Any]
) -> str:
    relative_path = str(method.get("path", ""))
    if upstream_base_url:
        return f"{upstream_base_url.rstrip('/')}/{relative_path.lstrip('/')}"
    return f"/{relative_path.lstrip('/')}"


def _method_to_tool(
    *,
    method_id: str,
    tool_name: str,
    method: dict[str, Any],
    http_method: str,
    upstream_base_url: str | None,
    definitions: dict[str, Any],
    read_only: bool,
) -> ToolSpec:
    semantically_read_only = read_only
    input_schema, params, body_fields, raw_body = _collect_tool_parameters(
        method,
        definitions,
        body_required=not semantically_read_only,
    )
    return ToolSpec(
        name=tool_name,
        description=str(method.get("description") or method_id),
        input_schema=input_schema,
        execution_kind="rest",
        rest=RestOperation(
            method=http_method,
            path_template=_to_rest_path_template(upstream_base_url, method),
            params=params,
            body_fields=body_fields,
            content_type="application/json" if raw_body else None,
            raw_body=raw_body,
        ),
        title=method_id,
        output_schema=_method_output_schema(method, definitions),
        read_only=semantically_read_only,
        destructive=http_method == "DELETE",
        idempotent=semantically_read_only or http_method in {"PUT", "DELETE"},
        open_world=True,
    )


def parse_google_discovery(
    path: Path,
    *,
    tags: set[str] | None = None,
    read_only: bool = False,
) -> GenerationResult:
    spec = load_google_discovery(path)
    upstream_base_url = _base_url(spec)
    definitions = spec.get("schemas", {})
    definitions = definitions if isinstance(definitions, dict) else {}
    tools: list[ToolSpec] = []
    seen_names: set[str] = set()
    skipped_media: list[str] = []

    for resource_path, method_name, method in _walk_methods(spec):
        if tags and not tags.intersection(resource_path):
            # Root-level methods have an empty path and cannot be tagged.
            continue
        http_method = str(method.get("httpMethod", "GET")).upper()
        method_id = str(method.get("id") or ".".join((*resource_path, method_name)))
        semantically_read_only = _is_read_only(method_id, http_method)
        if read_only and not semantically_read_only:
            continue
        if method.get("mediaUpload"):
            skipped_media.append(str(method.get("id") or ".".join((*resource_path, method_name))))
            continue

        api_name_value = spec.get("name")
        if isinstance(api_name_value, str) and api_name_value:
            api_prefix = f"{api_name_value}."
            if method_id.startswith(api_prefix):
                method_id = method_id[len(api_prefix) :]
        tool_name = unique_name(
            sanitize_tool_name(method_id.replace(".", "_")),
            seen_names,
        )
        tools.append(
            _method_to_tool(
                method_id=method_id,
                tool_name=tool_name,
                method=method,
                http_method=http_method,
                upstream_base_url=upstream_base_url,
                definitions=definitions,
                read_only=semantically_read_only,
            )
        )

    if skipped_media:
        warnings.warn(
            "Skipped Google Discovery media-upload operations because mcp-factory "
            f"does not yet model binary request bodies: {', '.join(skipped_media)}",
            GoogleDiscoveryCompatibilityWarning,
            stacklevel=2,
        )

    schema_text = path.read_text(encoding="utf-8")
    api_name = str(spec.get("name") or "api")
    api_version = str(spec.get("version") or "unknown")
    embedded_resources = build_embedded_resources(
        tools=tools,
        schema_uri=f"schema://google-discovery/{api_name}/{api_version}",
        schema_name=f"{api_name}-{api_version}-google-discovery",
        schema_description="Embedded Google API Discovery document",
        schema_mime_type="application/json",
        schema_text=schema_text,
        tool_index_uri=f"meta://tools/{api_name}/{api_version}",
        tool_index_name=f"{api_name}-{api_version}-tools",
    )

    return GenerationResult(
        tools=tools,
        resources=embedded_resources,
        schema_kind="google_discovery",
        base_url=upstream_base_url,
    )
