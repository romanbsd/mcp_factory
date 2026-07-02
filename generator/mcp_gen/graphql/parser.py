from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from graphql import (
    GraphQLArgument,
    GraphQLInputObjectType,
    GraphQLList,
    GraphQLNonNull,
    GraphQLScalarType,
    build_ast_schema,
    build_client_schema,
    parse,
)

from mcp_gen.models import GenerationResult, GraphQLOperation, ResourceSpec, ToolSpec

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


def _selection_for_type(gql_type: Any, depth: int = 2) -> str:
    gql_type = _unwrap_type(gql_type)
    if depth == 0 or not hasattr(gql_type, "fields"):
        return ""
    selections = []
    for name, field in gql_type.fields.items():
        child = _selection_for_type(field.type, depth - 1)
        if child:
            selections.append(f"{name} {{ {child} }}")
        else:
            selections.append(name)
    return " ".join(selections)


def _build_document(operation_type: str, field_name: str, args: dict[str, GraphQLArgument], return_type: Any) -> tuple[str, list[str]]:
    arg_defs = []
    arg_vars = []
    bindings = []
    for name, arg in args.items():
        gql_type = arg.type
        while isinstance(gql_type, GraphQLNonNull):
            gql_type = gql_type.of_type
        type_name = gql_type.name if hasattr(gql_type, "name") else "String"
        arg_defs.append(f"${name}: {type_name}")
        arg_vars.append(f"{name}: ${name}")
        bindings.append(name)
    selection = _selection_for_type(return_type)
    operation = operation_type.lower()
    if arg_defs:
        document = (
            f"{operation}({', '.join(arg_defs)}) {{ {field_name}({', '.join(arg_vars)})"
            f" {{ {selection} }} }}"
        )
    else:
        document = f"{operation} {{ {field_name} {{ {selection} }} }}"
    return document, bindings


def _load_schema(path: Path):
    text = path.read_text(encoding="utf-8")
    if path.suffix == ".json":
        introspection = json.loads(text)
        return build_client_schema(introspection["data"]), text, "application/json"
    return build_ast_schema(parse(text)), text, "text/plain"


def parse_graphql(path: Path) -> GenerationResult:
    schema, schema_text, mime_type = _load_schema(path)
    tools: list[ToolSpec] = []

    for operation_type in ("query", "mutation"):
        root = schema.query_type if operation_type == "query" else schema.mutation_type
        if not root:
            continue
        gql_operation = "query" if operation_type == "query" else "mutation"
        for field_name, field in root.fields.items():
            document, bindings = _build_document(
                gql_operation, field_name, field.args, field.type
            )
            tools.append(
                ToolSpec(
                    name=field_name,
                    description=field.description or f"GraphQL {operation_type} {field_name}",
                    input_schema=_field_input_schema(field.args),
                    execution_kind="graphql",
                    graphql=GraphQLOperation(document=document, variable_bindings=bindings),
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
