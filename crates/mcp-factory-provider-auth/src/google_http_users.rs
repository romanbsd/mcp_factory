//! Provisioned MCP credentials are separate from each user's upstream Google ADC.
use crate::GoogleAdcAuthProvider;
use async_trait::async_trait;
use mcp_factory_core::{
    auth::{AuthProvider, HttpIdentity},
    ProxyError,
};
use serde::Deserialize;
use std::{path::Path, sync::Arc};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct User {
    principal: String,
    bearer_token: String,
    credentials_file: std::path::PathBuf,
    quota_project: Option<String>,
}

pub struct GoogleHttpUsers {
    key: ring::hmac::Key,
    users: Vec<(ring::hmac::Tag, HttpIdentity)>,
}
impl GoogleHttpUsers {
    pub fn from_file(path: &Path, scopes: &[String]) -> Result<Self, ProxyError> {
        use std::io::Read;
        crate::validate_google_scopes(scopes)?;
        let invalid = || {
            ProxyError::Config("invalid HTTP users file: require 1-100 unique principals, distinct MCP bearer tokens of at least 32 characters, and absolute Google ADC credential paths".into())
        };
        let file = std::fs::File::open(path).map_err(|_| invalid())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if file.metadata().map_err(|_| invalid())?.permissions().mode() & 0o077 != 0 {
                return Err(ProxyError::Config(
                    "HTTP users file must have owner-only permissions (chmod 600)".into(),
                ));
            }
        }
        let mut bytes = Vec::new();
        file.take(1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| invalid())?;
        if bytes.len() > 1024 * 1024 {
            return Err(invalid());
        }
        let users: Vec<User> = serde_json::from_slice(&bytes).map_err(|_| invalid())?;
        if users.is_empty() || users.len() > 100 {
            return Err(invalid());
        }
        let mut tokens = std::collections::HashSet::new();
        let mut principals = std::collections::HashSet::new();
        let key =
            ring::hmac::Key::generate(ring::hmac::HMAC_SHA256, &ring::rand::SystemRandom::new())
                .map_err(|_| invalid())?;
        let mut result = Vec::with_capacity(users.len());
        for user in users {
            if user.principal.is_empty()
                || user.principal.len() > 256
                || user.principal.trim() != user.principal
                || !principals.insert(user.principal.clone())
                || user.bearer_token.len() < 32
                || user.bearer_token.len() > 8192
                || user.bearer_token.bytes().any(|b| !b.is_ascii_graphic())
                || !tokens.insert(user.bearer_token.clone())
                || !user.credentials_file.is_absolute()
            {
                return Err(invalid());
            }
            let provider = GoogleAdcAuthProvider::from_selected_file(
                &user.credentials_file,
                scopes,
                user.quota_project,
            )?;
            result.push((
                ring::hmac::sign(&key, user.bearer_token.as_bytes()),
                HttpIdentity {
                    principal: user.principal,
                    auth: Arc::new(provider),
                },
            ));
        }
        Ok(Self { key, users: result })
    }
}
#[async_trait]
impl AuthProvider for GoogleHttpUsers {
    fn requires_http_identity(&self) -> bool {
        true
    }
    async fn authenticate_http(&self, bearer: &str) -> Result<HttpIdentity, ProxyError> {
        // Scan every fixed-size digest with a constant-time equality check.
        let mut found = None;
        for (tag, identity) in &self.users {
            if ring::hmac::verify(&self.key, bearer.as_bytes(), tag.as_ref()).is_ok() {
                found = Some(identity.clone());
            }
        }
        found.ok_or_else(|| ProxyError::Validation("invalid MCP credential".into()))
    }
    async fn apply_request_auth(
        &self,
        _: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ProxyError> {
        Err(ProxyError::Validation(
            "authenticated HTTP identity required; host ADC fallback is disabled".into(),
        ))
    }
    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    fn credentials(dir: &Path, name: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        std::fs::write(&path,json!({"type":"authorized_user","client_id":name,"client_secret":"test-secret","refresh_token":"test-refresh","quota_project_id":format!("quota-{name}")}).to_string()).unwrap();
        path
    }
    fn manifest(value: serde_json::Value) -> tempfile::NamedTempFile {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "{value}").unwrap();
        file
    }
    #[tokio::test]
    async fn users_are_distinct_and_invalid_credentials_never_fall_back() {
        let dir = tempfile::tempdir().unwrap();
        let a = credentials(dir.path(), "a");
        let b = credentials(dir.path(), "b");
        let file = manifest(json!([
            {"principal":"a","bearer_token":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","credentials_file":a},
            {"principal":"b","bearer_token":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","credentials_file":b}
        ]));
        let users = GoogleHttpUsers::from_file(file.path(), &["read".into()]).unwrap();
        let a = users
            .authenticate_http("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
            .await
            .unwrap();
        let b = users
            .authenticate_http("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
            .await
            .unwrap();
        assert_eq!(a.principal, "a");
        assert_eq!(b.principal, "b");
        assert!(!Arc::ptr_eq(&a.auth, &b.auth));
        for token in [
            "",
            "invalid",
            "test-refresh",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab",
        ] {
            assert!(users.authenticate_http(token).await.is_err());
        }
        assert!(users
            .apply_request_auth(reqwest::Client::new().get("https://example.com"))
            .await
            .is_err());
    }
    #[test]
    fn invalid_manifest_rejected_without_echoing_secrets() {
        let dir = tempfile::tempdir().unwrap();
        let adc = credentials(dir.path(), "valid");
        let user = json!({"principal":"a","bearer_token":"very-secret-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","credentials_file":adc});
        let mut wrong = user.clone();
        wrong["credentials_file"] = json!(dir.path().join("missing"));
        for value in [
            json!([]),
            json!([user.clone(), user.clone()]),
            json!([wrong]),
            json!([{"principal":"a","bearer_token":"short","credentials_file":adc}]),
            json!([{"unexpected-secret":"very-secret"}]),
        ] {
            let file = manifest(value);
            let error = GoogleHttpUsers::from_file(file.path(), &["read".into()])
                .err()
                .unwrap()
                .to_string();
            assert!(!error.contains("very-secret"));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let file = manifest(json!([user]));
            std::fs::set_permissions(file.path(), std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(GoogleHttpUsers::from_file(file.path(), &["read".into()]).is_err());
        }
    }
}
