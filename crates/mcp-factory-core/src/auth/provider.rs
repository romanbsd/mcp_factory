use std::sync::Arc;

use async_trait::async_trait;
use reqwest::RequestBuilder;

use crate::auth::oauth2::OAuth2Provider;
use crate::config::AuthConfig;
use crate::error::ProxyError;

#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Opt in to authenticated HTTP with caller-owned upstream credentials.
    fn requires_http_identity(&self) -> bool {
        false
    }

    /// Validate an MCP credential, never an upstream token. The principal must
    /// be stable, bounded by the configured user set, and unique per identity.
    async fn authenticate_http(&self, _bearer: &str) -> Result<HttpIdentity, ProxyError> {
        Err(ProxyError::Validation(
            "HTTP authentication is unavailable".into(),
        ))
    }

    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError>;

    fn api_key_query(&self) -> Option<(String, String)>;
}

/// Immutable credentials for exactly one authenticated HTTP request.
#[derive(Clone)]
pub struct HttpIdentity {
    pub principal: String,
    pub auth: Arc<dyn AuthProvider>,
}

pub struct StaticAuthProvider {
    auth: AuthConfig,
    /// Secret resolved once at construction — env vars are effectively immutable
    /// for the process, so there's no need to re-read them on every request.
    secret: Option<String>,
}

impl StaticAuthProvider {
    pub fn new(auth: AuthConfig) -> Self {
        let secret = auth.resolve_secret();
        Self { auth, secret }
    }
}

#[async_trait]
impl AuthProvider for StaticAuthProvider {
    async fn apply_request_auth(
        &self,
        mut request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        match (&self.auth, &self.secret) {
            (AuthConfig::Bearer { .. }, Some(token)) => {
                request = request.bearer_auth(token);
            }
            (AuthConfig::ApiKeyHeader { header, .. }, Some(key)) => {
                request = request.header(header.as_str(), key);
            }
            _ => {}
        }
        Ok(request)
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        match (&self.auth, &self.secret) {
            (AuthConfig::ApiKeyQuery { param, .. }, Some(secret)) => {
                Some((param.clone(), secret.clone()))
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
    interactive: bool,
) -> Result<Arc<dyn AuthProvider>, ProxyError> {
    match auth {
        AuthConfig::Auto | AuthConfig::None => {
            Ok(Arc::new(StaticAuthProvider::new(AuthConfig::None)))
        }
        AuthConfig::OAuth2 { .. } => {
            let provider = Arc::new(OAuth2Provider::new(auth, http, interactive)?);
            Ok(Arc::new(OAuth2AuthProvider::new(provider)))
        }
        AuthConfig::External { provider, .. } => Err(ProxyError::Config(format!(
            "external auth provider {provider:?} requires a server adapter"
        ))),
        _ => Ok(Arc::new(StaticAuthProvider::new(auth.clone()))),
    }
}

pub fn oauth_provider_from_config(
    auth: &AuthConfig,
    http: reqwest::Client,
) -> Result<Arc<OAuth2Provider>, ProxyError> {
    // Explicit login command drives the flow directly via perform_login, so the
    // lazy interactive gate is irrelevant here.
    Ok(Arc::new(OAuth2Provider::new(auth, http, true)?))
}
