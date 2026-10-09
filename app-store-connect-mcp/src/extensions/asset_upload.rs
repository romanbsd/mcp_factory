//! Apple's asset upload protocol, shared by screenshots, previews, review
//! attachments, export-compliance documents and IAP/subscription review
//! screenshots:
//!
//! 1. reserve: create the asset resource with `fileName`/`fileSize`; Apple
//!    answers with `uploadOperations` (method, URL, offset, length, headers);
//! 2. upload: send each byte range to its presigned URL;
//! 3. commit: PATCH `uploaded: true` with the file's MD5 `sourceFileChecksum`;
//! 4. processing: poll `assetDeliveryState` until COMPLETE or FAILED.

use std::io::SeekFrom;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mcp_factory_core::{
    async_trait, resolve_media_file, CustomToolHandler, CustomToolSpec, ProxyConfig, ProxyError,
    ReadOnlyToolInvoker, ToolHints, ToolResult,
};
use md5::{Digest, Md5};
use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use super::{check_transfer_url, failure, json_of, proxy_error_result, success, transfer_client};

/// One uploadable asset family: its JSON:API type, the generated tools for
/// each protocol step, and the relationship that names its parent.
pub(super) struct Kind {
    pub(super) name: &'static str,
    resource_type: &'static str,
    pub(super) create_tool: &'static str,
    pub(super) update_tool: &'static str,
    pub(super) get_tool: &'static str,
    relationship: &'static str,
    parent_type: &'static str,
    /// Extra create attributes accepted from the caller.
    extra_attributes: &'static [&'static str],
    /// Profile (tools.toml) that provides this kind's generated tools; a kind
    /// is offered only when that profile is enabled.
    pub(super) profile: &'static str,
}

pub(super) const KINDS: &[Kind] = &[
    Kind {
        name: "appScreenshot",
        resource_type: "appScreenshots",
        create_tool: "appScreenshots_createInstance",
        update_tool: "appScreenshots_updateInstance",
        get_tool: "appScreenshots_getInstance",
        relationship: "appScreenshotSet",
        parent_type: "appScreenshotSets",
        extra_attributes: &[],
        profile: "assets",
    },
    Kind {
        name: "appPreview",
        resource_type: "appPreviews",
        create_tool: "appPreviews_createInstance",
        update_tool: "appPreviews_updateInstance",
        get_tool: "appPreviews_getInstance",
        relationship: "appPreviewSet",
        parent_type: "appPreviewSets",
        extra_attributes: &["previewFrameTimeCode", "mimeType"],
        profile: "assets",
    },
    Kind {
        name: "appStoreReviewAttachment",
        resource_type: "appStoreReviewAttachments",
        create_tool: "appStoreReviewAttachments_createInstance",
        update_tool: "appStoreReviewAttachments_updateInstance",
        get_tool: "appStoreReviewAttachments_getInstance",
        relationship: "appStoreReviewDetail",
        parent_type: "appStoreReviewDetails",
        extra_attributes: &[],
        profile: "assets",
    },
    Kind {
        name: "appEncryptionDeclarationDocument",
        resource_type: "appEncryptionDeclarationDocuments",
        create_tool: "appEncryptionDeclarationDocuments_createInstance",
        update_tool: "appEncryptionDeclarationDocuments_updateInstance",
        get_tool: "appEncryptionDeclarationDocuments_getInstance",
        relationship: "appEncryptionDeclaration",
        parent_type: "appEncryptionDeclarations",
        extra_attributes: &[],
        profile: "assets",
    },
    Kind {
        name: "subscriptionAppStoreReviewScreenshot",
        resource_type: "subscriptionAppStoreReviewScreenshots",
        create_tool: "subscriptionAppStoreReviewScreenshots_createInstance",
        update_tool: "subscriptionAppStoreReviewScreenshots_updateInstance",
        get_tool: "subscriptionAppStoreReviewScreenshots_getInstance",
        relationship: "subscription",
        parent_type: "subscriptions",
        extra_attributes: &[],
        profile: "monetization",
    },
    Kind {
        name: "inAppPurchaseAppStoreReviewScreenshot",
        resource_type: "inAppPurchaseAppStoreReviewScreenshots",
        create_tool: "inAppPurchaseAppStoreReviewScreenshots_createInstance",
        update_tool: "inAppPurchaseAppStoreReviewScreenshots_updateInstance",
        get_tool: "inAppPurchaseAppStoreReviewScreenshots_getInstance",
        relationship: "inAppPurchaseV2",
        parent_type: "inAppPurchases",
        extra_attributes: &[],
        profile: "monetization",
    },
];

const MAX_WAIT_SECONDS: u64 = 600;

pub(super) fn tool(http: Result<reqwest::Client, String>, config: &ProxyConfig) -> CustomToolSpec {
    let enabled = enabled_kinds(config);
    let kinds: Vec<&str> = enabled.iter().map(|kind| kind.name).collect();
    CustomToolSpec {
        name: "asset_upload".to_string(),
        description: "Upload a file to App Store Connect using Apple's asset protocol: reserve, \
upload byte ranges, commit with an MD5 checksum, then wait for processing. Kinds: app \
screenshots/previews (parent = screenshot/preview set ID), App Review attachments (parent = \
appStoreReviewDetail ID), export-compliance documents (parent = appEncryptionDeclaration ID), \
and subscription/in-app purchase review screenshots (parent = subscription/inAppPurchase ID). \
`file` is relative to MCP_FACTORY_MEDIA_ROOT. Failures name the stage (validate, reserve, \
upload, commit, processing); a failed upload leaves the reservation in AWAITING_UPLOAD for \
you to delete or retry."
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "kind": {"type": "string", "enum": kinds},
                "parentId": {"type": "string", "minLength": 1,
                    "description": "ID of the set/detail/subscription/IAP/declaration that owns the asset."},
                "file": {"type": "string", "minLength": 1,
                    "description": "Path relative to MCP_FACTORY_MEDIA_ROOT."},
                "fileName": {"type": "string", "minLength": 1,
                    "description": "Name shown in App Store Connect (default: the file's name)."},
                "previewFrameTimeCode": {"type": "string",
                    "description": "appPreview only: poster frame, e.g. 00:00:05:00."},
                "mimeType": {"type": "string", "description": "appPreview only, e.g. video/mp4."},
                "waitSeconds": {"type": "integer", "minimum": 0, "maximum": MAX_WAIT_SECONDS,
                    "description": "How long to wait for processing (default 120; 0 returns right after commit)."}
            },
            "required": ["kind", "parentId", "file"],
            "additionalProperties": false
        }),
        hints: ToolHints {
            title: Some("Upload App Store Connect asset".to_string()),
            read_only: Some(false),
            destructive: Some(false),
            idempotent: Some(false),
            open_world: Some(true),
            ..Default::default()
        },
        handler: Arc::new(AssetUpload {
            http,
            media_root: config.media_root.clone(),
            kinds: enabled,
            poll_interval: Duration::from_secs(2),
        }),
    }
}

pub(super) struct AssetUpload {
    pub(super) http: Result<reqwest::Client, String>,
    pub(super) media_root: Option<PathBuf>,
    pub(super) kinds: Vec<&'static Kind>,
    pub(super) poll_interval: Duration,
}

/// Kinds whose generated tools are present under the active profiles. With
/// none (the tool is then filtered out by its own profiles) offer them all
/// so the schema stays valid.
pub(super) fn enabled_kinds(config: &ProxyConfig) -> Vec<&'static Kind> {
    let enabled: Vec<_> = KINDS
        .iter()
        .filter(|kind| config.profile_enabled(kind.profile))
        .collect();
    if enabled.is_empty() {
        KINDS.iter().collect()
    } else {
        enabled
    }
}

#[async_trait]
impl CustomToolHandler for AssetUpload {
    async fn call(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        Ok(match self.run(invoker, &arguments).await {
            Ok(result) | Err(result) => result,
        })
    }
}

impl AssetUpload {
    async fn run(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        args: &Value,
    ) -> Result<ToolResult, ToolResult> {
        let kind_name = args["kind"].as_str().unwrap_or_default();
        let kind = self
            .kinds
            .iter()
            .copied()
            .find(|kind| kind.name == kind_name)
            .ok_or_else(|| {
                failure(
                    "validate",
                    "asset kind is unknown or its profile is not enabled",
                    json!(kind_name),
                )
            })?;
        let http = transfer_client(&self.http, "validate")?;
        let parent_id = args["parentId"].as_str().unwrap_or_default();
        let relative = args["file"].as_str().unwrap_or_default();
        let wait = Duration::from_secs(
            args["waitSeconds"]
                .as_u64()
                .unwrap_or(120)
                .min(MAX_WAIT_SECONDS),
        );

        let (path, metadata) = resolve_media_file(self.media_root.as_deref(), relative)
            .await
            .map_err(|error| proxy_error_result("validate", error))?;
        let file_name = args["fileName"]
            .as_str()
            .map(str::to_string)
            .or_else(|| {
                path.file_name()
                    .map(|name| name.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "upload".to_string());
        let file_size = metadata.len();
        let checksum = md5_hex(&path).await.map_err(|error| {
            failure("validate", "cannot read the file", json!(error.to_string()))
        })?;

        // 1. Reserve.
        let mut attributes = json!({"fileName": file_name, "fileSize": file_size});
        for name in kind.extra_attributes {
            if let Some(value) = args.get(*name) {
                attributes[*name] = value.clone();
            }
        }
        let reservation = invoker
            .invoke_mutating(
                kind.create_tool,
                json!({"data": {
                    "type": kind.resource_type,
                    "attributes": attributes,
                    "relationships": {kind.relationship: {"data": {"type": kind.parent_type, "id": parent_id}}}
                }}),
            )
            .await
            .map_err(|error| proxy_error_result("reserve", error))
            .and_then(|result| json_of(result, "reserve"))?;
        let asset_id = reservation["data"]["id"]
            .as_str()
            .ok_or_else(|| {
                failure(
                    "reserve",
                    "reservation response has no asset id",
                    Value::Null,
                )
            })?
            .to_string();
        let operations = reservation["data"]["attributes"]["uploadOperations"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        if operations.is_empty() {
            return Err(failure(
                "reserve",
                "reservation response has no uploadOperations",
                json!({"assetId": asset_id}),
            ));
        }

        // 2. Upload every byte range.
        let total = operations.len();
        for (index, operation) in operations.iter().enumerate() {
            self.upload_part(&http, &path, file_size, operation).await.map_err(|message| {
                failure(
                    "upload",
                    &format!("upload operation {} of {total} failed", index + 1),
                    json!({"assetId": asset_id, "error": message,
                           "note": "the reservation remains in AWAITING_UPLOAD; delete it or retry"}),
                )
            })?;
        }

        // 3. Commit.
        let committed = invoker
            .invoke_mutating(
                kind.update_tool,
                json!({"id": asset_id, "data": {
                    "type": kind.resource_type,
                    "id": asset_id,
                    "attributes": {"uploaded": true, "sourceFileChecksum": checksum}
                }}),
            )
            .await
            .map_err(|error| proxy_error_result("commit", error))
            .and_then(|result| json_of(result, "commit"))?;

        // 4. Wait for processing.
        let mut resource = committed["data"].clone();
        let deadline = Instant::now() + wait;
        loop {
            let state = resource["attributes"]["assetDeliveryState"]["state"]
                .as_str()
                .unwrap_or("UNKNOWN")
                .to_string();
            let summary = json!({
                "kind": kind.name,
                "assetId": asset_id,
                "fileName": file_name,
                "fileSize": file_size,
                "sourceFileChecksum": checksum,
                "uploadOperations": total,
                "state": state,
                "assetDeliveryState": resource["attributes"]["assetDeliveryState"],
            });
            match state.as_str() {
                "COMPLETE" => {
                    return Ok(success(
                        json!({"ok": true, "stage": "complete", "asset": summary}),
                    ))
                }
                "FAILED" => {
                    return Err(failure(
                        "processing",
                        "Apple failed to process the asset",
                        summary,
                    ))
                }
                _ if Instant::now() >= deadline => {
                    return Ok(success(json!({
                        "ok": true,
                        "stage": "processing",
                        "message": format!("still processing; check with {} later", kind.get_tool),
                        "asset": summary
                    })))
                }
                _ => {}
            }
            tokio::time::sleep(self.poll_interval).await;
            let polled = invoker
                .invoke_read_only(kind.get_tool, json!({"id": asset_id}))
                .await
                .map_err(|error| proxy_error_result("processing", error))
                .and_then(|result| json_of(result, "processing"))?;
            resource = polled["data"].clone();
        }
    }

    async fn upload_part(
        &self,
        http: &reqwest::Client,
        path: &std::path::Path,
        file_size: u64,
        operation: &Value,
    ) -> Result<(), String> {
        let url = check_transfer_url(operation["url"].as_str().unwrap_or_default())?;
        let method =
            reqwest::Method::from_bytes(operation["method"].as_str().unwrap_or("PUT").as_bytes())
                .map_err(|_| "invalid upload method".to_string())?;
        let offset = operation["offset"].as_u64().unwrap_or(0);
        let length = operation["length"]
            .as_u64()
            .ok_or_else(|| "upload operation has no length".to_string())?;
        // The range comes from Apple's response: bound it by the real file
        // before allocating or seeking.
        let end = offset
            .checked_add(length)
            .filter(|end| *end <= file_size)
            .ok_or_else(|| {
                format!("upload range {offset}+{length} is outside the {file_size}-byte file")
            })?;

        let mut file = tokio::fs::File::open(path)
            .await
            .map_err(|error| format!("cannot open file: {error}"))?;
        file.seek(SeekFrom::Start(offset))
            .await
            .map_err(|error| format!("cannot seek file: {error}"))?;
        let mut chunk = vec![0; usize::try_from(length).map_err(|_| "part too large")?];
        file.read_exact(&mut chunk)
            .await
            .map_err(|error| format!("cannot read bytes {offset}..{end}: {error}"))?;

        let mut request = http.request(method, url).body(chunk);
        for header in operation["requestHeaders"].as_array().into_iter().flatten() {
            if let (Some(name), Some(value)) = (header["name"].as_str(), header["value"].as_str()) {
                request = request.header(name, value);
            }
        }
        let response = request
            .send()
            .await
            .map_err(|error| format!("request failed: {error}"))?;
        if response.status().is_success() {
            return Ok(());
        }
        let status = response.status();
        let body: String = response
            .text()
            .await
            .unwrap_or_default()
            .chars()
            .take(500)
            .collect();
        Err(format!("upload host returned {status}: {body}"))
    }
}

async fn md5_hex(path: &std::path::Path) -> std::io::Result<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Md5::new();
    let mut buffer = vec![0; 1 << 20];
    loop {
        let read = file.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn md5_matches_known_digest() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.txt");
        std::fs::write(&path, b"hello").unwrap();
        assert_eq!(
            md5_hex(&path).await.unwrap(),
            "5d41402abc4b2a76b9719d911017c592"
        );
    }

    #[tokio::test]
    async fn upload_ranges_outside_the_file_are_rejected_before_reading() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.png");
        std::fs::write(&path, b"0123456789").unwrap();
        let upload = AssetUpload {
            http: Ok(reqwest::Client::new()),
            media_root: None,
            kinds: Vec::new(),
            poll_interval: Duration::from_millis(1),
        };
        let client = reqwest::Client::new();
        for (offset, length) in [(0u64, 11u64), (8, 3), (u64::MAX, 2)] {
            let operation = json!({"method": "PUT", "url": "https://upload.example.com/p",
                                   "offset": offset, "length": length});
            let error = upload
                .upload_part(&client, &path, 10, &operation)
                .await
                .unwrap_err();
            assert!(error.contains("outside the 10-byte file"), "{error}");
        }
    }

    #[test]
    fn every_kind_names_distinct_tools() {
        let mut names: Vec<_> = KINDS.iter().map(|kind| kind.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), KINDS.len());
        for kind in KINDS {
            assert!(kind.create_tool.starts_with(kind.resource_type));
            assert!(kind.update_tool.starts_with(kind.resource_type));
            assert!(kind.get_tool.starts_with(kind.resource_type));
        }
    }
}
