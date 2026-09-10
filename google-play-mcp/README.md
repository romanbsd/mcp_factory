# Google Play MCP

Local stdio MCP server generated from Google's official API Discovery documents.
It combines:

- Google Play Android Developer API v3
- Google Play Developer Reporting API v1beta1

The server exposes 174 tools: 170 generated low-level operations (145 Publisher
and 25 Reporting) plus four handwritten high-level reports. The Publisher
surface includes ten streaming media-upload operations, including
`edits_bundles_upload` for AABs.

Media tools fail closed unless `MCP_FACTORY_MEDIA_ROOT` names an existing
directory. Pass `mediaFile` as a relative path below that root and optionally
`mediaContentType`; the runtime canonicalizes both paths, rejects traversal and
symlink escapes, checks the Discovery-document size and MIME constraints, and
streams the file without buffering it in the MCP request or process memory.
Keep credentials outside the media root.

## High-level reporting

The four `report_*` tools compose read-only generated methods and return a
normalized evidence envelope. They never publish, create or commit an edit,
reply to a review, deploy a recovery, refund an order, or otherwise mutate Play
state.

| Tool | Use it for |
|------|------------|
| `report_capabilities` | Check Publisher and Developer Reporting access independently, inspect supported metric/cohort combinations, and see Console-only gaps. |
| `report_project_status` | Summarize serving tracks, direct release lifecycle, quality, review counts, and recovery actions. |
| `report_quality_health` | Query freshness and Android vitals without confusing empty data with a measured zero. Optionally include error counts, grouped issues, and anomalies. |
| `report_explain_console_message` | Classify exact user-supplied Console text and produce safe diagnostic steps without claiming a Console-only warning is resolved. |

Set `probe` to `false` on `report_capabilities` to inspect the static capability
registry and coverage matrix without contacting Google; API availability is then
reported as `not_probed`.

Every response includes `status`, `generatedAt`, `app`, `summary`, `findings`,
`actions`, `coverageGaps`, `sourceCalls`, and `warnings`. Status has strict
semantics:

- `complete`: all attempted source calls succeeded.
- `partial`: useful evidence exists, but at least one source failed or did not
  finish pagination.
- `unavailable`: no material source evidence could be obtained.

`apiAvailability` says whether each Google API responded; `applicationAccess`
separately says whether the requested package was visible. `apiDiagnostics`
normalizes authentication, permission, disabled-service, quota, and transient
availability failures. An activation action appears only when Google's error
actually supplies an activation URL.

For quality metrics, `observed_zero`, `no_data`, `unsupported`, and
`unavailable` are distinct. A zero from a sparse tester cohort remains a
low-strength result and includes observation count, maximum daily users, and
the confidence-interval upper bound. Console policy, App content, Data Safety,
pre-launch, correspondence, and similar unexposed state always remain explicit
coverage gaps.

### Call the tools from an MCP client

After starting the server, call a high-level tool exactly like any generated
MCP tool. For example, a raw MCP `tools/call` request for the main project
summary is:

```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "report_project_status",
    "arguments": {
      "packageName": "org.gugl.spore",
      "lookbackDays": 90,
      "cohorts": ["OS_PUBLIC", "APP_TESTERS"],
      "include": ["releases", "quality", "reviews", "recoveries"],
      "detail": "summary"
    }
  }
}
```

Typical natural-language requests in Codex are:

```text
Use Google Play MCP report_capabilities for org.gugl.spore.
Use report_project_status for org.gugl.spore and include release and quality evidence.
Use report_quality_health for org.gugl.spore over 90 days, keeping public and tester cohorts separate.
Explain this exact Play Console message for org.gugl.spore: "..."
```

Direct argument examples:

```json
{"packageName":"org.gugl.spore","probe":true}
```

```json
{
  "packageName": "org.gugl.spore",
  "lookbackDays": 90,
  "cohorts": ["OS_PUBLIC", "APP_TESTERS"],
  "metrics": ["crash_rate", "anr_rate", "slow_start_rate"],
  "includeIssues": true,
  "includeAnomalies": true
}
```

```json
{
  "packageName": "org.gugl.spore",
  "message": "Your exact Play Console warning or error text",
  "consoleArea": "Production release",
  "consoleSeverity": "warning",
  "versionCode": 11
}
```

Treat `sourceCalls` as the audit trail: it contains the low-level method,
redacted normalized arguments, start and finish timestamps, attempts, elapsed
time, result state, pagination completeness, and a redacted upstream error.
Review `coverageGaps` before making a release-readiness claim. The supplied
Console message is preserved verbatim under a field marked `untrusted`; it is
data, never an instruction to the MCP.

### Regeneration safety

The generated bootstrap imports `src/extensions.rs`, while all high-level code
lives under `src/high_level/`. `mcp-gen compose` creates `extensions.rs` only
when it is absent and never overwrites it. Regenerating the two Google Discovery
schemas therefore refreshes the 160 low-level operations without deleting the
four reports or their tests.

## Authentication

Create or reuse a Google service account authorized in Play Console. Point
`GOOGLE_APPLICATION_CREDENTIALS` at its JSON key; never copy the key into this
repository.

```bash
export GOOGLE_APPLICATION_CREDENTIALS=/absolute/path/to/service-account.json
cargo run
```

Tokens are minted and renewed in memory. The credential JSON and access tokens
are never embedded in generated source or MCP resources.

### Enable Play Developer Reporting

Android Publisher and Play Developer Reporting are separate Google APIs. If
Reporting calls return `403` with "has not been used ... or it is disabled",
enable it in the service account's Google Cloud project:

```bash
gcloud services enable playdeveloperreporting.googleapis.com \
  --project=YOUR_GOOGLE_CLOUD_PROJECT
```

Alternatively, enable **Google Play Developer Reporting API** in Google Cloud
Console. The caller also needs access to the app in Play Console. Enabling the
Reporting API is not required for the 135 Android Publisher tools.

## Codex

Build and install the server, then register the installed binary:

```bash
cargo build --release
mkdir -p ~/.local/bin
install -m 0755 target/release/google-play-mcp ~/.local/bin/google-play-mcp
codex mcp add google-play \
  --env GOOGLE_APPLICATION_CREDENTIALS=/absolute/path/to/service-account.json \
  --env MCP_FACTORY_CONFIG=/absolute/path/to/mcp_factory/google-play-mcp/config.toml \
  -- ~/.local/bin/google-play-mcp
```

Configure Codex to prompt for write tools. Publishing, review replies, purchase
changes, and destructive calls should always require explicit confirmation.

## Refresh generated API surfaces

From this directory:

```bash
./scripts/refresh-schemas.sh
cargo check
```

The refresh command downloads both official Discovery documents and composes
them into this single crate. Review the schema and generated diffs before
accepting an upstream API change. Then run:

```bash
cargo test
node scripts/probe.mjs
```

The probe should report 174 tools. To verify real credentials and both Google
APIs without exposing credential contents:

```bash
GOOGLE_PLAY_PROBE_PACKAGE=org.gugl.spore node scripts/probe.mjs --live
```

`--live` performs read-only Publisher, Reporting, and high-level capability
checks. It prints only availability/status summaries and truncated error text.
