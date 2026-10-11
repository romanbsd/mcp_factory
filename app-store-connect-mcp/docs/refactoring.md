# DRY refactoring

This follow-up implements the eight reviewed simplifications without changing the public workflow schemas or profile membership. The generated Apple capability layer remains authoritative; regeneration still owns generated files.

| Simplification | Implementation |
|---|---|
| Shared upload inspection and resume reconciliation | `upload_session.rs` owns file identity, streamed hashing, typed ranges, Apple delivery state and checkpoints. One shared `AssetUpload` instance serves direct uploads and screenshot workflows. Its resume inspector reconciles remote identity/state for dry runs and uploads. |
| Declarative workflow registry | `WorkflowKind` has one definition for each name, description, schema constructor and safety hint, plus exhaustive dispatch. Common selector/schema helpers replace duplicated declarations. |
| Shared localization discovery | `Asc::localizations` constructs sparse fieldsets and locale filters for metadata, screenshots and release preflight. Callers retain their own missing/ambiguous-resource policies. |
| Shared runtime validation | `resolve_and_validate` centralizes operation selection and argument validation. Public exposure and internal permission checks remain explicit; permission checks run before schema validation. |
| Shared transfer primitives | Core `transfers.rs` owns the process-wide budget, default report/error bounds, incremental plain/gzip reader and bounded error prefixes. Partial response-body failures retain useful Apple diagnostics without exposing request URLs. |
| Composable raw profiles | Generator `[profile_extends]` configuration composes existing domain profiles and custom-tool membership. Unknown parents and cycles fail validation. An independent frozen fixture checks all 293 tool memberships. |
| Typed workflow change reports | `contracts.rs` centralizes the localized metadata field list and serializable metadata, screenshot and TestFlight changes. Missing metadata fields remain untouched; explicit null clears supported fields. |
| Shared mock-server setup | `test_support.rs` registers the current capability catalog, custom tools, profiles and workflow resources. Individual scenarios and assertions remain explicit. Legacy eager-registration coverage remains separate. |

## Boundaries retained

A screenshot dry run validates local files and existing reservations without mutation. The actual upload revalidates file identity and remote state at the mutation boundary: that second check is intentional protection against changes between discovery and upload. Screenshot-set membership checks remain in the screenshot service.

Checkpoints preserve legacy minimal inputs while rejecting malformed supplied fields. After reservation, failures retain resumable state. Typed delivery states preserve unknown Apple state strings rather than silently treating them as complete.

Transfer consumers still own their permits and compressed/decompressed limits. Sharing gzip decoding does not move expensive work outside its concurrency guard. Error prefixes stay bounded to 2KiB, including lossy UTF-8 expansion; a prematurely closed response retains the prefix and a URL-free read diagnostic.

Internal operations remain subject to read-only/destructive restrictions. Refactoring operation lookup does not make hidden capabilities public or allow a read-only workflow to invoke a write.

## Validation

Before/after snapshots show exact equality of all 15 workflow tool names, descriptions, schemas and safety hints. Actual stdio discovery remains 15 tools / 14,777 compact bytes; every measured raw profile has unchanged counts and listing bytes. All 293 operation/custom-tool profile memberships match the independent pre-refactor fixture.

New tests exercise checkpoint compatibility and malformed inputs, file/hash/parent/range reconciliation, read-only resume inspection, workflow registry contracts, sparse localization discovery, omitted-versus-null metadata behavior, short/corrupt/concatenated gzip streams, byte-bounded UTF-8 error prefixes and premature HTTP body closure. Profile inheritance tests cover composition, deduplication, invalid references and cycles.

The complete-task mock measurements also remain unchanged: release status, metadata updates, screenshot sync and TestFlight distribution each take one public workflow call. See [validation.md](validation.md) for full check results and [measurements.json](measurements.json) for measurements. This refactoring has no measured RSS or CPU speedup claim.
