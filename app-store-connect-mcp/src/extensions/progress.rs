//! Progress is diagnostic, never a replacement for retry reconciliation.
use super::failure;
use mcp_factory_core::ToolResult;
use serde_json::{json, Value};

pub(super) struct ApplyProgress {
    planned: Vec<Value>,
    applied: usize,
}
impl ApplyProgress {
    pub fn new(planned: Vec<Value>) -> Self {
        Self {
            planned,
            applied: 0,
        }
    }
    pub fn applied(&mut self) {
        self.applied += 1;
    }
    pub fn interrupted(&self, error: ToolResult) -> ToolResult {
        let upstream = error
            .structured
            .clone()
            .unwrap_or_else(|| json!({"message":error.into_text()}));
        failure(
            "apply",
            "workflow interrupted; retry reconciles current Apple state",
            json!({
            "progress":{"plannedCount":self.planned.len(),"applied":self.planned[..self.applied],"pending":self.planned[self.applied..]},
                "upstream":upstream
            }),
        )
    }
}
