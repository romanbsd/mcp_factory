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
use std::time::Duration;

use tokio::sync::Semaphore;
use tokio::time::Instant;
use tokio_util::io::ReaderStream;

use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, OperationInvoker, ProxyConfig, ProxyError,
    ToolHints, ToolResult,
};
use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[cfg(test)]
pub(super) use super::upload_session::md5_hex;
use super::{
    check_transfer_url,
    contracts::value,
    failure, json_of, proxy_error_result, success, transfer_client,
    upload_session::{
        AssetState, FileIdentity, PreparedFile, ResumeCheckpoint, UploadRange, MAX_UPLOAD_BYTES,
    },
};

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
    },
];

const MAX_WAIT_SECONDS: u64 = 600;
const STREAM_BUFFER_BYTES: usize = 64 * 1024;
const MAX_ERROR_BYTES: usize = mcp_factory_core::TransferLimits::ERROR_PREFIX_BYTES;

pub(super) fn tool(upload: Arc<AssetUpload>) -> CustomToolSpec {
    let kinds: Vec<_> = upload.kinds.iter().map(|kind| kind.name).collect();
    CustomToolSpec {
        name: "asset_upload".to_string(),
        description: "Upload a file to App Store Connect using Apple's asset protocol: reserve, \
upload byte ranges, commit with an MD5 checksum, then wait for processing. Kinds: app \
screenshots/previews (parent = screenshot/preview set ID), App Review attachments (parent = \
appStoreReviewDetail ID), export-compliance documents (parent = appEncryptionDeclaration ID), \
and subscription/in-app purchase review screenshots (parent = subscription/inAppPurchase ID). \
`file` is relative to MCP_FACTORY_MEDIA_ROOT. Failures name the stage (validate, reserve, \
upload, commit, processing); a failed upload leaves the reservation in AWAITING_UPLOAD for \
you to resume using the returned checkpoint. `resume` retries the existing reservation without creating another asset."
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
                "resume": {"type": "object", "properties": {
                    "assetId": {"type": "string", "minLength": 1},
                    "kind": {"type": "string"}, "parentId": {"type": "string"},
                    "file": {"type": "string"}, "fileName": {"type": "string"},
                    "fileSize": {"type": "integer", "minimum": 0, "maximum": MAX_UPLOAD_BYTES},
                    "sourceFileChecksum": {"type": "string", "pattern": "^[a-fA-F0-9]{32}$"},
                    "state": {"type": "string"}, "committed": {"type": "boolean"},
                    "completedRanges": {"type": "array", "items": {"type": "integer", "minimum": 0}, "uniqueItems": true},
                    "pendingRanges": {"type": "array", "items": {"type": "integer", "minimum": 0}},
                    "ranges": {"type": "array", "items": {"type": "object", "properties": {
                        "offset": {"type": "integer", "minimum": 0}, "length": {"type": "integer", "minimum": 0}
                    }, "required": ["offset", "length"], "additionalProperties": false}}
                }, "required": ["assetId", "fileSize", "sourceFileChecksum", "completedRanges"], "additionalProperties": false},
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
        handler: upload,
    }
}

pub(super) struct AssetUpload {
    pub(super) transfers: Arc<Semaphore>,
    pub(super) http: Result<reqwest::Client, String>,
    pub(super) media_root: Option<PathBuf>,
    pub(super) kinds: Vec<&'static Kind>,
    pub(super) poll_interval: Duration,
}

/// Kinds whose generated tools are present under the active profiles. With
/// none (the tool is then filtered out by its own profiles) offer them all
/// so the schema stays valid.
pub(super) fn enabled_kinds(config: &ProxyConfig) -> Vec<&'static Kind> {
    let profiles = crate::tools::build_tool_profiles();
    let enabled: Vec<_> = KINDS
        .iter()
        .filter(|kind| {
            config.profile_enabled("workflow")
                || [kind.create_tool, kind.get_tool, kind.update_tool]
                    .iter()
                    .all(|tool| {
                        profiles.iter().find(|(name, _)| name == tool).is_some_and(
                            |(_, memberships)| {
                                memberships
                                    .iter()
                                    .any(|profile| config.profile_enabled(profile))
                            },
                        )
                    })
        })
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
        invoker: &dyn OperationInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        Ok(match self.run(invoker, &arguments).await {
            Ok(result) | Err(result) => result,
        })
    }
}

impl AssetUpload {
    pub(super) fn new(
        http: Result<reqwest::Client, String>,
        config: &ProxyConfig,
        transfers: Arc<Semaphore>,
    ) -> Self {
        Self {
            http,
            transfers,
            media_root: config.media_root.clone(),
            kinds: enabled_kinds(config),
            poll_interval: Duration::from_secs(2),
        }
    }
    fn kind(&self, name: &str) -> Result<&'static Kind, ToolResult> {
        self.kinds
            .iter()
            .copied()
            .find(|kind| kind.name == name)
            .ok_or_else(|| {
                failure(
                    "validate",
                    "asset kind is unknown or its profile is not enabled",
                    json!(name),
                )
            })
    }

    /// Read-only resume preflight. Sync uses it before any mutation; run_inner
    /// rehashes/reconciles again at the write boundary to reject changed state.
    pub(super) async fn inspect_resume(
        &self,
        invoker: &dyn OperationInvoker,
        kind_name: &str,
        parent_id: &str,
        identity: &FileIdentity,
        checkpoint: &Value,
    ) -> Result<ResumeInspection, ToolResult> {
        let inspected = async {
            let kind = self.kind(kind_name)?;
            let resume = ResumeCheckpoint::parse(checkpoint)?;
            resume.validate_file(kind_name, parent_id, identity)?;
            let remote = read_reservation(invoker, kind, &resume).await?;
            reconcile_resume(kind, parent_id, identity, &resume, &remote["data"])
        }
        .await;
        inspected.map_err(|error| with_checkpoint(error, checkpoint.clone()))
    }

    pub(super) async fn run(
        &self,
        invoker: &dyn OperationInvoker,
        args: &Value,
    ) -> Result<ToolResult, ToolResult> {
        self.run_inner(invoker, args)
            .await
            .map_err(|error| match args.get("resume") {
                Some(checkpoint)
                    if error
                        .structured
                        .as_ref()
                        .and_then(|body| body.get("resume"))
                        .is_none() =>
                {
                    with_checkpoint(error, checkpoint.clone())
                }
                _ => error,
            })
    }

    async fn run_inner(
        &self,
        invoker: &dyn OperationInvoker,
        args: &Value,
    ) -> Result<ToolResult, ToolResult> {
        let kind_name = args["kind"].as_str().unwrap_or_default();
        let kind = self.kind(kind_name)?;
        let http = transfer_client(&self.http, "validate")?;
        let parent_id = args["parentId"].as_str().unwrap_or_default();
        let relative = args["file"].as_str().unwrap_or_default();
        if parent_id.is_empty() || relative.is_empty() {
            return Err(failure(
                "validate",
                "parentId and file must be nonempty",
                Value::Null,
            ));
        }
        // Bound concurrent file hashing, reservation and transfer work together.
        let _permit = self
            .transfers
            .clone()
            .acquire_owned()
            .await
            .map_err(|_| failure("validate", "transfer service is closed", Value::Null))?;
        let wait = Duration::from_secs(
            args["waitSeconds"]
                .as_u64()
                .unwrap_or(120)
                .min(MAX_WAIT_SECONDS),
        );
        let prepared = PreparedFile::inspect(
            self.media_root.as_deref(),
            relative,
            args["fileName"].as_str(),
        )
        .await?;
        let identity = &prepared.identity;
        let path = &prepared.path;
        let file_name = &identity.file_name;
        let file_size = identity.file_size;
        let checksum = &identity.checksum;
        let resume = args
            .get("resume")
            .map(ResumeCheckpoint::parse)
            .transpose()?;
        if let Some(resume) = &resume {
            resume.validate_file(kind.name, parent_id, identity)?;
        }
        let mut attributes = json!({"fileName": file_name, "fileSize": file_size});
        for name in kind.extra_attributes {
            if let Some(value) = args.get(*name) {
                attributes[*name] = value.clone();
            }
        }
        // Resume always starts with a read of the existing asset, never a POST.
        let reservation = if let Some(resume) = &resume {
            read_reservation(invoker, kind, resume)
                .await
                .map_err(|error| with_checkpoint(error, args["resume"].clone()))?
        } else {
            invoker.invoke_mutating(kind.create_tool, json!({"data": {
                "type": kind.resource_type, "attributes": attributes,
                "relationships": {kind.relationship: {"data": {"type": kind.parent_type, "id": parent_id}}}
            }})).await.map_err(|error| proxy_error_result("reserve", error))
                .and_then(|result| json_of(result, "reserve"))?
        };
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
        let mut resource = reservation["data"].clone();
        let operations = resource["attributes"]["uploadOperations"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let ranges: Vec<UploadRange> = operations.iter().map(UploadRange::snapshot).collect();
        let mut completed = Vec::new();
        let mut committed = false;
        let checkpoint = |completed: &[usize], committed: bool, resource: &Value| {
            value(ResumeCheckpoint {
                asset_id: asset_id.clone(),
                file_size,
                source_file_checksum: checksum.clone(),
                completed_ranges: completed.to_vec(),
                kind: Some(kind.name.to_owned()),
                parent_id: Some(parent_id.to_owned()),
                file: Some(relative.to_owned()),
                file_name: Some(file_name.clone()),
                state: Some(AssetState::from_resource(resource).as_str().to_owned()),
                committed: Some(committed),
                pending_ranges: Some(
                    (0..operations.len())
                        .filter(|index| !completed.contains(index))
                        .collect(),
                ),
                ranges: Some(ranges.clone()),
            })
        };
        if let Some(resume) = &resume {
            let inspection = reconcile_resume(kind, parent_id, identity, resume, &resource)
                .map_err(|error| {
                    with_checkpoint(error, checkpoint(&completed, committed, &resource))
                })?;
            completed = inspection.completed;
            committed = inspection.committed;
        }
        if !committed {
            if operations.is_empty() {
                return Err(with_checkpoint(
                    failure(
                        "reserve",
                        "reservation response has no uploadOperations",
                        json!({"assetId": asset_id}),
                    ),
                    checkpoint(&completed, committed, &resource),
                ));
            }
            // Validate the full operation plan before sending the first range.
            for operation in &operations {
                validate_range(file_size, operation).map_err(|message| {
                    with_checkpoint(
                        failure("upload", &message, json!({"assetId": asset_id})),
                        checkpoint(&completed, committed, &resource),
                    )
                })?;
            }
            for (index, operation) in operations.iter().enumerate() {
                if completed.contains(&index) {
                    continue;
                }
                self.upload_part(&http, path, file_size, operation)
                    .await
                    .map_err(|message| {
                        with_checkpoint(
                            failure(
                                "upload",
                                &format!(
                                    "upload operation {} of {} failed",
                                    index + 1,
                                    operations.len()
                                ),
                                json!({"assetId": asset_id, "error": message, "note": "the reservation remains in AWAITING_UPLOAD; delete it or retry"}),
                            ),
                            checkpoint(&completed, committed, &resource),
                        )
                    })?;
                completed.push(index);
            }
            let response = invoker
                .invoke_mutating(
                    kind.update_tool,
                    json!({"id": asset_id, "data": {
                        "type": kind.resource_type, "id": asset_id,
                        "attributes": {"uploaded": true, "sourceFileChecksum": checksum}
                    }}),
                )
                .await
                .map_err(|error| proxy_error_result("commit", error))
                .and_then(|result| json_of(result, "commit"))
                .map_err(|error| {
                    with_checkpoint(error, checkpoint(&completed, committed, &resource))
                })?;
            resource = response["data"].clone();
            committed = true;
        }
        let deadline = Instant::now() + wait;
        loop {
            let state = AssetState::from_resource(&resource);
            let summary = json!({"kind": kind.name, "assetId": asset_id, "fileName": file_name,
                "fileSize": file_size, "sourceFileChecksum": checksum, "uploadOperations": operations.len(),
                "state": state, "assetDeliveryState": resource["attributes"]["assetDeliveryState"]});
            let cp = checkpoint(&completed, committed, &resource);
            match state {
                AssetState::Complete => {
                    return Ok(success(
                        json!({"ok": true, "stage": "complete", "asset": summary, "resume": cp}),
                    ))
                }
                AssetState::Failed => {
                    return Err(with_checkpoint(
                        failure("processing", "Apple failed to process the asset", summary),
                        cp,
                    ))
                }
                AssetState::UploadComplete
                | AssetState::Processing
                | AssetState::AwaitingUpload => {}
                AssetState::Unknown(_) => {
                    return Err(with_checkpoint(
                        failure("processing", "unknown Apple asset delivery state", summary),
                        cp,
                    ))
                }
            }
            if Instant::now() >= deadline {
                return Ok(success(
                    json!({"ok": true, "stage": "processing", "message": format!("still processing; check with {} later", kind.get_tool), "asset": summary, "resume": cp}),
                ));
            }
            // The deadline bounds both sleeps and slow API requests.
            let poll = async {
                tokio::time::sleep(self.poll_interval).await;
                invoker
                    .invoke_read_only(kind.get_tool, json!({"id": asset_id}))
                    .await
                    .map_err(|error| proxy_error_result("processing", error))
                    .and_then(|result| json_of(result, "processing"))
            };
            match tokio::time::timeout_at(deadline, poll).await {
                Ok(Ok(polled)) => resource = polled["data"].clone(),
                Ok(Err(error)) => return Err(with_checkpoint(error, cp)),
                Err(_) => {
                    return Ok(success(
                        json!({"ok": true, "stage": "processing", "message": format!("still processing; check with {} later", kind.get_tool), "asset": summary, "resume": cp}),
                    ))
                }
            }
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
        let (offset, length) = validate_range(file_size, operation)?;

        let stream = upload_range_stream(path, offset, length).await?;
        let mut request = http
            .request(method, url)
            .body(reqwest::Body::wrap_stream(stream));
        for header in operation["requestHeaders"].as_array().into_iter().flatten() {
            if let (Some(name), Some(value)) = (header["name"].as_str(), header["value"].as_str()) {
                if name.eq_ignore_ascii_case("content-length")
                    || name.eq_ignore_ascii_case("transfer-encoding")
                {
                    continue;
                }
                request = request.header(name, value);
            }
        }
        let request = request.header(reqwest::header::CONTENT_LENGTH, length);
        let response = request
            .send()
            .await
            .map_err(|error| format!("request failed: {error}"))?;
        if response.status().is_success() {
            return Ok(());
        }
        let status = response.status();
        let body = bounded_error_body(response).await;
        Err(format!("upload host returned {status}: {body}"))
    }
}

/// Stream an already validated range without retaining the full part in memory.
pub(super) async fn upload_range_stream(
    path: &std::path::Path,
    offset: u64,
    length: u64,
) -> Result<ReaderStream<tokio::io::Take<tokio::fs::File>>, String> {
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|error| format!("cannot open file: {error}"))?;
    file.seek(SeekFrom::Start(offset))
        .await
        .map_err(|error| format!("cannot seek file: {error}"))?;
    Ok(ReaderStream::with_capacity(
        file.take(length),
        STREAM_BUFFER_BYTES,
    ))
}

async fn read_reservation(
    invoker: &dyn OperationInvoker,
    kind: &Kind,
    resume: &ResumeCheckpoint,
) -> Result<Value, ToolResult> {
    let mut query = json!({"id":resume.asset_id});
    if kind.name != "appEncryptionDeclarationDocument" {
        query["include"] = json!([kind.relationship]);
    }
    invoker
        .invoke_read_only(kind.get_tool, query)
        .await
        .map_err(|error| proxy_error_result("resume", error))
        .and_then(|result| json_of(result, "resume"))
}

pub(super) struct ResumeInspection {
    pub resource: Value,
    completed: Vec<usize>,
    committed: bool,
}

fn reconcile_resume(
    kind: &Kind,
    parent_id: &str,
    identity: &FileIdentity,
    resume: &ResumeCheckpoint,
    resource: &Value,
) -> Result<ResumeInspection, ToolResult> {
    let parent = &resource["relationships"][kind.relationship]["data"];
    if resource["type"].as_str() != Some(kind.resource_type)
        || resource["id"].as_str() != Some(resume.asset_id.as_str())
        || parent["id"].as_str() != Some(parent_id)
        || parent["type"].as_str() != Some(kind.parent_type)
        || resource["attributes"]["fileSize"].as_u64() != Some(identity.file_size)
        || resource["attributes"]["fileName"].as_str() != Some(identity.file_name.as_str())
    {
        return Err(failure(
            "resume",
            "existing asset identity, parent linkage or file metadata does not match",
            Value::Null,
        ));
    }
    let state = AssetState::from_resource(resource);
    let operations = resource["attributes"]["uploadOperations"].as_array();
    let completed = match state {
        AssetState::AwaitingUpload => {
            let operations = operations.filter(|ops| !ops.is_empty()).ok_or_else(|| {
                failure(
                    "resume",
                    "resume asset has no upload operations",
                    Value::Null,
                )
            })?;
            let ranges: Vec<_> = operations.iter().map(UploadRange::snapshot).collect();
            if !resume.completed_ranges.is_empty() && resume.ranges.as_ref() != Some(&ranges) {
                return Err(failure(
                    "resume",
                    "upload operation ranges changed or checkpoint has no ranges",
                    Value::Null,
                ));
            }
            let mut seen = std::collections::HashSet::new();
            for index in &resume.completed_ranges {
                if *index >= operations.len() || !seen.insert(*index) {
                    return Err(failure(
                        "resume",
                        "invalid completed range index",
                        Value::Null,
                    ));
                }
            }
            for operation in operations {
                let (_, length) = validate_range(identity.file_size, operation)
                    .map_err(|e| failure("resume", &e, Value::Null))?;
                if length == 0 {
                    return Err(failure(
                        "resume",
                        "resume upload range is empty",
                        Value::Null,
                    ));
                }
                check_transfer_url(operation["url"].as_str().unwrap_or_default())
                    .map_err(|e| failure("resume", &e, Value::Null))?;
            }
            resume.completed_ranges.clone()
        }
        AssetState::Complete | AssetState::UploadComplete | AssetState::Processing => {
            if !resource["attributes"]["sourceFileChecksum"]
                .as_str()
                .is_some_and(|s| s.eq_ignore_ascii_case(&identity.checksum))
            {
                return Err(failure(
                    "resume",
                    "Apple checksum does not match the local file",
                    Value::Null,
                ));
            }
            (0..operations.map_or(0, Vec::len)).collect()
        }
        AssetState::Failed => {
            return Err(failure(
                "processing",
                "Apple failed to process the asset",
                resource.clone(),
            ))
        }
        AssetState::Unknown(_) => {
            return Err(failure(
                "resume",
                "unknown Apple asset delivery state",
                resource.clone(),
            ))
        }
    };
    Ok(ResumeInspection {
        resource: resource.clone(),
        completed,
        committed: state.committed(),
    })
}

fn with_checkpoint(mut result: ToolResult, checkpoint: Value) -> ToolResult {
    if let Some(body) = result.structured.as_mut() {
        body["resume"] = checkpoint.clone();
        // Keep the original error details (including Apple's errors[]) intact.
        if body["details"].is_object() {
            body["details"]["resume"] = checkpoint;
        }
        return ToolResult {
            is_error: true,
            ..ToolResult::text(body.to_string()).with_structured(Some(body.clone()))
        };
    }
    result
}

fn validate_range(file_size: u64, operation: &Value) -> Result<(u64, u64), String> {
    UploadRange::from_operation(operation)?.validate(file_size)
}

async fn bounded_error_body(response: reqwest::Response) -> String {
    let mut body = match mcp_factory_core::read_error_prefix(response, MAX_ERROR_BYTES).await {
        Ok(prefix) => match prefix.read_error {
            Some(error) => format!("{} [response read failed: {error}]", prefix.text),
            None => prefix.text,
        },
        Err(error) => format!("response read failed: {}", error.without_url()),
    };
    if body.len() > MAX_ERROR_BYTES {
        let mut end = MAX_ERROR_BYTES;
        while !body.is_char_boundary(end) {
            end -= 1;
        }
        body.truncate(end);
    }
    body
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
            transfers: Arc::new(Semaphore::new(2)),
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
    fn enabled_kinds_follow_generated_membership_for_raw_and_composite_profiles() {
        const ASSETS: &[&str] = &[
            "appScreenshot",
            "appPreview",
            "appStoreReviewAttachment",
            "appEncryptionDeclarationDocument",
        ];
        const MONETIZATION: &[&str] = &[
            "subscriptionAppStoreReviewScreenshot",
            "inAppPurchaseAppStoreReviewScreenshot",
        ];
        const ALL: &[&str] = &[
            "appScreenshot",
            "appPreview",
            "appStoreReviewAttachment",
            "appEncryptionDeclarationDocument",
            "subscriptionAppStoreReviewScreenshot",
            "inAppPurchaseAppStoreReviewScreenshot",
        ];
        for (selected, expected) in [
            (None, ALL),
            (Some(&["workflow"][..]), ALL),
            (Some(&["all"][..]), ALL),
            (Some(&["assets"][..]), ASSETS),
            (Some(&["monetization"][..]), MONETIZATION),
            (Some(&["release_raw"][..]), ASSETS),
            (Some(&["monetization_raw"][..]), MONETIZATION),
            (Some(&["assets", "monetization"][..]), ALL),
            (Some(&["release_raw", "monetization"][..]), ALL),
            (Some(&["assets", "monetization_raw"][..]), ALL),
        ] {
            let config = ProxyConfig {
                profiles: selected
                    .map(|profiles| profiles.iter().map(|name| (*name).to_owned()).collect()),
                ..Default::default()
            };
            let actual: Vec<_> = enabled_kinds(&config)
                .iter()
                .map(|kind| kind.name)
                .collect();
            assert_eq!(actual, expected, "selected profiles: {selected:?}");
        }
    }

    #[test]
    fn hidden_upload_helper_keeps_a_valid_kind_schema() {
        for selected in [vec!["reports".to_owned()], vec![]] {
            let config = ProxyConfig {
                profiles: Some(selected),
                ..Default::default()
            };
            let actual: Vec<_> = enabled_kinds(&config)
                .iter()
                .map(|kind| kind.name)
                .collect();
            assert_eq!(
                actual,
                vec![
                    "appScreenshot",
                    "appPreview",
                    "appStoreReviewAttachment",
                    "appEncryptionDeclarationDocument",
                    "subscriptionAppStoreReviewScreenshot",
                    "inAppPurchaseAppStoreReviewScreenshot",
                ]
            );
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
