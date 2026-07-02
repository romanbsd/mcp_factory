from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Literal


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
    schema_kind: Literal["openapi", "graphql"]
    schema_text: str
