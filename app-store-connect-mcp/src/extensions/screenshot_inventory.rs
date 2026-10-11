//! Shared screenshot discovery; callers choose complete reconciliation or bounded pages.
use super::{
    failure,
    services::{required, Asc, Collection},
};
use mcp_factory_core::ToolResult;
use serde_json::{json, Value};
use std::collections::HashSet;
pub(super) const SETS: &str = "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated";
pub(super) const ASSETS: &str = "appScreenshotSets_appScreenshots_getToManyRelated";
pub(super) const ASSET_FIELDS: &[&str] = &[
    "fileName",
    "fileSize",
    "sourceFileChecksum",
    "assetDeliveryState",
];
pub(super) struct ScreenshotInventory<'c, 'i> {
    pub client: &'c Asc<'i>,
}
impl ScreenshotInventory<'_, '_> {
    pub async fn sets(&self, localization: &Value) -> Result<Vec<Value>, ToolResult> {
        let rows=self.client.pages(SETS,json!({"id":localization["id"],"fields[appScreenshotSets]":["screenshotDisplayType"],"limit":200})).await?;
        let mut types = HashSet::new();
        for row in &rows {
            let display = required(&row["attributes"], "screenshotDisplayType")?;
            if !types.insert(display) {
                return Err(failure(
                    "lookup",
                    "ambiguous screenshot sets for display type",
                    json!(display),
                ));
            }
            required(row, "id")?;
        }
        Ok(rows)
    }
    pub fn asset_pages(&self, set: &Value, fields: &[&str]) -> Collection<'_, '_> {
        self.client.collection(
            ASSETS,
            json!({"id":set["id"],"fields[appScreenshots]":fields,"limit":200}),
        )
    }
    pub async fn assets(&self, set: &Value, fields: &[&str]) -> Result<Vec<Value>, ToolResult> {
        let mut pages = self.asset_pages(set, fields);
        let mut rows = Vec::new();
        let mut ids = HashSet::new();
        while let Some(page) = pages.next().await? {
            for row in page.rows {
                if !ids.insert(required(&row, "id")?.to_owned()) {
                    return Err(failure("lookup", "duplicate screenshot id", Value::Null));
                }
                rows.push(row);
            }
        }
        Ok(rows)
    }
}
pub(super) fn state(row: &Value) -> &str {
    row["attributes"]["assetDeliveryState"]["state"]
        .as_str()
        .unwrap_or("UNKNOWN")
}
pub(super) fn complete(rows: &[Value]) -> bool {
    !rows.is_empty() && rows.iter().all(|row| state(row) == "COMPLETE")
}
