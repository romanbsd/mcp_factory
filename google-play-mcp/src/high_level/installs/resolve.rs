//! Report-bucket configuration and resolution.
//!
//! Order: explicit bucket (tool input or configuration), then a cached verified
//! mapping, then probing `pubsite_prod_<developerId>` and
//! `pubsite_prod_rev_<developerId>` with one bounded install-object listing
//! each. The naming rule is a heuristic, not a Google guarantee, so a failure
//! ends in specific guidance rather than wider guessing.

use mcp_factory_core::chrono::Utc;
use serde_json::{json, Value};

use super::storage::{Storage, StorageError};
use super::Recorder;

pub const INSTALLS_PREFIX: &str = "stats/installs/installs_";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AccountConfig {
    pub developer_id: Option<String>,
    pub bucket: Option<String>,
}

impl AccountConfig {
    pub fn key(&self) -> String {
        self.developer_id
            .clone()
            .or_else(|| self.bucket.clone())
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReportsConfig {
    pub accounts: Vec<AccountConfig>,
    /// Configuration problems found while reading the environment; surfaced on
    /// tool calls instead of failing server startup.
    pub problems: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Failure {
    pub code: &'static str,
    pub message: String,
    pub remedy: Option<String>,
}

impl Failure {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            remedy: None,
        }
    }

    pub fn remedy(mut self, remedy: impl Into<String>) -> Self {
        self.remedy = Some(remedy.into());
        self
    }

    pub fn to_json(&self) -> Value {
        json!({"code": self.code, "message": self.message, "remedy": self.remedy})
    }
}

impl From<&StorageError> for Failure {
    fn from(error: &StorageError) -> Self {
        Self::new(error.code(), error.message())
    }
}

impl ReportsConfig {
    pub fn from_env() -> Self {
        let var = |name: &str| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
        };
        Self::from_vars(
            var("GOOGLE_PLAY_REPORTS_ACCOUNTS").as_deref(),
            var("GOOGLE_PLAY_DEVELOPER_ID").as_deref(),
            var("GOOGLE_PLAY_REPORTS_BUCKET").as_deref(),
        )
    }

    /// `accounts` is `devId[=bucket],devId2[=bucket2]`; when present it replaces
    /// the single-account `developer_id` / `bucket` variables.
    pub fn from_vars(
        accounts: Option<&str>,
        developer_id: Option<&str>,
        bucket: Option<&str>,
    ) -> Self {
        let mut config = Self::default();
        let mut add = |developer_id: Option<&str>, bucket: Option<&str>| {
            let developer_id = match developer_id.map(str::trim).filter(|id| !id.is_empty()) {
                Some(id) if valid_developer_id(id) => Some(id.to_string()),
                Some(id) => {
                    config
                        .problems
                        .push(format!("ignored invalid developer id {id:?} (digits only)"));
                    None
                }
                None => None,
            };
            let bucket = match bucket.map(str::trim).filter(|bucket| !bucket.is_empty()) {
                Some(bucket) => match normalize_bucket(bucket) {
                    Ok(bucket) => Some(bucket),
                    Err(failure) => {
                        config.problems.push(failure.message);
                        None
                    }
                },
                None => None,
            };
            if developer_id.is_some() || bucket.is_some() {
                config.accounts.push(AccountConfig {
                    developer_id,
                    bucket,
                });
            }
        };
        match accounts {
            Some(accounts) => {
                for entry in accounts.split(',').filter(|entry| !entry.trim().is_empty()) {
                    let (id, bucket) = match entry.split_once('=') {
                        Some((id, bucket)) => (Some(id), Some(bucket)),
                        None => (Some(entry), None),
                    };
                    add(id, bucket);
                }
            }
            None => add(developer_id, bucket),
        }
        config
    }

    /// Picks the account named by `selector` (developer id or bucket), or the
    /// only configured account when there is exactly one.
    pub fn select(&self, selector: Option<&str>) -> Result<&AccountConfig, Failure> {
        let problems = || {
            if self.problems.is_empty() {
                String::new()
            } else {
                format!(" Configuration problems: {}.", self.problems.join("; "))
            }
        };
        if self.accounts.is_empty() {
            return Err(Failure::new(
                "missing_developer_id",
                format!(
                    "No Play developer account is configured, so the report bucket cannot be resolved automatically.{}",
                    problems()
                ),
            )
            .remedy(
                "Set GOOGLE_PLAY_DEVELOPER_ID to the numeric id from the Play Console URL \
                 (play.google.com/console/developers/<id>), or set GOOGLE_PLAY_REPORTS_BUCKET to the \
                 Cloud Storage URI shown in Play Console > Download reports > Statistics.",
            ));
        }
        match selector {
            Some(selector) => self
                .accounts
                .iter()
                .find(|account| {
                    account.developer_id.as_deref() == Some(selector)
                        || (account.bucket.is_some()
                            && account.bucket == normalize_bucket(selector).ok())
                })
                .ok_or_else(|| {
                    Failure::new("unknown_account", "account is not configured")
                        .remedy("Use a developer id listed in GOOGLE_PLAY_REPORTS_ACCOUNTS.")
                }),
            None if self.accounts.len() == 1 => Ok(&self.accounts[0]),
            None => Err(Failure::new(
                "account_required",
                "Several developer accounts are configured; pass `account`.",
            )),
        }
    }
}

pub fn valid_developer_id(id: &str) -> bool {
    (1..=30).contains(&id.len()) && id.bytes().all(|byte| byte.is_ascii_digit())
}

/// Accepts a bare bucket name or `gs://bucket[/]`. Anything carrying a path,
/// credentials, a query, another scheme, or a non-Play bucket name is rejected.
pub fn normalize_bucket(input: &str) -> Result<String, Failure> {
    let invalid = |why: &str| {
        Failure::new(
            "invalid_bucket",
            format!("invalid report bucket {input:?}: {why}"),
        )
        .remedy(
            "Use the bucket from Play Console > Download reports > Statistics, e.g. \
             gs://pubsite_prod_1234567890 (no object path).",
        )
    };
    let input = input.trim();
    let name = input.strip_prefix("gs://").unwrap_or(input);
    let name = name.strip_suffix('/').unwrap_or(name);
    if name.contains("://") {
        return Err(invalid("only gs:// URIs or bare bucket names are accepted"));
    }
    if !(3..=63).contains(&name.len())
        || !name.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
    {
        return Err(invalid("not a plain bucket name"));
    }
    // ponytail: Play report buckets are all `pubsite_prod_*`; widen this only if
    // Google issues a different Console URI.
    if !name.starts_with("pubsite_prod_") {
        return Err(invalid("not a Google Play report bucket (pubsite_prod_*)"));
    }
    Ok(name.to_string())
}

pub fn valid_package(package: &str) -> bool {
    !package.is_empty()
        && package.len() <= 255
        && package
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'.' || byte == b'_')
}

#[derive(Debug, Clone)]
pub struct Resolution {
    pub bucket: String,
    pub provenance: String,
    pub verified_at: Option<String>,
    /// Whether install reports for the requested package were seen. An
    /// accessible bucket without them proves access, not package relevance.
    pub relevant: Option<bool>,
}

impl Resolution {
    pub fn to_json(&self) -> Value {
        json!({
            "bucket": self.bucket,
            "bucketUri": format!("gs://{}", self.bucket),
            "provenance": self.provenance,
            "verifiedAt": self.verified_at,
            "packageReportsSeen": self.relevant,
        })
    }
}

pub struct ProbeOutcome {
    pub result: Result<Resolution, Failure>,
    /// Per-candidate diagnostics, kept even when another candidate succeeded.
    pub candidates: Vec<Value>,
}

pub fn candidates(account: &AccountConfig, override_bucket: Option<&str>) -> Vec<(String, String)> {
    if let Some(bucket) = override_bucket {
        return vec![(bucket.to_string(), "explicit_override".to_string())];
    }
    if let Some(bucket) = &account.bucket {
        return vec![(bucket.clone(), "configured_bucket".to_string())];
    }
    let Some(id) = &account.developer_id else {
        return Vec::new();
    };
    vec![
        (
            format!("pubsite_prod_{id}"),
            "probe:pubsite_prod".to_string(),
        ),
        (
            format!("pubsite_prod_rev_{id}"),
            "probe:pubsite_prod_rev".to_string(),
        ),
    ]
}

/// Validates each candidate with one listing restricted to this package's
/// install exports; no other prefix (financial, reviews, ...) is touched.
pub async fn probe(
    storage: &dyn Storage,
    recorder: &Recorder,
    candidates: Vec<(String, String)>,
    package: &str,
) -> ProbeOutcome {
    let prefix = format!("{INSTALLS_PREFIX}{package}_");
    let mut diagnostics = Vec::new();
    let mut relevant = Vec::new();
    let mut accessible = Vec::new();
    let mut errors: Vec<StorageError> = Vec::new();
    let verified_at = Utc::now().to_rfc3339();
    for (bucket, provenance) in &candidates {
        let listing = recorder
            .list(storage, bucket, &prefix, (None, None), None, 10)
            .await;
        let (state, detail) = match &listing {
            Ok(page) if !page.items.is_empty() => {
                relevant.push((bucket, provenance));
                ("relevant_reports", json!({"objectsSeen": page.items.len()}))
            }
            Ok(_) => {
                accessible.push((bucket, provenance));
                ("accessible_empty", Value::Null)
            }
            Err(error) => {
                errors.push(error.clone());
                (error.code(), json!(error.message()))
            }
        };
        diagnostics.push(json!({
            "bucket": bucket,
            "provenance": provenance,
            "state": state,
            "detail": detail,
        }));
    }
    let resolved = |(bucket, provenance): (&String, &String), seen: bool| Resolution {
        bucket: bucket.clone(),
        provenance: provenance.clone(),
        verified_at: Some(verified_at.clone()),
        relevant: Some(seen),
    };
    let result = match (relevant.as_slice(), accessible.as_slice()) {
        ([one], _) => Ok(resolved(*one, true)),
        ([], [one]) => Ok(resolved(*one, false)),
        ([], []) => Err(failure_from(&errors, candidates.len())),
        // Several relevant buckets, or several accessible and none relevant.
        _ => Err(Failure::new(
            "ambiguous",
            "More than one candidate bucket matches; refusing to pick one silently.",
        )
        .remedy("Set GOOGLE_PLAY_REPORTS_BUCKET to the URI shown in Play Console > Download reports > Statistics.")),
    };
    ProbeOutcome {
        result,
        candidates: diagnostics,
    }
}

fn failure_from(errors: &[StorageError], candidate_count: usize) -> Failure {
    let any = |code: &str| errors.iter().find(|error| error.code() == code);
    if let Some(error) = any("auth_failed") {
        return Failure::new("auth_failed", error.message()).remedy(
            "Check that GOOGLE_APPLICATION_CREDENTIALS points at a valid service-account key; \
             a token for the devstorage.read_only scope could not be used.",
        );
    }
    if let Some(error) = any("permission_denied") {
        return Failure::new("permission_denied", error.message()).remedy(
            "In Play Console > Users and permissions, grant this service account the account-level \
             'View app information and download bulk reports' permission. App-level access alone \
             may not include bulk reports; changes can take up to 24 hours. This tool never changes permissions.",
        );
    }
    if let Some(error) = errors.iter().find(|error| {
        !matches!(
            error,
            StorageError::BucketNotFound(_) | StorageError::ObjectNotFound(_)
        )
    }) {
        return Failure::from(error);
    }
    if candidate_count == 1 {
        return Failure::new(
            "bucket_not_found",
            "The configured report bucket does not exist.",
        )
        .remedy(
            "Re-copy the Cloud Storage URI from Play Console > Download reports > Statistics.",
        );
    }
    Failure::new(
        "override_required",
        "Neither pubsite_prod_<developerId> nor pubsite_prod_rev_<developerId> exists for this account.",
    )
    .remedy(
        "Copy the Cloud Storage URI from Play Console > Download reports > Statistics and set \
         GOOGLE_PLAY_REPORTS_BUCKET (or developerId=bucket in GOOGLE_PLAY_REPORTS_ACCOUNTS).",
    )
}
