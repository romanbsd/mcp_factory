# Repository instructions

<!-- codebase-memory-mcp:start -->
For structural codebase exploration, use the installed `codebase-memory` skill.
<!-- codebase-memory-mcp:end -->

## Keep architectural boundaries intact

- Keep `mcp-factory-core` provider agnostic. Do not add vendor SDK dependencies, provider-specific auth/config variants, credential environment variables, API method allowlists, or service/business rules to the core.
- Put vendor authentication in optional adapter crates and wire it from the owning server through the generic `AuthProvider` interface. Dependency direction must remain server/adapter -> core; core must never depend on a server or vendor adapter. A missing configured adapter must fail closed.
- Keep service-specific reporting, interpretation, defaults, safety classifications, and operation selection in that server's handwritten code or generation policy. Shared parsers may understand a schema format, but must not embed named-service behavior.
- Add generic extension points only when they are useful beyond one provider. Keep generated API code authoritative and handwritten extensions regeneration-safe; do not patch generated output as the permanent solution.
- When changing a shared interface, migrate its consumers together, document configuration changes, and verify boundary behavior and regeneration. Do not weaken these boundaries to save implementation effort.
