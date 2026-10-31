# Google Analytics MCP

Rust MCP server with 19 default workflows and 248 generated Data/Admin API operations.
The pinned Discovery revisions are Admin **20261027** and Data **20261028**, for both
v1beta and v1alpha. Alpha methods retain Google's preview limitations.

## Run

Enable the Google Analytics Data and Admin APIs in your Cloud project. Grant your
identity access to the Analytics properties. Configure Application Default
Credentials with `analytics.readonly` scope (for example, using `gcloud auth
application-default login` with your OAuth client). This server uses gcp_auth's ADC
discovery: service-account credentials, the standard gcloud ADC location, metadata
credentials, or the gcloud CLI. It does not create credentials or grant access.

```sh
cargo build --release -p google-analytics-mcp
MCP_FACTORY_CONFIG="$PWD/google-analytics-mcp/config.toml" target/release/google-analytics-mcp
```

For an explicit service-account JSON key, use `[auth] type = "external"`, `provider = "google_service_account"`, and
set `[auth.options] credentials_path_env =
"GOOGLE_APPLICATION_CREDENTIALS"`, retain the scopes, and set that environment
variable to the key path. Never commit credentials.

## Default tools

| Tool suffix (all start with `analytics_`) | Purpose |
|---|---|
| `discover_properties` | Account/property summaries, with pageToken |
| `property_configuration` | Property, streams, key events, retention, Ads/Firebase/BigQuery links |
| `metadata` | Search standard and custom metrics/dimensions; at most 50 matches per kind |
| `validate_report` | Core metric/dimension compatibility |
| `run_report` | Full Core REST report request |
| `overview`, `acquisition`, `content`, `events`, `ecommerce` | Deterministic report presets; optional daily grouping |
| `compare` | Two explicit date ranges in one report |
| `realtime`, `funnel`, `conversions`, `pivot` | Specialized REST reporting |
| `batch` | Up to five Core requests for one property |
| `quota` | Alpha quota snapshot |
| `access_audit`, `change_history` | Data access and configuration change evidence |

Pass a numeric **string** `propertyId` (or `accountId` for change history).
`request` uses Google's REST **camelCase** fields. Presets choose dimensions and
metrics; request filters and ordering remain available. Default dates are
`28daysAgo` through `yesterday`; `limit` defaults to 100 and is capped at 1,000.

```json
{"propertyId":"123456","daily":true,"limit":100}
```

For `analytics_compare`:

```json
{"propertyId":"123456","request":{"dateRanges":[{"startDate":"28daysAgo","endDate":"15daysAgo"},{"startDate":"14daysAgo","endDate":"yesterday"}],"metrics":[{"name":"activeUsers"}]}}
```

For custom reports, use `analytics_metadata` to discover fields and
`analytics_validate_report` to check compatibility. Core compatibility does not
validate realtime or funnel combinations.

Report rows use a `columns` array and value arrays; typed headers, totals,
rowCount, quota and Analytics metadata remain available. Missing rows are not
interpreted as zero. Each source has a status and operation name; partial list
pages are marked explicitly. Configuration reads are bounded first-page snapshots.
A source has a 15-second timeout. Source data over 48 KB is replaced with an
explicit omission notice and partial status; narrow the request or use raw tools.
Combined source data is also bounded to 60 KB, with explicit omissions when the
budget is exceeded. Error details are truncated and non-JSON upstream responses
are reported as unavailable evidence.
Report offsets are supplied through `request.offset`; raw collection tools accept
Google's page tokens. The server does not silently fetch all report rows.

## Advanced coverage

Set `MCP_FACTORY_PROFILES` to a comma-separated combination of:

- `workflow` (default): 19 public workflows, generated dependencies kept internal.
- `analyticsdata_read` / `analyticsadmin_read`: generated read operations.
- `analyticsdata_write` / `analyticsadmin_write`: generated operations that create
  or change resources, including audience exports/report tasks and alpha chat.
- `raw`: all 248 generated operations; includes writes.

Google scopes and account permissions still apply. The default readonly scope
cannot authorize Admin edits; changing exposure does not grant access. Chat uses
its own scope and is kept out of the default profile. Full API coverage includes
both versions, with version-prefixed tool names to avoid ambiguity. Generated
input schemas are lazy; full output schemas are omitted from tool discovery.

## Regenerate and validate

```sh
PYTHONPATH=generator generator/.venv/bin/python google-analytics-mcp/scripts/generate.py
cargo fmt --all
cargo test -p google-analytics-mcp
```

Generation preserves `src/extensions.rs` and the workspace manifest. Schemas are
pinned local inputs; updating them is a deliberate refresh from
`https://{analyticsdata,analyticsadmin}.googleapis.com/$discovery/rest?version={v1beta,v1alpha}`.
The contract test measures the real MCP tool metadata and enforces a 25 KB default
listing budget. These measurements compare workflow and raw profiles, not process
RSS or the official Python server. Live-account report correctness requires valid
credentials and property access; local tests use deterministic evidence.

ADC user credentials may need an explicit quota project. Set
`[auth.options] quota_project = "your-project-id"` to attach
`x-goog-user-project`. Failed ADC discovery is cached until restart; fix the
credentials and restart after a discovery failure. Core/Realtime reports and
access audits enforce the configured row limit and return explicit pagination
state; funnel and pivot requests use their own API-specific limits.

To authorize raw Admin edits, configure `analytics.edit` in the provider's scopes
and grant the corresponding Analytics role. Merely selecting a write profile
never upgrades OAuth permissions. The shipped config stays read-only.
