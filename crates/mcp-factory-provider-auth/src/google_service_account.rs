use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use gcp_auth::{CustomServiceAccount, TokenProvider};
use reqwest::RequestBuilder;

use mcp_factory_core::auth::AuthProvider;
use mcp_factory_core::ProxyError;

pub struct GoogleServiceAccountAuthProvider {
    provider: Arc<dyn TokenProvider>,
    scopes: Vec<String>,
}

impl GoogleServiceAccountAuthProvider {
    pub fn from_env(credentials_path_env: &str, scopes: &[String]) -> Result<Self, ProxyError> {
        crate::validate_google_scopes(scopes)?;
        let path = match env::var(credentials_path_env) {
            Ok(value) if !value.trim().is_empty() => value.trim().to_owned(),
            Ok(_) | Err(env::VarError::NotPresent) => {
                return Err(ProxyError::Config(format!(
                    "Google credential path variable {credentials_path_env} is missing or blank"
                )))
            }
            Err(env::VarError::NotUnicode(_)) => {
                return Err(ProxyError::Config(format!(
                    "Google credential path variable {credentials_path_env} is not valid Unicode"
                )))
            }
        };
        let provider = CustomServiceAccount::from_file(PathBuf::from(path)).map_err(|error| {
            ProxyError::Config(format!(
                "failed to load Google service-account credentials from {credentials_path_env}: {error}"
            ))
        })?;
        Ok(Self {
            provider: Arc::new(provider),
            scopes: scopes.to_vec(),
        })
    }
}

#[async_trait]
impl AuthProvider for GoogleServiceAccountAuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        crate::google_token::apply_request_auth(self.provider.as_ref(), &self.scopes, None, request)
            .await
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}
