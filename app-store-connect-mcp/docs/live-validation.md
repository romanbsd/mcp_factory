# Live validation — October 11, 2026

The freshly built debug binary was exercised over stdio MCP against Apple's official App Store Connect API using the user-specified account credentials. The harness permitted an explicit allowlist of read operations and required `dryRun: true` for every mutating workflow invocation. No reservation, upload, patch, relationship write, review submission or release request was sent. Private-key material existed only in a restricted temporary file that was removed after each session; it is absent from these results.

**69 calls completed: 60 successful responses, 9 expected blockers/negative lookups, zero unexpected failures.** Checks exercised 13 of the 15 workflow tools across Mycologist, Spore Microdosing and JobSteward. Nine calls were repeated under the default `workflow` profile, confirming that hidden generated dependencies work without exposing raw operations.

| Area | Live result |
|---|---|
| Discovery | JWT authentication, raw app/version discovery and bundle-ID resolution succeeded for all three apps. A nonexistent bundle ID was rejected clearly. |
| Version/release status | Resolved each iOS version and attached build state; release status and preflight completed. |
| Metadata | Normalized reads succeeded. Dry runs distinguished unchanged fields from a proposed promotional-text update, returning concise plans. |
| Screenshots | Inventories contained no assets. Empty desired-state dry runs were no-ops; a temporary local 1290×2796 PNG produced set/create, upload and ordering plans without writes. |
| TestFlight | Resolved live builds and beta states for all apps. JobSteward's existing Beta Testers assignment was unchanged on distribute dry run. Removal dry run produced a plan and warned that the group retains access to all builds. Missing groups were rejected before writes. |
| Review submission | Dry runs returned known readiness blockers rather than submitting. Mycologist lacks complete primary-locale screenshots. Spore and JobSteward also lack attached builds, required primary metadata and review details. |
| Manual release | All three versions were `PREPARE_FOR_SUBMISSION`; dry runs correctly required `PENDING_DEVELOPER_RELEASE`. |
| Analytics discovery | All three apps returned zero existing analytics report requests. A real report download was unavailable to test. |

Largest successful structured summaries were 120 bytes for app lookup, 379 for version lookup, 528 for release status, 969 for preflight, 1,258 for metadata reads, 148 for metadata plans, 62 for empty screenshot inventories, 447 for screenshot plans, and 547 for TestFlight status. These are observed fixture/account results, not guarantees for larger inventories. All fell within their engineering budgets.

Machine-readable results and the tested binary SHA-256 are in [live-validation.json](live-validation.json). The artifact contains normalized check outcomes and byte/timing measurements; it omits credential identifiers, keys and raw provider bodies.

Actual transfer/resume, report parsing, mutation permissions, missing-group assignment, review duplicate reconciliation, submission and post-approval release remain covered by local tests rather than this live run. The available account state did not supply report data, existing screenshots or eligible release/submission states. This run does not establish that any version is ready for submission or that final Apple validation would pass.
