//! One bounded page of property annotations, with server-side overlap filtering.
use mcp_factory_core::{chrono::NaiveDate, OperationInvoker, ProxyError};
use serde_json::{json, Value};

pub fn schema() -> Value {
    json!({"type":"object","additionalProperties":false,"required":["propertyId"],"properties":{
        "propertyId":{"type":"string","pattern":"^[0-9]+$"},
        "startDate":{"type":"string","format":"date","description":"Inclusive ISO annotation-window start; supply endDate too."},
        "endDate":{"type":"string","format":"date","description":"Inclusive ISO annotation-window end; supply startDate too."},
        "filter":{"type":"string","maxLength":2000,"description":"Optional Google annotation filter, ANDed with the date-window filter."},
        "limit":{"type":"integer","minimum":1,"maximum":200,"default":50},
        "pageToken":{"type":"string","maxLength":4096,"description":"Continue with the same property, dates, filter and limit."}
    }})
}
pub async fn read(
    invoker: &dyn OperationInvoker,
    args: &Value,
    property: &str,
) -> Result<Value, ProxyError> {
    let mut filters = Vec::new();
    let mut window = Value::Null;
    match (args.get("startDate"), args.get("endDate")) {
        (None, None) => {}
        (Some(start), Some(end)) => {
            let date = |v: &Value| {
                v.as_str()
                    .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                    .ok_or_else(|| {
                        ProxyError::Validation("annotation dates must be ISO YYYY-MM-DD".into())
                    })
            };
            let (start, end) = (date(start)?, date(end)?);
            if end < start {
                return Err(ProxyError::Validation(
                    "annotation endDate precedes startDate".into(),
                ));
            }
            filters.push(format!(
                "is_annotation_in_range(\"{start}\", \"{end}\") = true"
            ));
            window = json!({"startDate":start.to_string(),"endDate":end.to_string()});
        }
        _ => {
            return Err(ProxyError::Validation(
                "annotation startDate and endDate must be supplied together".into(),
            ))
        }
    }
    if let Some(filter) = args["filter"].as_str().filter(|s| !s.trim().is_empty()) {
        filters.push(filter.to_string());
    }
    let filter = filters
        .into_iter()
        .map(|f| format!("({f})"))
        .collect::<Vec<_>>()
        .join(" AND ");
    let mut params =
        json!({"parent":property,"pageSize":args["limit"].as_u64().unwrap_or(50).clamp(1,200)});
    if !filter.is_empty() {
        params["filter"] = json!(filter);
    }
    if let Some(token) = args.get("pageToken") {
        params["pageToken"] = token.clone();
    }
    let mut result = super::invoke(
        invoker,
        "analyticsadmin_v1alpha_properties_reportingDataAnnotations_list",
        params,
    )
    .await;
    result["scope"] = json!({"property":property,"annotationDateRange":window,"filter":filter,"note":"One page of property annotations. Continue with nextPageToken and unchanged filters; empty or partial pages do not establish absence of notes."});
    Ok(result)
}
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_factory_core::{async_trait, CustomToolHandler, ToolResult};
    use std::sync::Mutex;
    struct Mock(Mutex<Vec<Value>>);
    #[async_trait]
    impl OperationInvoker for Mock {
        async fn invoke_read_only(
            &self,
            name: &str,
            args: Value,
        ) -> Result<ToolResult, ProxyError> {
            assert_eq!(
                name,
                "analyticsadmin_v1alpha_properties_reportingDataAnnotations_list"
            );
            let op = crate::tools::operation_catalog()
                .iter()
                .find(|o| o.name == name)
                .unwrap();
            let tool = (op.build)();
            assert_eq!(tool.hints.read_only, Some(true));
            jsonschema::validator_for(&tool.input_schema)
                .unwrap()
                .validate(&args)
                .unwrap();
            self.0.lock().unwrap().push(args);
            Ok(ToolResult::text("{}").with_structured(Some(json!({"reportingDataAnnotations":[{"title":"Release","annotationDateRange":{"startDate":{"year":2026,"month":1,"day":1},"endDate":{"year":2026,"month":1,"day":5}}}],"nextPageToken":"next"}))))
        }
    }
    #[tokio::test]
    async fn overlap_filter_and_page_are_forwarded_and_partial_is_preserved() {
        let mock = Mock(Mutex::new(vec![]));
        let report=super::super::Handler("analytics_annotations").call(&mock,json!({"propertyId":"123","startDate":"2026-01-02","endDate":"2026-01-03","filter":"title:Release OR color=BLUE","limit":200,"pageToken":"previous"})).await.unwrap().structured.unwrap();
        assert_eq!(report["status"], "partial");
        assert_eq!(report["sources"][0]["data"]["nextPageToken"], "next");
        assert_eq!(
            report["sources"][0]["data"]["reportingDataAnnotations"][0]["title"],
            "Release"
        );
        let calls = mock.0.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0]["parent"], "properties/123");
        assert_eq!(calls[0]["pageSize"], 200);
        assert_eq!(calls[0]["pageToken"], "previous");
        assert_eq!(calls[0]["filter"],"(is_annotation_in_range(\"2026-01-02\", \"2026-01-03\") = true) AND (title:Release OR color=BLUE)");
    }
    #[tokio::test]
    async fn invalid_or_unpaired_dates_do_not_make_upstream_calls() {
        let mock = Mock(Mutex::new(vec![]));
        for args in [
            json!({"startDate":"2026-01-01"}),
            json!({"endDate":"2026-01-01"}),
            json!({"startDate":"not-a-date","endDate":"2026-01-01"}),
            json!({"startDate":"2026-02-01","endDate":"2026-01-01"}),
        ] {
            assert!(read(&mock, &args, "properties/123").await.is_err());
        }
        assert!(mock.0.lock().unwrap().is_empty());
    }
}
