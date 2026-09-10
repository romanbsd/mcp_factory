# Changelog

## Unreleased

### Added
- Media-upload support: Google Discovery `mediaUpload.protocols.simple`
  methods now generate a tool that streams a file from a configured
  `MCP_FACTORY_MEDIA_ROOT`, with path-traversal, size, and content-type
  enforcement (`MediaUploadOperation` in `mcp-factory-core`).
- `google-play-mcp` regenerated with 170 low-level tools (up from 160),
  including 10 Android Publisher / App Store media-upload operations.

### Fixed
- REST proxy (`mcp-factory-core`): POST/PUT/PATCH requests with no request
  body now send `Content-Length: 0` explicitly. `reqwest` sends no
  `Content-Length` at all for a request built without a `.body(...)` call,
  and some gRPC-transcoded action endpoints (Android Publisher's
  `edits:validate`/`edits:commit`) reject a bodyless POST with `411 Length
  Required` unless the header is present, even as `0`.
- REST proxy (`mcp-factory-core`): `reqwest` now advertises and decompresses
  `gzip`/`brotli`/`deflate`. Without these features `reqwest` neither sent
  `Accept-Encoding` nor decoded compressed bodies, so upstream gRPC-transcoded
  APIs that reply compressed anyway (e.g. Android Publisher's track-releases
  and App Recovery endpoints) surfaced in `google-play-mcp`'s high-level
  reports as "malformed/non-JSON data".
- `google-play-mcp` high-level `report_*` tools: `applications_tracks_releases_list`
  quota errors ("Listing releases quota exceeded.") return HTTP 403
  PERMISSION_DENIED instead of 429 RESOURCE_EXHAUSTED. `EvidenceClient`'s
  retry logic now treats a 403 whose message contains "quota" as transient,
  matching Google's own guidance to back off and retry these the same as
  429s.
- `google-play-mcp` high-level `report_*` tools: source-evidence calls
  (metric x cohort queries, error/anomaly lookups) now run concurrently
  instead of sequentially, reducing wall-clock time well under the
  45-second reporting deadline for `report_quality_health` and
  `report_project_status`. `EvidenceClient` moved its call bookkeeping
  behind a `Mutex` (`&self` instead of `&mut self`) to allow this.
- REST proxy (`mcp-factory-core`): a successful 204 No Content response is now
  treated as `{}` instead of the literal text `"204 No Content"`. Callers that
  parse the body as JSON (e.g. `google-play-mcp`'s high-level report client)
  previously failed with "trailing characters at line 1 column 5" — some
  gRPC-transcoded APIs (Android Publisher's App Recovery endpoint) use 204 for
  an empty list rather than a 200 with `{}`.
- REST proxy (`mcp-factory-core`): a successful (2xx, non-204) JSON response
  with a zero-byte body is now treated as `{}` instead of a JSON parse
  failure. Some gRPC-transcoded Google APIs (e.g. Android Publisher's
  track-releases and App Recovery endpoints) send an empty body for an
  all-default response, which previously surfaced in `google-play-mcp`'s
  high-level reports as "invalid JSON" / a dropped reviews summary for
  tracks, recoveries, and reviews with no results.
- `google-play-mcp` high-level `report_*` tools: deduplicated
  `applications_tracks_releases_list` calls across tracks that resolve to
  the same track ID, avoiding "Listing releases quota exceeded" errors.
- `report_project_status` recovery lookup now passes the detected serving
  version code (instead of `0`) to `apprecovery_list`, or skips the call
  with an explicit assessment when no serving version is known yet.
- `report_quality` error-count queries send a date-only `endTime` for
  DAILY aggregation instead of a full timestamp, fixing "invalid hour"
  rejections.
- `report_quality` grouped-error lookups send `UTC` instead of
  `America/Los_Angeles`, fixing timezone rejections from the errorIssues
  search endpoint.
