//! Read-only Cloud Storage access for Play report exports. Only the three
//! operations the install-report tools need exist here: list, object metadata,
//! and a generation-pinned download. Nothing writes, signs URLs, or changes ACLs.

use std::sync::Arc;
use std::time::Duration;

use gcp_auth::{CustomServiceAccount, TokenProvider};
use mcp_factory_core::async_trait;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use serde_json::Value;

pub const STORAGE_SCOPE: &str = "https://www.googleapis.com/auth/devstorage.read_only";
const STORAGE_BASE: &str = "https://storage.googleapis.com/storage/v1";
/// Object names go in one path segment, so `/` must be escaped; RFC 3986
/// unreserved characters stay literal.
const OBJECT_SEGMENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectMeta {
    pub name: String,
    pub generation: String,
    pub size: u64,
    pub updated: Option<String>,
    pub md5_hash: Option<String>,
    pub crc32c: Option<String>,
    /// `gzip` when Google stores the export compressed. `size`, `md5Hash`,
    /// and `crc32c` then describe the stored bytes, while downloads arrive
    /// decompressed (the CSV itself).
    pub content_encoding: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ListPage {
    pub items: Vec<ObjectMeta>,
    pub next_page_token: Option<String>,
}

/// Lexicographic `[startOffset, endOffset)` bounds on object names, so a month
/// range is filtered by Storage instead of page by page.
pub type Offsets<'a> = (Option<&'a str>, Option<&'a str>);

#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    /// Token could not be obtained, or Storage answered 401.
    Auth(String),
    PermissionDenied(String),
    BucketNotFound(String),
    /// Object (or the pinned generation of it) does not exist.
    ObjectNotFound(String),
    TooLarge {
        size: u64,
        limit: u64,
    },
    Transient(String),
    Other(String),
}

impl StorageError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Auth(_) => "auth_failed",
            Self::PermissionDenied(_) => "permission_denied",
            Self::BucketNotFound(_) => "bucket_not_found",
            Self::ObjectNotFound(_) => "object_not_found",
            Self::TooLarge { .. } => "too_large",
            Self::Transient(_) => "storage_unavailable",
            Self::Other(_) => "storage_error",
        }
    }

    pub fn to_json(&self) -> Value {
        serde_json::json!({"code": self.code(), "message": self.message()})
    }

    pub fn message(&self) -> String {
        match self {
            Self::TooLarge { size, limit } => {
                format!("object is {size} bytes; the limit is {limit} bytes")
            }
            Self::Auth(m)
            | Self::PermissionDenied(m)
            | Self::BucketNotFound(m)
            | Self::ObjectNotFound(m)
            | Self::Transient(m)
            | Self::Other(m) => m.clone(),
        }
    }
}

#[async_trait]
pub trait Storage: Send + Sync {
    /// Identity the requests run as (service-account email); part of cache keys
    /// so a credential swap never reuses another principal's verified mapping.
    fn principal(&self) -> &str;
    async fn list(
        &self,
        bucket: &str,
        prefix: &str,
        offsets: Offsets<'_>,
        page_token: Option<&str>,
        max_results: u32,
    ) -> Result<ListPage, StorageError>;
    async fn metadata(&self, bucket: &str, object: &str) -> Result<ObjectMeta, StorageError>;
    async fn download(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
        max_bytes: u64,
    ) -> Result<Vec<u8>, StorageError>;
}

pub struct GcsStorage {
    http: reqwest::Client,
    tokens: Option<Arc<dyn TokenProvider>>,
    base: String,
    principal: String,
}

impl GcsStorage {
    /// Builds a client from the same service-account key file the Publisher
    /// tools use. The Storage token is requested separately with only the
    /// read-only Storage scope; Publisher tokens are untouched.
    pub fn from_key_file(path: &str) -> Result<Self, StorageError> {
        let json = std::fs::read_to_string(path)
            .map_err(|error| StorageError::Auth(format!("cannot read credential file: {error}")))?;
        let principal = serde_json::from_str::<Value>(&json)
            .ok()
            .and_then(|key| key["client_email"].as_str().map(str::to_string))
            .unwrap_or_else(|| "unknown-principal".to_string());
        let account = CustomServiceAccount::from_json(&json).map_err(|error| {
            StorageError::Auth(format!("invalid service-account credentials: {error}"))
        })?;
        Ok(Self {
            http: http_client(),
            tokens: Some(Arc::new(account)),
            base: STORAGE_BASE.to_string(),
            principal,
        })
    }

    #[cfg(test)]
    pub fn unauthenticated(base: &str) -> Self {
        Self {
            http: http_client(),
            tokens: None,
            base: base.to_string(),
            principal: "test-principal".to_string(),
        }
    }

    async fn get(
        &self,
        url: String,
        query: &[(&str, String)],
    ) -> Result<reqwest::Response, StorageError> {
        let url = reqwest::Url::parse_with_params(&url, query)
            .map_err(|error| StorageError::Other(format!("invalid Storage URL: {error}")))?;
        let mut request = self.http.get(url);
        if let Some(tokens) = &self.tokens {
            let token = tokens.token(&[STORAGE_SCOPE]).await.map_err(|error| {
                StorageError::Auth(format!("failed to obtain Storage access token: {error}"))
            })?;
            request = request.bearer_auth(token.as_str());
        }
        let response = request
            .send()
            .await
            .map_err(|error| StorageError::Transient(format!("request failed: {error}")))?;
        let status = response.status().as_u16();
        if (200..300).contains(&status) {
            return Ok(response);
        }
        let message = error_message(status, &response.text().await.unwrap_or_default());
        Err(match status {
            401 => StorageError::Auth(message),
            403 => StorageError::PermissionDenied(message),
            404 if message.to_ascii_lowercase().contains("bucket") => {
                StorageError::BucketNotFound(message)
            }
            404 => StorageError::ObjectNotFound(message),
            408 | 429 | 500..=599 => StorageError::Transient(message),
            _ => StorageError::Other(message),
        })
    }

    async fn get_json(&self, url: String, query: &[(&str, String)]) -> Result<Value, StorageError> {
        self.get(url, query)
            .await?
            .json()
            .await
            .map_err(|error| StorageError::Other(format!("invalid Storage JSON: {error}")))
    }

    fn object_url(&self, bucket: &str, object: &str) -> String {
        format!(
            "{}/b/{bucket}/o/{}",
            self.base,
            utf8_percent_encode(object, OBJECT_SEGMENT)
        )
    }
}

#[async_trait]
impl Storage for GcsStorage {
    fn principal(&self) -> &str {
        &self.principal
    }

    async fn list(
        &self,
        bucket: &str,
        prefix: &str,
        offsets: Offsets<'_>,
        page_token: Option<&str>,
        max_results: u32,
    ) -> Result<ListPage, StorageError> {
        let mut query = vec![
            ("prefix", prefix.to_string()),
            ("maxResults", max_results.to_string()),
        ];
        if let Some(token) = page_token {
            query.push(("pageToken", token.to_string()));
        }
        if let Some(start) = offsets.0 {
            query.push(("startOffset", start.to_string()));
        }
        if let Some(end) = offsets.1 {
            query.push(("endOffset", end.to_string()));
        }
        let body = self
            .get_json(format!("{}/b/{bucket}/o", self.base), &query)
            .await?;
        Ok(ListPage {
            items: body["items"]
                .as_array()
                .map(|items| items.iter().filter_map(object_meta).collect())
                .unwrap_or_default(),
            next_page_token: body["nextPageToken"].as_str().map(str::to_string),
        })
    }

    async fn metadata(&self, bucket: &str, object: &str) -> Result<ObjectMeta, StorageError> {
        let body = self.get_json(self.object_url(bucket, object), &[]).await?;
        object_meta(&body).ok_or_else(|| StorageError::Other("metadata missing fields".into()))
    }

    async fn download(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
        max_bytes: u64,
    ) -> Result<Vec<u8>, StorageError> {
        let mut response = self
            .get(
                self.object_url(bucket, object),
                &[("alt", "media".into()), ("generation", generation.into())],
            )
            .await?;
        if let Some(size) = response.content_length().filter(|size| *size > max_bytes) {
            return Err(StorageError::TooLarge {
                size,
                limit: max_bytes,
            });
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| StorageError::Transient(format!("download interrupted: {error}")))?
        {
            bytes.extend_from_slice(&chunk);
            if bytes.len() as u64 > max_bytes {
                return Err(StorageError::TooLarge {
                    size: bytes.len() as u64,
                    limit: max_bytes,
                });
            }
        }
        Ok(bytes)
    }
}

fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap_or_default()
}

fn error_message(status: u16, body: &str) -> String {
    let detail = serde_json::from_str::<Value>(body)
        .ok()
        .and_then(|value| value["error"]["message"].as_str().map(str::to_string))
        .unwrap_or_else(|| body.chars().take(300).collect());
    format!("HTTP {status}: {detail}")
}

fn object_meta(value: &Value) -> Option<ObjectMeta> {
    let text = |key: &str| value[key].as_str().map(str::to_string);
    Some(ObjectMeta {
        name: text("name")?,
        generation: text("generation")?,
        size: text("size")?.parse().ok()?,
        updated: text("updated"),
        md5_hash: text("md5Hash"),
        crc32c: text("crc32c"),
        content_encoding: text("contentEncoding"),
    })
}
