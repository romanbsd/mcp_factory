# Bounded discovery and reconciliation follow-up

All seven reviewed improvements are implemented. The public workflow inputs, tool schemas, hints and profile membership remain unchanged; actual stdio listings still measure 15 default tools / 14,777 compact bytes, and 293 tools in `all`.

| Change | Implementation and evidence |
|---|---|
| Incremental discovery | `services::Collection` fetches one Apple page at a time with cursor-loop, page and row limits. Screenshot discovery normalizes candidates incrementally and stops when its output budget fills. Unfiltered TestFlight status requests 25 groups per Apple page and reads only the requested window. Multi-page tests verify early stopping, continuation positions and membership reads. |
| Partial-write progress | Metadata and TestFlight errors include `details.progress` with `plannedCount`, confirmed `applied` changes and `pending` changes, plus the original upstream error. Two HTTP integration tests reject the second write, verify the exact progress and Apple error, then retry against updated state and write only the remaining change. |
| Shared screenshot inventory | `ScreenshotInventory` owns sparse set/asset traversal, duplicate checks and delivery-state interpretation. Screenshot reconciliation and release preflight reuse it; preflight requests only delivery state and uses the shared completion predicate. |
| Shared output budgets | `output_budget::json_bytes` counts encoded JSON with a writer, avoiding serialized String/Vec allocations. `ArrayBudget` counts candidates incrementally including brackets/commas. Reports and screenshots no longer repeatedly serialize their accumulated rows; metadata and TestFlight use the same byte counter. Boundary tests cover UTF-8, escaping, serialization failure, exact limits and retained-row behavior. |
| Typed domain actions | Workflow routing passes private metadata, screenshot, TestFlight and release enums directly. Services no longer validate string action names. One version-editability predicate serves both preflight and review reconciliation. Released-version no-ops now also echo `dryRun`. |
| Generated profile membership | Upload-kind availability derives from the generated create/read/update operation memberships. Handwritten raw-profile aliases and obsolete kind profile metadata are removed. Independent profile fixtures retain expected schema coverage. |
| Reproducible live checks | `scripts/live-smoke.py` requires explicit opt-in, pins Apple's official API, enforces a read/strict-dry-run allowlist, bounds requests/responses and emits sanitized counts/size/timing results. Temporary keys/config/media are cleaned on success and failure. Unit tests cover bypass attempts, malformed results, metadata plan semantics, credential cleanup, response deadlines and output-file protection. |

## Continuation and safety

Screenshot `offset` and TestFlight `offset` inputs remain supported. A later offset can require walking earlier Apple pages again; the server stops after the requested output window instead of collecting the complete resource inventory. Offsets are relative to the current Apple ordering, so concurrent changes can shift them; this does not provide snapshot isolation.

When Apple supplies a paging total, TestFlight returns it as `groupCount`. If discovery stops early without a total, `groupCount` is null and `groupCountLowerBound` gives the observed count. Exhausted inventories have an exact count. Named-group selection and mutating reconciliation still inspect complete bounded inventories to detect ambiguity before writes.

Progress is diagnostic, not a transaction or a replay instruction. `applied` means a successful response was confirmed. `pending` includes the failed request, whose outcome can be uncertain after a network error. Retry always rereads Apple's state; no rollback is claimed. Upload hash/identity and remote-state revalidation remain at the mutation boundary.

## Live runner

Build with `cargo build -p app-store-connect-mcp --offline`, then explicitly opt in:

```sh
python3 app-store-connect-mcp/scripts/live-smoke.py --live --apps 3 --versions 2 --screenshots --max-requests 100
```

`--credentials PATH` selects a different credential JSON. The inline key is copied to a mode-0600 temporary PEM only for the subprocess lifetime. `--output PATH` saves the sanitized summary and refuses credential-file aliases/symlinks. There are no upload, report-creation, submission, release or other write calls. Screenshot fixtures test local dry-run planning, not Apple's image acceptance. Discovery uses bounded first pages; unavailable data and exhausted budgets are reported as skips.

Offline runner checks use `python3 -m unittest discover -s app-store-connect-mcp/scripts/tests -v`. See [validation.md](validation.md) for integrated results and [iteration2-live.json](iteration2-live.json) for the opt-in live run.

Validation passes 293 workspace Rust tests (104 ASC, 86 Google Play, 63 core unit and 40 integration), 154 generator tests at 91.72% branch-inclusive coverage, and 23 offline live-runner tests. Clippy, formatting, packaging, regeneration/hash checks and exact workflow metadata comparison pass. The initial generator run attempted blocked crates.io DNS; the offline rerun passed using cached dependencies.

No RSS or CPU speedup is claimed for this follow-up; pagination and byte counting are covered by semantic/behavioral tests rather than a new resource benchmark.

The final opt-in live run completed 72 tool calls: 56 passes, 15 expected readiness blockers and one expected negative lookup, with zero unexpected failures. Five checks were skipped for absent groups or analytics segments. The saved runner and binary hashes match the final files. No live writes were sent; actual upload/resume and report download remain unverified live.
