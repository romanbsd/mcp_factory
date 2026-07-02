use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::ProxyError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoredTokens {
    pub access_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default = "default_token_type")]
    pub token_type: String,
}

fn default_token_type() -> String {
    "Bearer".to_string()
}

impl StoredTokens {
    pub fn is_valid(&self, skew_secs: i64) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at > Utc::now() + chrono::Duration::seconds(skew_secs),
            None => true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileTokenStore {
    path: PathBuf,
}

impl FileTokenStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn load(&self) -> Result<Option<StoredTokens>, ProxyError> {
        if !self.path.exists() {
            return Ok(None);
        }
        let contents = std::fs::read_to_string(&self.path)
            .map_err(|e| ProxyError::Config(format!("failed to read token store: {e}")))?;
        let tokens: StoredTokens = serde_json::from_str(&contents)
            .map_err(|e| ProxyError::Config(format!("failed to parse token store: {e}")))?;
        Ok(Some(tokens))
    }

    pub fn save(&self, tokens: &StoredTokens) -> Result<(), ProxyError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ProxyError::Config(format!("failed to create token dir: {e}")))?;
        }
        let contents = serde_json::to_string_pretty(tokens)
            .map_err(|e| ProxyError::Config(format!("failed to serialize tokens: {e}")))?;
        // Restrict permissions BEFORE the secret hits disk, not after — otherwise
        // the tokens are briefly world/group-readable at the default umask.
        #[cfg(unix)]
        {
            use std::io::Write;
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&self.path)
                .map_err(|e| ProxyError::Config(format!("failed to write token store: {e}")))?;
            // `.mode()` only applies on create; enforce 0600 on a pre-existing file
            // (now truncated/empty) before writing the new secret into it.
            restrict_permissions(&self.path)?;
            file.write_all(contents.as_bytes())
                .map_err(|e| ProxyError::Config(format!("failed to write token store: {e}")))?;
        }
        #[cfg(not(unix))]
        {
            std::fs::write(&self.path, contents)
                .map_err(|e| ProxyError::Config(format!("failed to write token store: {e}")))?;
        }
        Ok(())
    }

    pub fn delete(&self) -> Result<(), ProxyError> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| ProxyError::Config(format!("failed to delete token store: {e}")))?;
        }
        Ok(())
    }
}

fn restrict_permissions(path: &Path) -> Result<(), ProxyError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o600);
        std::fs::set_permissions(path, perms).map_err(|e| {
            ProxyError::Config(format!("failed to set token file permissions: {e}"))
        })?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn roundtrip_tokens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        let store = FileTokenStore::new(path);
        let tokens = StoredTokens {
            access_token: "access".to_string(),
            refresh_token: Some("refresh".to_string()),
            expires_at: Some(Utc::now() + Duration::hours(1)),
            token_type: "Bearer".to_string(),
        };
        store.save(&tokens).unwrap();
        let loaded = store.load().unwrap().unwrap();
        assert_eq!(loaded.access_token, "access");
        assert_eq!(loaded.refresh_token, Some("refresh".to_string()));
    }

    #[test]
    fn expiry_with_skew() {
        let valid = StoredTokens {
            access_token: "a".to_string(),
            refresh_token: None,
            expires_at: Some(Utc::now() + Duration::minutes(5)),
            token_type: "Bearer".to_string(),
        };
        assert!(valid.is_valid(60));

        let expired = StoredTokens {
            access_token: "a".to_string(),
            refresh_token: None,
            expires_at: Some(Utc::now() + Duration::seconds(30)),
            token_type: "Bearer".to_string(),
        };
        assert!(!expired.is_valid(60));
    }

    #[cfg(unix)]
    #[test]
    fn sets_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("tokens.json");
        let store = FileTokenStore::new(path.clone());
        store
            .save(&StoredTokens {
                access_token: "x".to_string(),
                refresh_token: None,
                expires_at: None,
                token_type: "Bearer".to_string(),
            })
            .unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }
}
