use async_trait::async_trait;
use gcp_auth::TokenProvider;
use mcp_factory_core::auth::AuthProvider;
use mcp_factory_core::ProxyError;
use reqwest::RequestBuilder;
use std::sync::Arc;

/// Lazily discover ADC, preserving stdio startup and tools/list without credentials.
pub struct GoogleAdcAuthProvider {
    provider: tokio::sync::OnceCell<Result<Arc<dyn TokenProvider>, String>>,
    quota_project: Option<String>,
    scopes: Vec<String>,
}
impl GoogleAdcAuthProvider {
    pub fn new(scopes: &[String]) -> Result<Self, ProxyError> {
        Self::with_quota_project(scopes, None)
    }
    pub fn with_quota_project(
        scopes: &[String],
        quota_project: Option<String>,
    ) -> Result<Self, ProxyError> {
        if let Some(value) = &quota_project {
            if value.trim().is_empty() || value.trim() != value {
                return Err(ProxyError::Config(
                    "quota_project must be non-blank without surrounding whitespace".into(),
                ));
            }
            reqwest::header::HeaderValue::from_str(value)
                .map_err(|_| ProxyError::Config("invalid quota_project header value".into()))?;
        }

        crate::validate_google_scopes(scopes)?;
        Ok(Self {
            provider: tokio::sync::OnceCell::new(),
            quota_project,
            scopes: scopes.to_vec(),
        })
    }
}
#[async_trait]
impl AuthProvider for GoogleAdcAuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        let provider = self
            .provider
            .get_or_init(|| async { gcp_auth::provider().await.map_err(|e| e.to_string()) })
            .await
            .as_ref()
            .map_err(|e| {
                ProxyError::Config(format!(
                    "Google ADC discovery failed: {e}; fix credentials and restart the server"
                ))
            })?;
        let scopes = self.scopes.iter().map(String::as_str).collect::<Vec<_>>();
        let token = provider
            .token(&scopes)
            .await
            .map_err(|e| ProxyError::Other(format!("Google ADC token failed: {e}")))?;
        let request = request.bearer_auth(token.as_str());
        Ok(if let Some(project) = &self.quota_project {
            request.header("x-goog-user-project", project)
        } else {
            request
        })
    }
    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}

#[cfg(test)]
mod adc_tests {
    use super::*;
    struct FixedToken;
    #[async_trait]
    impl TokenProvider for FixedToken {
        async fn token(&self, _: &[&str]) -> Result<Arc<gcp_auth::Token>, gcp_auth::Error> {
            Ok(Arc::new(
                serde_json::from_value(
                    serde_json::json!({"access_token":"test-token","expires_in":3600}),
                )
                .unwrap(),
            ))
        }
        async fn project_id(&self) -> Result<Arc<str>, gcp_auth::Error> {
            Ok(Arc::from("project"))
        }
    }
    #[tokio::test]
    async fn adc_quota_header_and_cached_failure() {
        let provider = GoogleAdcAuthProvider::with_quota_project(
            &["read".into()],
            Some("billing-project".into()),
        )
        .unwrap();
        assert!(provider.provider.set(Ok(Arc::new(FixedToken))).is_ok());
        let request = provider
            .apply_request_auth(reqwest::Client::new().get("https://example.com"))
            .await
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(request.headers()["x-goog-user-project"], "billing-project");
        assert_eq!(request.headers()["authorization"], "Bearer test-token");
        let failed = GoogleAdcAuthProvider::new(&["read".into()]).unwrap();
        assert!(failed.provider.set(Err("missing".into())).is_ok());
        for _ in 0..2 {
            assert!(matches!(
                failed
                    .apply_request_auth(reqwest::Client::new().get("https://example.com"))
                    .await,
                Err(ProxyError::Config(_))
            ));
        }
    }
    #[tokio::test]
    async fn adc_without_quota_project_does_not_attach_project_header() {
        let provider = GoogleAdcAuthProvider::new(&["read".into()]).unwrap();
        assert!(provider.provider.set(Ok(Arc::new(FixedToken))).is_ok());
        let request = provider
            .apply_request_auth(reqwest::Client::new().get("https://example.com"))
            .await
            .unwrap()
            .build()
            .unwrap();
        assert!(!request.headers().contains_key("x-goog-user-project"));
    }
    #[test]
    fn adc_is_lazy_and_requires_scopes() {
        assert!(GoogleAdcAuthProvider::new(&[]).is_err());
        let provider = GoogleAdcAuthProvider::new(&[
            "https://www.googleapis.com/auth/analytics.readonly".into(),
        ])
        .unwrap();
        assert!(provider.provider.get().is_none());
    }
}
