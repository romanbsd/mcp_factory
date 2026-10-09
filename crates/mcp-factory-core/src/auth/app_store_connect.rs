//! App Store Connect API keys: short-lived ES256 JWTs signed with the team's
//! `.p8` private key, renewed transparently before they expire.

use std::env;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use reqwest::RequestBuilder;
use ring::rand::SystemRandom;
use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING};
use serde_json::json;

use crate::auth::AuthProvider;
use crate::error::ProxyError;

/// Apple rejects tokens that live longer than 20 minutes.
const TOKEN_LIFETIME: Duration = Duration::from_secs(15 * 60);
/// Mint a fresh token when the cached one has less than this left.
const RENEW_MARGIN: Duration = Duration::from_secs(60);

pub struct AppStoreConnectAuthProvider {
    key_id: String,
    issuer_id: String,
    key: EcdsaKeyPair,
    rng: SystemRandom,
    cached: Mutex<Option<(String, u64)>>,
}

impl AppStoreConnectAuthProvider {
    /// Read the key ID, issuer ID and `.p8` key path from the named
    /// environment variables. Errors name the variables, never their values.
    pub fn from_env(
        key_id_env: &str,
        issuer_id_env: &str,
        private_key_path_env: &str,
    ) -> Result<Self, ProxyError> {
        let key_id = required_env(key_id_env)?;
        let issuer_id = required_env(issuer_id_env)?;
        let path = required_env(private_key_path_env)?;
        let pem = std::fs::read_to_string(&path).map_err(|error| {
            ProxyError::Config(format!(
                "failed to read App Store Connect private key from {private_key_path_env}: {error}"
            ))
        })?;
        Self::new(key_id, issuer_id, &pem).map_err(|error| {
            ProxyError::Config(format!(
                "invalid App Store Connect private key in {private_key_path_env}: {error}"
            ))
        })
    }

    /// Build from a PKCS#8 PEM (`-----BEGIN PRIVATE KEY-----`) P-256 key.
    pub fn new(key_id: String, issuer_id: String, pem: &str) -> Result<Self, String> {
        let body: String = pem
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with("-----"))
            .collect();
        let der = STANDARD
            .decode(body)
            .map_err(|_| "not a base64 PEM document".to_string())?;
        let rng = SystemRandom::new();
        let key = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &der, &rng)
            .map_err(|_| "expected a PKCS#8 P-256 (ES256) key".to_string())?;
        Ok(Self {
            key_id,
            issuer_id,
            key,
            rng,
            cached: Mutex::new(None),
        })
    }

    /// A token valid at `now` (Unix seconds), reusing the cached one until it
    /// is within `RENEW_MARGIN` of expiry.
    pub(crate) fn token_at(&self, now: u64) -> Result<String, ProxyError> {
        let mut cached = self
            .cached
            .lock()
            .map_err(|_| ProxyError::Other("App Store Connect token cache poisoned".to_string()))?;
        if let Some((token, expires_at)) = cached.as_ref() {
            if now + RENEW_MARGIN.as_secs() < *expires_at {
                return Ok(token.clone());
            }
        }
        let expires_at = now + TOKEN_LIFETIME.as_secs();
        let token = self.sign(now, expires_at)?;
        *cached = Some((token.clone(), expires_at));
        Ok(token)
    }

    fn sign(&self, issued_at: u64, expires_at: u64) -> Result<String, ProxyError> {
        let header = json!({"alg": "ES256", "kid": self.key_id, "typ": "JWT"});
        let claims = json!({
            "iss": self.issuer_id,
            "iat": issued_at,
            "exp": expires_at,
            "aud": "appstoreconnect-v1",
        });
        let signing_input = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(header.to_string()),
            URL_SAFE_NO_PAD.encode(claims.to_string())
        );
        let signature = self
            .key
            .sign(&self.rng, signing_input.as_bytes())
            .map_err(|_| ProxyError::Other("failed to sign App Store Connect token".to_string()))?;
        Ok(format!(
            "{signing_input}.{}",
            URL_SAFE_NO_PAD.encode(signature.as_ref())
        ))
    }
}

fn required_env(name: &str) -> Result<String, ProxyError> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            ProxyError::Config(format!(
                "App Store Connect auth requires environment variable {name}"
            ))
        })
}

fn unix_now() -> Result<u64, ProxyError> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs())
        .map_err(|_| ProxyError::Other("system clock is before 1970".to_string()))
}

#[async_trait]
impl AuthProvider for AppStoreConnectAuthProvider {
    async fn apply_request_auth(
        &self,
        request: RequestBuilder,
    ) -> Result<RequestBuilder, ProxyError> {
        Ok(request.bearer_auth(self.token_at(unix_now()?)?))
    }

    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature::{KeyPair, UnparsedPublicKey, ECDSA_P256_SHA256_FIXED};
    use serde_json::Value;

    fn test_key() -> (String, Vec<u8>) {
        let rng = SystemRandom::new();
        let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
        let public =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref(), &rng)
                .unwrap()
                .public_key()
                .as_ref()
                .to_vec();
        let body = STANDARD.encode(pkcs8.as_ref());
        let lines: Vec<&str> = body
            .as_bytes()
            .chunks(64)
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect();
        let pem = format!(
            "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
            lines.join("\n")
        );
        (pem, public)
    }

    fn decode(part: &str) -> Value {
        serde_json::from_slice(&URL_SAFE_NO_PAD.decode(part).unwrap()).unwrap()
    }

    #[test]
    fn signs_verifiable_es256_token_with_apple_claims() {
        let (pem, public) = test_key();
        let provider =
            AppStoreConnectAuthProvider::new("KEY123".into(), "issuer-uuid".into(), &pem).unwrap();

        let token = provider.token_at(1_000_000).unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(
            decode(parts[0]),
            json!({"alg": "ES256", "kid": "KEY123", "typ": "JWT"})
        );
        assert_eq!(
            decode(parts[1]),
            json!({
                "iss": "issuer-uuid",
                "iat": 1_000_000,
                "exp": 1_000_000 + 15 * 60,
                "aud": "appstoreconnect-v1"
            })
        );
        let signature = URL_SAFE_NO_PAD.decode(parts[2]).unwrap();
        UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, public)
            .verify(format!("{}.{}", parts[0], parts[1]).as_bytes(), &signature)
            .expect("signature verifies with the key's public half");
    }

    #[test]
    fn reuses_token_until_renewal_margin() {
        let (pem, _) = test_key();
        let provider = AppStoreConnectAuthProvider::new("k".into(), "i".into(), &pem).unwrap();
        let first = provider.token_at(1_000).unwrap();
        // Still more than RENEW_MARGIN before expiry: cached.
        assert_eq!(provider.token_at(1_000 + 13 * 60).unwrap(), first);
        // Inside the margin: renewed with a later expiry.
        let renewed = provider.token_at(1_000 + 14 * 60 + 30).unwrap();
        assert_ne!(renewed, first);
        let claims = decode(renewed.split('.').nth(1).unwrap());
        assert_eq!(claims["exp"], 1_000 + 14 * 60 + 30 + 15 * 60);
    }

    #[test]
    fn rejects_non_p256_material_without_echoing_it() {
        let error = AppStoreConnectAuthProvider::new(
            "k".into(),
            "i".into(),
            "-----BEGIN PRIVATE KEY-----\nc2VjcmV0LW1hdGVyaWFs\n-----END PRIVATE KEY-----",
        )
        .err()
        .unwrap();
        assert!(!error.contains("c2VjcmV0"), "{error}");
        assert!(error.contains("PKCS#8 P-256"), "{error}");
    }

    #[test]
    fn missing_env_names_the_variable() {
        // Unique names that nothing sets, so no process-env mutation is needed.
        let error = AppStoreConnectAuthProvider::from_env(
            "ASC_TEST_UNSET_KEY_ID",
            "ASC_TEST_UNSET_ISSUER",
            "ASC_TEST_UNSET_KEY",
        )
        .err()
        .unwrap();
        assert!(error.to_string().contains("ASC_TEST_UNSET_KEY_ID"));
    }
}
