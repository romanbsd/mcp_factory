mod capabilities;
mod client;
mod explain;
mod project_status;
mod quality;
mod registry;

use std::sync::Arc;
use std::time::Duration;

use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, ProxyError, ReadOnlyToolInvoker, ToolHints,
    ToolResult,
};
use serde_json::{json, Value};

#[derive(Clone, Copy)]
enum ReportKind {
    Capabilities,
    ProjectStatus,
    QualityHealth,
    ExplainConsoleMessage,
}

struct ReportHandler(ReportKind);

#[async_trait]
impl CustomToolHandler for ReportHandler {
    async fn call(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        let mut client = client::EvidenceClient::new(invoker);
        let report_future = async {
            match self.0 {
                ReportKind::Capabilities => capabilities::report(&mut client, &arguments).await,
                ReportKind::ProjectStatus => project_status::report(&mut client, &arguments).await,
                ReportKind::QualityHealth => quality::report(&mut client, &arguments).await,
                ReportKind::ExplainConsoleMessage => explain::report(&mut client, &arguments).await,
            }
        };
        let report = match tokio::time::timeout(Duration::from_secs(45), report_future).await {
            Ok(report) => report,
            Err(_) => json!({
                "status": if client.source_calls.is_empty() { "unavailable" } else { "partial" },
                "generatedAt": mcp_factory_core::chrono::Utc::now().to_rfc3339(),
                "app": quality::app(arguments["packageName"].as_str().unwrap_or_default()),
                "summary": "The 45-second reporting deadline was exhausted; completed source evidence is preserved.",
                "findings": [],
                "actions": [],
                "coverageGaps": registry::console_coverage_gaps(),
                "sourceCalls": client.source_calls,
                "warnings": [{"message": "Reporting deadline exhausted"}]
            }),
        };
        let text = serde_json::to_string_pretty(&report)
            .map_err(|error| ProxyError::Other(format!("failed to encode report: {error}")))?;
        Ok(ToolResult::text(text).with_structured(Some(report)))
    }
}

pub fn build_tools() -> Vec<CustomToolSpec> {
    vec![
        tool(
            "report_capabilities",
            "Diagnose Google Play Publisher and Developer Reporting access, validated metric combinations, and Console-only gaps.",
            package_schema(json!({
                "probe": {"type": "boolean", "default": true}
            })),
            ReportKind::Capabilities,
        ),
        tool(
            "report_project_status",
            "Build an evidence-bearing, read-only summary of release lifecycle, serving state, quality, reviews, and recoveries.",
            package_schema(json!({
                "lookbackDays": {"type": "integer", "minimum": 1, "maximum": 365, "default": 90},
                "cohorts": {"type": "array", "items": {"enum": ["OS_PUBLIC", "APP_TESTERS"]}},
                "include": {"type": "array", "items": {"enum": ["releases", "quality", "reviews", "recoveries"]}},
                "detail": {"enum": ["summary", "evidence"], "default": "summary"}
            })),
            ReportKind::ProjectStatus,
        ),
        tool(
            "report_quality_health",
            "Normalize Android vitals while distinguishing observed zero, no data, unsupported combinations, and unavailable sources.",
            package_schema(json!({
                "lookbackDays": {"type": "integer", "minimum": 1, "maximum": 365, "default": 90},
                "cohorts": {"type": "array", "items": {"enum": ["OS_PUBLIC", "APP_TESTERS"]}},
                "metrics": {"type": "array", "items": {"type": "string"}},
                "includeIssues": {"type": "boolean", "default": true},
                "includeAnomalies": {"type": "boolean", "default": true}
            })),
            ReportKind::QualityHealth,
        ),
        tool(
            "report_explain_console_message",
            "Classify user-supplied Play Console text, correlate applicable API evidence, and identify Console-only limitations without claiming resolution.",
            {
                let mut schema = package_schema(json!({
                    "message": {"type": "string", "minLength": 1},
                    "consoleArea": {"type": "string"},
                    "consoleSeverity": {"enum": ["blocking", "action_required", "warning", "info"]},
                    "versionCode": {"type": "integer"}
                }));
                schema["required"] = json!(["packageName", "message"]);
                schema
            },
            ReportKind::ExplainConsoleMessage,
        ),
    ]
}

fn package_schema(extra_properties: Value) -> Value {
    let mut properties = extra_properties.as_object().cloned().unwrap_or_default();
    properties.insert(
        "packageName".to_string(),
        json!({"type": "string", "minLength": 1}),
    );
    json!({
        "type": "object",
        "properties": properties,
        "required": ["packageName"],
        "additionalProperties": false
    })
}

fn tool(name: &str, description: &str, input_schema: Value, kind: ReportKind) -> CustomToolSpec {
    CustomToolSpec {
        name: name.to_string(),
        description: description.to_string(),
        input_schema,
        hints: ToolHints {
            title: Some(name.replace('_', " ")),
            output_schema: Some(json!({"type": "object"})),
            read_only: Some(true),
            destructive: Some(false),
            idempotent: Some(true),
            open_world: Some(true),
        },
        handler: Arc::new(ReportHandler(kind)),
    }
}

#[cfg(test)]
mod tests;
