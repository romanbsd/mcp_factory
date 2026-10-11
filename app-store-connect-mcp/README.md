# App Store Connect MCP

Local MCP server for Apple's official [App Store Connect API](https://developer.apple.com/documentation/appstoreconnectapi), based on the bundled OpenAPI 4.5 specification. The default `workflow` profile exposes **15 task tools** for an existing app: release checks, version metadata, screenshots, TestFlight, asset uploads and analytics/sales/finance report queries. Raw Apple operations remain available through explicit profiles.

Only public, documented endpoints are used. App creation and App Privacy labels remain in the App Store Connect web UI; building and uploading an IPA uses your existing Apple tooling.

## Quickstart

From the repository root:

```bash
cargo build --release -p app-store-connect-mcp
```

Create an App Store Connect API key, set the three authentication variables below, and configure your MCP client:

```json
{
  "mcpServers": {
    "app-store-connect": {
      "command": "/abs/path/to/target/release/app-store-connect-mcp",
      "env": {
        "ASC_KEY_ID": "2X9R4HXF34",
        "ASC_ISSUER_ID": "57246542-96fe-1a63-e053-0824d011072a",
        "ASC_PRIVATE_KEY_PATH": "/abs/path/AuthKey_2X9R4HXF34.p8",
        "MCP_FACTORY_PROFILES": "workflow",
        "MCP_FACTORY_MEDIA_ROOT": "/abs/path/to/store-assets"
      }
    }
  }
}
```

The paths and key identifiers in this configuration are examples; substitute your local binary, downloaded key and asset directory. Omitting `MCP_FACTORY_PROFILES` selects `workflow`. Files passed to upload tools must be relative to `MCP_FACTORY_MEDIA_ROOT`.

Start with `app_get`, `version_get`, then `release_prepare`. Mutating workflows accept `dryRun: true` to discover and compare without upstream writes. See the [workflow reference](docs/workflows.md) for every input, examples, resumable uploads and bounded analytics/sales/finance report queries.

For Codex, put the server under `[mcp_servers.app-store-connect]` in `~/.codex/config.toml`, with the same command and environment variables. `config.toml` is read from the working directory, binary directory, or `MCP_FACTORY_CONFIG`; environment variables override it.

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

## Select task or raw API profiles

Set `MCP_FACTORY_PROFILES=workflow,release_raw` or `profiles = ["workflow", "release_raw"]` in `config.toml`. Unknown names fail startup. Raw operation names are Apple's `operationId`s, such as `appStoreVersions_updateInstance`.

| Profile | Public surface |
|---|---|
| `workflow` (default) | 13 domain workflows, `asset_upload`, `report_query` |
| `release_raw` | Release, version, metadata, review and screenshot/preview operations with their lookups; asset upload |
| `testflight_raw` | TestFlight operations and supporting app/build lookups |
| `monetization_raw` | Subscription/IAP operations and supporting lookups; asset upload |
| `signing_raw` | Signing operations and supporting lookups |
| `reports_raw` | Report operations and `analytics_segment_download` |
| `core` | Apps, versions, builds, review submission, release and phased release |
| `metadata` | App Info/localizations, version localizations, categories, age rating and export compliance |
| `assets` | Screenshot/preview sets, assets, ordering, review attachments and asset upload |
| `testflight` | Beta groups/testers, memberships, builds, beta metadata and review |
| `monetization`, `pricing`, `signing`, `reviews`, `reports` | Existing raw domain profiles |
| `all` | All curated generated operations and custom tools |

Raw collections use Apple's pagination: read `links.next`, extract its `cursor`, then pass `cursor` to the same tool. Use `include` for related resources and `fields[type]` to trim attributes. Compact JSON:API responses retain pagination while removing other links and link-only relationships.

The previous default exposed 137 tools / 126,861 compact bytes. The measured workflow default exposes **15 tools / 14,777 bytes**. See [validation and measurements](docs/validation.md) for profile sizes, task call counts, test results and resource observations; [exact tool schemas](docs/workflow-tools.json) are exported from the built server. [Live read-only and dry-run validation](docs/live-validation.md) covers the three available apps. The [bounded-discovery follow-up](docs/iteration2.md) adds incremental pagination, partial-write progress and the opt-in live runner. The [DRY refactoring notes](docs/refactoring.md) describe shared contracts, resume inspection, transfer primitives and profile composition.

## Architecture and on-demand reference

`tools.toml` controls curated Apple operations, profiles and description overrides. Its `profile_extends` table composes raw presets from canonical domain profiles and inherits handwritten tool membership, avoiding copied operation lists. `mcp-gen` generates operation factories; the internal capability catalog resolves schemas and validators lazily. Workflow services call these capabilities through `OperationInvoker`, which validates input and executes HTTP directly. Public profile visibility does not limit a workflow's internal dependencies, and workflows do not call public MCP tools recursively. Read-only workflows cannot invoke mutating operations.

Public `tools/list` metadata is sorted and cached. Request `schema://operation/<operationId>` for one generated operation's schema. Short guides are available at `docs://workflow/release`, `docs://workflow/metadata`, `docs://workflow/screenshots`, `docs://workflow/testflight` and `docs://workflow/reports`. The full bundled specification remains available through `schema://openapi` for advanced use.

Handwritten orchestration lives in `src/extensions.rs` and `src/extensions/`: an exhaustive workflow registry defines tool contracts, shared typed upload sessions validate checkpoints, localization queries are centralized, and transfer primitives live in the core; generated files are `src/tools.rs`, `src/main.rs` and `src/resources.rs`. Shared HTTP/authentication, internal invocation and profile filtering live in `mcp-factory-core`.

## Mutation behavior

`metadata_apply`, `screenshots_sync`, `testflight_distribute`, `testflight_remove`, `review_submit` and `release` compare state before writing. Their dry runs return `changes`, `unchanged` and `warnings`; resource-specific entries identify locales, groups, files or submission IDs. A dry run performs reads and local validation, so it can still report lookup or preflight errors.

`release_prepare` reports known blockers and unknown Apple-evaluated conditions. `no_known_blockers` does not establish Apple's eventual approval. `review_submit` reconciles a draft/submitted review request; `release` separately requests manual publication of an approved version in `PENDING_DEVELOPER_RELEASE`.

Raw GET tools are read-only; DELETE tools are destructive, including relationship unlinking. Certificate revocation affects apps/profiles that use that certificate. `release` and the raw manual-release request publish the app. Tool hints expose mutation/destructive behavior to MCP clients.

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

## Regenerate and validate

Run from this crate for regeneration, then from the repository root for Cargo commands:

```bash
cd app-store-connect-mcp
scripts/regenerate.sh
python3 scripts/coverage-table.py
cd ..
cargo test -p app-store-connect-mcp
```

The coverage script prints the curated raw-operation table. To update the specification, replace `openapi.oas.json` with Apple's current OpenAPI document and regenerate. Unmatched `tools.toml` patterns fail generation. Review generated-file diffs and retain the handwritten extensions.

Tests exercise generation/profile contracts, schema validation, internal operation access boundaries and mock HTTP workflow flows. They use local fixtures rather than production credentials; passing them does not verify live App Store Connect permissions or Apple's approval decisions.
