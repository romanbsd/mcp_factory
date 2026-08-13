from mcp_gen.render import _rust_json_literal, _rust_string_literal


def test_rust_string_literal_preserves_unicode_and_escapes_controls() -> None:
    assert _rust_string_literal('buyer’s "name"\n\b') == (
        '"buyer’s \\"name\\"\\n\\u{8}"'
    )


def test_rust_json_literal_uses_collision_free_raw_delimiter() -> None:
    literal = _rust_json_literal({"description": 'buyer’s "# account'})

    assert literal == 'r##"{"description":"buyer’s \\"# account"}"##'
    assert "\\u2019" not in literal
