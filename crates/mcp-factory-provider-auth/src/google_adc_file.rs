//! Select an ADC file explicitly; never fall through to a different identity on parse errors.
use async_trait::async_trait;
use gcp_auth::{Token, TokenProvider};
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub(super) struct ResolvedAdc {
    pub provider: Arc<dyn TokenProvider>,
    pub quota_project: Option<String>,
}

pub(super) fn selected_file(
    explicit: Option<std::ffi::OsString>,
    default: Option<PathBuf>,
) -> Result<Option<PathBuf>, String> {
    if let Some(path) = explicit {
        if path.is_empty() {
            return Err("GOOGLE_APPLICATION_CREDENTIALS is empty".into());
        }
        return Ok(Some(path.into()));
    }
    if let Some(path) = default {
        return match std::fs::symlink_metadata(&path) {
            Ok(_) => Ok(Some(path)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(_) => Err("Cannot inspect the default ADC credential file".into()),
        };
    }
    Ok(None)
}

pub(super) fn default_file() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("CLOUDSDK_CONFIG").filter(|s| !s.is_empty()) {
        return Some(PathBuf::from(dir).join("application_default_credentials.json"));
    }
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA")
            .map(|p| PathBuf::from(p).join("gcloud/application_default_credentials.json"))
    }
    #[cfg(not(windows))]
    {
        std::env::home_dir().map(|p| p.join(".config/gcloud/application_default_credentials.json"))
    }
}

pub(super) async fn discover() -> Result<ResolvedAdc, String> {
    if let Some(path) = selected_file(
        std::env::var_os("GOOGLE_APPLICATION_CREDENTIALS"),
        default_file(),
    )? {
        return from_file(&path);
    }
    // Only absent ADC files allow the documented metadata/gcloud discovery path.
    // Do not call provider(): it would inspect a different default file again,
    // ignoring CLOUDSDK_CONFIG and the selection already made above.
    let provider: Arc<dyn TokenProvider> = match gcp_auth::MetadataServiceAccount::new().await {
        Ok(provider) => Arc::new(provider),
        Err(_) => Arc::new(
            gcp_auth::GCloudAuthorizedUser::new()
                .await
                .map_err(|_| "No supported ADC credentials were found")?,
        ),
    };
    Ok(ResolvedAdc {
        provider,
        quota_project: None,
    })
}

pub(super) fn from_file(path: &Path) -> Result<ResolvedAdc, String> {
    let file =
        std::fs::File::open(path).map_err(|_| "Cannot open the selected ADC credential file")?;
    use std::io::Read;
    let mut bytes = Vec::new();
    file.take(1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "Cannot read the selected ADC credential file")?;
    if bytes.len() > 1024 * 1024 {
        return Err("ADC credential file exceeds 1 MiB".into());
    }
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| "Invalid JSON in the selected ADC credential file")?;
    let quota_project = match value.get("quota_project_id") {
        None => None,
        Some(serde_json::Value::String(s)) => Some(s.clone()),
        _ => return Err("ADC quota_project_id must be a string".into()),
    };
    let provider: Arc<dyn TokenProvider> = match value["type"].as_str() {
        Some("service_account") => Arc::new(gcp_auth::CustomServiceAccount::from_json(std::str::from_utf8(&bytes).map_err(|_| "Invalid ADC encoding")?).map_err(|_| "Invalid service-account ADC credentials")?),
        Some("authorized_user") => {
            let credentials: UserCredentials = serde_json::from_value(value).map_err(|_| "Incomplete authorized-user ADC credentials")?;
            if [&credentials.client_id,&credentials.client_secret,&credentials.refresh_token].iter().any(|s| s.trim().is_empty()) { return Err("Incomplete authorized-user ADC credentials".into()); }
            Arc::new(UserProvider { credentials, http: token_client()?,
                cached: Default::default(), endpoint: "https://oauth2.googleapis.com/token".into() })
        }
        _ => return Err("Unsupported ADC credential type: use service_account or authorized_user. Impersonated and federated ADC files are not supported; refusing to fall back to another identity.".into()),
    };
    Ok(ResolvedAdc {
        provider,
        quota_project,
    })
}
fn token_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|_| "Cannot initialize ADC token client".into())
}
#[derive(Deserialize)]
struct UserCredentials {
    client_id: String,
    client_secret: String,
    refresh_token: String,
}
struct UserProvider {
    credentials: UserCredentials,
    http: reqwest::Client,
    cached: tokio::sync::Mutex<Option<Arc<Token>>>,
    endpoint: String,
}
#[async_trait]
impl TokenProvider for UserProvider {
    async fn token(&self, _scopes: &[&str]) -> Result<Arc<Token>, gcp_auth::Error> {
        // Refresh tokens retain their granted scopes; configuration cannot expand them.
        let mut cached = self.cached.lock().await;
        if let Some(token) = cached.as_ref().filter(|t| !t.has_expired()) {
            return Ok(token.clone());
        }
        let mut response = self
            .http
            .post(&self.endpoint)
            .form(&[
                ("grant_type", "refresh_token"),
                ("client_id", &self.credentials.client_id),
                ("client_secret", &self.credentials.client_secret),
                ("refresh_token", &self.credentials.refresh_token),
            ])
            .send()
            .await
            .map_err(|_| gcp_auth::Error::Str("ADC token refresh failed"))?;
        if !response.status().is_success() {
            return Err(gcp_auth::Error::Str(
                "ADC token refresh rejected; re-authorize the selected credentials",
            ));
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| gcp_auth::Error::Str("ADC token response unreadable"))?
        {
            if bytes.len() + chunk.len() > 65536 {
                return Err(gcp_auth::Error::Str("ADC token response too large"));
            }
            bytes.extend_from_slice(&chunk);
        }
        let value: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|_| gcp_auth::Error::Str("Invalid ADC token response"))?;
        if value["access_token"].as_str().is_none_or(|s| s.is_empty())
            || value["expires_in"]
                .as_u64()
                .is_none_or(|v| !(1..=86400).contains(&v))
        {
            return Err(gcp_auth::Error::Str("Invalid ADC token response"));
        }
        let token = Arc::new(
            serde_json::from_value::<Token>(value)
                .map_err(|_| gcp_auth::Error::Str("Invalid ADC token response"))?,
        );
        *cached = Some(token.clone());
        Ok(token)
    }
    async fn project_id(&self) -> Result<Arc<str>, gcp_auth::Error> {
        Err(gcp_auth::Error::Str(
            "No resource project ID for authorized-user credentials",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn fixture(value: serde_json::Value) -> tempfile::NamedTempFile {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), value.to_string()).unwrap();
        file
    }
    #[test]
    fn explicit_and_existing_default_files_do_not_fall_through() {
        let missing = PathBuf::from("/nonexistent/mcp-adc-fixture");
        assert_eq!(
            selected_file(Some(missing.clone().into_os_string()), None).unwrap(),
            Some(missing.clone())
        );
        assert!(selected_file(Some("".into()), None).is_err());
        assert!(selected_file(None, Some(missing)).unwrap().is_none());
        let file = fixture(
            json!({"type":"authorized_user","client_id":"id","client_secret":"secret","refresh_token":"refresh","quota_project_id":"billing"}),
        );
        assert_eq!(
            selected_file(None, Some(file.path().into())).unwrap(),
            Some(file.path().into())
        );
        let resolved = from_file(file.path()).unwrap();
        assert_eq!(resolved.quota_project.as_deref(), Some("billing"));
        for kind in [
            "impersonated_service_account",
            "external_account",
            "secret-do-not-echo",
        ] {
            let file = fixture(json!({"type":kind,"secret":"secret-do-not-echo"}));
            let error = from_file(file.path()).err().unwrap();
            assert!(error.contains("Unsupported ADC"));
            assert!(!error.contains("secret-do-not-echo"));
        }
        let file = fixture(json!({"type":"authorized_user","client_secret":"secret-do-not-echo"}));
        assert!(!from_file(file.path())
            .err()
            .unwrap()
            .contains("secret-do-not-echo"));
    }
    #[tokio::test]
    async fn user_refresh_is_cached_and_does_not_echo_rejected_secrets() {
        use wiremock::matchers::{body_string_contains, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"access_token":"fresh","expires_in":3600})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let provider = UserProvider {
            credentials: UserCredentials {
                client_id: "id".into(),
                client_secret: "secret".into(),
                refresh_token: "refresh".into(),
            },
            http: token_client().unwrap(),
            cached: Default::default(),
            endpoint: format!("{}/token", server.uri()),
        };
        let (first, second) = tokio::join!(provider.token(&["read"]), provider.token(&["read"]));
        assert_eq!(first.unwrap().as_str(), "fresh");
        assert_eq!(second.unwrap().as_str(), "fresh");
        let denied = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(400).set_body_string("secret-do-not-echo"))
            .mount(&denied)
            .await;
        let provider = UserProvider {
            endpoint: denied.uri(),
            cached: Default::default(),
            ..provider
        };
        let error = provider.token(&["read"]).await.unwrap_err().to_string();
        assert!(error.contains("rejected"));
        assert!(!error.contains("secret-do-not-echo"));
    }
}

#[cfg(test)]
mod redirect_tests {
    use super::*;
    #[tokio::test]
    async fn refresh_credentials_are_not_forwarded_to_redirects() {
        use wiremock::matchers::method;
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let source = MockServer::start().await;
        let destination = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(307).insert_header("location", destination.uri()))
            .mount(&source)
            .await;
        let provider = UserProvider {
            credentials: UserCredentials {
                client_id: "id".into(),
                client_secret: "secret".into(),
                refresh_token: "refresh".into(),
            },
            http: token_client().unwrap(),
            cached: Default::default(),
            endpoint: source.uri(),
        };
        assert!(provider.token(&["read"]).await.is_err());
        assert!(destination.received_requests().await.unwrap().is_empty());
    }
}

#[cfg(all(test, unix))]
mod dangling_file_tests {
    use super::*;
    #[test]
    fn dangling_default_symlink_is_selected_and_fails_closed() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("adc.json");
        std::os::unix::fs::symlink(dir.path().join("missing"), &path).unwrap();
        assert_eq!(
            selected_file(None, Some(path.clone())).unwrap(),
            Some(path.clone())
        );
        assert!(from_file(&path).is_err());
    }
}
