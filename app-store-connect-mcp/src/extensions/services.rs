//! Domain resolution over internal generated operations, never over MCP calls.
use super::{failure, json_of, proxy_error_result};
use mcp_factory_core::OperationInvoker;
use serde_json::{json, Value};

pub(super) struct Asc<'a> {
    pub invoker: &'a dyn OperationInvoker,
}
pub(super) struct CollectionPage {
    pub rows: Vec<Value>,
    pub total: Option<usize>,
    pub has_more: bool,
}
pub(super) struct CollectionWindow {
    pub rows: Vec<Value>,
    pub total: Option<usize>,
    pub has_more: bool,
    pub observed: usize,
}
pub(super) struct Collection<'c, 'i> {
    client: &'c Asc<'i>,
    operation: String,
    args: Value,
    seen: std::collections::HashSet<String>,
    pages: usize,
    rows: usize,
    done: bool,
}
impl Collection<'_, '_> {
    pub async fn next(&mut self) -> Result<Option<CollectionPage>, mcp_factory_core::ToolResult> {
        if self.done {
            return Ok(None);
        }
        if self.pages == 100 {
            return Err(failure(
                "lookup",
                "collection exceeds page limit",
                json!(self.operation),
            ));
        }
        let mut result = self.client.get(&self.operation, self.args.clone()).await?;
        let total = result["meta"]["paging"]["total"]
            .as_u64()
            .and_then(|n| usize::try_from(n).ok());
        let next = result["links"]["next"].as_str().map(str::to_owned);
        let rows = result["data"]
            .as_array_mut()
            .map(std::mem::take)
            .ok_or_else(|| failure("lookup", "expected a collection", json!(self.operation)))?;
        self.pages += 1;
        self.rows += rows.len();
        if self.rows > 10000 {
            return Err(failure(
                "lookup",
                "collection exceeds workflow limit; narrow the selection",
                json!(self.operation),
            ));
        }
        self.done = next.is_none();
        if let Some(next) = next {
            let cursor = reqwest::Url::parse(&next)
                .ok()
                .and_then(|url| {
                    url.query_pairs()
                        .find(|(key, _)| key == "cursor")
                        .map(|(_, v)| v.into_owned())
                })
                .ok_or_else(|| {
                    failure("lookup", "pagination has no cursor", json!(self.operation))
                })?;
            if !self.seen.insert(cursor.clone()) {
                return Err(failure(
                    "lookup",
                    "repeated pagination cursor",
                    json!(self.operation),
                ));
            }
            self.args["cursor"] = json!(cursor);
        }
        Ok(Some(CollectionPage {
            rows,
            total,
            has_more: !self.done,
        }))
    }
}
impl Asc<'_> {
    pub fn collection(&self, operation: &str, args: Value) -> Collection<'_, '_> {
        Collection {
            client: self,
            operation: operation.to_owned(),
            args,
            seen: Default::default(),
            pages: 0,
            rows: 0,
            done: false,
        }
    }
    pub async fn window(
        &self,
        operation: &str,
        mut args: Value,
        offset: usize,
        limit: usize,
    ) -> Result<CollectionWindow, mcp_factory_core::ToolResult> {
        args["limit"] = json!(limit);
        let mut collection = self.collection(operation, args);
        let mut rows = Vec::new();
        let mut observed = 0usize;
        let mut total = None;
        while let Some(page) = collection.next().await? {
            if page.total.is_some() {
                total = page.total;
            }
            for row in page.rows {
                if observed >= offset && rows.len() < limit {
                    rows.push(row);
                }
                observed += 1;
            }
            if !page.has_more && total.is_none() {
                total = Some(observed);
            }
            if rows.len() == limit || !page.has_more {
                return Ok(CollectionWindow {
                    has_more: page.has_more || observed > offset.saturating_add(rows.len()),
                    rows,
                    total,
                    observed,
                });
            }
        }
        Ok(CollectionWindow {
            rows,
            total,
            observed,
            has_more: false,
        })
    }
    pub async fn get(
        &self,
        operation: &str,
        args: Value,
    ) -> Result<Value, mcp_factory_core::ToolResult> {
        self.invoker
            .invoke_read_only(operation, args)
            .await
            .map_err(|e| proxy_error_result("lookup", e))
            .and_then(|r| json_of(r, "lookup"))
    }
    pub async fn optional(
        &self,
        operation: &str,
        args: Value,
    ) -> Result<Value, mcp_factory_core::ToolResult> {
        let result = self
            .invoker
            .invoke_read_only(operation, args)
            .await
            .map_err(|e| proxy_error_result("lookup", e))?;
        if result.is_error
            && result
                .structured
                .as_ref()
                .is_some_and(|s| s["status"] == 404)
        {
            return Ok(json!({"data":null}));
        }
        json_of(result, "lookup")
    }
    pub async fn write(
        &self,
        operation: &str,
        args: Value,
    ) -> Result<Value, mcp_factory_core::ToolResult> {
        self.invoker
            .invoke_mutating(operation, args)
            .await
            .map_err(|e| proxy_error_result("apply", e))
            .and_then(|r| json_of(r, "apply"))
    }
    pub async fn pages(
        &self,
        operation: &str,
        args: Value,
    ) -> Result<Vec<Value>, mcp_factory_core::ToolResult> {
        let mut rows = Vec::new();
        let mut collection = self.collection(operation, args);
        while let Some(page) = collection.next().await? {
            rows.extend(page.rows);
        }
        Ok(rows)
    }
    pub async fn app(&self, args: &Value) -> Result<Value, mcp_factory_core::ToolResult> {
        let bundle = required(args, "bundleId")?;
        let rows = self.pages("apps_getCollection", json!({"filter[bundleId]":[bundle],"fields[apps]":["name","bundleId","sku","primaryLocale"],"limit":200})).await?;
        unique(
            rows.into_iter()
                .filter(|r| r["attributes"]["bundleId"] == bundle)
                .collect(),
            "app",
        )
    }
    pub async fn version(
        &self,
        args: &Value,
    ) -> Result<(Value, Value), mcp_factory_core::ToolResult> {
        let app = self.app(args).await?;
        let version = required(args, "version")?;
        let platform = args["platform"].as_str().unwrap_or("IOS");
        let rows = self.pages("apps_appStoreVersions_getToManyRelated", json!({"id":app["id"],"filter[versionString]":[version],"filter[platform]":[platform],"limit":200,"fields[appStoreVersions]":["versionString","platform","appStoreState","releaseType","earliestReleaseDate","build"]})).await?;
        let selected = unique(
            rows.into_iter()
                .filter(|r| {
                    r["attributes"]["versionString"] == version
                        && r["attributes"]["platform"] == platform
                })
                .collect(),
            "version",
        )?;
        Ok((app, selected))
    }
    pub async fn build(
        &self,
        args: &Value,
    ) -> Result<(Value, Value), mcp_factory_core::ToolResult> {
        let app = self.app(args).await?;
        let number = required(args, "buildNumber")?;
        let mut query = json!({"filter[app]":[app["id"]],"filter[version]":[number],"limit":200,"filter[preReleaseVersion.platform]":[args["platform"].as_str().unwrap_or("IOS")],"include":["preReleaseVersion"],"fields[builds]":["version","processingState","expired","expirationDate","usesNonExemptEncryption","preReleaseVersion"],"fields[preReleaseVersions]":["platform","version"]});
        if let Some(version) = args["version"].as_str() {
            query["filter[preReleaseVersion.version]"] = json!([version]);
        }
        let rows = self.pages("builds_getCollection", query).await?;
        // A build number alone may be ambiguous across platform/version trains.
        Ok((
            app,
            unique(
                rows.into_iter()
                    .filter(|r| r["attributes"]["version"] == number)
                    .collect(),
                "build",
            )?,
        ))
    }

    /// Localization inventory with caller-selected sparse fields. Callers own
    /// missing/ambiguous locale policy (preflight reports blockers; sync fails).
    pub async fn localizations(
        &self,
        version: &Value,
        locales: Option<&[String]>,
        fields: &[&str],
    ) -> Result<Vec<Value>, mcp_factory_core::ToolResult> {
        let mut fields = fields.to_vec();
        if !fields.contains(&"locale") {
            fields.push("locale");
        }
        let mut query =
            json!({"id":version["id"],"fields[appStoreVersionLocalizations]":fields,"limit":200});
        if let Some(locales) = locales {
            query["filter[locale]"] = json!(locales);
        }
        self.pages(
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated",
            query,
        )
        .await
    }
}
pub(super) fn required<'a>(
    args: &'a Value,
    key: &str,
) -> Result<&'a str, mcp_factory_core::ToolResult> {
    args[key]
        .as_str()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| failure("validate", &format!("{key} is required"), Value::Null))
}
pub(super) fn unique(
    mut rows: Vec<Value>,
    resource: &str,
) -> Result<Value, mcp_factory_core::ToolResult> {
    if rows.len() != 1 {
        return Err(failure(
            "lookup",
            &format!(
                "{resource} selection matched {} resources; expected exactly one",
                rows.len()
            ),
            Value::Null,
        ));
    }
    Ok(rows.remove(0))
}
pub(super) fn compact(resource: &Value, fields: &[&str]) -> Value {
    let mut result = json!({"id":resource["id"]});
    for field in fields {
        if let Some(value) = resource["attributes"].get(*field) {
            result[*field] = value.clone();
        }
    }
    result
}
