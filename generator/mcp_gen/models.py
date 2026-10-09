from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Any, Literal

_TOOL_NAME_RE = re.compile(r"[^A-Za-z0-9_-]+")


def sanitize_tool_name(name: str) -> str:
    """Coerce a raw operationId/field name into an MCP-safe tool name."""
    cleaned = _TOOL_NAME_RE.sub("_", name).strip("_")
    return cleaned or "tool"


def glob_match(name: str, pattern: str) -> bool:
    """Case-sensitive glob where only `*` and `?` are wildcards, so names with
    brackets such as `fields[apps]` can be matched literally."""
    regex = re.escape(pattern).replace(r"\*", ".*").replace(r"\?", ".")
    return re.fullmatch(regex, name) is not None


def unique_name(name: str, seen: set[str]) -> str:
    """Return a collision-free variant of ``name``, recording it in ``seen``."""
    candidate = name
    counter = 2
    while candidate in seen:
        candidate = f"{name}_{counter}"
        counter += 1
    seen.add(candidate)
    return candidate


@dataclass
class ParamBinding:
    name: str
    location: Literal["path", "query", "header"]


@dataclass
class RestOperation:
    method: str
    path_template: str
    params: list[ParamBinding] = field(default_factory=list)
    body_fields: list[str] = field(default_factory=list)
    content_type: str | None = None
    # When True, the single `body` argument is sent verbatim as the request
    # body (used for array/scalar/free-form request bodies).
    raw_body: bool = False
    # Optional Google Discovery simple media-upload endpoint. Files are
    # resolved relative to an explicit runtime media root and streamed.
    media_path_template: str | None = None
    media_accept: list[str] = field(default_factory=list)
    media_max_size: int | None = None


@dataclass
class GraphQLOperation:
    document: str
    variable_bindings: list[str] = field(default_factory=list)


@dataclass
class ToolSpec:
    name: str
    description: str
    input_schema: dict[str, Any]
    execution_kind: Literal["rest", "graphql"]
    rest: RestOperation | None = None
    graphql: GraphQLOperation | None = None
    # MCP hints threaded to the client (all optional).
    title: str | None = None
    output_schema: dict[str, Any] | None = None
    read_only: bool | None = None
    destructive: bool | None = None
    idempotent: bool | None = None
    open_world: bool | None = None
    # Source operation identifier (OpenAPI operationId) for config matching;
    # not emitted into the generated crate.
    operation_id: str | None = None


@dataclass
class ResourceSpec:
    uri: str
    name: str
    description: str
    mime_type: str
    content: str


@dataclass
class GenerationResult:
    tools: list[ToolSpec]
    resources: list[ResourceSpec]
    schema_kind: Literal["openapi", "graphql", "google_discovery", "composed"]
    # Upstream base URL detected from the schema (OpenAPI servers[0]), used as
    # the default when --base-url is omitted.
    base_url: str | None = None
    # Runtime tool profiles (see --tool-config): tool name -> profile names,
    # and the profiles enabled when config/env do not choose.
    tool_profiles: dict[str, list[str]] = field(default_factory=dict)
    default_profiles: list[str] | None = None
