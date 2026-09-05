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

impl std::str::FromStr for TransportMode {
    type Err = ProxyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "stdio" => Ok(TransportMode::Stdio),
            "http" => Ok(TransportMode::Http),
            "both" => Ok(TransportMode::Both),
            other => Err(ProxyError::Config(format!(
                "invalid transport value: {other} (expected stdio, http, or both)"
            ))),
        }
    }
}

fn default_client_secret_env() -> String {
    "MCP_FACTORY_OAUTH_CLIENT_SECRET".to_string()
}

fn default_google_credentials_env() -> String {
    "GOOGLE_APPLICATION_CREDENTIALS".to_string()
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
    GoogleServiceAccount {
        #[serde(default = "default_google_credentials_env")]
        credentials_path_env: String,
        scopes: Vec<String>,
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
            Self::None | Self::OAuth2 { .. } | Self::GoogleServiceAccount { .. } => return None,
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
        Self::default().apply_env_overrides()
    }

    fn apply_env_overrides(mut self) -> Result<Self, ProxyError> {
        if let Ok(base_url) = env::var("MCP_FACTORY_BASE_URL") {
            self.base_url = base_url;
        }
        if let Ok(transport) = env::var("MCP_TRANSPORT") {
            self.transport = transport.parse()?;
        }
        if let Ok(bind) = env::var("MCP_FACTORY_BIND_ADDR") {
            self.bind_addr = bind;
        }
        if let Ok(path) = env::var("MCP_FACTORY_HTTP_PATH") {
            self.http_path = path;
        }
        if let Ok(timeout) = env::var("MCP_FACTORY_TIMEOUT") {
            self.timeout_secs = timeout.parse().map_err(|_| {
                ProxyError::Config(format!("invalid MCP_FACTORY_TIMEOUT: {timeout}"))
            })?;
        }
        if matches!(self.auth, AuthConfig::None) {
            if env::var("MCP_FACTORY_BEARER_TOKEN")
                .ok()
                .filter(|v| !v.is_empty())
                .is_some()
            {
                self.auth = AuthConfig::bearer();
            } else if env::var("MCP_FACTORY_API_KEY")
                .ok()
                .filter(|v| !v.is_empty())
                .is_some()
            {
                self.auth = AuthConfig::api_key_header("X-API-Key");
            }
        }
        Ok(self)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ProxyError> {
        let contents = std::fs::read_to_string(path.as_ref())
            .map_err(|e| ProxyError::Config(format!("failed to read config: {e}")))?;
        toml::from_str(&contents)
            .map_err(|e| ProxyError::Config(format!("failed to parse config: {e}")))
    }

    /// Load runtime configuration using an explicit override first, then the
    /// process working directory, then the directory containing the executable.
    /// Generated defaults are used only when none of those files exists.
    pub fn load_runtime(defaults: Self) -> Result<Self, ProxyError> {
        let explicit = env::var_os("MCP_FACTORY_CONFIG")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from);
        let config = match select_runtime_config_path(explicit, env::current_dir, env::current_exe)?
        {
            Some(path) => Self::load(path)?,
            None => defaults,
        };
        config.apply_env_overrides()
    }

    pub fn merge_env(self) -> Result<Self, ProxyError> {
        self.apply_env_overrides()
    }
}

/// Select the runtime config path: explicit override first, then a
/// `config.toml` beside the working directory, then beside the executable.
/// The working directory and executable path are resolved lazily (via the
/// provided closures) so a failure to read either never aborts startup when
/// an earlier candidate already answers.
fn select_runtime_config_path<C, E>(
    explicit: Option<PathBuf>,
    cwd: C,
    executable: E,
) -> Result<Option<PathBuf>, ProxyError>
where
    C: FnOnce() -> std::io::Result<PathBuf>,
    E: FnOnce() -> std::io::Result<PathBuf>,
{
    if explicit.is_some() {
        return Ok(explicit);
    }
    let cwd =
        cwd().map_err(|e| ProxyError::Config(format!("failed to read current directory: {e}")))?;
    let cwd_config = cwd.join("config.toml");
    if cwd_config.is_file() {
        return Ok(Some(cwd_config));
    }
    // A missing executable path only means we skip the beside-binary config
    // candidate and fall back to generation-time defaults; it must not abort.
    let executable_config = executable()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join("config.toml")));
    Ok(executable_config.filter(|path| path.is_file()))
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
    fn timeout_from_env() {
        temp_env::with_var("MCP_FACTORY_TIMEOUT", Some("5"), || {
            let config = ProxyConfig::default().merge_env().unwrap();
            assert_eq!(config.timeout_secs, 5);
        });
        temp_env::with_var("MCP_FACTORY_TIMEOUT", Some("nope"), || {
            assert!(ProxyConfig::default().merge_env().is_err());
        });
    }

    #[test]
    fn runtime_config_path_precedence() {
        let dir = tempfile::tempdir().unwrap();
        let cwd = dir.path().join("cwd");
        let bin = dir.path().join("bin");
        std::fs::create_dir_all(&cwd).unwrap();
        std::fs::create_dir_all(&bin).unwrap();
        let cwd_config = cwd.join("config.toml");
        let executable_config = bin.join("config.toml");
        std::fs::write(&cwd_config, "cwd").unwrap();
        std::fs::write(&executable_config, "bin").unwrap();
        let executable = bin.join("server");

        let cwd_provider = || Ok(cwd.clone());
        let exe_provider = || Ok(executable.clone());
        assert_eq!(
            select_runtime_config_path(None, cwd_provider, exe_provider).unwrap(),
            Some(cwd_config.clone())
        );
        let explicit = dir.path().join("explicit.toml");
        assert_eq!(
            select_runtime_config_path(Some(explicit.clone()), cwd_provider, exe_provider).unwrap(),
            Some(explicit)
        );
        std::fs::remove_file(cwd_config).unwrap();
        assert_eq!(
            select_runtime_config_path(None, cwd_provider, exe_provider).unwrap(),
            Some(executable_config)
        );
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

    #[test]
    fn google_service_account_config_deserializes() {
        let toml_str = r#"
            type = "google_service_account"
            scopes = ["https://www.googleapis.com/auth/androidpublisher"]
        "#;
        let auth: AuthConfig = toml::from_str(toml_str).unwrap();
        assert!(matches!(
            auth,
            AuthConfig::GoogleServiceAccount {
                credentials_path_env,
                scopes,
            } if credentials_path_env == "GOOGLE_APPLICATION_CREDENTIALS"
                && scopes == ["https://www.googleapis.com/auth/androidpublisher"]
        ));
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
