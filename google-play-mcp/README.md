# Google Play MCP

Local stdio MCP server generated from Google's official API Discovery documents.
It combines:

- Google Play Android Developer API v3
- Google Play Developer Reporting API v1beta1

The server exposes 174 tools: 170 generated low-level operations (145 Publisher
and 25 Reporting) plus four handwritten high-level reports. The Publisher
surface includes ten streaming media-upload operations, including
`edits_bundles_upload` for AABs. Four optional install-report tools read Play's
Cloud Storage exports; see [Install reports](#install-reports).

Media tools fail closed unless `MCP_FACTORY_MEDIA_ROOT` names an existing
directory. Pass `mediaFile` as a relative path below that root and optionally
`mediaContentType`; the runtime canonicalizes both paths, rejects traversal and
symlink escapes, checks the Discovery-document size and MIME constraints, and
streams the file without buffering it in the MCP request or process memory.
Keep credentials outside the media root.

## High-level reporting

The five `report_*` tools compose read-only generated methods and return a
normalized evidence envelope. They never publish, create or commit an edit,
reply to a review, deploy a recovery, refund an order, or otherwise mutate Play
state.

| Tool | Use it for |
|------|------------|
| `report_capabilities` | Check Publisher and Developer Reporting access independently, inspect supported metric/cohort combinations, and see Console-only gaps. |
| `report_project_status` | Summarize serving tracks, direct release lifecycle, quality, review counts, and recovery actions. Pass `track` + `versionCode` to report that release first, so an older serving release is not mistaken for it. |
| `report_quality_health` | Query freshness and Android vitals without confusing empty data with a measured zero. Optionally include error counts, grouped issues, and anomalies. |
| `report_explain_console_message` | Classify exact user-supplied Console text and produce safe diagnostic steps without claiming a Console-only warning is resolved. |
| `report_release_verification` | Verify one release in a single call: version code on track, lifecycle, and optionally bundle checksum and release notes. Checksum and notes need an open `editId` (create with `edits_insert`, delete with `edits_delete` after); without it they are reported `unverifiable`. |

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

## Install reports

Four optional `reports_install*` tools answer "has version N been installed?"
from Play's official monthly install exports. Neither the Publisher API nor the
Reporting API exposes install statistics. Google writes them as CSV files to a private Cloud
Storage bucket:

```text
gs://pubsite_prod_<developerId>/stats/installs/installs_<package>_<yyyyMM>_<dimension>.csv
```

| Tool | Use it for |
|------|------------|
| `reports_install_access_check` | Resolve and validate the bucket, with diagnostics for every candidate: missing configuration, authentication failure, denied permission, missing bucket, ambiguous candidates, accessible-but-empty prefix, or confirmed reports. |
| `reports_installs_query` | Daily rows and per-version summaries for a date range, from the `app_version` breakdown. |
| `reports_installs_list` | Export descriptors: opaque `reportId`, object URI, month, dimension, size, generation, timestamps, `md5Hash`/`crc32c`, `contentEncoding`. |
| `reports_installs_get_raw` | One export by `reportId`, pinned to its generation. `original_csv` attaches the exact file bytes (embedded `text/csv` blob) with a computed SHA-256. `rows` pages through unnormalized string cells. |

### Setup

The tools reuse the service-account key in `GOOGLE_APPLICATION_CREDENTIALS`.
They request a separate token limited to
`https://www.googleapis.com/auth/devstorage.read_only`, so Publisher and
Reporting tokens stay as they are. No login or `config.toml` change is needed.

| Variable | Meaning |
|----------|---------|
| `GOOGLE_PLAY_DEVELOPER_ID` | Numeric id from `play.google.com/console/developers/<id>`. Enables automatic bucket discovery. |
| `GOOGLE_PLAY_REPORTS_BUCKET` | Explicit bucket, as a bare name or `gs://` URI, copied from Play Console > Download reports > Statistics. Skips discovery. |
| `GOOGLE_PLAY_REPORTS_ACCOUNTS` | Several accounts, written as `devId[=bucket],devId2[=bucket2]`. Replaces the two variables above. Select one with the `account` tool argument. |
| `GOOGLE_PLAY_INSTALL_REPORTS=off` | Do not register the four tools. |

Automatic discovery lists only this package's install prefix in
`pubsite_prod_<id>` and in `pubsite_prod_rev_<id>`, with one small listing
each. Verified mappings are cached in memory per account and principal. A
cached mapping is dropped after an access failure, and the next call probes
again. The naming rule is a heuristic, not a Google guarantee. If neither
candidate validates, the error tells you which override to set.

Only `pubsite_prod_*` bucket names are accepted. Paths, other schemes, and
credential-bearing URLs are rejected.

**Permissions.** Bulk reports require the account-level Play Console permission
*View app information and download bulk reports*. App-level access may not
include it. These tools never change permissions, create objects, or sign URLs.

**OAuth.** With `oauth2` auth the tools return `auth_unsupported`. An existing
refresh token cannot be assumed to cover Cloud Storage. Use
`google_service_account` for install reports.

Configuration, permission, and export problems are isolated. Startup and every
other tool keep working, and the install tools return a structured `error`
envelope with a `remedy`.

### Semantics and limits

- **Delay.** Google captures data daily but posts it within 3–7 days, into
  monthly files, on no stated schedule. These exports cannot answer "did
  the release I just pushed get installed today?" An unobserved date is
  reported in `coverage.datesWithoutObservations`. It is never a zero.
- **Aggregation.** Daily event columns (`dailyDeviceInstalls`,
  `dailyDeviceUpgrades`, `installEvents`, ...) are summed over observed dates
  (`sum_over_observed_dates`). Snapshot columns (`totalUserInstalls`,
  `activeDeviceInstalls`, ...) report the value at the latest observed date
  (`latest_snapshot`, with `asOfDate`). Summed daily *user* counts are not
  distinct users. Installs and upgrades stay separate.
- **Absence.** A blank cell becomes `null` and is counted in `blankCells`. A
  version with no rows is listed in `requestedVersionsNotObserved` and gets no
  summary. A month with no export appears in `missingMonths`. Only an observed
  `0` cell is a zero.
- **Validation.** Columns are matched by header name, so reordered or extra
  columns are fine. A requested metric whose column is missing is a schema
  error. Malformed rows, package mismatches, and duplicate `(date, version)`
  keys are rejected and listed per source; they are never silently dropped. A
  schema failure makes the result `partial`, and the raw export is still
  retrievable.
- **Time zone.** The day boundary of the exports is not documented, so
  `timeZone.status` is `unknown`.
- **Gzip.** Google may store an export gzip-compressed (`contentEncoding:
  gzip`). In that case GCS `size`, `md5Hash`, and `crc32c` describe the
  compressed object, while `original_csv` returns the decompressed CSV, and
  its `sha256` covers exactly those bytes.
- **Limits.** Query range ≤ 92 days; only months intersecting the range
  are read; ≤ 20 MiB per export parsed; ≤ 1000 rows returned (`rowsTruncated`);
  list ≤ 100 per page; raw rows ≤ 500 per page; `original_csv` ≤ 8 MiB
  (`too_large_for_host` above). 45-second deadline per call. Transient Storage
  failures (429/5xx/network) are retried up to three attempts; auth and
  permission failures are not retried.
- **Pinning.** A `reportId` encodes bucket, object, and generation. It grants
  nothing: every read re-authenticates, checks the account's own bucket, and
  allows only `stats/installs/installs_*.csv`. If the generation has changed,
  the call returns `source_changed`, so pages never mix revisions.
- Only the `app_version` breakdown is queried. There is no track filter,
  because exports carry no track attribution.

### Sample response (fixture, abbreviated)

Generated by `cargo test`
(`query_version_filter_observed_zero_blank_and_absent_version`). This is not
live data:

```json
{
  "status": "partial",
  "request": {"packageName": "org.example.app", "startDate": "2026-10-01", "endDate": "2026-10-04",
              "dimension": "app_version", "dimensionValues": ["26", "27"],
              "metrics": ["dailyDeviceInstalls", "dailyDeviceUpgrades"]},
  "summaries": [{
    "dimensionValue": "26", "firstObservedDate": "2026-10-01", "lastObservedDate": "2026-10-02",
    "metrics": [
      {"metric": "dailyDeviceInstalls", "sourceColumn": "Daily Device Installs",
       "aggregation": "sum_over_observed_dates", "value": 3, "observedDates": 2, "blankCells": 0},
      {"metric": "dailyDeviceUpgrades", "sourceColumn": "Daily Device Upgrades",
       "aggregation": "sum_over_observed_dates", "value": 40, "observedDates": 1, "blankCells": 1}
    ]
  }],
  "rows": [
    {"date": "2026-10-01", "dimensionValue": "26", "values": {"dailyDeviceInstalls": 3, "dailyDeviceUpgrades": 40},
     "reportId": "gpir1_…", "sourceRowNumber": 2},
    {"date": "2026-10-02", "dimensionValue": "26", "values": {"dailyDeviceInstalls": 0, "dailyDeviceUpgrades": null},
     "reportId": "gpir1_…", "sourceRowNumber": 3}
  ],
  "coverage": {"observedFrom": "2026-10-01", "observedTo": "2026-10-02", "missingMonths": [], "failedMonths": [],
               "datesWithoutObservations": ["2026-10-03", "2026-10-04"],
               "requestedVersionsNotObserved": ["27"]},
  "timeZone": {"status": "unknown", "note": "…"},
  "freshness": {"latestObservedDate": "2026-10-02", "objectsUpdated": [{"reportId": "gpir1_…", "updated": "2026-10-03T08:00:00Z"}],
                "retrievedAt": "…", "note": "…3-7 days…"},
  "sources": [{"reportId": "gpir1_…", "objectUri": "gs://pubsite_prod_123/stats/installs/installs_org.example.app_202610_app_version.csv",
               "generation": "21", "updated": "2026-10-03T08:00:00Z", "encoding": "UTF-16LE (BOM)", "state": "parsed", "…": "…"}],
  "resolution": {"bucket": "pubsite_prod_123", "provenance": "probe:pubsite_prod", "packageReportsSeen": true, "…": "…"},
  "warnings": [{"message": "2 requested dates have no observations; they are unreported, not zero installs"}],
  "sourceCalls": [{"id": "metadata-2026-10", "method": "storage.objects.get", "attempts": 1, "resultState": "success", "…": "…"}]
}
```

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
