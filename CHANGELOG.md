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
