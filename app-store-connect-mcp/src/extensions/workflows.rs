//! Agent-facing task contracts; domain services call internal API capabilities.
use super::{
    asset_upload::AssetUpload,
    contracts::METADATA_FIELDS,
    metadata, release, screenshots,
    services::{compact, Asc},
    success, testflight,
};
use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, OperationInvoker, ProxyError, ToolHints,
    ToolResult,
};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone, Copy)]
enum WorkflowKind {
    AppGet,
    VersionGet,
    ReleaseStatus,
    ReleasePrepare,
    MetadataGet,
    MetadataApply,
    ScreenshotsGet,
    ScreenshotsSync,
    TestflightStatus,
    TestflightDistribute,
    TestflightRemove,
    ReviewSubmit,
    Release,
}
impl WorkflowKind {
    const ALL: [Self; 13] = [
        Self::AppGet,
        Self::VersionGet,
        Self::ReleaseStatus,
        Self::ReleasePrepare,
        Self::MetadataGet,
        Self::MetadataApply,
        Self::ScreenshotsGet,
        Self::ScreenshotsSync,
        Self::TestflightStatus,
        Self::TestflightDistribute,
        Self::TestflightRemove,
        Self::ReviewSubmit,
        Self::Release,
    ];

    fn definition(self) -> Definition {
        match self {
            Self::AppGet => definition("app_get", "Resolve an app by bundle ID; return compact identity.",
                || schema(&["bundleId"], json!({})), true, false, true),
            Self::VersionGet => definition("version_get", "Resolve app/version and attached build; return compact release state.",
                || schema(&["bundleId", "version"], json!({})), true, false, true),
            Self::ReleaseStatus => definition("release_status", "Inspect version, attached build, review submission and phased release in one call.",
                || schema(&["bundleId", "version"], json!({})), true, false, true),
            Self::ReleasePrepare => definition("release_prepare", "Check submission readiness: known blockers, warnings and unknown Apple-evaluated conditions. Read-only.",
                || schema(&["bundleId", "version"], review_properties()), true, false, true),
            Self::MetadataGet => definition("metadata_get", "Read normalized version metadata; defaults to primary locale. Select locales/fields for larger listings.",
                || schema(&["bundleId", "version"], json!({"locales":strings(), "fields":{"type":"array","items":{"type":"string","enum":METADATA_FIELDS}}})), true, false, true),
            Self::MetadataApply => definition("metadata_apply", "Reconcile desired locale-to-fields metadata: create missing localizations, update changed fields, leave others untouched.",
                || schema(&["bundleId", "version", "localizations"], metadata_properties()), false, false, true),
            Self::ScreenshotsGet => definition("screenshots_get", "Read normalized screenshots by locale/display type with ordering, states and bounded continuation.",
                || schema(&["bundleId", "version"], json!({"locales":strings(),"displayType":{"type":"string"},"offset":{"type":"integer","minimum":0}})), true, false, true),
            Self::ScreenshotsSync => definition("screenshots_sync", "Reconcile screenshot files/checksums, resume uploads and establish ordering. Retain obsolete assets unless removeObsolete=true.",
                || schema(&["bundleId", "version", "locale", "displayType", "files"], json!({"locale":{"type":"string","minLength":1},"displayType":{"type":"string","minLength":1},"files":{"type":"array","items":{"type":"string","minLength":1},"maxItems":10,"uniqueItems":true},"waitSeconds":{"type":"integer","minimum":0,"maximum":600,"default":120,"description":"Processing wait per asset; 0 returns checkpoints immediately."},"removeObsolete":{"type":"boolean","default":false},"resumes":{"type":"object","description":"Relative file path -> checkpoint returned by an earlier upload."}})), false, true, true),
            Self::TestflightStatus => definition("testflight_status", "Resolve a build, inspect beta state/expiration and current group availability.",
                || { let mut properties=testflight_properties(); properties["offset"]=json!({"type":"integer","minimum":0,"default":0,"description":"Group-page offset; follow nextOffset for larger apps."}); schema(&["bundleId", "buildNumber"], properties) }, true, false, true),
            Self::TestflightDistribute => definition("testflight_distribute", "Assign a build to named groups after resolving app/build/groups; already assigned groups are unchanged.",
                || schema(&["bundleId", "buildNumber", "groupNames"], testflight_properties()), false, false, true),
            Self::TestflightRemove => definition("testflight_remove", "Remove existing build/group relationships; warn when groups retain access to all builds.",
                || schema(&["bundleId", "buildNumber", "groupNames"], testflight_properties()), false, true, true),
            Self::ReviewSubmit => definition("review_submit", "Reconcile draft/submitted review items, run preflight and submit once. Dry run returns a plan.",
                || schema(&["bundleId", "version"], review_properties()), false, false, false),
            Self::Release => definition("release", "Request manual release of an approved version in PENDING_DEVELOPER_RELEASE. Separate from review submission.",
                || schema(&["bundleId", "version"], json!({})), false, true, false),
        }
    }
}
struct Definition {
    name: &'static str,
    description: &'static str,
    schema: fn() -> Value,
    hints: ToolHints,
}
fn definition(
    name: &'static str,
    description: &'static str,
    schema: fn() -> Value,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
) -> Definition {
    Definition {
        name,
        description,
        schema,
        hints: ToolHints {
            read_only: Some(read_only),
            destructive: Some(destructive),
            idempotent: Some(idempotent),
            open_world: Some(true),
            ..Default::default()
        },
    }
}
fn schema(required: &[&str], extras: Value) -> Value {
    let mut properties = json!({"bundleId":{"type":"string","minLength":1},"platform":{"type":"string","enum":["IOS","MAC_OS","TV_OS","VISION_OS"],"default":"IOS"}});
    if required.contains(&"version") {
        properties["version"] = json!({"type":"string","minLength":1});
    }
    properties
        .as_object_mut()
        .unwrap()
        .extend(extras.as_object().unwrap().clone());
    json!({"type":"object","properties":properties,"required":required,"additionalProperties":false})
}
fn review_properties() -> Value {
    json!({"requiredDisplayTypes":{"type":"array","items":{"type":"string"},"uniqueItems":true}})
}
fn testflight_properties() -> Value {
    json!({"buildNumber":{"type":"string","minLength":1},"version":{"type":"string","description":"Optional marketing version to disambiguate build numbers."},"groupNames":strings()})
}
fn metadata_properties() -> Value {
    let fields: serde_json::Map<String, Value> = METADATA_FIELDS
        .iter()
        .map(|field| ((*field).to_owned(), json!({"type":["string","null"]})))
        .collect();
    json!({"localizations":{"type":"object","minProperties":1,"maxProperties":100,"additionalProperties":{"type":"object","properties":fields,"additionalProperties":false}}})
}
pub(super) fn tools(upload: Arc<AssetUpload>) -> Vec<CustomToolSpec> {
    WorkflowKind::ALL
        .into_iter()
        .map(|kind| {
            let definition = kind.definition();
            let mut input_schema = (definition.schema)();
            if definition.hints.read_only == Some(false) {
                input_schema["properties"]["dryRun"] = json!({"type":"boolean","default":false,"description":"Discover and compare, without any mutation."});
            }
            CustomToolSpec {
                name: definition.name.into(),
                description: definition.description.into(),
                input_schema,
                hints: definition.hints,
                handler: Arc::new(Workflow {
                    kind,
                    upload: upload.clone(),
                }),
            }
        })
        .collect()
}
fn strings() -> Value {
    json!({"type":"array","items":{"type":"string","minLength":1},"uniqueItems":true,"minItems":1,"maxItems":100})
}
struct Workflow {
    kind: WorkflowKind,
    upload: Arc<AssetUpload>,
}
#[async_trait]
impl CustomToolHandler for Workflow {
    async fn call(
        &self,
        invoker: &dyn OperationInvoker,
        args: Value,
    ) -> Result<ToolResult, ProxyError> {
        let client = Asc { invoker };
        let result = match self.kind {
            WorkflowKind::AppGet => client
                .app(&args)
                .await
                .map(|app| compact(&app, &["name", "bundleId", "sku", "primaryLocale"])),
            WorkflowKind::MetadataGet => {
                metadata::execute(&client, metadata::MetadataAction::Get, &args).await
            }
            WorkflowKind::MetadataApply => {
                metadata::execute(&client, metadata::MetadataAction::Apply, &args).await
            }
            WorkflowKind::ScreenshotsGet => {
                screenshots::execute(
                    &client,
                    screenshots::ScreenshotAction::Get,
                    &args,
                    &self.upload,
                )
                .await
            }
            WorkflowKind::ScreenshotsSync => {
                screenshots::execute(
                    &client,
                    screenshots::ScreenshotAction::Sync,
                    &args,
                    &self.upload,
                )
                .await
            }
            WorkflowKind::TestflightStatus => {
                testflight::execute(&client, testflight::TestflightAction::Status, &args).await
            }
            WorkflowKind::TestflightDistribute => {
                testflight::execute(&client, testflight::TestflightAction::Distribute, &args).await
            }
            WorkflowKind::TestflightRemove => {
                testflight::execute(&client, testflight::TestflightAction::Remove, &args).await
            }
            WorkflowKind::VersionGet => {
                release::execute(&client, release::ReleaseAction::VersionGet, &args).await
            }
            WorkflowKind::ReleaseStatus => {
                release::execute(&client, release::ReleaseAction::Status, &args).await
            }
            WorkflowKind::ReleasePrepare => {
                release::execute(&client, release::ReleaseAction::Prepare, &args).await
            }
            WorkflowKind::ReviewSubmit => {
                release::execute(&client, release::ReleaseAction::Submit, &args).await
            }
            WorkflowKind::Release => {
                release::execute(&client, release::ReleaseAction::Release, &args).await
            }
        };
        Ok(match result {
            Ok(value) => success(value),
            Err(error) => error,
        })
    }
}
