use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::error::ProxyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TransportMode {
    #[default]
    Stdio,
    Http,
    Both,
}

fn default_client_secret_env() -> String {
    "MCP_FACTORY_OAUTH_CLIENT_SECRET".to_string()
}

pub fn default_token_store_path() -> PathBuf {
    if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(dir).join("mcp-factory").join("tokens.json");
    }
    PathBuf::from(".mcp-factory/tokens.json")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthConfig {
    #[default]
    None,
    Bearer {
        #[serde(default = "default_bearer_env")]
        env_var: String,
    },
    ApiKeyHeader {
        header: String,
        #[serde(default = "default_api_key_env")]
        env_var: String,
    },
    ApiKeyQuery {
        param: String,
        #[serde(default = "default_api_key_env")]
        env_var: String,
    },
    #[serde(rename = "oauth2")]
    OAuth2 {
        authorization_endpoint: String,
        token_endpoint: String,
        client_id: String,
        #[serde(default = "default_client_secret_env")]
        client_secret_env: String,
        scopes: Vec<String>,
        #[serde(default)]
        redirect_uri: Option<String>,
        #[serde(default = "default_token_store_path")]
        token_store: PathBuf,
    },
}

fn default_bearer_env() -> String {
    "MCP_FACTORY_BEARER_TOKEN".to_string()
}

fn default_api_key_env() -> String {
    "MCP_FACTORY_API_KEY".to_string()
}

impl AuthConfig {
    pub fn bearer() -> Self {
        Self::Bearer {
            env_var: default_bearer_env(),
        }
    }

    pub fn api_key_header(header: impl Into<String>) -> Self {
        Self::ApiKeyHeader {
            header: header.into(),
            env_var: default_api_key_env(),
        }
    }

    pub fn api_key_query(param: impl Into<String>) -> Self {
        Self::ApiKeyQuery {
            param: param.into(),
            env_var: default_api_key_env(),
        }
    }

    pub fn resolve_secret(&self) -> Option<String> {
        let env_var = match self {
            Self::None | Self::OAuth2 { .. } => return None,
            Self::Bearer { env_var } => env_var,
            Self::ApiKeyHeader { env_var, .. } => env_var,
            Self::ApiKeyQuery { env_var, .. } => env_var,
        };
        env::var(env_var).ok().filter(|v| !v.is_empty())
    }

    pub fn oauth_client_secret(&self) -> Option<String> {
        let Self::OAuth2 {
            client_secret_env, ..
        } = self
        else {
            return None;
        };
        env::var(client_secret_env).ok().filter(|v| !v.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub base_url: String,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default = "default_timeout_secs")]
    pub timeout_secs: u64,
    #[serde(default)]
    pub transport: TransportMode,
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    #[serde(default = "default_http_path")]
    pub http_path: String,
    #[serde(default)]
    pub server_name: String,
    #[serde(default = "default_server_version")]
    pub server_version: String,
}

fn default_timeout_secs() -> u64 {
    30
}

fn default_bind_addr() -> String {
    "127.0.0.1:8080".to_string()
}

fn default_http_path() -> String {
    "/mcp".to_string()
}

fn default_server_version() -> String {
    "0.1.0".to_string()
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            base_url: String::new(),
            auth: AuthConfig::None,
            timeout_secs: default_timeout_secs(),
            transport: TransportMode::Stdio,
            bind_addr: default_bind_addr(),
            http_path: default_http_path(),
            server_name: String::new(),
            server_version: default_server_version(),
        }
    }
}

impl ProxyConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }

    pub fn from_env() -> Result<Self, ProxyError> {
        let mut config = Self::default();

        if let Ok(base_url) = env::var("MCP_FACTORY_BASE_URL") {
            config.base_url = base_url;
        }
        if let Ok(transport) = env::var("MCP_TRANSPORT") {
            config.transport = match transport.to_lowercase().as_str() {
                "stdio" => TransportMode::Stdio,
                "http" => TransportMode::Http,
                "both" => TransportMode::Both,
                other => {
                    return Err(ProxyError::Config(format!(
                        "invalid MCP_TRANSPORT value: {other}"
                    )));
                }
            };
        }
        if let Ok(bind) = env::var("MCP_FACTORY_BIND_ADDR") {
            config.bind_addr = bind;
        }
        if let Ok(path) = env::var("MCP_FACTORY_HTTP_PATH") {
            config.http_path = path;
        }
        if env::var("MCP_FACTORY_BEARER_TOKEN")
            .ok()
            .filter(|v| !v.is_empty())
            .is_some()
        {
            config.auth = AuthConfig::bearer();
        } else if env::var("MCP_FACTORY_API_KEY")
            .ok()
            .filter(|v| !v.is_empty())
            .is_some()
        {
            config.auth = AuthConfig::api_key_header("X-API-Key");
        }

        Ok(config)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ProxyError> {
        let contents = std::fs::read_to_string(path.as_ref())
            .map_err(|e| ProxyError::Config(format!("failed to read config: {e}")))?;
        toml::from_str(&contents)
            .map_err(|e| ProxyError::Config(format!("failed to parse config: {e}")))
    }

    pub fn merge_env(mut self) -> Result<Self, ProxyError> {
        let env_config = Self::from_env()?;
        if !env_config.base_url.is_empty() {
            self.base_url = env_config.base_url;
        }
        if matches!(self.auth, AuthConfig::None) && env_config.auth != AuthConfig::None {
            self.auth = env_config.auth;
        }
        if env::var("MCP_TRANSPORT").is_ok() {
            self.transport = env_config.transport;
        }
        if env::var("MCP_FACTORY_BIND_ADDR").is_ok() {
            self.bind_addr = env_config.bind_addr;
        }
        if env::var("MCP_FACTORY_HTTP_PATH").is_ok() {
            self.http_path = env_config.http_path;
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_resolve_secret_from_env() {
        temp_env::with_var("MCP_FACTORY_BEARER_TOKEN", Some("secret"), || {
            let auth = AuthConfig::bearer();
            assert_eq!(auth.resolve_secret(), Some("secret".to_string()));
        });
    }

    #[test]
    fn transport_mode_from_env() {
        temp_env::with_var("MCP_TRANSPORT", Some("http"), || {
            let config = ProxyConfig::from_env().unwrap();
            assert_eq!(config.transport, TransportMode::Http);
        });
    }

    #[test]
    fn oauth_config_deserializes() {
        let toml_str = r#"
            type = "oauth2"
            authorization_endpoint = "https://auth.example.com/authorize"
            token_endpoint = "https://auth.example.com/token"
            client_id = "cid"
            scopes = ["read"]
        "#;
        let auth: AuthConfig = toml::from_str(toml_str).unwrap();
        assert!(matches!(auth, AuthConfig::OAuth2 { .. }));
    }
}

#[cfg(test)]
mod temp_env {
    use std::env;

    pub fn with_var<F: FnOnce()>(key: &str, value: Option<&str>, f: F) {
        let previous = env::var(key).ok();
        match value {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
        f();
        match previous {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
    }
}
