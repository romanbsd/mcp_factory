//! Typed local identities and resumable state shared by upload and screenshot workflows.
use super::{failure, proxy_error_result};
use mcp_factory_core::{resolve_media_file, ToolResult};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

pub(super) const MAX_UPLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;

#[derive(Clone, Debug)]
pub(super) struct FileIdentity {
    pub file: String,
    pub file_name: String,
    pub file_size: u64,
    pub checksum: String,
}
pub(super) struct PreparedFile {
    pub identity: FileIdentity,
    pub path: PathBuf,
}
impl PreparedFile {
    /// The caller holds the transfer permit across bounded file inspection.
    pub async fn inspect(
        root: Option<&Path>,
        file: &str,
        name: Option<&str>,
    ) -> Result<Self, ToolResult> {
        let (path, metadata) = resolve_media_file(root, file)
            .await
            .map_err(|error| proxy_error_result("validate", error))?;
        let file_size = metadata.len();
        if file_size > MAX_UPLOAD_BYTES {
            return Err(failure(
                "validate",
                "file exceeds the 2 GiB upload limit",
                serde_json::json!({"fileSize":file_size}),
            ));
        }
        let file_name = name
            .map(str::to_owned)
            .or_else(|| path.file_name().map(|s| s.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "upload".to_owned());
        let checksum = md5_hex(&path).await.map_err(|error| {
            failure(
                "validate",
                "cannot read the file",
                serde_json::json!(error.to_string()),
            )
        })?;
        Ok(Self {
            identity: FileIdentity {
                file: file.to_owned(),
                file_name,
                file_size,
                checksum,
            },
            path,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct UploadRange {
    pub offset: u64,
    /// None preserves incomplete Apple plans in failure checkpoints. Such a
    /// range is never transferable, and a retry must reconcile a valid plan.
    pub length: Option<u64>,
}
impl UploadRange {
    pub fn from_operation(operation: &Value) -> Result<Self, String> {
        let offset = match operation.get("offset") {
            None => 0,
            Some(value) => value
                .as_u64()
                .ok_or_else(|| "upload operation has an invalid offset".to_owned())?,
        };
        Ok(Self {
            offset,
            length: operation["length"].as_u64(),
        })
    }
    pub fn snapshot(operation: &Value) -> Self {
        Self {
            offset: operation["offset"].as_u64().unwrap_or(0),
            length: operation["length"].as_u64(),
        }
    }
    pub fn validate(&self, file_size: u64) -> Result<(u64, u64), String> {
        let length = self
            .length
            .ok_or_else(|| "upload operation has no length".to_owned())?;
        if file_size > MAX_UPLOAD_BYTES || length > MAX_UPLOAD_BYTES {
            return Err("file or upload range exceeds the 2 GiB upload limit".into());
        }
        if !self
            .offset
            .checked_add(length)
            .is_some_and(|end| end <= file_size)
        {
            return Err(format!(
                "upload range {}+{length} is outside the {file_size}-byte file",
                self.offset
            ));
        }
        Ok((self.offset, length))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum AssetState {
    AwaitingUpload,
    UploadComplete,
    Processing,
    Complete,
    Failed,
    Unknown(String),
}
impl AssetState {
    pub fn from_resource(resource: &Value) -> Self {
        match resource["attributes"]["assetDeliveryState"]["state"]
            .as_str()
            .unwrap_or("UNKNOWN")
        {
            "AWAITING_UPLOAD" => Self::AwaitingUpload,
            "UPLOAD_COMPLETE" => Self::UploadComplete,
            "PROCESSING" => Self::Processing,
            "COMPLETE" => Self::Complete,
            "FAILED" => Self::Failed,
            unknown => Self::Unknown(unknown.to_owned()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Self::AwaitingUpload => "AWAITING_UPLOAD",
            Self::UploadComplete => "UPLOAD_COMPLETE",
            Self::Processing => "PROCESSING",
            Self::Complete => "COMPLETE",
            Self::Failed => "FAILED",
            Self::Unknown(s) => s,
        }
    }
    pub fn committed(&self) -> bool {
        matches!(
            self,
            Self::UploadComplete | Self::Processing | Self::Complete
        )
    }
}
impl Serialize for AssetState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

// Optional legacy identity keys may be absent, but a provided null/wrong type
// must fail instead of being treated as an absent constraint.
fn present<'de, D: serde::Deserializer<'de>, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    T::deserialize(deserializer).map(Some)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResumeCheckpoint {
    pub asset_id: String,
    pub file_size: u64,
    pub source_file_checksum: String,
    pub completed_ranges: Vec<usize>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub kind: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub parent_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub file: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub file_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub committed: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub pending_ranges: Option<Vec<usize>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub ranges: Option<Vec<UploadRange>>,
}
impl ResumeCheckpoint {
    pub fn parse(value: &Value) -> Result<Self, ToolResult> {
        serde_json::from_value(value.clone()).map_err(|error| {
            failure(
                "validate",
                "invalid resume checkpoint",
                serde_json::json!(error.to_string()),
            )
        })
    }
    pub fn validate_file(
        &self,
        kind: &str,
        parent: &str,
        identity: &FileIdentity,
    ) -> Result<(), ToolResult> {
        let identities = [
            (&self.kind, kind),
            (&self.parent_id, parent),
            (&self.file, identity.file.as_str()),
            (&self.file_name, identity.file_name.as_str()),
        ];
        if self.asset_id.is_empty()
            || self.file_size != identity.file_size
            || !self
                .source_file_checksum
                .eq_ignore_ascii_case(&identity.checksum)
            || identities
                .iter()
                .any(|(value, expected)| value.as_deref().is_some_and(|actual| actual != *expected))
        {
            return Err(failure(
                "validate",
                "resume identity, file size or local checksum does not match",
                Value::Null,
            ));
        }
        Ok(())
    }
}

pub(super) async fn md5_hex(path: &Path) -> std::io::Result<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = vec![0; 64 * 1024];
    let mut digest = Md5::new();
    loop {
        let read = file.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
