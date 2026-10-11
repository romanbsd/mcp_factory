# Implementation and validation

All three implementation stages, the [DRY follow-up](refactoring.md) and the [bounded-discovery iteration](iteration2.md) are complete. Automated validation used the local bundled Apple specification and mock HTTP APIs. Subsequent user-authorized live read-only and dry-run checks are recorded in [live-validation.md](live-validation.md); no live resources were changed.

## Architecture and source ownership

The public MCP interface delegates to reusable app/version/build resolution, metadata, screenshot, TestFlight, release/review, upload and report services. Services invoke generated internal operations through `OperationInvoker`; they never serialize an MCP-to-MCP call. Generated request validation, authentication/JWT caching, reusable HTTP clients and mutation scope checks remain in the shared runtime. `ReadOnlyToolInvoker` remains a compatibility alias.

The runtime stores lightweight operation descriptors separately from public exposure. Hidden schemas and validators initialize on first internal use. Public schemas initialize for selected profiles, then their sorted MCP metadata is cached. Generated tools are constructed in their registry by ownership; the legacy eager `build_tools` convenience function is retained for compatibility/tests. Generator templates, parameter serialization and nullability translation are the source of truth; ASC output was regenerated.

Repository inspection found **275 generated operations plus two handwritten helpers** in the previous 277-tool `all` surface. This iteration preserves all 275 operation IDs and adds two documented TestFlight reads needed for workflows, yielding **277 generated capabilities plus 16 handwritten tools** in `all`.

## Workflow contracts and profiles

The default tools are `app_get`, `version_get`, `release_status`, `release_prepare`, `metadata_get`, `metadata_apply`, `screenshots_get`, `screenshots_sync`, `asset_upload`, `testflight_status`, `testflight_distribute`, `testflight_remove`, `review_submit`, `release`, and `report_query`.

Exact measured MCP schemas are exported in [workflow-tools.json](workflow-tools.json); the [workflow reference](workflows.md) documents inputs, dry runs, continuations, checkpoint reuse and examples. Mutating reconcilers support `dryRun`; `asset_upload` is the direct reservation/upload protocol. Raw/domain profiles remain, with `release_raw`, `testflight_raw`, `monetization_raw`, `signing_raw` and `reports_raw` supplying lookup dependencies. Composite profiles such as `workflow,testflight_raw` are supported.

Operation reference is available through `schema://operation/<operationId>`; five concise `docs://workflow/*` guides complement the retained full-spec resource.

## Transfers and reports

Uploads stream bounded ranges from files with 64KiB buffers and explicit Content-Length. File/range sizes cap at 2GiB and upstream transfer errors at 2KiB. Successful reservations produce checkpoints containing asset ID, parent/type, relative file path/name, size, MD5, ranges, completion indices and commit/processing state. Subsequent failures preserve these checkpoints. Resume rehashes the local file and reconciles Apple identity, parent, range plan and delivery state before uploading or committing. It does not reserve another resource.

Uploads, report download/decoding and advanced raw binary response handling share two process-wide permits. Blocking decoding retains its permit even if its waiting async call is canceled. JSON reservation/commit requests stay outside the transfer budget to avoid nested acquisition. Advanced raw responses preserve their prior binary/text compatibility and 64MiB compressed/decompressed caps.

`report_query` supports analytics segment/report IDs and sales/finance period downloads. It defaults to summary; exact filters, projection, grouping and numeric sums operate incrementally on CSV/TSV. Downloads spool to bounded temporary files. Limits include 256MiB downloaded per segment/response, 256MiB decoded per query, 1MiB logical records, 64KiB cells, 100 groups, 200 inline rows and 20KB output. Artifact mode exports full matching projected rows to bounded local temporary CSVs; the last eight are retained. Raw analytics preview decoding is incremental and retains only the requested prefix.

## Measured MCP surface

The supplied previous default was **137 tools / 126,861 bytes**; the measured default is **15 tools / 14,777 bytes** (about 88.4% fewer listing bytes). Measurements are compact serialized `tools/list` result objects, excluding the JSON-RPC envelope. Ordering and identical repeated listings were checked over actual stdio MCP.

| Profile | Tools | Compact result bytes |
|---|---:|---:|
| `workflow` | 15 | 14,777 |
| `core` | 31 | 47,845 |
| `release_raw` | 85 | 99,777 |
| `testflight_raw` | 85 | 87,163 |
| `monetization_raw` | 126 | 140,779 |
| `signing_raw` | 57 | 71,049 |
| `reports_raw` | 44 | 57,127 |
| `all` | 293 | 282,490 |

The supplied previous `core` and `all` sizes were 43,107 and 249,123 bytes respectively. Their expanded raw listings grew: corrected OpenAPI nullability/query encoding metadata and added workflows/capabilities remain explicit in raw profiles. The default reduction comes from workflow exposure, rather than globally dropping compatibility representations.

## Representative complete tasks

These fixtures run both raw public dispatch and one workflow dispatch against the same local HTTP scenarios. They verify completion state: release inspection, one metadata patch, one screenshot upload/order reconciliation, and one missing TestFlight group assignment. Processing is immediately COMPLETE in the screenshot fixture; live Apple timing and polling vary.

| Task | Raw public calls | Workflow public calls | Compact workflow summary bytes |
|---|---:|---:|---:|
| `release_status` | 6 | 1 | 435 |
| `metadata_apply` | 4 | 1 | 145 |
| `screenshots_sync` | 7 | 1 | 592 |
| `testflight_distribute` | 6 | 1 | 151 |

These are measured choreography comparisons, not historical telemetry. Inline summary bytes count one structured payload; MCP still supplies compatible text and structuredContent. The report benchmark returned 134-byte summaries.

## Resource observation

During the initial workflow implementation, before the DRY follow-up, the debug MCP process handled two simultaneous gzip report queries, each decoding 16,000,016 bytes and aggregating 3,200,000 rows. Peak RSS was **30,965,760 bytes (29.5MiB)**; wall time **2.16s**, user CPU **4.07s**, system CPU **0.07s**. `/usr/bin/time -l` measured the MCP process; the fixture server ran outside that process. There is no comparable pre-change RSS/CPU measurement and no claimed speedup. This benchmark covers report processing, while the sparse upload test verifies the bounded upload stream independently.

Reproduce discovery with `python3 scripts/measure-surface.py`; run `python3 scripts/benchmark-transfers.py` on macOS after building. See [measurements.json](measurements.json) for machine-readable results.

## Tests and results

| Check | Result |
|---|---|
| `cargo test --workspace --offline` | 293 passed, zero failures: ASC 104, Google Play 86, core unit 63, core integration 40 |
| Full generator pytest with coverage | 154 passed; 91.72% branch-inclusive coverage, above 85% gate |
| `cargo clippy --workspace --all-targets --offline -- -D warnings` | Passed |
| `cargo fmt --all -- --check` | Passed |
| Opt-in live-runner stdlib unit tests | 23 passed, zero failures |
| `CARGO_NET_OFFLINE=true bash tests/e2e/run.sh` | Passed OpenAPI/GraphQL generation, compilation, binary packaging and portable config launch |
| ASC regeneration/hash comparison | Passed; generated source/config/manifest and preserved Cargo dependencies unchanged |
| Actual stdio surface measurement | Passed sorted and repeated-identical listings; all 15 workflow schemas/hints and all profile counts/bytes unchanged by DRY refactoring |
| `git diff --check` | Passed |

The original ASC suite remains, with expectations adjusted for the intentional default/profile changes and correct nullable enums. Added ASC coverage comprises nine upload tests, eleven report tests, ten screenshot tests, nine metadata/TestFlight tests, seven upload-session tests, ten release integration tests, four surface/dependency contracts and one complete-task metric test. Shared runtime tests cover lazy hidden capabilities and mutation/read-only enforcement, plus four query-encoding/transfer-budget cases. Four additional core transfer tests cover incremental decoding and bounded/partial error diagnostics. Generator tests cover lazy catalogs, extension resources, nullability translation, non-exploded array queries and profile inheritance with an independent membership fixture.

The bounded-discovery follow-up adds 15 ASC tests for encoded byte budgets, generated profile selection, multi-page early stopping/continuation, partial-write reconciliation and typed release states.

The fixtures cover no-op/create/update/dry-run reconciliation, wrong identity/hash/range rejection, post-reservation failures, resume without duplication, processing deadlines, a sparse 64MiB bounded stream, bounded error bodies/decompression/records/output, cancellation-safe permits, release blockers, draft/submitted/archived review reconciliation and raw-profile preservation. Contract tests assert generated operation availability, HTTP method, required parameter capabilities and safety classification, causing Apple spec drift to fail tests.

## Limits and follow-up

- Live Apple reads and dry-run workflow paths passed on the three available apps. Uploads, report downloads, write permissions and final review validation remain unverified. Preflight reports known blockers separately from unknown App Privacy, monetization, age-answer completeness and server/device requirements; no-known-blockers does not guarantee submission acceptance.
- Metadata workflows cover App Store version localized fields, not App Info name/subtitle; use `metadata`/`release_raw` for those fields. Screenshot sync handles one locale/display type per invocation and requires an existing version localization.
- Analytics needs an existing segment/report ID; report-request discovery/creation remains raw. `dateRange` selects analytics processingDate, not dates within rows. Filters are exact; numeric sums use finite `f64`, without accounting-grade decimal semantics or currency conversion.
- Artifacts are local to the server, expire on eviction/process exit, and are not remotely hosted download URLs. Checkpoints must be retained by the caller; there is no durable crash-recovery session database. Encryption-document resume refuses to proceed when Apple's response lacks a verifiable parent link.
- Transfer permits and limits are fixed. Configurable budgets, production fixtures/contract smoke checks, optional durable sessions, and a release-build upload RSS benchmark are useful follow-up work. Existing raw report compatibility can still return large explicitly requested content; use `report_query` for normal agent tasks.
- Larger metadata inventories require locale/field selection; screenshots and TestFlight status expose continuation. Unusual oversized fields and report/group/transfer limits return explicit errors rather than silently dropping data.

## Files changed

- [Cargo.lock](../../Cargo.lock)
- [app-store-connect-mcp/Cargo.toml](../../app-store-connect-mcp/Cargo.toml)
- [app-store-connect-mcp/README.md](../../app-store-connect-mcp/README.md)
- [app-store-connect-mcp/config.toml](../../app-store-connect-mcp/config.toml)
- [app-store-connect-mcp/docs/measurements.json](../../app-store-connect-mcp/docs/measurements.json)
- [app-store-connect-mcp/docs/validation.md](../../app-store-connect-mcp/docs/validation.md)
- [app-store-connect-mcp/docs/workflow-tools.json](../../app-store-connect-mcp/docs/workflow-tools.json)
- [app-store-connect-mcp/docs/workflows.md](../../app-store-connect-mcp/docs/workflows.md)
- [app-store-connect-mcp/mcp-gen.manifest.json](../../app-store-connect-mcp/mcp-gen.manifest.json)
- [app-store-connect-mcp/scripts/benchmark-transfers.py](../../app-store-connect-mcp/scripts/benchmark-transfers.py)
- [app-store-connect-mcp/scripts/measure-surface.py](../../app-store-connect-mcp/scripts/measure-surface.py)
- [app-store-connect-mcp/scripts/regenerate.sh](../../app-store-connect-mcp/scripts/regenerate.sh)
- [app-store-connect-mcp/src/extensions.rs](../../app-store-connect-mcp/src/extensions.rs)
- [app-store-connect-mcp/src/extensions/analytics.rs](../../app-store-connect-mcp/src/extensions/analytics.rs)
- [app-store-connect-mcp/src/extensions/asset_upload.rs](../../app-store-connect-mcp/src/extensions/asset_upload.rs)
- [app-store-connect-mcp/src/extensions/contract_tests.rs](../../app-store-connect-mcp/src/extensions/contract_tests.rs)
- [app-store-connect-mcp/src/extensions/metadata.rs](../../app-store-connect-mcp/src/extensions/metadata.rs)
- [app-store-connect-mcp/src/extensions/metrics_tests.rs](../../app-store-connect-mcp/src/extensions/metrics_tests.rs)
- [app-store-connect-mcp/src/extensions/release.rs](../../app-store-connect-mcp/src/extensions/release.rs)
- [app-store-connect-mcp/src/extensions/release_tests.rs](../../app-store-connect-mcp/src/extensions/release_tests.rs)
- [app-store-connect-mcp/src/extensions/report_tests.rs](../../app-store-connect-mcp/src/extensions/report_tests.rs)
- [app-store-connect-mcp/src/extensions/reports.rs](../../app-store-connect-mcp/src/extensions/reports.rs)
- [app-store-connect-mcp/src/extensions/screenshot_tests.rs](../../app-store-connect-mcp/src/extensions/screenshot_tests.rs)
- [app-store-connect-mcp/src/extensions/screenshots.rs](../../app-store-connect-mcp/src/extensions/screenshots.rs)
- [app-store-connect-mcp/src/extensions/services.rs](../../app-store-connect-mcp/src/extensions/services.rs)
- [app-store-connect-mcp/src/extensions/testflight.rs](../../app-store-connect-mcp/src/extensions/testflight.rs)
- [app-store-connect-mcp/src/extensions/tests.rs](../../app-store-connect-mcp/src/extensions/tests.rs)
- [app-store-connect-mcp/src/extensions/upload_tests.rs](../../app-store-connect-mcp/src/extensions/upload_tests.rs)
- [app-store-connect-mcp/src/extensions/workflow_leaf_tests.rs](../../app-store-connect-mcp/src/extensions/workflow_leaf_tests.rs)
- [app-store-connect-mcp/src/extensions/workflows.rs](../../app-store-connect-mcp/src/extensions/workflows.rs)
- [app-store-connect-mcp/src/main.rs](../../app-store-connect-mcp/src/main.rs)
- [app-store-connect-mcp/src/resources.rs](../../app-store-connect-mcp/src/resources.rs)
- [app-store-connect-mcp/src/tools.rs](../../app-store-connect-mcp/src/tools.rs)
- [app-store-connect-mcp/tools.toml](../../app-store-connect-mcp/tools.toml)
- [crates/mcp-factory-core/src/custom.rs](../../crates/mcp-factory-core/src/custom.rs)
- [crates/mcp-factory-core/src/lib.rs](../../crates/mcp-factory-core/src/lib.rs)
- [crates/mcp-factory-core/src/operations.rs](../../crates/mcp-factory-core/src/operations.rs)
- [crates/mcp-factory-core/src/rest/mod.rs](../../crates/mcp-factory-core/src/rest/mod.rs)
- [crates/mcp-factory-core/src/server.rs](../../crates/mcp-factory-core/src/server.rs)
- [crates/mcp-factory-core/tests/openapi_query_encoding.rs](../../crates/mcp-factory-core/tests/openapi_query_encoding.rs)
- [generator/mcp_gen/openapi/parser.py](../../generator/mcp_gen/openapi/parser.py)
- [generator/mcp_gen/render.py](../../generator/mcp_gen/render.py)
- [generator/mcp_gen/templates/main.rs.j2](../../generator/mcp_gen/templates/main.rs.j2)
- [generator/mcp_gen/templates/tools.rs.j2](../../generator/mcp_gen/templates/tools.rs.j2)
- [generator/tests/golden/minimal-openapi/tools.rs](../../generator/tests/golden/minimal-openapi/tools.rs)
- [generator/tests/unit/test_render.py](../../generator/tests/unit/test_render.py)
- [generator/tests/unit/test_tool_config.py](../../generator/tests/unit/test_tool_config.py)
- [app-store-connect-mcp/docs/refactoring.md](../../app-store-connect-mcp/docs/refactoring.md)
- [app-store-connect-mcp/src/extensions/contracts.rs](../../app-store-connect-mcp/src/extensions/contracts.rs)
- [app-store-connect-mcp/src/extensions/upload_session.rs](../../app-store-connect-mcp/src/extensions/upload_session.rs)
- [app-store-connect-mcp/src/extensions/upload_session_tests.rs](../../app-store-connect-mcp/src/extensions/upload_session_tests.rs)
- [app-store-connect-mcp/src/extensions/test_support.rs](../../app-store-connect-mcp/src/extensions/test_support.rs)
- [crates/mcp-factory-core/src/transfers.rs](../../crates/mcp-factory-core/src/transfers.rs)
- [generator/README.md](../../generator/README.md)
- [generator/tests/fixtures/asc-profile-memberships.json](../../generator/tests/fixtures/asc-profile-memberships.json)
- [app-store-connect-mcp/docs/iteration2.md](../../app-store-connect-mcp/docs/iteration2.md)
- [app-store-connect-mcp/docs/iteration2-live.json](../../app-store-connect-mcp/docs/iteration2-live.json)
- [app-store-connect-mcp/src/extensions/output_budget.rs](../../app-store-connect-mcp/src/extensions/output_budget.rs)
- [app-store-connect-mcp/src/extensions/progress.rs](../../app-store-connect-mcp/src/extensions/progress.rs)
- [app-store-connect-mcp/src/extensions/progress_tests.rs](../../app-store-connect-mcp/src/extensions/progress_tests.rs)
- [app-store-connect-mcp/src/extensions/pagination_tests.rs](../../app-store-connect-mcp/src/extensions/pagination_tests.rs)
- [app-store-connect-mcp/src/extensions/screenshot_inventory.rs](../../app-store-connect-mcp/src/extensions/screenshot_inventory.rs)
- [app-store-connect-mcp/scripts/live-smoke.py](../../app-store-connect-mcp/scripts/live-smoke.py)
- [app-store-connect-mcp/scripts/tests/test_live_smoke.py](../../app-store-connect-mcp/scripts/tests/test_live_smoke.py)
