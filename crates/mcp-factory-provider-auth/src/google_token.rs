//! Shared request authentication after a Google credential source is selected.
use gcp_auth::TokenProvider;
use mcp_factory_core::ProxyError;
use reqwest::RequestBuilder;

pub(super) fn validate_quota_project(value: &str) -> Result<(), ProxyError> {
    if value.trim().is_empty() || value.trim() != value {
        return Err(ProxyError::Config(
            "quota_project must be non-blank without surrounding whitespace".into(),
        ));
    }
    reqwest::header::HeaderValue::from_str(value)
        .map_err(|_| ProxyError::Config("invalid quota_project header value".into()))?;
    Ok(())
}

pub(super) async fn apply_request_auth(
    provider: &dyn TokenProvider,
    scopes: &[String],
    quota_project: Option<&str>,
    request: RequestBuilder,
) -> Result<RequestBuilder, ProxyError> {
    // Validate before any token signing, refresh or network activity.
    if let Some(project) = quota_project {
        validate_quota_project(project)?;
    }
    let scopes = scopes.iter().map(String::as_str).collect::<Vec<_>>();
    let token = provider.token(&scopes).await.map_err(|error| {
        ProxyError::Other(format!("failed to obtain Google access token: {error}"))
    })?;
    let request = request.bearer_auth(token.as_str());
    Ok(if let Some(project) = quota_project {
        request.header("x-goog-user-project", project)
    } else {
        request
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};
    #[derive(Default)]
    struct RecordingProvider(Mutex<Vec<Vec<String>>>);
    #[async_trait]
    impl TokenProvider for RecordingProvider {
        async fn token(&self, scopes: &[&str]) -> Result<Arc<gcp_auth::Token>, gcp_auth::Error> {
            self.0
                .lock()
                .unwrap()
                .push(scopes.iter().map(|s| s.to_string()).collect());
            Ok(Arc::new(
                serde_json::from_value(
                    serde_json::json!({"access_token":"test-token","expires_in":3600}),
                )
                .unwrap(),
            ))
        }
        async fn project_id(&self) -> Result<Arc<str>, gcp_auth::Error> {
            panic!("request authentication must not infer quota from the resource project")
        }
    }
    #[tokio::test]
    async fn forwards_scopes_and_only_the_selected_quota_project() {
        let provider = RecordingProvider::default();
        let scopes = vec!["read".into(), "other-read".into()];
        for quota in [None, Some("billing-project")] {
            let request = apply_request_auth(
                &provider,
                &scopes,
                quota,
                reqwest::Client::new().get("https://example.com"),
            )
            .await
            .unwrap()
            .build()
            .unwrap();
            assert_eq!(request.headers()["authorization"], "Bearer test-token");
            assert_eq!(
                request
                    .headers()
                    .get("x-goog-user-project")
                    .map(|h| h.to_str().unwrap()),
                quota
            );
        }
        assert_eq!(*provider.0.lock().unwrap(), vec![scopes.clone(), scopes]);
    }
    #[tokio::test]
    async fn invalid_quota_is_rejected_before_requesting_a_token() {
        let provider = RecordingProvider::default();
        for quota in ["", " project", "project\nheader"] {
            assert!(apply_request_auth(
                &provider,
                &["read".into()],
                Some(quota),
                reqwest::Client::new().get("https://example.com")
            )
            .await
            .is_err());
        }
        assert!(provider.0.lock().unwrap().is_empty());
    }
}
