use std::sync::Arc;

use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::auth::oauth2::OAuth2Provider;
use crate::config::AuthConfig;
use crate::error::ProxyError;

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError>;

    fn api_key_query(&self) -> Option<(String, String)>;
}

pub struct StaticAuthProvider {
    auth: AuthConfig,
}

impl StaticAuthProvider {
    pub fn new(auth: AuthConfig) -> Self {
        Self { auth }
    }
}

#[async_trait]
impl AuthProvider for StaticAuthProvider {
    async fn apply_request_auth(
        &self,
        mut request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        match &self.auth {
            AuthConfig::None => {}
            AuthConfig::Bearer { .. } => {
                if let Some(token) = self.auth.resolve_secret() {
                    request = request.bearer_auth(token);
                }
            }
            AuthConfig::ApiKeyHeader { header, .. } => {
                if let Some(key) = self.auth.resolve_secret() {
                    request = request.header(header.as_str(), key);
                }
            }
            AuthConfig::ApiKeyQuery { .. } | AuthConfig::OAuth2 { .. } => {}
        }
        Ok(request)
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        match &self.auth {
            AuthConfig::ApiKeyQuery { param, .. } => {
                self.auth.resolve_secret().map(|v| (param.clone(), v))
            }
            _ => None,
        }
    }
}

pub struct OAuth2AuthProvider {
    inner: Arc<OAuth2Provider>,
}

impl OAuth2AuthProvider {
    pub fn new(inner: Arc<OAuth2Provider>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl AuthProvider for OAuth2AuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        let token = self.inner.bearer_token().await?;
        Ok(request.bearer_auth(token))
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}

pub fn auth_provider_from_config(
    auth: &AuthConfig,
    http: reqwest::Client,
) -> Result<Arc<dyn AuthProvider>, ProxyError> {
    match auth {
        AuthConfig::None => Ok(Arc::new(StaticAuthProvider::new(AuthConfig::None))),
        AuthConfig::OAuth2 { .. } => {
            let provider = Arc::new(OAuth2Provider::new(auth, http)?);
            Ok(Arc::new(OAuth2AuthProvider::new(provider)))
        }
        _ => Ok(Arc::new(StaticAuthProvider::new(auth.clone()))),
    }
}

pub fn oauth_provider_from_config(
    auth: &AuthConfig,
    http: reqwest::Client,
) -> Result<Arc<OAuth2Provider>, ProxyError> {
    Ok(Arc::new(OAuth2Provider::new(auth, http)?))
}
