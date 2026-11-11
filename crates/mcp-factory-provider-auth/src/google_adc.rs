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
    retry_state: std::sync::Arc<tokio::sync::Mutex<RetryState>>,
}

#[derive(Default)]
struct RetryState {
    last_failure: Option<std::time::Instant>,
    consecutive_failures: u32,
    retrying: bool,
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
            retry_state: std::sync::Arc::new(tokio::sync::Mutex::new(RetryState::default())),
        })
    }
}
#[async_trait]
impl AuthProvider for GoogleAdcAuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        // First, try to use the cached provider if available and successful
        if let Some(Ok(provider)) = self.provider.get() {
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

        // Check if we should retry discovery after a previous failure
        let (should_retry, retry_in_progress) = {
            let mut retry_state = self.retry_state.lock().await;
            if let Some(failure_time) = retry_state.last_failure {
                // Exponential backoff: 5s, 10s, 20s, 40s, 60s (capped)
                // Use consecutive_failures - 1 so first retry is at 5s
                let delay_secs =
                    (5u64 << retry_state.consecutive_failures.saturating_sub(1).min(4)).min(60);
                let retry_after = std::time::Duration::from_secs(delay_secs);
                let elapsed = failure_time.elapsed();
                let can_retry = elapsed >= retry_after && !retry_state.retrying;

                if can_retry {
                    // Mark that we're retrying to prevent concurrent attempts
                    retry_state.retrying = true;
                    (true, false)
                } else {
                    (false, retry_state.retrying)
                }
            } else {
                (false, false)
            }
        };

        if should_retry {
            // Spawn discovery in a detached task so cancellation of this request doesn't leave retrying=true
            let provider_cell = self.provider.clone();
            let retry_state_arc = self.retry_state.clone();
            let discovery_handle = tokio::spawn(async move {
                let result = google_adc_file::discover().await;
                let mut retry_state = retry_state_arc.lock().await;

                match &result {
                    Ok(provider) => {
                        // Success! Reset failure state
                        retry_state.last_failure = None;
                        retry_state.consecutive_failures = 0;
                        retry_state.retrying = false;

                        // Try to store in the OnceCell (may fail if another task beat us to it)
                        let _ = provider_cell.set(Ok(provider.clone()));
                        Ok(provider.clone())
                    }
                    Err(e) => {
                        // Failed again, update retry state
                        retry_state.last_failure = Some(std::time::Instant::now());
                        retry_state.consecutive_failures =
                            retry_state.consecutive_failures.saturating_add(1);
                        retry_state.retrying = false;
                        Err(e.clone())
                    }
                }
            });

            // Await the discovery result
            match discovery_handle.await {
                Ok(Ok(provider)) => {
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
                Ok(Err(e)) => {
                    let retry_state = self.retry_state.lock().await;
                    let next_delay =
                        (5u64 << retry_state.consecutive_failures.saturating_sub(1).min(4)).min(60);
                    return Err(ProxyError::Config(format!(
                        "Google ADC discovery failed: {e}; will retry after {}s",
                        next_delay
                    )));
                }
                Err(_) => {
                    // Task panicked; clear retry flag and treat as error
                    let mut retry_state = self.retry_state.lock().await;
                    retry_state.retrying = false;
                    return Err(ProxyError::Config(
                        "Google ADC discovery task failed".into(),
                    ));
                }
            }
        }

        // If another request is currently retrying, return the cached error
        if retry_in_progress {
            return Err(ProxyError::Config(
                "Google ADC discovery failed; retry in progress by another request".into(),
            ));
        }

        // First-time discovery or not yet time to retry - check if already initialized
        if let Some(result) = self.provider.get() {
            // Already initialized (either success or cached error)
            let provider = result.as_ref().map_err(|e| {
                ProxyError::Config(format!(
                    "Google ADC discovery failed: {e}; will retry after 5 seconds"
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

        // First-time discovery
        let result = google_adc_file::discover().await;

        match result {
            Ok(provider) => {
                // Success on first try
                let _ = self.provider.set(Ok(provider.clone()));
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
            Err(e) => {
                // Initial failure - record it and cache the error
                let _ = self.provider.set(Err(e.clone()));
                let mut retry_state = self.retry_state.lock().await;
                retry_state.last_failure = Some(std::time::Instant::now());
                retry_state.consecutive_failures = 1;
                drop(retry_state);

                Err(ProxyError::Config(format!(
                    "Google ADC discovery failed: {e}; will retry after 5 seconds"
                )))
            }
        }
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

#[cfg(test)]
mod retry_tests {
    use super::*;

    #[tokio::test]
    async fn retry_state_allows_recovery() {
        let auth = GoogleAdcAuthProvider::new(&["read".into()]).unwrap();

        // Simulate initial failure
        assert!(auth.provider.set(Err("initial failure".into())).is_ok());
        {
            let mut retry_state = auth.retry_state.lock().await;
            retry_state.last_failure =
                Some(std::time::Instant::now() - std::time::Duration::from_secs(10));
            retry_state.consecutive_failures = 1;
        }

        // Verify retry state was set
        let retry_state = auth.retry_state.lock().await;
        assert_eq!(retry_state.consecutive_failures, 1);
        assert!(retry_state.last_failure.is_some());
    }

    #[tokio::test]
    async fn exponential_backoff_calculation() {
        let auth = GoogleAdcAuthProvider::new(&["read".into()]).unwrap();

        // Test backoff calculation
        let test_cases = vec![
            (0, 5),  // 5 * 2^0 = 5
            (1, 10), // 5 * 2^1 = 10
            (2, 20), // 5 * 2^2 = 20
            (3, 40), // 5 * 2^3 = 40
            (4, 60), // 5 * 2^4 = 80, capped at 60
            (5, 60), // 5 * 2^5 = 160, capped at 60
        ];

        for (failures, expected_delay) in test_cases {
            let calculated = (5u64 << failures.min(4)).min(60);
            assert_eq!(
                calculated, expected_delay,
                "Backoff for {} failures should be {}s",
                failures, expected_delay
            );
        }
    }

    #[test]
    fn retry_state_tracks_consecutive_failures() {
        let mut state = RetryState::default();
        assert_eq!(state.consecutive_failures, 0);
        assert!(state.last_failure.is_none());

        state.last_failure = Some(std::time::Instant::now());
        state.consecutive_failures = 1;
        assert_eq!(state.consecutive_failures, 1);
    }
}
