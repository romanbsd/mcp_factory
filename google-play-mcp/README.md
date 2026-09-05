# Google Play MCP

Local stdio MCP server generated from Google's official API Discovery documents.
It combines:

- Google Play Android Developer API v3
- Google Play Developer Reporting API v1beta1

The current generated surface contains 160 tools: 135 Publisher operations and
25 Reporting operations. Ten binary media-upload operations are intentionally
omitted because `mcp-factory` does not yet model binary request bodies. Use CI,
Fastlane, or Play Console for AAB/APK and image uploads.

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
accepting an upstream API change.
