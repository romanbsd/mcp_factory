use serde_json::{json, Value};

#[derive(Clone, Copy)]
pub struct MetricCapability {
    pub public_name: &'static str,
    pub method_prefix: &'static str,
    pub resource_suffix: &'static str,
    pub metric_name: &'static str,
    pub required_dimensions: &'static [&'static str],
    pub tester_supported: bool,
}

pub const METRICS: &[MetricCapability] = &[
    MetricCapability {
        public_name: "crash_rate",
        method_prefix: "vitals_crashrate",
        resource_suffix: "crashRateMetricSet",
        metric_name: "crashRate",
        required_dimensions: &[],
        tester_supported: true,
    },
    MetricCapability {
        public_name: "user_perceived_crash_rate",
        method_prefix: "vitals_crashrate",
        resource_suffix: "crashRateMetricSet",
        metric_name: "userPerceivedCrashRate",
        required_dimensions: &[],
        tester_supported: true,
    },
    MetricCapability {
        public_name: "anr_rate",
        method_prefix: "vitals_anrrate",
        resource_suffix: "anrRateMetricSet",
        metric_name: "anrRate",
        required_dimensions: &[],
        tester_supported: true,
    },
    MetricCapability {
        public_name: "user_perceived_anr_rate",
        method_prefix: "vitals_anrrate",
        resource_suffix: "anrRateMetricSet",
        metric_name: "userPerceivedAnrRate",
        required_dimensions: &[],
        tester_supported: true,
    },
    MetricCapability {
        public_name: "user_perceived_lmk_rate",
        method_prefix: "vitals_lmkrate",
        resource_suffix: "lmkRateMetricSet",
        metric_name: "userPerceivedLmkRate",
        required_dimensions: &[],
        tester_supported: true,
    },
    MetricCapability {
        public_name: "slow_start_rate",
        method_prefix: "vitals_slowstartrate",
        resource_suffix: "slowStartRateMetricSet",
        metric_name: "slowStartRate",
        required_dimensions: &["startType"],
        tester_supported: false,
    },
    MetricCapability {
        public_name: "excessive_wakeup_rate",
        method_prefix: "vitals_excessivewakeuprate",
        resource_suffix: "excessiveWakeupRateMetricSet",
        metric_name: "excessiveWakeupRate",
        required_dimensions: &[],
        tester_supported: false,
    },
    MetricCapability {
        public_name: "stuck_background_wakelock_rate",
        method_prefix: "vitals_stuckbackgroundwakelockrate",
        resource_suffix: "stuckBackgroundWakelockRateMetricSet",
        metric_name: "stuckBgWakelockRate",
        required_dimensions: &[],
        tester_supported: false,
    },
];

pub const ALLOWED_LOW_LEVEL_METHODS: &[&str] = &[
    "apps_search",
    "apps_fetchReleaseFilterOptions",
    "applications_tracks_releases_list",
    "reviews_list",
    "anomalies_list",
    "vitals_crashrate_get",
    "vitals_crashrate_query",
    "vitals_anrrate_get",
    "vitals_anrrate_query",
    "vitals_lmkrate_get",
    "vitals_lmkrate_query",
    "vitals_slowstartrate_get",
    "vitals_slowstartrate_query",
    "vitals_excessivewakeuprate_get",
    "vitals_excessivewakeuprate_query",
    "vitals_stuckbackgroundwakelockrate_get",
    "vitals_stuckbackgroundwakelockrate_query",
    "vitals_errors_counts_get",
    "vitals_errors_counts_query",
    "vitals_errors_issues_search",
    "apprecovery_list",
    "generatedapks_list",
    "applications_deviceTierConfigs_list",
    "applications_deviceTierConfigs_get",
];

pub fn metric(name: &str) -> Option<&'static MetricCapability> {
    METRICS
        .iter()
        .find(|capability| capability.public_name == name)
}

pub fn capabilities_json() -> Value {
    Value::Array(
        METRICS
            .iter()
            .map(|capability| {
                json!({
                    "metric": capability.public_name,
                    "metricSet": capability.resource_suffix,
                    "metrics": [capability.metric_name, "distinctUsers"],
                    "aggregationPeriods": ["DAILY"],
                    "requiredDimensions": capability.required_dimensions,
                    "optionalDimensions": ["versionCode", "apiLevel", "deviceModel"],
                    "cohorts": if capability.tester_supported {
                        vec!["OS_PUBLIC", "APP_TESTERS"]
                    } else {
                        vec!["OS_PUBLIC"]
                    },
                    "verifiedAgainst": "playdeveloperreporting-v1beta1",
                    "verifiedAt": "2026-09-04",
                })
            })
            .collect(),
    )
}

pub fn console_coverage_gaps() -> Vec<Value> {
    [
        ("policy", "Policy status is available only in Play Console."),
        (
            "app_content",
            "App content declarations are not exposed by the retained APIs.",
        ),
        (
            "data_safety",
            "Current Data Safety answers are not read through a write endpoint.",
        ),
        (
            "pre_launch",
            "Pre-launch report findings are not exposed by these public APIs.",
        ),
        (
            "review_correspondence",
            "Review correspondence requires Console text or email.",
        ),
        (
            "artifact_warnings",
            "Some artifact warnings and advisory notices are Console-only.",
        ),
    ]
    .into_iter()
    .map(|(area, detail)| json!({"area": area, "detail": detail, "apiCoverage": "none"}))
    .collect()
}

pub fn allowed(method: &str) -> bool {
    ALLOWED_LOW_LEVEL_METHODS.contains(&method)
}
