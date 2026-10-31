# Provider authentication adapters

Optional Google ADC, Google service-account and App Store Connect authentication.
Only provider-backed servers depend on this crate. The core exposes the generic
`AuthProvider` interface and builder injection; it does not depend on this crate.

Server extensions return `from_config(config)` from `build_auth_provider`. The
shared generator recognizes that optional hook and passes the provider to the
builder without importing any vendor adapter.

```toml
[auth]
type = "external"
provider = "google_service_account"
[auth.options]
credentials_path_env = "GOOGLE_APPLICATION_CREDENTIALS"
scopes = ["https://www.googleapis.com/auth/analytics.readonly"]
```

`google_adc` takes `scopes`. `app_store_connect` takes optional `key_id_env`,
`issuer_id_env`, and `private_key_path_env`; defaults use the corresponding ASC
variables. Unknown adapters and missing required credentials fail closed.

Migration: old provider-specific `[auth] type` values become `type = "external"`
with the previous type as `provider`. Move all remaining auth fields into
`[auth.options]`. Plain bearer/API-key/OAuth2 configuration is handled by core.

Adapters are feature-gated: Google servers enable `google`; the App Store server
enables `app-store-connect`. Neither is enabled by default. Provider options are
typed and reject unknown keys. Google ADC optionally accepts `quota_project` to
send `x-goog-user-project`; discovery failures are cached until process restart.
