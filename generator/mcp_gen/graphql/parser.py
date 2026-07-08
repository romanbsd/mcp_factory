from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from graphql import (
    GraphQLArgument,
    GraphQLEnumType,
    GraphQLInputObjectType,
    GraphQLList,
    GraphQLNonNull,
    GraphQLScalarType,
    Undefined,
    build_ast_schema,
    build_client_schema,
    parse,
)

from mcp_gen.models import (
    GenerationResult,
    GraphQLOperation,
    ResourceSpec,
    ToolSpec,
    sanitize_tool_name,
    unique_name,
)

SCALAR_TO_JSON: dict[str, dict[str, str]] = {
    "String": {"type": "string"},
    "ID": {"type": "string"},
    "Int": {"type": "integer"},
    "Float": {"type": "number"},
    "Boolean": {"type": "boolean"},
}


def _unwrap_type(gql_type: Any) -> Any:
    while isinstance(gql_type, (GraphQLNonNull, GraphQLList)):
        gql_type = gql_type.of_type
    return gql_type


def _type_to_schema(gql_type: Any) -> dict[str, Any]:
    if isinstance(gql_type, GraphQLNonNull):
        return _type_to_schema(gql_type.of_type)
    if isinstance(gql_type, GraphQLList):
        return {"type": "array", "items": _type_to_schema(gql_type.of_type)}
    if isinstance(gql_type, GraphQLScalarType):
        return SCALAR_TO_JSON.get(gql_type.name, {"type": "string"})
    if isinstance(gql_type, GraphQLEnumType):
        # Constrain to the enum's members so the client picks a valid value.
        return {"type": "string", "enum": list(gql_type.values.keys())}
    if isinstance(gql_type, GraphQLInputObjectType):
        properties: dict[str, Any] = {}
        required: list[str] = []
        for name, field in gql_type.fields.items():
            properties[name] = _type_to_schema(field.type)
            if isinstance(field.type, GraphQLNonNull):
                required.append(name)
        schema: dict[str, Any] = {"type": "object", "properties": properties}
        if required:
            schema["required"] = required
        return schema
    return {"type": "string"}


def _field_input_schema(arguments: dict[str, GraphQLArgument]) -> dict[str, Any]:
    properties: dict[str, Any] = {}
    required: list[str] = []
    for name, arg in arguments.items():
        properties[name] = _type_to_schema(arg.type)
        if isinstance(arg.type, GraphQLNonNull):
            required.append(name)
    schema: dict[str, Any] = {"type": "object", "properties": properties}
    if required:
        schema["required"] = required
    return schema


def _is_leaf(gql_type: Any) -> bool:
    return isinstance(_unwrap_type(gql_type), (GraphQLScalarType, GraphQLEnumType))


def _requires_args(field: Any) -> bool:
    """True if the field has a required argument we cannot supply in a selection."""
    return any(
        isinstance(arg.type, GraphQLNonNull) and arg.default_value is Undefined
        for arg in getattr(field, "args", {}).values()
    )


def _selection_for_type(gql_type: Any, depth: int = 2) -> str:
    """Inner selection set for a composite type, or "" if nothing is selectable.

    Leaf (scalar/enum) fields are emitted bare. Composite fields are only
    emitted when we can expand them into a non-empty subselection — at the depth
    limit, on unions (no ``fields``), or when a field needs arguments, they are
    skipped rather than emitted as an invalid bare object field.
    """
    gql_type = _unwrap_type(gql_type)
    fields = getattr(gql_type, "fields", None)
    if not fields or depth <= 0:
        return ""
    selections = []
    for name, field in fields.items():
        if _requires_args(field):
            continue
        if _is_leaf(field.type):
            selections.append(name)
        else:
            child = _selection_for_type(field.type, depth - 1)
            if child:
                selections.append(f"{name} {{ {child} }}")
    return " ".join(selections)


def _output_schema(gql_type: Any, depth: int = 3) -> dict[str, Any]:
    """Approximate JSON schema of a field's return type for the tool's
    outputSchema. Bounded by ``depth`` to stay finite on cyclic graphs; unions
    and the depth limit collapse to a loose object."""
    if isinstance(gql_type, GraphQLNonNull):
        return _output_schema(gql_type.of_type, depth)
    if isinstance(gql_type, GraphQLList):
        return {"type": "array", "items": _output_schema(gql_type.of_type, depth)}
    if isinstance(gql_type, GraphQLScalarType):
        return SCALAR_TO_JSON.get(gql_type.name, {"type": "string"})
    if isinstance(gql_type, GraphQLEnumType):
        return {"type": "string", "enum": list(gql_type.values.keys())}
    fields = getattr(gql_type, "fields", None)
    if fields and depth > 0:
        properties = {
            name: _output_schema(field.type, depth - 1)
            for name, field in fields.items()
            if not _requires_args(field)
        }
        return {"type": "object", "properties": properties}
    return {"type": "object"}


def _build_document(operation_type: str, field_name: str, args: dict[str, GraphQLArgument], return_type: Any) -> tuple[str, list[str]]:
    arg_defs = []
    arg_vars = []
    bindings = []
    for name, arg in args.items():
        # str(arg.type) yields the full SDL type reference, preserving `!` and
        # `[...]` so required and list variables are declared correctly.
        arg_defs.append(f"${name}: {arg.type}")
        arg_vars.append(f"{name}: ${name}")
        bindings.append(name)

    call = f"{field_name}({', '.join(arg_vars)})" if arg_vars else field_name
    if _is_leaf(return_type):
        field_selection = call
    else:
        # __typename is a valid selection on any object/interface/union, so we
        # always produce a well-formed query even when nothing else expands.
        selection = _selection_for_type(return_type) or "__typename"
        field_selection = f"{call} {{ {selection} }}"

    operation = operation_type.lower()
    if arg_defs:
        document = f"{operation}({', '.join(arg_defs)}) {{ {field_selection} }}"
    else:
        document = f"{operation} {{ {field_selection} }}"
    return document, bindings


def _load_schema(path: Path):
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".json":
        introspection = json.loads(text)
        # Accept both the {"data": {"__schema": ...}} response envelope and a
        # bare {"__schema": ...} introspection dump.
        data = introspection.get("data", introspection)
        return build_client_schema(data), text, "application/json"
    return build_ast_schema(parse(text)), text, "text/plain"


def parse_graphql(path: Path) -> GenerationResult:
    schema, schema_text, mime_type = _load_schema(path)
    tools: list[ToolSpec] = []
    seen_names: set[str] = set()

    for operation_type in ("query", "mutation"):
        root = schema.query_type if operation_type == "query" else schema.mutation_type
        if not root:
            continue
        gql_operation = "query" if operation_type == "query" else "mutation"
        for field_name, field in root.fields.items():
            document, bindings = _build_document(
                gql_operation, field_name, field.args, field.type
            )
            tool_name = unique_name(sanitize_tool_name(field_name), seen_names)
            is_query = operation_type == "query"
            tools.append(
                ToolSpec(
                    name=tool_name,
                    description=field.description or f"GraphQL {operation_type} {field_name}",
                    input_schema=_field_input_schema(field.args),
                    execution_kind="graphql",
                    graphql=GraphQLOperation(document=document, variable_bindings=bindings),
                    output_schema=_output_schema(field.type),
                    # Queries are read-only/idempotent; mutations are neither.
                    read_only=is_query,
                    idempotent=is_query,
                    open_world=True,
                )
            )

    resources = [
        ResourceSpec(
            uri="schema://graphql",
            name="graphql",
            description="Embedded GraphQL schema",
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
        schema_kind="graphql",
        schema_text=schema_text,
    )
