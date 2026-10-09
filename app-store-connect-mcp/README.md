# App Store Connect MCP

Local MCP server for Apple's official [App Store Connect API](https://developer.apple.com/documentation/appstoreconnectapi)
(OpenAPI spec 4.5, `openapi.oas.json`). It covers setting up and releasing an
existing app: store metadata, screenshots and previews, builds, App Review,
release, TestFlight, subscriptions and in-app purchases, pricing, signing,
customer reviews and reports. Only public, documented endpoints are used:
no private web APIs, scraping or Apple ID sessions.

## Architecture

```text
openapi.oas.json ──► mcp-gen (+ tools.toml) ──► src/tools.rs     generated, never hand-edited
                                              src/main.rs      generated
                                              src/resources.rs generated (embedded spec)
src/extensions.rs, src/extensions/            handwritten: asset_upload, analytics download, tests
mcp-factory-core                              runtime: proxying, auth, profiles, errors
```

`tools.toml` is the source of truth for which Apple operations exist, which
profile they belong to and any description overrides. Tool names are Apple's
operationIds (e.g. `appStoreVersions_updateInstance`) for traceability.
Operations without an Apple summary get a description derived from the
method and path, plus hand-written overrides for workflow-critical tools.

Responses are compacted: JSON:API `links` and link-only relationships are
removed and JSON is minified (`compact_jsonapi`). Use `include=` to embed related
resources and `fields[type]=` to trim attributes.

## Authentication

Create an API key in App Store Connect (Users and Access → Integrations →
App Store Connect API) and set:

| Variable | Value |
|---|---|
| `ASC_KEY_ID` | Key ID (e.g. `2X9R4HXF34`) |
| `ASC_ISSUER_ID` | Issuer ID (UUID) |
| `ASC_PRIVATE_KEY_PATH` | Path to the downloaded `AuthKey_<KEY_ID>.p8` |

When all three are set (and `config.toml` has no `[auth]` section) every request
carries an ES256 JWT (`aud: appstoreconnect-v1`) that is cached for 15 minutes
and renewed automatically before Apple's 20-minute limit. Key material never
appears in tool schemas, results or error messages. An explicit
`[auth] type = "app_store_connect"` with `key_id_env`, `issuer_id_env` and
`private_key_path_env` selects other variable names.

A 401 result means the token was rejected (wrong key, issuer, clock skew); a
403 means the key's role lacks permission for that resource. The two carry
different `hint`s.

## Profiles

Tools are grouped into profiles so a session only loads what it needs.
Choose them with `MCP_FACTORY_PROFILES=core,assets` (or `profiles = [...]` in
`config.toml`); `all` exposes everything. Unknown profile names stop the
server at startup.

| Profile | Covers | Tools | `tools/list` |
|---|---|---:|---:|
| `core` | apps, versions, builds, build attach, App Review details, review submission, release, phased release | 31 | 45 KB |
| `metadata` | App Info + localizations (name, subtitle), version localizations, categories, age rating, export compliance | 26 | 25 KB |
| `assets` | screenshot/preview sets, ordering, review attachments, `asset_upload` | 29 | 25 KB |
| `testflight` | groups, testers, memberships, build assignment, What to Test, beta app info, Beta App Review, license | 52 | 37 KB |
| `monetization` | subscription groups, subscriptions, offers, IAP v2, localizations, availability, submissions, review screenshots | 64 | 62 KB |
| `pricing` | territories, app availability, app/subscription/IAP price schedules and price points | 31 | 27 KB |
| `signing` | bundle IDs, capabilities, certificates, provisioning profiles, devices | 26 | 22 KB |
| `reviews` | customer reviews and developer responses | 7 | 8 KB |
| `reports` | sales, finance and analytics reports, `analytics_segment_download` | 13 | 8 KB |
| **default** | `core` + `metadata` + `assets` + `testflight` | 137 | 133 KB |
| `all` | everything | 277 | 262 KB |

### Coverage (callable operations)

Counts of generated operations per profile by HTTP method
(`scripts/coverage-table.py` regenerates this table):

| Profile | Read (GET) | Create (POST) | Update (PATCH) | Delete (DELETE) | Upload |
|---|---:|---:|---:|---:|:---:|
| `core` | 15 | 6 | 7 | 3 | — |
| `metadata` | 15 | 3 | 6 | 2 | — |
| `assets` | 11 | 6 | 6 | 5 | ✓ |
| `testflight` | 28 | 9 | 6 | 9 | — |
| `monetization` | 27 | 14 | 12 | 10 | ✓ |
| `pricing` | 24 | 4 | 1 | 2 | — |
| `signing` | 13 | 5 | 4 | 4 | — |
| `reviews` | 5 | 1 | 0 | 1 | — |
| `reports` | 10 | 1 | 0 | 1 | — |
| **all** (275 generated) | 147 | 49 | 42 | 37 | ✓ |

Relationship operations count by method too (linking testers to a group is a
POST, unlinking a DELETE).

## MCP client configuration

```json
{
  "mcpServers": {
    "app-store-connect": {
      "command": "/abs/path/to/target/release/app-store-connect-mcp",
      "env": {
        "ASC_KEY_ID": "2X9R4HXF34",
        "ASC_ISSUER_ID": "57246542-96fe-1a63-e053-0824d011072a",
        "ASC_PRIVATE_KEY_PATH": "/abs/path/AuthKey_2X9R4HXF34.p8",
        "MCP_FACTORY_PROFILES": "core,metadata,assets,testflight",
        "MCP_FACTORY_MEDIA_ROOT": "/abs/path/to/store-assets"
      }
    }
  }
}
```

Codex (`~/.codex/config.toml`):

```toml
[mcp_servers.app-store-connect]
command = "/abs/path/to/target/release/app-store-connect-mcp"
env = { ASC_KEY_ID = "...", ASC_ISSUER_ID = "...", ASC_PRIVATE_KEY_PATH = "/abs/path/AuthKey.p8", MCP_FACTORY_MEDIA_ROOT = "/abs/path/to/store-assets" }
```

Build with `cargo build --release -p app-store-connect-mcp` from the repository
root. `config.toml` is read from the working directory, the binary's
directory, or `MCP_FACTORY_CONFIG`; environment variables override it.

## Workflows

Never hard-code Apple IDs; resolve them with list/filter tools:

| You have | Tool |
|---|---|
| bundle ID / name → app | `apps_getCollection` with `filter[bundleId]` |
| version string → version | `apps_appStoreVersions_getToManyRelated` with `filter[versionString]` |
| locale → localization | `appStoreVersions_appStoreVersionLocalizations_getToManyRelated` |
| build number → build | `builds_getCollection` with `filter[app]`, `filter[version]` |
| group name → beta group | `betaGroups_getCollection` with `filter[app]`, `filter[name]` |
| product ID → subscription | `apps_subscriptionGroups_getToManyRelated` with `include=subscriptions` |
| territory code → territory | `territories_getCollection` |
| display type → screenshot set | `appStoreVersionLocalizations_appScreenshotSets_getToManyRelated` with `filter[screenshotDisplayType]` |

Collections return at most `limit` items. If the result has `links.next`, pass
its `cursor` value back as the `cursor` argument; the last page has no
`links.next`. `meta.paging.total` gives the total count.

**Release** (an existing app):

1. `apps_getCollection` → app; `apps_appStoreVersions_getToManyRelated` → editable version, or `appStoreVersions_createInstance`.
2. Read, then update localizations: `appStoreVersionLocalizations_getInstance` / `_updateInstance` (What's New, description, keywords, promotional text, URLs); app-level name/subtitle via `appInfoLocalizations_*`.
3. Screenshots and previews (below).
4. `builds_getCollection` with `filter[processingState]=VALID` → `appStoreVersions_build_updateToOneRelationship`.
5. `appStoreVersions_appStoreReviewDetail_getToOneRelated` → `appStoreReviewDetails_createInstance` or `_updateInstance`.
6. `reviewSubmissions_createInstance` → `reviewSubmissionItems_createInstance` (the version) → `reviewSubmissions_updateInstance` with `submitted: true`.
7. Watch `reviewSubmissions_getCollection` (`filter[state]`) and the version's `appStoreState`.
8. `appStoreVersionReleaseRequests_createInstance` for manual release, or manage `appStoreVersionPhasedReleases_*` (ACTIVE / PAUSED / COMPLETE).

**Screenshots** (files live under `MCP_FACTORY_MEDIA_ROOT`):

1. `appStoreVersionLocalizations_appScreenshotSets_getToManyRelated` → find the set for a `screenshotDisplayType`, or `appScreenshotSets_createInstance`.
2. `asset_upload` with `kind: appScreenshot`, `parentId: <set id>`, `file: en-US/iphone67/01.png`. It reserves the asset, uploads each byte range Apple asks for, commits with the MD5 checksum and waits for `assetDeliveryState` COMPLETE (`waitSeconds`, default 120).
3. Order with `appScreenshotSets_appScreenshots_replaceToManyRelationship`; remove with `appScreenshots_deleteInstance`.

Previews use `kind: appPreview` (optional `previewFrameTimeCode`, `mimeType`);
App Review attachments, export-compliance documents and subscription/IAP
review screenshots use the same tool. `asset_upload` only offers the kinds
whose profile is enabled: the subscription/IAP review screenshot kinds need
`monetization`; the others need `assets`. A failure reports its stage
(`validate`, `reserve`, `upload`, `commit`, `processing`) and Apple's error;
after an upload failure the reservation stays in `AWAITING_UPLOAD` for you to
delete or retry.

**TestFlight**: find the build → `betaBuildLocalizations_*` (What to Test) →
`betaGroups_createInstance` / `_updateInstance` → `betaTesters_createInstance`
(invites by email) and `betaGroups_betaTesters_createToManyRelationship` →
`betaGroups_builds_createToManyRelationship` → `betaAppReviewDetails_updateInstance`
→ `betaAppReviewSubmissions_createInstance` → inspect `betaAppReviewSubmissions_getCollection`
and `buildBetaDetails_getInstance`.

**Monetization** (`monetization,pricing` profiles): `apps_subscriptionGroups_getToManyRelated`
→ `subscriptionGroups_createInstance` / `subscriptions_createInstance` →
`subscriptionLocalizations_*` → price points (`subscriptions_pricePoints_getToManyRelated`
with `filter[territory]`) → `subscriptionPrices_createInstance` →
`subscriptionPlanAvailabilities_*` → review screenshot via `asset_upload`
(`kind: subscriptionAppStoreReviewScreenshot`) → `subscriptionSubmissions_createInstance`.
In-app purchases follow the same shape with `inAppPurchasesV2_*`,
`inAppPurchaseLocalizations_*`, `inAppPurchasePriceSchedules_createInstance`
and `inAppPurchaseSubmissions_createInstance`. Price-point and territory IDs
always come from the API.

**Reports**: `salesReports_getCollection` / `financeReports_getCollection`
return the gzip TSV as text. Analytics: `analyticsReportRequests_createInstance`
→ `analyticsReportRequests_reports_getToManyRelated` → `analyticsReports_instances_getToManyRelated`
→ `analyticsReportInstances_segments_getToManyRelated` → `analytics_segment_download`
(verifies the MD5, decompresses, truncates to `maxChars`).

## Destructive operations

Tools are annotated for MCP clients: every GET is read-only; every DELETE is
destructive, including certificate revocation, profile, tester, screenshot
and preview deletion, and relationship unlinking; creates are not
idempotent. Destructive descriptions say "This cannot be undone." Review such
calls before approving them. Revoking a certificate breaks apps and profiles
signed with it. `appStoreVersionReleaseRequests_createInstance` publishes the
app.

## Known Apple limitations

- **App creation**: the public API has no `apps` create operation; create the app in the App Store Connect web UI.
- **App Privacy** (nutrition labels): no public endpoints; manage it in the web UI. `PrivacyInfo.xcprivacy` is Xcode project configuration.
- **Version localizations** have no top-level collection; list them with `appStoreVersions_appStoreVersionLocalizations_getToManyRelated`.
- **Beta App Review details** are created by Apple with the app; only read/update exist.
- **Customer review responses** have no update; creating a response replaces the existing one.
- **Age rating**: the declaration is read through `appInfos_ageRatingDeclaration_getToOneRelated` and updated with `ageRatingDeclarations_updateInstance`; answers must come from you.
- **Export compliance**: the server records your answers (`builds_updateInstance` `usesNonExemptEncryption`, `appEncryptionDeclarations_createInstance`); it never decides them.
- **Deprecated APIs are excluded** (e.g. `subscriptionAvailabilities` → `subscriptionPlanAvailabilities`, `appEncryptionDeclarations_builds_*` → `builds_appEncryptionDeclaration_updateToOneRelationship`).
- **Build upload**: Apple's spec includes `buildUploads`/`buildUploadFiles`, but they are not exposed; compile with `flutter build ipa`/Xcode and upload with Transporter, `xcrun altool` or Xcode. Game Center, Xcode Cloud, App Clips and alternative marketplaces are out of scope.
- **Analytics segment URLs expire**; fetch the segment again for a fresh URL.

## Regenerating

```bash
scripts/regenerate.sh                    # openapi.oas.json + tools.toml → src/*.rs
python3 scripts/coverage-table.py        # refresh the coverage table above
cargo test -p app-store-connect-mcp      # generation, hints, schemas, mock-API flows
```

To update the spec, download Apple's current OpenAPI document over
`openapi.oas.json` and run the script. A `tools.toml` pattern that no longer
matches an operation (renamed or deprecated by Apple) fails generation.
Review the `src/tools.rs` diff.
