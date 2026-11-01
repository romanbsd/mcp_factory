use super::*;
use std::sync::Mutex;
struct Capture(Mutex<Vec<(String, Value)>>);
#[async_trait]
impl OperationInvoker for Capture {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        let spec = crate::tools::operation_catalog()
            .iter()
            .find(|o| o.name == name)
            .unwrap();
        jsonschema::validator_for(&(spec.build)().input_schema)
            .unwrap()
            .validate(&args)
            .unwrap();
        self.0.lock().unwrap().push((name.into(), args));
        Ok(ToolResult::text("{}").with_structured(Some(json!({}))))
    }
}
#[tokio::test]
async fn scoped_presets_and_batch_preserve_user_filters() {
    for name in [
        "analytics_overview",
        "analytics_acquisition",
        "analytics_content",
        "analytics_events",
        "analytics_ecommerce",
        "analytics_compare",
        "analytics_batch",
    ] {
        let mock = Capture(Mutex::new(vec![]));
        let original = json!({"filter":{"fieldName":"country","stringFilter":{"value":"Israel","matchType":"EXACT"}}});
        let mut request = json!({"dimensionFilter":original});
        if name == "analytics_compare" {
            request["dateRanges"] = json!([{"startDate":"14daysAgo","endDate":"8daysAgo"},{"startDate":"7daysAgo","endDate":"yesterday"}]);
        }
        if name == "analytics_batch" {
            request = json!({"requests":[request.clone(),request]});
        }
        let result = Handler(name)
            .call(
                &mock,
                json!({"propertyId":"123","platform":"Android","streamId":"456","request":request}),
            )
            .await
            .unwrap()
            .structured
            .unwrap();
        assert_eq!(result["scope"]["kind"], "selected_event_data");
        let calls = mock.0.lock().unwrap();
        let body = if name == "analytics_batch" {
            &calls[0].1["body"]["requests"][1]
        } else {
            &calls[0].1["body"]
        };
        let filters = body["dimensionFilter"]["andGroup"]["expressions"]
            .as_array()
            .unwrap();
        assert_eq!(filters.len(), 3);
        assert_eq!(filters[0]["filter"]["stringFilter"]["value"], "Android");
        assert_eq!(filters[1]["filter"]["fieldName"], "streamId");
        assert_eq!(filters[2], original);
        if name == "analytics_content" {
            assert_eq!(body["dimensions"][0]["name"], "unifiedPagePathScreen");
        }
    }
    let mock = Capture(Mutex::new(vec![]));
    let result = Handler("analytics_overview")
        .call(&mock, json!({"propertyId":"123"}))
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(result["scope"]["kind"], "property_wide");
    let realtime = Handler("analytics_realtime")
        .call(
            &mock,
            json!({"propertyId":"123","request":{"metrics":[{"name":"activeUsers"}]}}),
        )
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(realtime["scope"]["kind"], "property_wide");
    assert!(Handler("analytics_overview")
        .call(&mock, json!({"propertyId":"123","streamId":"G-ABC"}))
        .await
        .is_err());
    assert!(Handler("analytics_realtime")
        .call(&mock, json!({"propertyId":"123","platform":"Android"}))
        .await
        .is_err());
}
#[tokio::test]
async fn lifecycle_contracts_use_user_funnels_and_mature_first_session_cohorts() {
    let mock = Capture(Mutex::new(vec![]));
    let result=Handler("analytics_activation").call(&mock,json!({"propertyId":"123","platform":"Android","streamId":"456","steps":[{"eventName":"first_open"},{"eventName":"onboarding_complete"},{"eventName":"core_action"}],"withinSeconds":3600})).await.unwrap().structured.unwrap();
    assert_eq!(
        result["interpretation"]["kind"],
        "ordered_closed_user_funnel"
    );
    {
        let calls = mock.0.lock().unwrap();
        let body = &calls[0].1["body"];
        assert!(calls[0].0.ends_with("runFunnelReport"));
        assert_eq!(body["funnel"]["isOpenFunnel"], false);
        assert_eq!(
            body["funnel"]["steps"][1]["withinDurationFromPriorStep"],
            "3600s"
        );
        assert_eq!(
            body["funnel"]["steps"][2]["filterExpression"]["funnelEventFilter"]["eventName"],
            "core_action"
        );
    }
    Handler("analytics_retention").call(&mock,json!({"propertyId":"123","cohortDate":"2025-01-01","horizonDays":7,"streamId":"456","request":{"dateRanges":[{"startDate":"yesterday","endDate":"today"}]}})).await.unwrap();
    {
        let calls = mock.0.lock().unwrap();
        let body = &calls[1].1["body"];
        assert!(body.get("dateRanges").is_none());
        assert_eq!(
            body["cohortSpec"]["cohorts"][0]["dimension"],
            "firstSessionDate"
        );
        assert_eq!(body["cohortSpec"]["cohortsRange"]["endOffset"], 7);
        assert_eq!(
            body["metrics"][2]["expression"],
            "cohortActiveUsers/cohortTotalUsers"
        );
    }
    assert!(Handler("analytics_retention")
        .call(&mock, json!({"propertyId":"123","cohortDate":"2999-01-01"}))
        .await
        .is_err());
    assert!(Handler("analytics_activation")
        .call(
            &mock,
            json!({"propertyId":"123","steps":[{"eventName":"first_open"}]})
        )
        .await
        .is_err());
}
#[test]
fn expanded_comparisons_do_not_display_six_out_of_three_or_invent_next_offset() {
    let rows: Vec<Value> = ["current", "prior"]
        .into_iter()
        .flat_map(|range| (0..3).map(move |_| json!({"dimensionValues":[{"value":range}]})))
        .collect();
    let mut result = json!({"rowCount":3,"dimensionHeaders":[{"name":"dateRange"}],"rows":rows});
    assert!(!mark_pagination(
        &mut result,
        &json!({"dateRanges":[{},{}]})
    ));
    assert_eq!(result["pagination"]["upstreamRowCount"], 3);
    assert_eq!(result["pagination"]["returnedRows"], 6);
    assert_eq!(
        result["pagination"]["returnedRowsByDateRange"]["current"],
        3
    );
    assert!(result["pagination"].get("totalRows").is_none());
    assert!(result["pagination"].get("nextOffset").is_none());
    result["rowCount"] = json!(10);
    assert!(mark_pagination(&mut result, &json!({"dateRanges":[{},{}]})));
    assert!(result["pagination"]["complete"].is_null());
    assert!(result["pagination"].get("nextOffset").is_none());
}
#[tokio::test]
async fn structured_google_errors_are_classified_without_guessing_from_403() {
    struct Denied(Value);
    #[async_trait]
    impl OperationInvoker for Denied {
        async fn invoke_read_only(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
            let mut r =
                ToolResult::text("Google denied request").with_structured(Some(self.0.clone()));
            r.is_error = true;
            Ok(r)
        }
    }
    for (reason, expected) in [
        ("SERVICE_DISABLED", "api_disabled"),
        ("USER_PROJECT_DENIED", "credential_or_quota_project"),
        (
            "ACCESS_TOKEN_SCOPE_INSUFFICIENT",
            "insufficient_oauth_scope",
        ),
        ("UNRECOGNIZED", "permission_denied_unclassified"),
    ] {
        let source=invoke(&Denied(json!({"status":403,"body":{"error":{"code":403,"details":[{"@type":"type.googleapis.com/google.rpc.ErrorInfo","reason":reason,"metadata":{"service":"analyticsdata.googleapis.com"}}]}}})),"operation",json!({})).await;
        assert_eq!(source["diagnostic"]["category"], expected);
        assert_eq!(source["error"], "Google denied request");
    }
    assert_eq!(
        diagnostics::explain(
            &json!({"error":{"code":403,"message":"User does not have sufficient permissions for this property."}})
        )["category"],
        "property_permission_denied"
    );
}

#[test]
fn legacy_insufficient_permissions_is_not_automatically_an_oauth_scope_error() {
    let error = |message: &str| json!({"status":403,"body":{"error":{"code":403,"message":message,"errors":[{"reason":"insufficientPermissions"}]}}});
    assert_eq!(
        diagnostics::explain(&error(
            "User does not have sufficient permissions for this property."
        ))["category"],
        "property_permission_denied"
    );
    assert_eq!(
        diagnostics::explain(&error("Permission denied."))["category"],
        "permission_denied_unclassified"
    );
    assert_eq!(
        diagnostics::explain(&error("Request had insufficient authentication scopes."))["category"],
        "insufficient_oauth_scope"
    );
}

#[tokio::test]
async fn batch_scope_preserves_each_report_date_window() {
    let mock = Capture(Mutex::new(vec![]));
    let first = json!([{"startDate":"2026-01-01","endDate":"2026-01-07"}]);
    let second = json!([{"startDate":"2026-02-01","endDate":"2026-02-07"}]);
    let result=Handler("analytics_batch").call(&mock,json!({"propertyId":"123","platform":"Android","request":{"requests":[{"dateRanges":first,"metrics":[{"name":"activeUsers"}]},{"dateRanges":second,"metrics":[{"name":"activeUsers"}]}]}})).await.unwrap().structured.unwrap();
    let scopes = &result["sources"][0]["scope"]["batchScopes"];
    assert_eq!(scopes.as_array().unwrap().len(), 2);
    assert_eq!(scopes[0]["reportIndex"], 0);
    assert_eq!(scopes[1]["reportIndex"], 1);
    assert_eq!(scopes[0]["dateRanges"], first);
    assert_eq!(scopes[1]["dateRanges"], second);
    let calls = mock.0.lock().unwrap();
    for index in 0..2 {
        assert_eq!(
            scopes[index]["dimensionFilter"],
            calls[0].1["body"]["requests"][index]["dimensionFilter"]
        );
    }
}
