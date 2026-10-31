//! Optional vendor adapters; the core depends only on its AuthProvider interface.
#[cfg(feature = "app-store-connect")]
mod app_store_connect;
#[cfg(feature = "google")]
mod google_adc;
#[cfg(feature = "google")]
mod google_service_account;
#[cfg(feature = "app-store-connect")]
pub use app_store_connect::AppStoreConnectAuthProvider;
#[cfg(feature = "google")]
pub use google_adc::GoogleAdcAuthProvider;
#[cfg(feature = "google")]
pub use google_service_account::GoogleServiceAccountAuthProvider;
use mcp_factory_core::auth::AuthProvider;
use mcp_factory_core::{AuthConfig, ProxyConfig, ProxyError};
#[cfg(any(feature = "google", feature = "app-store-connect"))]
use serde::Deserialize;
use std::sync::Arc;

#[cfg(feature = "google")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdcOptions {
    #[serde(default)]
    scopes: Vec<String>,
    quota_project: Option<String>,
}
#[cfg(feature = "google")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceOptions {
    scopes: Vec<String>,
    credentials_path_env: Option<String>,
}
#[cfg(feature = "app-store-connect")]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AppleOptions {
    key_id_env: Option<String>,
    issuer_id_env: Option<String>,
    private_key_path_env: Option<String>,
}
#[allow(dead_code)]
fn options<T: serde::de::DeserializeOwned>(
    value: &serde_json::Map<String, serde_json::Value>,
) -> Result<T, ProxyError> {
    serde_json::from_value(serde_json::Value::Object(value.clone()))
        .map_err(|error| ProxyError::Config(format!("invalid provider options: {error}")))
}
pub fn from_config(config: &ProxyConfig) -> Result<Option<Arc<dyn AuthProvider>>, ProxyError> {
    let AuthConfig::External {
        provider,
        options: settings,
    } = &config.auth
    else {
        return Ok(None);
    };
    let _ = settings;
    match provider.as_str() {
        #[cfg(feature = "google")]
        "google_adc" => {
            let opt: AdcOptions = options(settings)?;
            Ok(Some(Arc::new(GoogleAdcAuthProvider::with_quota_project(
                &opt.scopes,
                opt.quota_project,
            )?)))
        }
        #[cfg(feature = "google")]
        "google_service_account" => {
            let opt: ServiceOptions = options(settings)?;
            Ok(Some(Arc::new(GoogleServiceAccountAuthProvider::from_env(
                opt.credentials_path_env
                    .as_deref()
                    .unwrap_or("GOOGLE_APPLICATION_CREDENTIALS"),
                &opt.scopes,
            )?)))
        }
        #[cfg(feature = "app-store-connect")]
        "app_store_connect" => {
            let opt: AppleOptions = options(settings)?;
            Ok(Some(Arc::new(AppStoreConnectAuthProvider::from_env(
                opt.key_id_env.as_deref().unwrap_or("ASC_KEY_ID"),
                opt.issuer_id_env.as_deref().unwrap_or("ASC_ISSUER_ID"),
                opt.private_key_path_env
                    .as_deref()
                    .unwrap_or("ASC_PRIVATE_KEY_PATH"),
            )?)))
        }
        _ => Err(ProxyError::Config(format!(
            "unsupported or disabled external auth provider: {provider}"
        ))),
    }
}

#[cfg(feature = "google")]
fn validate_google_scopes(scopes: &[String]) -> Result<(), ProxyError> {
    if scopes.is_empty()
        || scopes
            .iter()
            .any(|scope| scope.trim().is_empty() || scope.trim() != scope)
    {
        return Err(ProxyError::Config(
            "Google authentication requires non-blank OAuth scopes without surrounding whitespace"
                .into(),
        ));
    }
    Ok(())
}

#[cfg(all(test, feature = "google"))]
mod google_auth_tests;

#[cfg(test)]
mod tests {
    use super::*;
    fn configured(provider: &str, settings: serde_json::Value) -> ProxyConfig {
        ProxyConfig {
            auth: AuthConfig::External {
                provider: provider.into(),
                options: settings.as_object().unwrap().clone(),
            },
            ..Default::default()
        }
    }
    #[test]
    fn unknown_provider_rejected() {
        assert!(from_config(&configured("unknown", serde_json::json!({}))).is_err());
        assert!(from_config(&ProxyConfig::default()).unwrap().is_none());
    }
    #[cfg(feature = "google")]
    #[test]
    fn google_options_fail_closed() {
        for settings in [
            serde_json::json!({"scopes":["read"],"quota_projet":"typo"}),
            serde_json::json!({"scopes":["read"],"quota_project":3}),
            serde_json::json!({}),
        ] {
            assert!(from_config(&configured("google_adc", settings)).is_err());
        }
        assert!(from_config(&configured(
            "google_adc",
            serde_json::json!({"scopes":["read"],"quota_project":"project"})
        ))
        .unwrap()
        .is_some());
        assert!(from_config(&configured(
            "google_service_account",
            serde_json::json!({"scopes":["read"],"credentials_path_env":3})
        ))
        .is_err());
    }
    #[cfg(feature = "app-store-connect")]
    #[test]
    fn apple_options_are_typed() {
        assert!(options::<AppleOptions>(
            &serde_json::json!({"keyid_env":"typo"})
                .as_object()
                .unwrap()
                .clone()
        )
        .is_err());
        assert!(options::<AppleOptions>(
            &serde_json::json!({"key_id_env":3})
                .as_object()
                .unwrap()
                .clone()
        )
        .is_err());
        let opt:AppleOptions=options(&serde_json::json!({"key_id_env":"KEY","issuer_id_env":"ISSUER","private_key_path_env":"PATH"}).as_object().unwrap().clone()).unwrap();
        assert_eq!(opt.key_id_env.as_deref(), Some("KEY"));
        assert_eq!(opt.issuer_id_env.as_deref(), Some("ISSUER"));
        assert_eq!(opt.private_key_path_env.as_deref(), Some("PATH"));
    }
}
