#[path = "annotations.rs"]
mod annotations;
#[path = "diagnostics.rs"]
mod diagnostics;
#[path = "lifecycle.rs"]
mod lifecycle;
#[path = "scope.rs"]
mod scope;

/// Extract hostname from URL or bare domain
fn extract_hostname(input: &str) -> Option<String> {
    let trimmed = input.trim();
    
    // Handle full URLs
    if let Some(scheme_pos) = trimmed.find("://") {
        let after_scheme = &trimmed[scheme_pos + 3..];
        let hostname = after_scheme.split('/').next()?.split(':').next()?;
        return Some(hostname.to_lowercase());
    }
    
    // Handle www. prefix without scheme
    if trimmed.starts_with("www.") {
        let hostname = trimmed.split('/').next()?.split(':').next()?;
        return Some(hostname.to_lowercase());
    }
    
    // Treat as bare domain if it contains at least one dot and no spaces
    if trimmed.contains('.') && !trimmed.contains(' ') {
        let hostname = trimmed.split('/').next()?.split(':').next()?;
        return Some(hostname.to_lowercase());
    }
    
    None
}

/// Resolve property identifier to numeric ID. Accepts numeric IDs (pass-through),
/// display names, or web domains/URLs. Follows pagination to ensure complete search.
async fn resolve_property_id(
    invoker: &dyn OperationInvoker,
    identifier: &str,
) -> Result<String, ProxyError> {
    // If it's already a numeric ID, return it as-is
    if identifier.bytes().all(|b| b.is_ascii_digit()) && !identifier.is_empty() {
        return Ok(identifier.to_string());
    }

    // Normalize the identifier for comparison
    let normalized_identifier = identifier.to_lowercase();
    let target_hostname = extract_hostname(identifier);
    
    let mut matches = Vec::new();
    let mut next_token: Option<String> = None;
    let mut pages_fetched = 0;
    const MAX_PAGES: usize = 10; // Safety limit

    // Fetch all pages of account summaries
    loop {
        if pages_fetched >= MAX_PAGES {
            return Err(ProxyError::Validation(format!(
                "Property search incomplete after {} pages; use the numeric property ID directly",
                MAX_PAGES
            )));
        }

        let mut params = json!({"pageSize":200});
        if let Some(token) = &next_token {
            params["pageToken"] = json!(token);
        }

        let result = invoke(
            invoker,
            "analyticsadmin_v1beta_accountSummaries_list",
            params,
        )
        .await;

        if result["status"] != "complete" {
            return Err(ProxyError::Validation(format!(
                "Failed to resolve property identifier '{}': {}",
                identifier,
                result["error"].as_str().unwrap_or("unknown error")
            )));
        }

        let summaries = result["data"]["accountSummaries"]
            .as_array()
            .ok_or_else(|| ProxyError::Validation("No account summaries available".into()))?;

        // Search this page for matches by display name
        for account in summaries {
            if let Some(properties) = account["propertySummaries"].as_array() {
                for property in properties {
                    let property_id = property["property"]
                        .as_str()
                        .and_then(|s| s.strip_prefix("properties/"))
                        .unwrap_or("");

                    // Check display name
                    if let Some(display_name) = property["displayName"].as_str() {
                        if display_name.to_lowercase() == normalized_identifier {
                            matches.push((property_id.to_string(), display_name.to_string()));
                        }
                    }
                }
            }
        }

        // Check for more pages
        next_token = result["data"]["nextPageToken"]
            .as_str()
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        
        pages_fetched += 1;
        
        if next_token.is_none() {
            break;
        }
    }

    // If we found matches by name, return early
    if !matches.is_empty() {
        return match matches.len() {
            1 => Ok(matches[0].0.clone()),
            _ => {
                let names: Vec<String> = matches.iter().map(|(id, name)| format!("{} ({})", name, id)).collect();
                Err(ProxyError::Validation(format!(
                    "Property identifier '{}' is ambiguous. Matches: {}. Use the numeric property ID instead.",
                    identifier,
                    names.join(", ")
                )))
            }
        };
    }

    // If we have a hostname to match, search web streams
    if let Some(target_host) = target_hostname {
        // Re-fetch account summaries to search streams (we need property full names)
        let result = invoke(
            invoker,
            "analyticsadmin_v1beta_accountSummaries_list",
            json!({"pageSize":200}),
        )
        .await;

        if let Some(summaries) = result["data"]["accountSummaries"].as_array() {
            for account in summaries {
                if let Some(properties) = account["propertySummaries"].as_array() {
                    for property in properties {
                        let property_id = property["property"]
                            .as_str()
                            .and_then(|s| s.strip_prefix("properties/"))
                            .unwrap_or("");
                        let property_full_name = property["property"].as_str().unwrap_or("");

                        // Fetch data streams for this property
                        let streams_result = invoke(
                            invoker,
                            "analyticsadmin_v1beta_properties_dataStreams_list",
                            json!({"parent":property_full_name,"pageSize":50}),
                        )
                        .await;

                        if let Some(streams) = streams_result["data"]["dataStreams"].as_array() {
                            for stream in streams {
                                if let Some(web_data) = stream["webStreamData"].as_object() {
                                    if let Some(default_uri) = web_data.get("defaultUri").and_then(Value::as_str) {
                                        if let Some(stream_host) = extract_hostname(default_uri) {
                                            // Exact hostname match
                                            if stream_host == target_host {
                                                matches.push((
                                                    property_id.to_string(),
                                                    property["displayName"].as_str().unwrap_or(property_full_name).to_string(),
                                                ));
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    match matches.len() {
        0 => Err(ProxyError::Validation(format!(
            "Property identifier '{}' not found. Use a numeric property ID, exact display name, or web stream URL/domain.",
            identifier
        ))),
        1 => Ok(matches[0].0.clone()),
        _ => {
            let names: Vec<String> = matches.iter().map(|(id, name)| format!("{} ({})", name, id)).collect();
            Err(ProxyError::Validation(format!(
                "Property identifier '{}' is ambiguous. Matches: {}. Use the numeric property ID instead.",
                identifier,
                names.join(", ")
            )))
        }
    }
}
/// Server-owned provider selection; generated startup detects this optional hook.
pub fn build_auth_provider(
    config: &mcp_factory_core::ProxyConfig,
) -> Result<
    Option<std::sync::Arc<dyn mcp_factory_core::auth::AuthProvider>>,
    mcp_factory_core::ProxyError,
> {
    mcp_factory_provider_auth::from_config(config)
}

use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, OperationInvoker, ProxyError, ToolHints,
    ToolResult,
};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn build_custom_tools() -> Vec<CustomToolSpec> {
    let definitions = [
        ("analytics_discover_properties", "List accessible accounts and properties, one bounded page at a time."),
        ("analytics_property_configuration", "Read property identity, streams, key events, retention and linked products. Each source has its own status."),
        ("analytics_annotations", "Read one bounded page of release/campaign annotations. Optional date window uses Google's overlap filter; nextPageToken continues with unchanged filters."),
        ("analytics_metadata", "Search standard and custom report dimensions and metrics. Returns at most 50 matches per kind."),
        ("analytics_validate_report", "Check Core report dimension and metric compatibility before querying."),
        ("analytics_run_report", "Run a Core report with the complete Google REST request body, including cohorts and comparisons."),
        ("analytics_overview", "Traffic and engagement totals or daily trend."),
        ("analytics_acquisition", "Sessions, users and key events by session source and medium."),
        ("analytics_content", "Views and active users by web page path or app screen class."),
        ("analytics_events", "Event counts and users by event name."),
        ("analytics_ecommerce", "Purchased items and item revenue by item name."),
        ("analytics_activation", "Measure ordered user progression through 2-10 named event steps using a closed alpha funnel; not separate event totals."),
        ("analytics_retention", "Track first-session acquisition cohorts over mature daily offsets using user retention metrics; not an arbitrary activation-event cohort."),
        ("analytics_compare", "Compare two date ranges in one report. Supply dateRanges with exactly two ranges."),
        ("analytics_realtime", "Run a realtime report with Google REST request fields."),
        ("analytics_funnel", "Run a funnel report with the complete alpha REST request fields."),
        ("analytics_conversions", "Run an alpha report with conversionSpec and attribution settings."),
        ("analytics_pivot", "Run a pivot report with Google REST pivots and report fields."),
        ("analytics_batch", "Run up to five Core reports for one property in one API request."),
        ("analytics_quota", "Get the alpha quota snapshot without running a report."),
        ("analytics_access_audit", "Run an Admin data access report for a property. Requires appropriate Analytics permissions."),
        ("analytics_change_history", "Search account configuration history using Google REST filters and page tokens.")
    ];
    definitions.into_iter().map(|(name, description)| {
        let mut schema = json!({"type":"object","additionalProperties":false,"properties":{}});
        
        // Add base properties based on tool type
        if name != "analytics_change_history" {
            schema["properties"]["propertyId"] = json!({"type":"string","description":"Numeric property ID, property display name, or web stream URL/domain (e.g. '561397529', 'AnimalMeaning', or 'animalmeaning.com')"});
        }
        if name == "analytics_change_history" {
            schema["properties"]["accountId"] = json!({"type":"string","pattern":"^[0-9]+$"});
        }
        
        // Date parameters only for reporting tools
        let is_reporting_tool = ![
            "analytics_discover_properties",
            "analytics_property_configuration",
            "analytics_metadata",
            "analytics_quota",
            "analytics_change_history",
            "analytics_annotations"
        ].contains(&name);
        
        if is_reporting_tool {
            schema["properties"]["startDate"] = json!({"type":"string","default":"28daysAgo"});
            schema["properties"]["endDate"] = json!({"type":"string","default":"yesterday"});
        }
        
        // Tool-specific properties
        if name == "analytics_metadata" {
            schema["properties"]["query"] = json!({"type":"string","maxLength":200});
        }
        if is_reporting_tool && !["analytics_realtime","analytics_validate_report","analytics_funnel","analytics_conversions","analytics_pivot","analytics_batch","analytics_compare","analytics_access_audit","analytics_run_report","analytics_activation","analytics_retention"].contains(&name) {
            schema["properties"]["daily"] = json!({"type":"boolean","default":false});
        }
        
        schema["properties"]["limit"] = json!({"type":"integer","minimum":1,"maximum":1000,"default":100});
        
        // Add request parameter for tools that accept API request bodies
        // This includes explicit raw tools AND presets that support filters/ordering
        if ["analytics_run_report","analytics_realtime","analytics_validate_report","analytics_funnel","analytics_conversions","analytics_pivot","analytics_batch","analytics_compare","analytics_access_audit","analytics_overview","analytics_acquisition","analytics_content","analytics_events","analytics_ecommerce","analytics_change_history"].contains(&name) {
            schema["properties"]["request"] = json!({"type":"object","description":"Google REST camelCase request fields (e.g. dimensionFilter, orderBys); nested API fields are validated by the generated operation."});
        }
        
        if ["analytics_discover_properties","analytics_annotations","analytics_change_history"].contains(&name) {
            schema["properties"]["pageToken"] = json!({"type":"string"});
        }
        
        if scope::supported(name) {
            schema["properties"]["excludeHostnames"] = json!({"type":"array","items":{"type":"string","minLength":1,"maxLength":255},"maxItems":20,"description":"Hostnames to exclude from reports (e.g. localhost, 127.0.0.1). Combined with other filters via AND."});
        }
        
        schema["properties"]["verbose"] = json!({"type":"boolean","default":false,"description":"Include full propertyQuota and pagination details in responses."});
        if scope::supported(name) {
            schema["properties"]["platform"] = json!({"enum":["Android","iOS","web"],"description":"Explicit event platform; omitted means property-wide, potentially mixing Android, iOS and web."});
            schema["properties"]["streamId"] = json!({"type":"string","pattern":"^[0-9]+$","description":"Numeric GA data stream ID, not Firebase app ID, package name or measurement ID. ANDed with platform and request filters."});
        }
        if name == "analytics_activation" {
            schema["properties"]["steps"] = json!({"type":"array","minItems":2,"maxItems":10,"items":{"type":"object","additionalProperties":false,"required":["eventName"],"properties":{"eventName":{"type":"string","minLength":1,"maxLength":100},"name":{"type":"string","minLength":1,"maxLength":100}}}});
            schema["properties"]["withinSeconds"] = json!({"type":"integer","minimum":1,"maximum":604800,"description":"Maximum elapsed seconds between successive steps; omitted means any elapsed time within the report window."});
            schema["properties"]["directlyFollowed"] = json!({"type":"boolean","default":false});
        }
        if name == "analytics_retention" {
            schema["properties"]["cohortDate"] = json!({"type":"string","format":"date","description":"Acquisition date for firstSessionDate membership (not arbitrary event membership)."});
            schema["properties"]["horizonDays"] = json!({"type":"integer","minimum":1,"maximum":90,"default":7});
        }
        if name == "analytics_change_history" { schema["required"] = json!(["accountId"]); }
        else if name != "analytics_discover_properties" { schema["required"] = json!(["propertyId"]); }
        if ["analytics_run_report","analytics_realtime","analytics_validate_report","analytics_funnel","analytics_conversions","analytics_pivot","analytics_batch","analytics_compare","analytics_access_audit"].contains(&name) {
            schema["required"].as_array_mut().unwrap().push(json!("request"));
        }
        if name == "analytics_activation" { schema["required"].as_array_mut().unwrap().push(json!("steps")); }
        if name == "analytics_retention" { schema["required"].as_array_mut().unwrap().push(json!("cohortDate")); }
        if name == "analytics_annotations" {
            schema = annotations::schema();
            // Override propertyId to accept flexible identifiers
            schema["properties"]["propertyId"] = json!({"type":"string","description":"Numeric property ID, property display name, or web stream URL/domain (e.g. '561397529', 'AnimalMeaning', or 'animalmeaning.com')"});
        }
        CustomToolSpec { name:name.into(), description:description.into(), input_schema:schema,
            hints:ToolHints {read_only:Some(true), destructive:Some(false), idempotent:Some(true), open_world:Some(true), ..Default::default()},
            handler:Arc::new(Handler(name)) }
    }).collect()
}
struct Handler(&'static str);
async fn invoke(invoker: &dyn OperationInvoker, operation: &str, args: Value) -> Value {
    match tokio::time::timeout(
        std::time::Duration::from_secs(15),
        invoker.invoke_read_only(operation, args),
    )
    .await
    {
        Err(_) => {
            json!({"status":"unavailable","operation":operation,"error":"15-second source timeout"})
        }
        Ok(result) => match result {
            Ok(result) if !result.is_error => {
                let value = result
                    .structured
                    .clone()
                    .or_else(|| serde_json::from_str(&result.into_text()).ok())
                    .unwrap_or(Value::Null);
                if !value.is_object() {
                    return json!({"status":"unavailable","operation":operation,"error":"Upstream response was not a JSON object"});
                }
                json!({"status":"complete","operation":operation,"data":value})
            }
            Ok(result) => {
                let upstream = result.structured.clone();
                let text = result.into_text();
                let upstream = upstream
                    .or_else(|| serde_json::from_str(&text).ok())
                    .unwrap_or(Value::Null);
                json!({"status":"unavailable","operation":operation,"error":text,"diagnostic":diagnostics::explain(&upstream)})
            }
            Err(error) => {
                json!({"status":"unavailable","operation":operation,"error":error.to_string()})
            }
        },
    }
}
fn bounded_report(body: &mut Value, limit: u64, verbose: bool) {
    let requested = body["limit"]
        .as_str()
        .and_then(|s| s.parse::<u64>().ok())
        .or_else(|| body["limit"].as_u64())
        .unwrap_or(limit);
    body["limit"] = json!(requested.clamp(1, limit));
    // Discovery int64 fields use strings on the wire.
    body["limit"] = json!(body["limit"].as_u64().unwrap().to_string());
    body["returnPropertyQuota"] = json!(verbose);
}
/// Return true when the report contains only part of its advertised rows.
fn mark_pagination(value: &mut Value, request: &Value) -> bool {
    if let Some(reports) = value.get_mut("reports").and_then(Value::as_array_mut) {
        let mut partial = false;
        for (index, report) in reports.iter_mut().enumerate() {
            partial |= mark_pagination(report, &request["requests"][index]);
        }
        return partial;
    }
    let Some(total) = value["rowCount"].as_u64() else {
        return false;
    };
    let returned = value["rows"].as_array().map_or(0, Vec::len) as u64;
    let offset = request["offset"]
        .as_str()
        .and_then(|s| s.parse().ok())
        .or_else(|| request["offset"].as_u64())
        .unwrap_or(0);
    if request["dateRanges"]
        .as_array()
        .is_some_and(|r| r.len() > 1)
    {
        let index = value["dimensionHeaders"]
            .as_array()
            .and_then(|h| h.iter().position(|h| h["name"] == "dateRange"));
        let mut per_range = std::collections::BTreeMap::<String, u64>::new();
        if let Some(index) = index {
            for row in value["rows"].as_array().into_iter().flatten() {
                if let Some(name) = row["dimensionValues"][index]["value"].as_str() {
                    *per_range.entry(name.into()).or_default() += 1;
                }
            }
        }
        let ranges = request["dateRanges"].as_array().unwrap().len();
        let expanded_complete = total > 0
            && returned > total
            && per_range.len() == ranges
            && per_range.values().all(|count| *count == total);
        value["pagination"] = json!({"complete":if expanded_complete {Some(true)} else {None},"offset":offset,"returnedRows":returned,"upstreamRowCount":total,"returnedRowsByDateRange":per_range,"semantics":"multiple_date_ranges","note":"Date ranges expand physical rows. rowCount is preserved without treating it as a combined total; no global nextOffset is inferred. Use single-range reports for unambiguous pagination."});
        return !expanded_complete;
    }
    let complete = offset.saturating_add(returned) >= total;
    value["pagination"] =
        json!({"complete":complete,"offset":offset,"returnedRows":returned,"totalRows":total});
    if !complete && returned > 0 {
        value["pagination"]["nextOffset"] = json!((offset + returned).to_string());
    }
    !complete
}
fn compact(value: &mut Value, verbose: bool) {
    for key in ["funnelTable", "funnelVisualization"] {
        if let Some(report) = value.get_mut(key) {
            compact(report, verbose);
        }
    }
    if let Some(reports) = value.get_mut("reports").and_then(Value::as_array_mut) {
        for report in reports {
            compact(report, verbose);
        }
    }
    
    // Remove verbose quota information unless explicitly requested
    if !verbose {
        if let Some(obj) = value.as_object_mut() {
            obj.remove("propertyQuota");
            obj.remove("kind");
        }
    }
    
    let headers: Vec<String> = value["dimensionHeaders"]
        .as_array()
        .into_iter()
        .flatten()
        .chain(value["metricHeaders"].as_array().into_iter().flatten())
        .filter_map(|v| v["name"].as_str().map(str::to_string))
        .collect();
    if !headers.is_empty() {
        if let Some(rows) = value.get_mut("rows").and_then(Value::as_array_mut) {
            for row in rows {
                let cells: Vec<Value> = row["dimensionValues"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .chain(row["metricValues"].as_array().into_iter().flatten())
                    .map(|v| v["value"].clone())
                    .collect();
                *row = Value::Array(cells);
            }
        }
        value["columns"] = json!(headers);
        // Keep typed headers and response metadata; never hide sampling or threshold warnings.
    }
}
#[async_trait]
impl CustomToolHandler for Handler {
    async fn call(
        &self,
        invoker: &dyn OperationInvoker,
        mut args: Value,
    ) -> Result<ToolResult, ProxyError> {
        // Resolve property identifier to numeric ID if needed
        if let Some(property_id_value) = args.get("propertyId") {
            let identifier = property_id_value.as_str().unwrap_or_default();
            if !identifier.is_empty() {
                let resolved_id = resolve_property_id(invoker, identifier).await?;
                args["propertyId"] = json!(resolved_id);
            }
        }
        
        let property = format!(
            "properties/{}",
            args["propertyId"].as_str().unwrap_or_default()
        );
        let limit = args["limit"].as_u64().unwrap_or(100);
        let verbose = args["verbose"].as_bool().unwrap_or(false);
        let mut body = args.get("request").cloned().unwrap_or(json!({}));
        if !body.is_object() {
            return Err(ProxyError::Validation("request must be an object".into()));
        }
        if !scope::supported(self.0)
            && (args.get("platform").is_some() || args.get("streamId").is_some())
        {
            return Err(ProxyError::Validation(
                "platform/stream selectors are not supported for this workflow".into(),
            ));
        }
        lifecycle::prepare(self.0, &args, &mut body)?;
        let data = "analyticsdata_v1beta_properties_";
        let mut sources = Vec::new();
        match self.0 {
            "analytics_annotations" => {
                sources.push(annotations::read(invoker, &args, &property).await?)
            }
            "analytics_discover_properties" => {
                let mut params = json!({"pageSize":limit.min(200)});
                if let Some(token) = args.get("pageToken") {
                    params["pageToken"] = token.clone();
                }
                sources.push(
                    invoke(
                        invoker,
                        "analyticsadmin_v1beta_accountSummaries_list",
                        params,
                    )
                    .await,
                );
            }
            "analytics_property_configuration" => {
                for (operation, params) in [
                    (
                        "analyticsadmin_v1beta_properties_get",
                        json!({"name":property}),
                    ),
                    (
                        "analyticsadmin_v1beta_properties_dataStreams_list",
                        json!({"parent":property,"pageSize":50}),
                    ),
                    (
                        "analyticsadmin_v1beta_properties_keyEvents_list",
                        json!({"parent":property,"pageSize":50}),
                    ),
                    (
                        "analyticsadmin_v1beta_properties_getDataRetentionSettings",
                        json!({"name":format!("{property}/dataRetentionSettings")}),
                    ),
                    (
                        "analyticsadmin_v1beta_properties_googleAdsLinks_list",
                        json!({"parent":property,"pageSize":50}),
                    ),
                    (
                        "analyticsadmin_v1beta_properties_firebaseLinks_list",
                        json!({"parent":property,"pageSize":50}),
                    ),
                    (
                        "analyticsadmin_v1alpha_properties_bigQueryLinks_list",
                        json!({"parent":property,"pageSize":50}),
                    ),
                ] {
                    sources.push(invoke(invoker, operation, params).await);
                }
            }
            "analytics_metadata" => {
                let mut result = invoke(
                    invoker,
                    &format!("{data}getMetadata"),
                    json!({"name":format!("{property}/metadata")}),
                )
                .await;
                let query = args["query"].as_str().unwrap_or("").to_lowercase();
                for key in ["dimensions", "metrics"] {
                    if let Some(items) = result["data"][key].as_array() {
                        let matches: Vec<Value> = items
                            .iter()
                            .filter(|v| v.to_string().to_lowercase().contains(&query))
                            .cloned()
                            .collect();
                        result["data"][format!("{key}MatchCount")] = json!(matches.len());
                        result["data"][key] =
                            json!(matches.into_iter().take(50).collect::<Vec<_>>());
                    }
                }
                sources.push(result);
            }
            "analytics_quota" => sources.push(
                invoke(
                    invoker,
                    "analyticsdata_v1alpha_properties_getPropertyQuotasSnapshot",
                    json!({"name":format!("{property}/propertyQuotasSnapshot")}),
                )
                .await,
            ),
            "analytics_change_history" => {
                if let Some(token) = args.get("pageToken") {
                    body["pageToken"] = token.clone();
                }
                body["pageSize"] = json!(limit.min(100));
                sources.push(invoke(invoker,"analyticsadmin_v1beta_accounts_searchChangeHistoryEvents",json!({"account":format!("accounts/{}",args["accountId"].as_str().unwrap()),"body":body})).await);
            }
            _ => {
                let operation = match self.0 {
                    "analytics_validate_report" => format!("{data}checkCompatibility"),
                    "analytics_realtime" => format!("{data}runRealtimeReport"),
                    "analytics_funnel" | "analytics_activation" => {
                        "analyticsdata_v1alpha_properties_runFunnelReport".into()
                    }
                    "analytics_conversions" => "analyticsdata_v1alpha_properties_runReport".into(),
                    "analytics_pivot" => format!("{data}runPivotReport"),
                    "analytics_batch" => format!("{data}batchRunReports"),
                    "analytics_access_audit" => {
                        "analyticsadmin_v1beta_properties_runAccessReport".into()
                    }
                    _ => format!("{data}runReport"),
                };
                if self.0 == "analytics_compare"
                    && body["dateRanges"].as_array().map(Vec::len) != Some(2)
                {
                    return Err(ProxyError::Validation(
                        "compare requires exactly two dateRanges".into(),
                    ));
                }
                if self.0 == "analytics_conversions" && !body["conversionSpec"].is_object() {
                    return Err(ProxyError::Validation(
                        "conversions requires conversionSpec".into(),
                    ));
                }
                if self.0 == "analytics_batch" {
                    let requests = body["requests"]
                        .as_array_mut()
                        .ok_or_else(|| ProxyError::Validation("batch requires requests".into()))?;
                    if requests.is_empty() || requests.len() > 5 {
                        return Err(ProxyError::Validation(
                            "batch requires 1 to 5 requests".into(),
                        ));
                    }
                    if requests.iter().any(|request| !request.is_object()) {
                        return Err(ProxyError::Validation(
                            "every batch request must be an object".into(),
                        ));
                    }
                    for request in requests {
                        bounded_report(request, limit, verbose);
                    }
                } else if ![
                    "analytics_validate_report",
                    "analytics_funnel",
                    "analytics_pivot",
                ]
                .contains(&self.0)
                {
                    bounded_report(&mut body, limit, verbose);
                    if self.0 == "analytics_access_audit" {
                        body.as_object_mut().unwrap().remove("returnPropertyQuota");
                        body["returnEntityQuota"] = json!(verbose);
                    }
                } else if ["analytics_funnel", "analytics_pivot"].contains(&self.0) {
                    // Funnels and pivots skip bounded_report but still need quota flag
                    body["returnPropertyQuota"] = json!(verbose);
                }
                let preset = match self.0 {
                    "analytics_overview" => Some((
                        vec![],
                        vec![
                            "activeUsers",
                            "newUsers",
                            "sessions",
                            "engagedSessions",
                            "engagementRate",
                            "averageSessionDuration",
                            "screenPageViews",
                            "screenPageViewsPerSession",
                            "bounceRate",
                            "totalRevenue",
                        ],
                    )),
                    "analytics_acquisition" => Some((
                        vec!["sessionSourceMedium"],
                        vec!["sessions", "activeUsers", "keyEvents"],
                    )),
                    "analytics_content" => Some((
                        vec!["unifiedPagePathScreen"],
                        vec!["screenPageViews", "activeUsers"],
                    )),
                    "analytics_events" => {
                        Some((vec!["eventName"], vec!["eventCount", "totalUsers"]))
                    }
                    "analytics_ecommerce" => {
                        Some((vec!["itemName"], vec!["itemsPurchased", "itemRevenue"]))
                    }
                    _ => None,
                };
                if let Some((mut dimensions, metrics)) = preset {
                    if args["daily"].as_bool() == Some(true) {
                        dimensions.push("date");
                    }
                    body["dimensions"] = json!(dimensions
                        .into_iter()
                        .map(|name| json!({"name":name}))
                        .collect::<Vec<_>>());
                    body["metrics"] = json!(metrics
                        .into_iter()
                        .map(|name| json!({"name":name}))
                        .collect::<Vec<_>>());
                }
                if ![
                    "analytics_validate_report",
                    "analytics_realtime",
                    "analytics_batch",
                ]
                .contains(&self.0)
                    && body.get("cohortSpec").is_none()
                    && body.get("dateRanges").is_none()
                {
                    body["dateRanges"] = json!([{"startDate":args["startDate"].as_str().unwrap_or("28daysAgo"),"endDate":args["endDate"].as_str().unwrap_or("yesterday")}]);
                }
                if scope::supported(self.0) {
                    if self.0 == "analytics_batch" {
                        for request in body["requests"].as_array_mut().unwrap() {
                            scope::apply(request, &args)?;
                        }
                    } else {
                        scope::apply(&mut body, &args)?;
                    }
                }
                let request_body = body.clone();
                let params = if self.0 == "analytics_access_audit" {
                    json!({"entity":property,"body":body})
                } else {
                    json!({"property":property,"body":body})
                };
                let mut result = invoke(invoker, &operation, params).await;
                if mark_pagination(&mut result["data"], &request_body) {
                    result["status"] = json!("partial");
                }
                result["scope"] = json!({"dimensionFilter":request_body["dimensionFilter"],"batchScopes":request_body["requests"].as_array().map(|requests| requests.iter().enumerate().map(|(index, request)| json!({"reportIndex":index,"dimensionFilter":request["dimensionFilter"],"dateRanges":request["dateRanges"],"cohortSpec":request["cohortSpec"]})).collect::<Vec<_>>()),"dateRanges":request_body["dateRanges"],"cohortSpec":request_body["cohortSpec"]});
                compact(&mut result["data"], verbose);
                sources.push(result);
            }
        }
        let mut used_bytes = 0;
        for source in &mut sources {
            if source["data"]["nextPageToken"]
                .as_str()
                .is_some_and(|v| !v.is_empty())
            {
                source["status"] = json!("partial");
                source["paginationComplete"] = json!(false);
            }
            if let Some(error) = source["error"].as_str() {
                if error.len() > 4096 {
                    source["error"] = json!(error.chars().take(1000).collect::<String>());
                    source["errorTruncated"] = json!(true);
                }
            }
            let size = source.to_string().len();
            if source["data"].to_string().len() > 48_000 || used_bytes + size > 60_000 {
                source["data"] = json!({"omitted":true,"reason":"Response exceeds the context budget; narrow the request or use the raw profile."});
                if source["status"] != "unavailable" {
                    source["status"] = json!("partial");
                }
            }
            if used_bytes + source.to_string().len() > 60_000 {
                let operation = source["operation"].clone();
                let status = if source["status"] == "unavailable" {
                    "unavailable"
                } else {
                    "partial"
                };
                *source = json!({"status":status,"operation":operation,"outputOmitted":true,"error":"Source output omitted to keep the response context bounded"});
            }
            used_bytes += source.to_string().len();
        }
        let successes = sources
            .iter()
            .filter(|v| v["status"] != "unavailable")
            .count();
        let report = json!({"scope":scope::label(self.0, &args),"interpretation":lifecycle::interpretation(self.0),"status":if sources.iter().all(|v|v["status"]=="complete"){"complete"}else if successes==0{"unavailable"}else{"partial"},"sources":sources,"pagination":"Report limits bound returned rows; inspect rowCount and offset. List responses retain nextPageToken; configuration lists are first-page snapshots."});
        Ok(ToolResult::text(serde_json::to_string(&report).unwrap()).with_structured(Some(report)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;
    struct Mock(Mutex<Vec<(String, Value)>>);
    #[async_trait]
    impl OperationInvoker for Mock {
        async fn invoke_read_only(
            &self,
            name: &str,
            args: Value,
        ) -> Result<ToolResult, ProxyError> {
            let descriptor = crate::tools::operation_catalog()
                .iter()
                .find(|o| o.name == name)
                .expect("generated dependency");
            let tool = (descriptor.build)();
            assert_eq!(tool.hints.read_only, Some(true));
            jsonschema::validator_for(&tool.input_schema)
                .unwrap()
                .validate(&args)
                .unwrap();
            self.0.lock().unwrap().push((name.into(), args));
            Ok(ToolResult::text("{}").with_structured(Some(json!({"dimensionHeaders":[{"name":"eventName"}],"metricHeaders":[{"name":"eventCount","type":"TYPE_INTEGER"}],"rows":[{"dimensionValues":[{"value":"purchase"}],"metricValues":[{"value":"3"}]}],"rowCount":8,"metadata":{"subjectToThresholding":true}}))))
        }
    }
    #[tokio::test]
    async fn presets_bound_requests_and_preserve_evidence() {
        let mock = Mock(Mutex::new(vec![]));
        let result = Handler("analytics_events")
            .call(&mock, json!({"propertyId":"123","limit":20}))
            .await
            .unwrap()
            .structured
            .unwrap();
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[0].0, "analyticsdata_v1beta_properties_runReport");
        assert_eq!(calls[0].1["property"], "properties/123");
        assert_eq!(calls[0].1["body"]["limit"], "20");
        assert_eq!(
            result["sources"][0]["data"]["rows"],
            json!([["purchase", "3"]])
        );
        assert_eq!(
            result["sources"][0]["data"]["metadata"]["subjectToThresholding"],
            true
        );
    }
    #[tokio::test]
    async fn reject_invalid_batch_before_upstream() {
        let mock = Mock(Mutex::new(vec![]));
        assert!(Handler("analytics_batch")
            .call(
                &mock,
                json!({"propertyId":"123","request":{"requests":[{},{},{},{},{},{}]}})
            )
            .await
            .is_err());
        assert!(mock.0.lock().unwrap().is_empty());
    }
    #[tokio::test]
    async fn full_and_realtime_reports_cannot_bypass_row_limit() {
        for name in ["analytics_run_report", "analytics_realtime"] {
            let mock = Mock(Mutex::new(vec![]));
            let result = Handler(name)
                .call(
                    &mock,
                    json!({"propertyId":"123","limit":10,"request":{"limit":"250000"}}),
                )
                .await
                .unwrap()
                .structured
                .unwrap();
            let calls = mock.0.lock().unwrap();
            assert_eq!(calls[0].1["body"]["limit"], "10");
            assert_eq!(result["status"], "partial");
            assert_eq!(
                result["sources"][0]["data"]["pagination"]["nextOffset"],
                "1"
            );
        }
    }
    #[tokio::test]
    async fn access_audit_uses_admin_quota_field() {
        let mock = Mock(Mutex::new(vec![]));
        Handler("analytics_access_audit")
            .call(&mock, json!({"propertyId":"123","request":{},"verbose":true}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[0].1["entity"], "properties/123");
        assert_eq!(calls[0].1["body"]["returnEntityQuota"], true);
        assert!(calls[0].1["body"].get("returnPropertyQuota").is_none());
        
        // Test that verbose=false omits quota fields
        let mock2 = Mock(Mutex::new(vec![]));
        Handler("analytics_access_audit")
            .call(&mock2, json!({"propertyId":"123","request":{},"verbose":false}))
            .await
            .unwrap();
        let calls2 = mock2.0.lock().unwrap();
        assert_eq!(calls2[0].1["body"]["returnEntityQuota"], false);
    }
    #[test]
    fn batch_pagination_uses_each_reports_offset() {
        let mut response =
            json!({"reports":[{"rowCount":5,"rows":[{}]},{"rowCount":5,"rows":[{}]}]});
        assert!(mark_pagination(
            &mut response,
            &json!({"requests":[{"offset":"4"},{"offset":"1"}]})
        ));
        assert_eq!(response["reports"][0]["pagination"]["complete"], true);
        assert_eq!(response["reports"][1]["pagination"]["nextOffset"], "2");
    }
    #[tokio::test]
    async fn malformed_batch_entries_fail_before_invocation() {
        for entry in [json!(null), json!(true), json!(3), json!("text"), json!([])] {
            let mock = Mock(Mutex::new(vec![]));
            assert!(matches!(
                Handler("analytics_batch")
                    .call(
                        &mock,
                        json!({"propertyId":"123","request":{"requests":[entry]}})
                    )
                    .await,
                Err(ProxyError::Validation(_))
            ));
            assert!(mock.0.lock().unwrap().is_empty());
        }
    }
    struct Oversized(bool);
    #[async_trait]
    impl OperationInvoker for Oversized {
        async fn invoke_read_only(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
            if self.0 {
                let mut result = ToolResult::text("🙂".repeat(40000));
                result.is_error = true;
                Ok(result)
            } else {
                Ok(
                    ToolResult::text("{}")
                        .with_structured(Some(json!({"large":"a".repeat(40000)}))),
                )
            }
        }
    }
    #[tokio::test]
    async fn output_budget_covers_all_sources_and_errors() {
        for error in [false, true] {
            let result = Handler("analytics_property_configuration")
                .call(&Oversized(error), json!({"propertyId":"123"}))
                .await
                .unwrap();
            let text = result.into_text();
            assert!(text.len() < 64000, "{}", text.len());
            let parsed: Value = serde_json::from_str(&text).unwrap();
            assert_eq!(
                parsed["status"],
                if error { "unavailable" } else { "partial" }
            );
        }
    }
    struct Mixed(std::sync::atomic::AtomicUsize);
    #[async_trait]
    impl OperationInvoker for Mixed {
        async fn invoke_read_only(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
            if self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed) == 0 {
                Ok(
                    ToolResult::text("{}")
                        .with_structured(Some(json!({"large":"a".repeat(45000)}))),
                )
            } else {
                let mut result = ToolResult::text("🙂".repeat(40000));
                result.is_error = true;
                Ok(result)
            }
        }
    }
    #[tokio::test]
    async fn mixed_data_and_error_results_stay_within_context_budget() {
        let result = Handler("analytics_property_configuration")
            .call(
                &Mixed(std::sync::atomic::AtomicUsize::new(0)),
                json!({"propertyId":"123"}),
            )
            .await
            .unwrap()
            .into_text();
        assert!(result.len() < 64000, "{}", result.len());
    }
    struct NonJson;
    #[async_trait]
    impl OperationInvoker for NonJson {
        async fn invoke_read_only(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
            Ok(ToolResult::text("<html>sensitive-upstream-error</html>"))
        }
    }
    #[tokio::test]
    async fn invalid_upstream_content_is_not_successful_evidence() {
        let text = Handler("analytics_events")
            .call(&NonJson, json!({"propertyId":"123"}))
            .await
            .unwrap()
            .into_text();
        let parsed: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["status"], "unavailable");
        assert!(!text.contains("sensitive-upstream-error"));
    }
    #[test]
    fn workflow_schemas_are_small_and_valid() {
        let tools = build_custom_tools();
        assert_eq!(tools.len(), 22);
        for tool in &tools {
            jsonschema::validator_for(&tool.input_schema).unwrap();
        }
    }
}

#[cfg(test)]
mod contracts {
    use super::*;
    use mcp_factory_core::{McpProxyServer, ProxyConfig};
    #[test]
    fn public_profile_budget_and_internal_coverage() {
        let build = |profile: &str| {
            McpProxyServer::builder(ProxyConfig {
                profiles: Some(vec![profile.into()]),
                ..Default::default()
            })
            .tool_profiles(&crate::tools::build_tool_profiles())
            .unwrap()
            .operation_catalog(crate::tools::operation_catalog())
            .unwrap()
            .custom_tools(&build_custom_tools())
            .unwrap()
            .build()
            .unwrap()
        };
        let workflow = build("workflow");
        let raw = build("raw");
        let compact = serde_json::to_vec(&json!({"tools":workflow.tool_metadata()}))
            .unwrap()
            .len();
        let full = serde_json::to_vec(&json!({"tools":raw.tool_metadata()}))
            .unwrap()
            .len();
        println!(
            "Analytics workflow: {} tools, {} bytes; raw: {} tools, {} bytes",
            workflow.tool_names().len(),
            compact,
            raw.tool_names().len(),
            full
        );
        assert_eq!(workflow.tool_names().len(), 22);
        assert_eq!(raw.tool_names().len(), 248);
        // Allow for slightly larger size due to new features (excludeHostnames, verbose, property resolution, request field on presets)
        assert!(compact < 29_000, "Workflow metadata size {} exceeds 29KB budget", compact);
        assert!(compact * 4 < full);
        for name in [
            "analyticsdata_v1beta_properties_runReport",
            "analyticsdata_v1beta_properties_checkCompatibility",
            "analyticsadmin_v1beta_accounts_searchChangeHistoryEvents",
        ] {
            let op = crate::tools::operation_catalog()
                .iter()
                .find(|o| o.name == name)
                .unwrap();
            assert_eq!((op.build)().hints.read_only, Some(true));
        }
    }
}

#[cfg(test)]
mod generated_contracts {
    use super::*;
    struct Validator;
    #[async_trait]
    impl OperationInvoker for Validator {
        async fn invoke_read_only(
            &self,
            name: &str,
            args: Value,
        ) -> Result<ToolResult, ProxyError> {
            let descriptor = crate::tools::operation_catalog()
                .iter()
                .find(|o| o.name == name)
                .expect("generated dependency exists");
            let operation = (descriptor.build)();
            assert_eq!(operation.hints.read_only, Some(true), "{name}");
            jsonschema::validator_for(&operation.input_schema)
                .unwrap()
                .validate(&args)
                .unwrap_or_else(|e| panic!("{name}: {e}"));
            Ok(ToolResult::text("{}").with_structured(Some(json!({}))))
        }
    }
    #[tokio::test]
    async fn every_workflow_routes_valid_inputs_to_read_only_operations() {
        for tool in build_custom_tools() {
            let mut args = json!({"propertyId":"123","accountId":"456","request":{}});
            if tool.name == "analytics_activation" {
                args["steps"] = json!([{"eventName":"first_open"},{"eventName":"core_action"}]);
            }
            if tool.name == "analytics_retention" {
                args["cohortDate"] = json!("2025-01-01");
            }
            if tool.name == "analytics_compare" {
                args["request"]["dateRanges"] = json!([{"startDate":"14daysAgo","endDate":"8daysAgo"},{"startDate":"7daysAgo","endDate":"yesterday"}]);
            }
            if tool.name == "analytics_batch" {
                args["request"]["requests"] = json!([{}]);
            }
            if tool.name == "analytics_conversions" {
                args["request"]["conversionSpec"] = json!({});
            }
            let report = tool.handler.call(&Validator, args).await.unwrap();
            assert_eq!(
                report.structured.unwrap()["status"],
                "complete",
                "{}",
                tool.name
            );
        }
    }
}

#[cfg(test)]
#[path = "feedback_tests.rs"]
mod feedback_tests;

#[cfg(test)]
mod new_features_tests {
    use super::*;
    use std::sync::Mutex;

    struct Mock(Mutex<Vec<(String, Value)>>);
    #[async_trait]
    impl OperationInvoker for Mock {
        async fn invoke_read_only(
            &self,
            name: &str,
            args: Value,
        ) -> Result<ToolResult, ProxyError> {
            self.0.lock().unwrap().push((name.into(), args));
            // Return mock responses based on operation type
            let response = match name {
                "analyticsadmin_v1beta_accountSummaries_list" => {
                    json!({
                        "accountSummaries": [{
                            "propertySummaries": [
                                {
                                    "property": "properties/123456",
                                    "displayName": "TestProperty"
                                },
                                {
                                    "property": "properties/789012",
                                    "displayName": "AnotherProperty"
                                }
                            ]
                        }],
                        "nextPageToken": ""
                    })
                }
                "analyticsadmin_v1beta_properties_dataStreams_list" => {
                    json!({
                        "dataStreams": [{
                            "webStreamData": {
                                "defaultUri": "https://example.com"
                            }
                        }]
                    })
                }
                _ => json!({"dimensionHeaders": [], "metricHeaders": [], "rows": []})
            };
            Ok(ToolResult::text("{}").with_structured(Some(response)))
        }
    }

    #[tokio::test]
    async fn overview_respects_10_metric_limit() {
        let mock = Mock(Mutex::new(vec![]));
        let result = Handler("analytics_overview")
            .call(&mock, json!({"propertyId":"123"}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        let metrics = calls[0].1["body"]["metrics"].as_array().unwrap();
        assert_eq!(metrics.len(), 10, "Overview must have exactly 10 metrics to comply with API limit");
    }

    #[tokio::test]
    async fn hostname_exclusion_validation() {
        let mock = Mock(Mutex::new(vec![]));
        
        // Valid exclusion
        let _result = Handler("analytics_overview")
            .call(&mock, json!({
                "propertyId":"123",
                "excludeHostnames":["localhost","127.0.0.1"]
            }))
            .await;
        assert!(_result.is_ok());
        
        let calls = mock.0.lock().unwrap();
        let filter = &calls[0].1["body"]["dimensionFilter"];
        assert!(filter["andGroup"]["expressions"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn hostname_exclusion_rejects_invalid_entries() {
        // Schema validation will reject these, but scope::apply also validates
        use crate::extensions::scope;
        
        // Empty hostname
        let result = scope::apply(
            &mut json!({}),
            &json!({"excludeHostnames": [""]}),
        );
        assert!(result.is_err(), "Empty hostnames should be rejected");
        
        // Overlong hostname
        let long_host = "a".repeat(256);
        let result = scope::apply(
            &mut json!({}),
            &json!({"excludeHostnames": [long_host]}),
        );
        assert!(result.is_err(), "Hostnames >255 chars should be rejected");
    }

    #[tokio::test]
    async fn property_resolution_accepts_numeric_id() {
        let mock = Mock(Mutex::new(vec![]));
        let result = Handler("analytics_overview")
            .call(&mock, json!({"propertyId":"123456"}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[0].1["property"], "properties/123456");
    }

    #[tokio::test]
    async fn property_resolution_by_display_name() {
        let mock = Mock(Mutex::new(vec![]));
        let _result = Handler("analytics_overview")
            .call(&mock, json!({"propertyId":"TestProperty"}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        // Should resolve TestProperty to 123456
        assert_eq!(calls[1].1["property"], "properties/123456");
    }

    #[test]
    fn hostname_extraction_handles_various_formats() {
        assert_eq!(extract_hostname("https://example.com"), Some("example.com".into()));
        assert_eq!(extract_hostname("http://example.com"), Some("example.com".into()));
        assert_eq!(extract_hostname("example.com"), Some("example.com".into()));
        assert_eq!(extract_hostname("www.example.com"), Some("www.example.com".into()));
        assert_eq!(extract_hostname("https://example.com/path"), Some("example.com".into()));
        assert_eq!(extract_hostname("https://example.com:8080"), Some("example.com".into()));
        assert_eq!(extract_hostname("not a url"), None);
    }

    #[tokio::test]
    async fn presets_accept_request_field() {
        for tool_name in ["analytics_overview", "analytics_acquisition", "analytics_content", "analytics_events", "analytics_ecommerce"] {
            let mock = Mock(Mutex::new(vec![]));
            let _result = Handler(tool_name)
                .call(&mock, json!({
                    "propertyId":"123",
                    "request":{"orderBys":[{"metric":{"metricName":"sessions"},"desc":true}]}
                }))
                .await;
            assert!(_result.is_ok(), "{} should accept request field", tool_name);
        }
    }

    #[tokio::test]
    async fn change_history_accepts_request_field() {
        let mock = Mock(Mutex::new(vec![]));
        let _result = Handler("analytics_change_history")
            .call(&mock, json!({
                "accountId":"123",
                "request":{"resourceType":"ACCOUNT"}
            }))
            .await;
        assert!(_result.is_ok(), "change_history should accept request field");
    }

    #[tokio::test]
    async fn verbose_flag_controls_quota_request() {
        let mock = Mock(Mutex::new(vec![]));
        
        // verbose=false (default)
        Handler("analytics_overview")
            .call(&mock, json!({"propertyId":"123","verbose":false}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[0].1["body"]["returnPropertyQuota"], false);
        
        drop(calls);
        
        // verbose=true
        Handler("analytics_overview")
            .call(&mock, json!({"propertyId":"123","verbose":true}))
            .await
            .unwrap();
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[1].1["body"]["returnPropertyQuota"], true);
    }

    #[tokio::test]
    async fn funnel_and_pivot_respect_verbose_flag() {
        let mock = Mock(Mutex::new(vec![]));
        
        Handler("analytics_funnel")
            .call(&mock, json!({
                "propertyId":"123",
                "verbose":true,
                "request":{}
            }))
            .await
            .unwrap();
        
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls[0].1["body"]["returnPropertyQuota"], true, "funnel should set returnPropertyQuota when verbose");
    }

    #[tokio::test]
    async fn annotations_accepts_flexible_property_id() {
        let mock = Mock(Mutex::new(vec![]));
        
        // Should resolve property name to ID
        let _result = Handler("analytics_annotations")
            .call(&mock, json!({"propertyId":"TestProperty"}))
            .await;
        assert!(_result.is_ok(), "annotations should accept property names");
    }

    #[test]
    fn schema_exposes_request_for_appropriate_tools() {
        let tools = build_custom_tools();
        
        let overview = tools.iter().find(|t| t.name == "analytics_overview").unwrap();
        assert!(overview.input_schema["properties"]["request"].is_object(), "overview should expose request field");
        
        let change_history = tools.iter().find(|t| t.name == "analytics_change_history").unwrap();
        assert!(change_history.input_schema["properties"]["request"].is_object(), "change_history should expose request field");
        
        let discover = tools.iter().find(|t| t.name == "analytics_discover_properties").unwrap();
        assert!(!discover.input_schema["properties"]["request"].is_object(), "discover_properties should NOT expose request field");
    }

    #[test]
    fn schema_enforces_hostname_limits() {
        let tools = build_custom_tools();
        let overview = tools.iter().find(|t| t.name == "analytics_overview").unwrap();
        let exclude_schema = &overview.input_schema["properties"]["excludeHostnames"]["items"];
        
        assert_eq!(exclude_schema["minLength"], 1, "hostname minLength should be 1");
        assert_eq!(exclude_schema["maxLength"], 255, "hostname maxLength should be 255");
    }
}
