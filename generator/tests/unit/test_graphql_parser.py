from pathlib import Path

from mcp_gen.graphql.parser import parse_graphql


def test_parses_sdl_query_and_mutation(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "minimal.graphql")
    assert {tool.name for tool in result.tools} == {"user", "createUser"}
    user = next(tool for tool in result.tools if tool.name == "user")
    assert user.graphql is not None
    assert "user(id: $id)" in user.graphql.document
    # Required arg keeps its `!` so the variable is usable where `ID!` is expected.
    assert "$id: ID!" in user.graphql.document
    # Composite return type gets an explicit scalar subselection.
    assert "{ id name }" in user.graphql.document


def test_preserves_arg_types_and_valid_selection(tmp_path: Path) -> None:
    sdl = tmp_path / "deep.graphql"
    sdl.write_text(
        """
        type Query {
          posts(ids: [ID!]!, first: Int): PostConnection
        }
        type PostConnection { nodes: [Post!]! }
        type Post {
          id: ID!
          author: Author!
          comments(first: Int!): [Comment!]!
        }
        type Author { id: ID! profile: Profile! }
        type Profile { bio: String! }
        type Comment { id: ID! }
        """
    )
    posts = next(t for t in parse_graphql(sdl).tools if t.name == "posts")
    doc = posts.graphql.document
    # Finding 1: list/non-null and nullable arg types are declared correctly.
    assert "$ids: [ID!]!" in doc
    assert "$first: Int" in doc and "$first: Int!" not in doc
    # Finding 2: object fields past the depth limit and arg-requiring fields are
    # skipped, never emitted as invalid bare leaves.
    assert "nodes { id }" in doc
    assert "author" not in doc
    assert "comments" not in doc


def test_parses_nested_input_types(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "nested-inputs.graphql")
    search = next(tool for tool in result.tools if tool.name == "search")
    props = search.input_schema["properties"]["input"]["properties"]
    assert "term" in props and "limit" in props


def test_annotations_enum_and_output_schema(tmp_path: Path) -> None:
    sdl = tmp_path / "enum.graphql"
    sdl.write_text(
        """
        enum Role { ADMIN USER }
        type User { id: ID! role: Role! }
        type Query { user(role: Role!): User }
        type Mutation { deleteUser(id: ID!): Boolean }
        """
    )
    tools = {t.name: t for t in parse_graphql(sdl).tools}

    user = tools["user"]
    # Query → read-only/idempotent.
    assert user.read_only is True
    assert user.idempotent is True
    assert user.open_world is True
    # Enum argument is constrained to its members in the input schema.
    assert user.input_schema["properties"]["role"] == {
        "type": "string",
        "enum": ["ADMIN", "USER"],
    }
    # Output schema mirrors the return object, incl. the enum field.
    out_props = user.output_schema["properties"]
    assert out_props["id"] == {"type": "string"}
    assert out_props["role"] == {"type": "string", "enum": ["ADMIN", "USER"]}

    # Mutation → not read-only.
    assert tools["deleteUser"].read_only is False


def test_parses_introspection_json(fixtures_dir: Path) -> None:
    result = parse_graphql(fixtures_dir / "introspection.json")
    assert len(result.tools) == 1
    assert result.tools[0].name == "user"
