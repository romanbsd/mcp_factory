//! Small handwritten workflow contracts. Generated Apple schemas stay authoritative.
use serde::Serialize;
use serde_json::Value;

pub(super) const METADATA_FIELDS: &[&str] = &[
    "description",
    "keywords",
    "whatsNew",
    "promotionalText",
    "marketingUrl",
    "supportUrl",
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ChangeReport<C> {
    pub changes: Vec<C>,
    pub unchanged: Vec<String>,
    pub warnings: Vec<Value>,
    pub dry_run: bool,
}

#[derive(Serialize)]
pub(super) struct MetadataChange {
    pub action: &'static str,
    pub resource: &'static str,
    pub locale: String,
    pub fields: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GroupChange {
    pub action: &'static str,
    pub resource: &'static str,
    pub group_name: String,
    pub build_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ScreenshotChange<'a> {
    pub resource: &'a str,
    pub action: &'a str,
    pub locale: &'a str,
    pub display_type: &'a str,
    pub file: Option<&'a str>,
    pub id: Option<&'a str>,
}

pub(super) fn value<T: Serialize>(contract: T) -> Value {
    serde_json::to_value(contract).expect("workflow contracts contain JSON-compatible fields")
}
