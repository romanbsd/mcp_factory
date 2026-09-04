use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use async_trait::async_trait;
use gcp_auth::{CustomServiceAccount, TokenProvider};
use reqwest::RequestBuilder;

use crate::auth::AuthProvider;
use crate::error::ProxyError;

pub struct GoogleServiceAccountAuthProvider {
    provider: Arc<dyn TokenProvider>,
    scopes: Vec<String>,
}

impl GoogleServiceAccountAuthProvider {
    pub fn from_env(credentials_path_env: &str, scopes: &[String]) -> Result<Self, ProxyError> {
        if scopes.is_empty() {
            return Err(ProxyError::Config(
                "google_service_account auth requires at least one OAuth scope".to_string(),
            ));
        }
        let path = env::var(credentials_path_env)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| {
                ProxyError::Config(format!(
                    "google service-account credential path environment variable {credentials_path_env} is not set"
                ))
            })?;
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
        let scopes = self.scopes.iter().map(String::as_str).collect::<Vec<_>>();
        let token = self.provider.token(&scopes).await.map_err(|error| {
            ProxyError::Other(format!("failed to obtain Google access token: {error}"))
        })?;
        Ok(request.bearer_auth(token.as_str()))
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}
