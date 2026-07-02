use oauth2::basic::BasicClient;
use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, EndpointNotSet, EndpointSet, PkceCodeVerifier,
    RefreshToken, TokenResponse, TokenUrl, AuthUrl, RedirectUrl,
};
use reqwest::Client;
use tokio::sync::Mutex;

use crate::auth::token_store::{FileTokenStore, StoredTokens};
use crate::config::AuthConfig;
use crate::error::ProxyError;

pub const REFRESH_SKEW_SECS: i64 = 60;

type ConfiguredOAuthClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub struct OAuth2Provider {
    client: ConfiguredOAuthClient,
    http: Client,
    token_store: FileTokenStore,
    cache: Mutex<Option<StoredTokens>>,
}

impl OAuth2Provider {
    pub fn new(auth: &AuthConfig, http: Client) -> Result<Self, ProxyError> {
        let AuthConfig::OAuth2 {
            authorization_endpoint,
            token_endpoint,
            client_id,
            token_store,
            ..
        } = auth
        else {
            return Err(ProxyError::Config("expected OAuth2 auth config".to_string()));
        };

        let mut client = BasicClient::new(ClientId::new(client_id.clone()))
            .set_auth_uri(AuthUrl::new(authorization_endpoint.clone()).map_err(oauth_err)?)
            .set_token_uri(TokenUrl::new(token_endpoint.clone()).map_err(oauth_err)?);

        if let Some(secret) = auth.oauth_client_secret() {
            client = client.set_client_secret(ClientSecret::new(secret));
        }

        Ok(Self {
            client,
            http,
            token_store: FileTokenStore::new(token_store.clone()),
            cache: Mutex::new(None),
        })
    }

    pub fn token_store(&self) -> &FileTokenStore {
        &self.token_store
    }

    pub async fn get_tokens(&self) -> Result<StoredTokens, ProxyError> {
        // ponytail: hold the cache guard across load+refresh so concurrent tool
        // calls serialize — the loser sees the fresh token instead of firing a
        // second refresh (which a rotating-refresh-token provider would reject).
        let mut cache = self.cache.lock().await;
        if let Some(tokens) = cache.clone() {
            if tokens.is_valid(REFRESH_SKEW_SECS) {
                return Ok(tokens);
            }
        }

        if let Some(tokens) = self.token_store.load()? {
            if tokens.is_valid(REFRESH_SKEW_SECS) {
                *cache = Some(tokens.clone());
                return Ok(tokens);
            }
            if let Some(refresh) = tokens.refresh_token.clone() {
                let mut refreshed = self.refresh_tokens(&refresh).await?;
                // The token endpoint may omit a refresh token on refresh
                // (RFC 6749 §6); keep the existing one so we don't lose it.
                if refreshed.refresh_token.is_none() {
                    refreshed.refresh_token = Some(refresh);
                }
                self.token_store.save(&refreshed)?;
                *cache = Some(refreshed.clone());
                return Ok(refreshed);
            }
        }

        Err(ProxyError::Config(
            "OAuth not authenticated; run `mcp-factory-auth login` or `--auth-login`".to_string(),
        ))
    }

    pub async fn persist(&self, tokens: &StoredTokens) -> Result<(), ProxyError> {
        self.token_store.save(tokens)?;
        *self.cache.lock().await = Some(tokens.clone());
        Ok(())
    }

    pub async fn bearer_token(&self) -> Result<String, ProxyError> {
        Ok(self.get_tokens().await?.access_token)
    }

    pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<StoredTokens, ProxyError> {
        let token = self
            .client
            .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
            .request_async(&self.http)
            .await
            .map_err(|e| ProxyError::Config(format!("OAuth refresh failed: {e}")))?;

        Ok(token_response_to_stored(token))
    }

    pub async fn exchange_code(
        &self,
        code: &str,
        redirect_uri: &str,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<StoredTokens, ProxyError> {
        let redirect = RedirectUrl::new(redirect_uri.to_string()).map_err(oauth_err)?;
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_redirect_uri(std::borrow::Cow::Owned(redirect))
            .set_pkce_verifier(pkce_verifier)
            .request_async(&self.http)
            .await
            .map_err(|e| ProxyError::Config(format!("OAuth code exchange failed: {e}")))?;

        Ok(token_response_to_stored(token))
    }
}

pub fn token_response_to_stored(
    token: oauth2::StandardTokenResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
) -> StoredTokens {
    use chrono::{Duration, Utc};
    let expires_at = token
        .expires_in()
        .map(|d| Utc::now() + Duration::seconds(d.as_secs() as i64));
    StoredTokens {
        access_token: token.access_token().secret().clone(),
        refresh_token: token.refresh_token().map(|t| t.secret().clone()),
        expires_at,
        token_type: format!("{:?}", token.token_type()),
    }
}

pub fn oauth_err(err: oauth2::url::ParseError) -> ProxyError {
    ProxyError::Config(format!("invalid OAuth URL: {err}"))
}
