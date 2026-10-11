# App Store Connect workflows

The default `workflow` profile exposes 15 tools. They accept app/version/build identifiers that you already know and resolve Apple's resource IDs internally. Enable a raw profile for capabilities outside these contracts, such as creating versions, attaching builds, app name/subtitle, review-detail updates, previews, pricing or phased-release changes. [Setup and profiles](../README.md).

## Inputs shared by domain workflows

All 13 domain workflows require `bundleId` (nonempty string). `platform` is optional and defaults to `IOS`; accepted values are `IOS`, `MAC_OS`, `TV_OS`, `VISION_OS`. Version workflows require `version` (nonempty marketing-version string). TestFlight workflows instead require `buildNumber`; optional `version` disambiguates build numbers. Ambiguous selections fail rather than selecting an arbitrary resource.

Mutating domain workflows accept optional `dryRun` (boolean, default `false`). Unknown properties are rejected. In the table, `string[]` means an array of strings. `locales` and `groupNames` require 1–100 unique nonempty values; `requiredDisplayTypes` requires unique strings and may be empty.

| Tool | Required beyond `bundleId` | Optional beyond `platform` | Result or behavior |
|---|---|---|---|
| `app_get` | None | None | Compact app identity, name, bundle ID, SKU and primary locale |
| `version_get` | `version: string` | None | Resolved version and attached build |
| `release_status` | `version: string` | None | Version/build state, relevant review submissions, phased release and known issues |
| `release_prepare` | `version: string` | `requiredDisplayTypes: string[]` (may be empty) | Read-only submission checks, blockers, warnings and unknown checks |
| `metadata_get` | `version: string` | `locales: string[]`, `fields: array` of names from the six fields below (may be empty) | Normalized version localizations; defaults to primary locale |
| `metadata_apply` | `version: string`, `localizations: object` | `dryRun` | Create missing locales; update only changed desired fields |
| `screenshots_get` | `version: string` | `locales: string[]`, `displayType: string`, `offset: integer >= 0` | Ordered assets, state/checksum/position and bounded continuation |
| `screenshots_sync` | `version: string`, `locale: string`, `displayType: string`, `files: string[]` (0–10 ordered relative paths) | `removeObsolete: boolean=false`, `resumes: object`, `dryRun` | Reuse matching COMPLETE assets, upload/resume missing assets, order desired files |
| `testflight_status` | `buildNumber: string` | `version: string`, `groupNames: string[]`, `offset: integer=0` | Build/beta state, expiration and current explicit/all-build availability |
| `testflight_distribute` | `buildNumber: string`, `groupNames: string[]` | `version: string`, `dryRun` | Assign missing build/group relationships |
| `testflight_remove` | `buildNumber: string`, `groupNames: string[]` | `version: string`, `dryRun` | Remove existing explicit build/group relationships |
| `review_submit` | `version: string` | `requiredDisplayTypes: string[]` (may be empty), `dryRun` | Reconcile review draft/items, run preflight, submit once |
| `release` | `version: string` | `dryRun` | Request manual release only from `PENDING_DEVELOPER_RELEASE` |

`metadata_get.fields` may contain `description`, `keywords`, `whatsNew`, `promotionalText`, `marketingUrl`, `supportUrl`. `metadata_apply.localizations` maps 1–100 locale names to objects with only these keys; values are strings or `null`. Omitted fields stay untouched. App-level name/subtitle use `appInfoLocalizations_*` through `metadata` or `release_raw`.

## Preview and apply a change

These are MCP `tools/call` parameter objects; supply them to your MCP client. The app identifier and version are example values for an existing app.

```json
{
  "name": "metadata_apply",
  "arguments": {
    "bundleId": "com.example.app",
    "version": "1.2.0",
    "localizations": {
      "en-US": {"whatsNew": "Improved loading and fixed display issues."}
    },
    "dryRun": true
  }
}
```

Inspect `changes`, `unchanged` and `warnings`, then rerun the same arguments with `dryRun: false` to apply. Changes carry an `action` and `resource`, plus relevant locale/field/group/file details. The exact `unchanged` representation depends on the workflow: locale/group lists, screenshot entries or review/release counts. Failures include `ok: false`, `stage`, `message` and `details`; inspect these before retrying. Multi-request workflows are not transactions, so earlier successful writes can remain after a later request fails.

TestFlight status inspects at most 25 selected groups per page. Follow `nextOffset` using the same inputs for larger apps; `groupCount` (when known) and `scannedGroups` distinguish the complete selection from the inspected page.

For TestFlight assignment:

```json
{
  "name": "testflight_distribute",
  "arguments": {
    "bundleId": "com.example.app",
    "buildNumber": "42",
    "version": "1.2.0",
    "groupNames": ["Internal QA"],
    "dryRun": true
  }
}
```

All named groups and memberships are resolved before mutation. Repeating an already satisfied assignment performs no write. Removing an explicit relationship does not revoke access for a group with `hasAccessToAllBuilds: true`; the result warns about this. Distribution does not create groups/testers or submit external Beta App Review; use `testflight_raw` for those steps.

## Check submission readiness and publish

Run `release_prepare` after attaching a build and filling the primary version localization, screenshots and App Review details. Known checks include editable version state, attached VALID/unexpired build, recorded export-compliance answer, primary description/support URL, complete primary screenshots, review contact/demo details and age-rating declaration presence. Supply `requiredDisplayTypes` when you know the required device coverage.

```json
{
  "name": "release_prepare",
  "arguments": {
    "bundleId": "com.example.app",
    "version": "1.2.0",
    "requiredDisplayTypes": ["APP_IPHONE_67"]
  }
}
```

`readiness: "no_known_blockers"` means these checks passed. `unknownChecks` still includes Apple's submission validation, App Privacy labels, monetization compliance, completeness of age-rating answers and device screenshot coverage unless supplied by the caller. Non-exempt encryption approval requires Apple evaluation. Age-rating presence does not validate every answer.

`review_submit` finds an existing usable draft or submitted request before creating one, and avoids resubmitting a version already waiting/in review/completing/complete. Ambiguous drafts, unrelated draft items or another active submission require resolution with `release_raw`. Submission acceptance is distinct from approval and publication. After approval, run `release_status`; use `release` when the version is in `PENDING_DEVELOPER_RELEASE`. Automatic/scheduled/phased release configuration remains available through raw release operations.

## Reconcile screenshots and resume processing

`files` defines the desired order. All paths resolve under `MCP_FACTORY_MEDIA_ROOT` and are hashed before mutation. Duplicate paths/names, ambiguous sets/locales, mismatched checkpoints and a desired-plus-retained set exceeding 10 assets fail validation. A match requires COMPLETE state, matching filename and file size, and the same streamed MD5 checksum.

```json
{
  "name": "screenshots_sync",
  "arguments": {
    "bundleId": "com.example.app",
    "version": "1.2.0",
    "locale": "en-US",
    "displayType": "APP_IPHONE_67",
    "files": ["en-US/iphone67/01.png", "en-US/iphone67/02.png"],
    "dryRun": true
  }
}
```

A missing set is created only when files need it. Existing assets are retained after desired files by default. Set `removeObsolete: true` to delete specifically obsolete assets after every desired upload is COMPLETE and ordering succeeds. Pending processing defers both ordering and deletion. Sync does not wait for Apple processing; rerun with `resumes` mapping each relative path to its returned checkpoint until processing completes. Preserve checkpoints from successful prior uploads as well as failures.

`screenshots_get` returns `screenshots` entries with `locale`, `displayType`, `setId` and normalized `assets` (`id`, `fileName`, `fileSize`, `state`, `checksum`, zero-based `position`). Large output is limited to under 20KB: pass `nextOffset` back as `offset` while `hasMore` is true. Entries may split one set across multiple chunks. This is an offset over a freshly fetched listing, so concurrent upstream changes can alter the continuation.

## Upload an individual asset

`asset_upload` handles reserve → ranged upload → checksum commit → processing. It supports the following parent contracts:

| `kind` | `parentId` identifies |
|---|---|
| `appScreenshot` | Screenshot set |
| `appPreview` | Preview set |
| `appStoreReviewAttachment` | App Review detail |
| `appEncryptionDeclarationDocument` | Encryption declaration |
| `subscriptionAppStoreReviewScreenshot` | Subscription |
| `inAppPurchaseAppStoreReviewScreenshot` | In-app purchase v2 |

| Field | Type | Required/default |
|---|---|---|
| `kind` | String enum above | Required; available kinds depend on selected profiles |
| `parentId` | Nonempty string | Required |
| `file` | Nonempty relative path string | Required, under media root |
| `fileName` | Nonempty string | Optional; defaults to local basename |
| `previewFrameTimeCode` | String | Optional, `appPreview` only |
| `mimeType` | String | Optional, `appPreview` only |
| `waitSeconds` | Integer 0–600 | Optional; default 120; 0 returns after commit |
| `resume` | Checkpoint object | Optional; reuses reservation and never creates another asset |

```json
{
  "name": "asset_upload",
  "arguments": {
    "kind": "appScreenshot",
    "parentId": "screenshot-set-id",
    "file": "en-US/iphone67/01.png",
    "waitSeconds": 0
  }
}
```

The parent ID in this example must come from an actual screenshot-set lookup. Prefer `screenshots_sync` when starting from bundle/version/locale.

Uploads cap files at 2GiB and stream requested byte ranges through 64KiB buffers. Range offset/length is validated against file size. Transfer errors retain at most 2KiB of upstream body text. Asset stages include `validate`, `reserve`, `resume`, `upload`, `commit`, `processing`; COMPLETE is the successful processing state, FAILED is a processing failure.

The returned `resume` checkpoint contains `assetId`, `kind`, `parentId`, `file`, `fileName`, `fileSize`, `sourceFileChecksum`, `state`, `committed`, `completedRanges`, `pendingRanges` and stable `ranges` (`offset`, `length`). At minimum an input checkpoint requires `assetId`, `fileSize`, `sourceFileChecksum` and `completedRanges`; keep the entire returned object to validate completed range indices against the current operation plan. Resume rehashes the file and reads the reservation, verifies resource identity/type, parent relationship, name/size, and processing checksum before writing. Changed ranges with completed indices are rejected. Encryption-document resumes are rejected when Apple does not expose the parent linkage needed for verification.

Asset uploads and report transfers share two process-wide concurrency permits, including binary raw responses. Raw binary compatibility responses retain the existing 64MiB response/decompression caps; normal report queries use temporary artifacts and incremental decoding. Blocking report consumption owns its permit until it finishes, including when the async caller is canceled.

## Query analytics, sales or finance reports

`report_query` selects exactly one source: analytics `segmentId`, analytics `reportId`, or `report: "sales" | "finance"`. It streams the response into a temporary file, verifies Apple's checksum when provided, parses CSV/TSV incrementally and returns a bounded result. Sales/finance use the internal generated HTTP operation directly, so compressed report bodies do not enter a public raw-tool result. It does not create an analytics report request or discover an analytics report from a bundle ID; use `reports_raw` for those operations. `analytics_segment_download` is available only in raw report profiles.

| Source | Required source fields | Optional period/type fields |
|---|---|---|
| One analytics segment | `segmentId: nonempty string` | None |
| Analytics report instances | `reportId: nonempty string` | `dateRange` below |
| Sales report period | `report: "sales"`, `vendorNumber: nonempty string`, `reportDate: nonempty string` | `frequency: string=DAILY`, `reportType: string=SALES`, `reportSubType: string=SUMMARY`, `reportVersion: string` |
| Finance report period | `report: "finance"`, `vendorNumber: nonempty string`, `reportDate: nonempty string`, `regionCode: nonempty string` | `reportType: string=FINANCIAL` |

`vendorNumber` identifies the reporting vendor; `regionCode` is Apple's finance-report region code. `reportDate` is the actual sales/finance report period identifier required by the selected Apple report/frequency, forwarded unchanged. The wrapper leaves period/type validation to the generated operation and Apple. Sales frequency values supported by the bundled API are `DAILY`, `WEEKLY`, `MONTHLY`, `YEARLY`; use the operation schema for report-type/subtype options and version values. Finance accepts `FINANCIAL` or `FINANCE_DETAIL`; it rejects sales-only `frequency`, `reportSubType` and `reportVersion`. Sales and finance reject `dateRange`.

| Field | Type | Required/default |
|---|---|---|
| Source selection | Fields from the table above | Exactly one of `segmentId`, `reportId`, `report` |
| `dateRange` | `{from: "YYYY-MM-DD", to: "YYYY-MM-DD"}` | Optional inclusive ordered range; selects report-instance `processingDate` |
| `columns` | Unique string array, at most 100 | Optional projection; defaults to all columns |
| `filters` | Object, at most 100 scalar string/number/boolean/null values | Optional exact cell equality by column name |
| `groupBy` | Unique string array, at most 10 | Optional grouping column names |
| `metrics` | Unique string array, at most 100 | Optional numeric column names to sum |
| `limit` | Integer 1–200 | Optional, default 20; applies to inline `rows` |
| `output` | `summary`, `rows`, `artifact` | Optional, default `summary` |

`dateRange` selects the date Apple processed a report instance; it does not filter dates inside report rows. Use a `filters` entry for an exact row-date value. Filters are exact string comparisons after scalar conversion, with no fuzzy matching or expression language. Column names must match the actual CSV/TSV header. Metric cells and resulting sums must be finite `f64` values; nonnumeric/nonfinite values fail the query. Sums use binary floating point, without decimal financial rounding, currency conversion or report-specific semantic inference. Select compatible numeric columns and groups yourself; exported CSV cells retain their source text.

```json
{
  "name": "report_query",
  "arguments": {
    "reportId": "analytics-report-id",
    "dateRange": {"from": "2026-10-01", "to": "2026-10-10"},
    "columns": ["Territory", "Downloads"],
    "filters": {"Territory": "US"},
    "groupBy": ["Territory"],
    "metrics": ["Downloads"],
    "output": "summary"
  }
}
```

For a sales report period, supply your vendor number and Apple's period identifier:

```json
{
  "name": "report_query",
  "arguments": {
    "report": "sales",
    "vendorNumber": "12345678",
    "reportDate": "2026-10-10",
    "frequency": "DAILY",
    "output": "rows",
    "limit": 20
  }
}
```

For a finance export:

```json
{
  "name": "report_query",
  "arguments": {
    "report": "finance",
    "vendorNumber": "12345678",
    "reportDate": "2026-09",
    "regionCode": "US",
    "output": "artifact"
  }
}
```

The vendor, period and region values are examples; substitute the identifiers for your own report. Both modes support the same `columns`, exact `filters`, `groupBy`, `metrics`, `limit` and `output` controls as analytics.

The ID and header names above illustrate a caller-selected report; use its actual report ID and column names. Summary includes matching `rowCount`, `columns`, metric `sums`, `decodedBytes`, and groups when requested. Analytics results include `segmentCount`; sales/finance results include `report`. `rows` additionally returns bounded projected rows with `truncated`/`droppedRows`; the inline rows have a separate 10,000-byte budget and may return fewer than `limit`. Aggregates count all matching rows, including those omitted from inline output. `artifact` exports every matching projected row as a local CSV file and returns its path rather than file contents.

Limits are 20 analytics segments, 100 groups, 200 inline rows and 20KB serialized output. Downloaded data is capped at 256MiB per segment or sales/finance response; decoded data is capped at 256MiB across the query, each logical CSV record at 1MiB, each cell at 64KiB, and an exported artifact at 256MiB. Limit errors are explicit; narrow dates/filters/projection. Headers must agree across segments.

Artifact paths are local to the server machine. The service retains the last eight temporary artifacts until eviction or process exit; copy a needed artifact before another export invalidates its path. Expired segment URLs require a fresh segment lookup. `summary` is the default to keep downloaded report bodies out of conversation context.

## Inspect an underlying operation

Read `schema://operation/<operationId>` for its exact input contract without loading the full specification. Workflow guides are exposed at `docs://workflow/release`, `docs://workflow/metadata`, `docs://workflow/screenshots`, `docs://workflow/testflight`, `docs://workflow/reports`. `schema://openapi` remains available for advanced specification inspection.

The generated internal catalog contains lazy operation factories. Internal calls validate schemas and execute HTTP through `OperationInvoker`; mutating capability is unavailable to read-only callers. Public tool descriptions are sorted and cached separately. Selecting `workflow` exposes the 15 tasks while preserving the internal raw operations needed by those tasks.

## Implementation evidence

The task schemas are defined in [workflows.rs](../src/extensions/workflows.rs); resource guides and transfer sharing in [extensions.rs](../src/extensions.rs); resumable upload in [asset_upload.rs](../src/extensions/asset_upload.rs); bounded query parsing/export in [reports.rs](../src/extensions/reports.rs). These contracts are backed by local mock-API tests. Production readiness still depends on the supplied key, app state and Apple's current validation results.

Partial metadata/TestFlight write failures include `details.progress={plannedCount,applied,pending}` and the upstream Apple error. `pending` can include a request with an uncertain network outcome; retry reconciles current state.

Screenshot and unfiltered TestFlight discovery stop at the requested output window. Continue with `nextOffset`; offsets follow current Apple ordering. When Apple omits paging totals during partial TestFlight discovery, `groupCount` is null and `groupCountLowerBound` reports observed groups.
