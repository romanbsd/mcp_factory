//! Guided aggregate user progression and first-session cohort retention.
use mcp_factory_core::{
    chrono::{Days, NaiveDate, Utc},
    ProxyError,
};
use serde_json::{json, Value};
fn invalid(message: &str) -> ProxyError {
    ProxyError::Validation(message.into())
}
pub fn prepare(tool: &str, args: &Value, body: &mut Value) -> Result<(), ProxyError> {
    if tool == "analytics_activation" {
        let steps = args["steps"]
            .as_array()
            .filter(|s| (2..=10).contains(&s.len()))
            .ok_or_else(|| invalid("activation requires 2 to 10 ordered event steps"))?;
        let mut prepared = Vec::new();
        let within = args
            .get("withinSeconds")
            .map(|v| {
                v.as_u64()
                    .filter(|n| (1..=604800).contains(n))
                    .ok_or_else(|| invalid("withinSeconds must be 1 to 604800"))
            })
            .transpose()?;
        for (i, step) in steps.iter().enumerate() {
            let event = step["eventName"]
                .as_str()
                .filter(|s| !s.is_empty() && s.len() <= 100)
                .ok_or_else(|| invalid("each activation step requires a nonempty eventName"))?;
            let mut item = json!({"name":step["name"].as_str().unwrap_or(event),"filterExpression":{"funnelEventFilter":{"eventName":event}}});
            if i > 0 {
                item["isDirectlyFollowedBy"] =
                    json!(args["directlyFollowed"].as_bool().unwrap_or(false));
                if let Some(seconds) = within {
                    item["withinDurationFromPriorStep"] = json!(format!("{seconds}s"));
                }
            }
            prepared.push(item);
        }
        body["funnel"] = json!({"isOpenFunnel":false,"steps":prepared});
        body["limit"] = json!(args["limit"]
            .as_u64()
            .unwrap_or(100)
            .clamp(1, 1000)
            .to_string());
        body["returnPropertyQuota"] = json!(args["verbose"].as_bool().unwrap_or(false));
    } else if tool == "analytics_retention" {
        let day = args["cohortDate"].as_str().and_then(|s| NaiveDate::parse_from_str(s,"%Y-%m-%d").ok())
            .ok_or_else(|| invalid("retention requires cohortDate (YYYY-MM-DD), the first-session acquisition date"))?;
        let horizon = args
            .get("horizonDays")
            .map_or(Some(7), Value::as_u64)
            .filter(|n| (1..=90).contains(n))
            .ok_or_else(|| invalid("horizonDays must be 1 to 90"))?;
        if day
            .checked_add_days(Days::new(horizon))
            .is_none_or(|end| end >= Utc::now().date_naive())
        {
            return Err(invalid("cohortDate plus horizonDays must be before today; choose a mature cohort or a shorter horizon"));
        }
        body.as_object_mut()
            .ok_or_else(|| invalid("request must be an object"))?
            .remove("dateRanges");
        body["cohortSpec"] = json!({"cohorts":[{"name":"acquisition","dimension":"firstSessionDate","dateRange":{"startDate":day.to_string(),"endDate":day.to_string()}}],"cohortsRange":{"granularity":"DAILY","startOffset":0,"endOffset":horizon}});
        body["dimensions"] = json!([{"name":"cohort"},{"name":"cohortNthDay"}]);
        body["metrics"] = json!([{"name":"cohortActiveUsers"},{"name":"cohortTotalUsers"},{"name":"retentionFraction","expression":"cohortActiveUsers/cohortTotalUsers"}]);
    }
    Ok(())
}
pub fn interpretation(tool: &str) -> Value {
    match tool {
        "analytics_activation" => {
            json!({"kind":"ordered_closed_user_funnel","note":"GA-reported users must enter at step one and progress in order; these are not ratios of independent event totals. Other events may intervene unless directlyFollowed is true. Alpha API; consent, reporting identity, retention and thresholding can limit observed progression. No individual journey export or guaranteed cross-device linkage."})
        }
        "analytics_retention" => {
            json!({"kind":"first_session_cohort_retention","note":"Tracks aggregate returning active users acquired on cohortDate. The API only supports firstSessionDate cohort membership, not an arbitrary activation-event cohort. Platform/stream and custom dimension filters constrain reported activity; do not interpret them as a verified first-open/activation acquisition cohort. Missing or thresholded rows are not zero. This report is independent of the activation funnel."})
        }
        _ => Value::Null,
    }
}
