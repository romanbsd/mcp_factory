# mcp-gen

Python CLI for generating Rust MCP proxy servers from OpenAPI, GraphQL, and
Google API Discovery schemas.

Use `mcp-gen generate --input SCHEMA --output DIR` for one schema. Use
`mcp-gen compose --input SCHEMA_A --input SCHEMA_B --output DIR` to merge
several schemas into one server. Run `mcp-gen --help` or
`mcp-gen COMMAND --help` for all filtering, configuration, and packaging flags.
