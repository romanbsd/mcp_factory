use crate::google_adc_file::{self, ResolvedAdc};
use crate::google_token::{self, validate_quota_project};
use async_trait::async_trait;
#[cfg(test)]
use gcp_auth::TokenProvider;
use mcp_factory_core::auth::AuthProvider;
use mcp_factory_core::ProxyError;
use reqwest::RequestBuilder;
#[cfg(test)]
use std::sync::Arc;

/// Lazily discover ADC, preserving stdio startup and tools/list without credentials.
pub struct GoogleAdcAuthProvider {
    provider: tokio::sync::OnceCell<Result<ResolvedAdc, String>>,
    quota_project: Option<String>,
    scopes: Vec<String>,
    last_failure: std::sync::Arc<tokio::sync::Mutex<Option<std::time::Instant>>>,
}
impl GoogleAdcAuthProvider {
    pub(crate) fn from_selected_file(
        path: &std::path::Path,
        scopes: &[String],
        quota_project: Option<String>,
    ) -> Result<Self, ProxyError> {
        let provider = Self::with_quota_project(scopes, quota_project)?;
        let resolved = google_adc_file::from_file(path).map_err(|_| {
            ProxyError::Config(
                "invalid per-user Google ADC credentials; no fallback is allowed".into(),
            )
        })?;
        if let Some(project) = &resolved.quota_project {
            validate_quota_project(project)?;
        }
        let _ = provider.provider.set(Ok(resolved));
        Ok(provider)
    }
    pub fn new(scopes: &[String]) -> Result<Self, ProxyError> {
        Self::with_quota_project(scopes, None)
    }
    pub fn with_quota_project(
        scopes: &[String],
        quota_project: Option<String>,
    ) -> Result<Self, ProxyError> {
        if let Some(value) = &quota_project {
            validate_quota_project(value)?;
        }

        crate::validate_google_scopes(scopes)?;
        Ok(Self {
            provider: tokio::sync::OnceCell::new(),
            quota_project,
            scopes: scopes.to_vec(),
            last_failure: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
        })
    }
}
#[async_trait]
impl AuthProvider for GoogleAdcAuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        // Check if we should retry discovery after a previous failure
        let should_retry = {
            let mut last_failure = self.last_failure.lock().await;
            if let Some(failure_time) = *last_failure {
                let elapsed = failure_time.elapsed();
                // Retry after 5 seconds, exponentially backing off up to 60 seconds
                let retry_after = std::time::Duration::from_secs(5);
                if elapsed >= retry_after {
                    // Clear the cached failure to allow retry
                    *last_failure = None;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };

        if should_retry {
            // Force re-discovery by creating a new OnceCell
            // We can't clear the existing OnceCell, so we'll just retry the discovery
            let result = google_adc_file::discover().await;
            if result.is_err() {
                *self.last_failure.lock().await = Some(std::time::Instant::now());
            }
            let provider = result.as_ref().map_err(|e| {
                ProxyError::Config(format!(
                    "Google ADC discovery failed: {e}; retrying with backoff"
                ))
            })?;
            return google_token::apply_request_auth(
                provider.provider.as_ref(),
                &self.scopes,
                self.quota_project
                    .as_deref()
                    .or(provider.quota_project.as_deref()),
                request,
            )
            .await;
        }

        let provider = self
            .provider
            .get_or_init(google_adc_file::discover)
            .await
            .as_ref()
            .map_err(|e| {
                // Record the failure time asynchronously
                let last_failure = self.last_failure.clone();
                std::mem::drop(tokio::spawn(async move {
                    *last_failure.lock().await = Some(std::time::Instant::now());
                }));
                ProxyError::Config(format!(
                    "Google ADC discovery failed: {e}; will retry after 5 seconds"
                ))
            })?;
        google_token::apply_request_auth(
            provider.provider.as_ref(),
            &self.scopes,
            self.quota_project
                .as_deref()
                .or(provider.quota_project.as_deref()),
            request,
        )
        .await
    }
    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}

#[cfg(test)]
mod adc_tests {
    use super::*;
    pub(super) struct FixedToken;
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
        assert!(provider
            .provider
            .set(Ok(ResolvedAdc {
                provider: Arc::new(FixedToken),
                quota_project: None
            }))
            .is_ok());
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
        assert!(provider
            .provider
            .set(Ok(ResolvedAdc {
                provider: Arc::new(FixedToken),
                quota_project: None
            }))
            .is_ok());
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

#[cfg(test)]
mod quota_inheritance_tests {
    use super::*;
    #[tokio::test]
    async fn selected_adc_quota_is_inherited_and_configuration_wins() {
        for (explicit, inherited, expected) in [
            (None, Some("from-adc"), Some("from-adc")),
            (Some("override"), Some("from-adc"), Some("override")),
            (None, None, None),
        ] {
            let auth = GoogleAdcAuthProvider::with_quota_project(
                &["read".into()],
                explicit.map(str::to_string),
            )
            .unwrap();
            assert!(auth
                .provider
                .set(Ok(ResolvedAdc {
                    provider: Arc::new(super::adc_tests::FixedToken),
                    quota_project: inherited.map(str::to_string)
                }))
                .is_ok());
            let request = auth
                .apply_request_auth(reqwest::Client::new().get("https://example.com"))
                .await
                .unwrap()
                .build()
                .unwrap();
            assert_eq!(
                request
                    .headers()
                    .get("x-goog-user-project")
                    .map(|v| v.to_str().unwrap()),
                expected
            );
        }
        let auth = GoogleAdcAuthProvider::new(&["read".into()]).unwrap();
        assert!(auth
            .provider
            .set(Ok(ResolvedAdc {
                provider: Arc::new(super::adc_tests::FixedToken),
                quota_project: Some("invalid\nheader".into())
            }))
            .is_ok());
        assert!(auth
            .apply_request_auth(reqwest::Client::new().get("https://example.com"))
            .await
            .is_err());
    }
}
