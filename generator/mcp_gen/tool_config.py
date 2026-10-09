"""Per-server tool curation loaded from ``--tool-config`` (TOML).

```toml
default_profiles = ["core", "metadata"]   # exposed when config/env don't choose
cursor_param = "cursor"                    # optional; see apply_tool_config
strip_enums = ["fields[*]"]                # parameter-name globs; drop enum lists

[profiles]                                 # operationId globs (* and ?)
core = ["apps_getCollection", "builds_*"]

[custom_tools]                             # handwritten tools -> profiles
asset_upload = ["assets"]

[descriptions]                             # generated tool name -> text
apps_getCollection = "Find an app by bundle ID ..."
```
"""

from __future__ import annotations

import json
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

from mcp_gen.models import GenerationResult, ParamBinding, glob_match

_KEYS = {
    "default_profiles",
    "cursor_param",
    "strip_enums",
    "profiles",
    "custom_tools",
    "descriptions",
}


class ToolConfigError(ValueError):
    """The tool config is malformed or does not match the schema."""


@dataclass
class ToolConfig:
    profiles: dict[str, list[str]]
    default_profiles: list[str] | None = None
    custom_tools: dict[str, list[str]] = field(default_factory=dict)
    descriptions: dict[str, str] = field(default_factory=dict)
    cursor_param: str | None = None
    strip_enums: list[str] = field(default_factory=list)

    @property
    def operation_patterns(self) -> set[str]:
        return {pattern for patterns in self.profiles.values() for pattern in patterns}


def _string_list(value: object, where: str) -> list[str]:
    if not isinstance(value, list) or not all(isinstance(item, str) and item for item in value):
        raise ToolConfigError(f"{where} must be a list of non-empty strings")
    return list(value)


def load_tool_config(path: Path) -> ToolConfig:
    try:
        data = tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise ToolConfigError(f"cannot read tool config {path}: {error}") from error
    unknown = set(data) - _KEYS
    if unknown:
        raise ToolConfigError(f"unknown tool config key(s): {', '.join(sorted(unknown))}")

    raw_profiles = data.get("profiles")
    if not isinstance(raw_profiles, dict) or not raw_profiles:
        raise ToolConfigError("[profiles] must define at least one profile")
    if "all" in raw_profiles:
        raise ToolConfigError('"all" is reserved and cannot be defined as a profile')
    profiles = {
        name: _string_list(patterns, f"profiles.{name}") for name, patterns in raw_profiles.items()
    }

    def known_profiles(names: list[str], where: str) -> list[str]:
        unknown = [name for name in names if name != "all" and name not in profiles]
        if unknown:
            raise ToolConfigError(f"{where} names undefined profile(s): {', '.join(unknown)}")
        return names

    default_profiles = None
    if "default_profiles" in data:
        default_profiles = known_profiles(
            _string_list(data["default_profiles"], "default_profiles"), "default_profiles"
        )
        if not default_profiles:
            raise ToolConfigError(
                "default_profiles must not be empty; omit it to expose every tool by default"
            )

    raw_custom = data.get("custom_tools", {})
    if not isinstance(raw_custom, dict):
        raise ToolConfigError("[custom_tools] must be a table of tool = [profiles]")
    custom_tools = {}
    for name, names in raw_custom.items():
        custom_tools[name] = known_profiles(
            _string_list(names, f"custom_tools.{name}"), f"custom_tools.{name}"
        )

    descriptions = data.get("descriptions", {})
    if not isinstance(descriptions, dict):
        raise ToolConfigError("[descriptions] must be a table of tool = text")
    if not all(isinstance(text, str) and text.strip() for text in descriptions.values()):
        raise ToolConfigError("[descriptions] values must be non-empty strings")

    cursor_param = data.get("cursor_param")
    if cursor_param is not None and (not isinstance(cursor_param, str) or not cursor_param):
        raise ToolConfigError("cursor_param must be a non-empty string")

    strip_enums = _string_list(data.get("strip_enums", []), "strip_enums")

    return ToolConfig(
        profiles=profiles,
        strip_enums=strip_enums,
        default_profiles=default_profiles,
        custom_tools=custom_tools,
        descriptions=dict(descriptions),
        cursor_param=cursor_param,
    )


def apply_tool_config(result: GenerationResult, config: ToolConfig) -> None:
    """Annotate parsed tools with profiles, description overrides and the
    cursor pagination parameter. The parser has already restricted the tools
    to ``config.operation_patterns``."""
    by_name = {tool.name: tool for tool in result.tools}

    unknown = sorted(set(config.descriptions) - set(by_name))
    if unknown:
        raise ToolConfigError(f"[descriptions] name unknown tool(s): {', '.join(unknown)}")
    clashing = sorted(set(config.custom_tools) & set(by_name))
    if clashing:
        raise ToolConfigError(f"[custom_tools] clash with generated tool(s): {', '.join(clashing)}")

    tool_profiles: dict[str, list[str]] = {}
    for tool in result.tools:
        operation_id = tool.operation_id or tool.name
        tool_profiles[tool.name] = [
            profile
            for profile, patterns in config.profiles.items()
            if any(glob_match(operation_id, pattern) for pattern in patterns)
        ]
        if tool.name in config.descriptions:
            tool.description = config.descriptions[tool.name].strip()
        if config.cursor_param:
            _add_cursor_param(tool, config.cursor_param)
        if config.strip_enums:
            _strip_param_enums(tool, config.strip_enums)
    tool_profiles.update(config.custom_tools)
    result.tool_profiles = tool_profiles
    result.default_profiles = config.default_profiles
    _refresh_tool_index(result)


def _add_cursor_param(tool, name: str) -> None:
    """Paginated collection GETs (those taking `limit`) also accept the opaque
    cursor that the API puts in `links.next`."""
    rest = tool.rest
    if rest is None or rest.method != "GET":
        return
    query = {param.name for param in rest.params if param.location == "query"}
    if "limit" not in query or name in query:
        return
    rest.params.append(ParamBinding(name=name, location="query"))
    tool.input_schema.setdefault("properties", {})[name] = {
        "type": "string",
        "description": f"Next page: the `{name}` value from `links.next` (none = last page).",
    }


def _strip_param_enums(tool, patterns: list[str]) -> None:
    """Drop enum lists (e.g. every attribute name a sparse fieldset accepts)
    from matching parameters; the parameter itself stays usable."""
    for name, schema in tool.input_schema.get("properties", {}).items():
        if any(glob_match(name, pattern) for pattern in patterns):
            for node in (schema, schema.get("items") if isinstance(schema, dict) else None):
                if isinstance(node, dict):
                    node.pop("enum", None)


def _refresh_tool_index(result: GenerationResult) -> None:
    """Keep the `meta://tools` index in step with overridden descriptions."""
    for resource in result.resources:
        if resource.uri.startswith("meta://tools"):
            resource.content = json.dumps(
                [{"name": tool.name, "description": tool.description} for tool in result.tools],
                indent=2,
            )
