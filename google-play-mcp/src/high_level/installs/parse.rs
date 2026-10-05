//! Decoding and header-mapped parsing of Play install CSV exports.
//!
//! Two layers, deliberately separate: `decode` + `records` only turn bytes into
//! string cells (used by raw row retrieval, which must keep working when the
//! semantic mapping fails), and `parse_installs` maps those cells onto the
//! install schema and validates them.

use std::collections::{BTreeMap, HashMap};

use mcp_factory_core::chrono::NaiveDate;
use serde_json::{json, Value};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Aggregation {
    /// Daily event counts: summed over observed dates.
    Event,
    /// Current/cumulative counts: the latest observed date only, never summed.
    Snapshot,
}

#[derive(Debug)]
pub struct Metric {
    pub key: &'static str,
    pub column: &'static str,
    pub aggregation: Aggregation,
}

#[rustfmt::skip]
pub const METRICS: &[Metric] = &[
    metric("dailyDeviceInstalls", "Daily Device Installs", Aggregation::Event),
    metric("dailyDeviceUninstalls", "Daily Device Uninstalls", Aggregation::Event),
    metric("dailyDeviceUpgrades", "Daily Device Upgrades", Aggregation::Event),
    metric("dailyUserInstalls", "Daily User Installs", Aggregation::Event),
    metric("dailyUserUninstalls", "Daily User Uninstalls", Aggregation::Event),
    metric("currentDeviceInstalls", "Current Device Installs", Aggregation::Snapshot),
    metric("installsOnActiveDevices", "Installs on active devices", Aggregation::Snapshot),
    metric("currentUserInstalls", "Current User Installs", Aggregation::Snapshot),
    metric("totalUserInstalls", "Total User Installs", Aggregation::Snapshot),
    // Present in live exports (verified 2026-10-05) alongside the columns above.
    metric("activeDeviceInstalls", "Active Device Installs", Aggregation::Snapshot),
    metric("installEvents", "Install events", Aggregation::Event),
    metric("updateEvents", "Update events", Aggregation::Event),
    metric("uninstallEvents", "Uninstall events", Aggregation::Event),
];

const fn metric(key: &'static str, column: &'static str, aggregation: Aggregation) -> Metric {
    Metric {
        key,
        column,
        aggregation,
    }
}

pub fn metric_by_key(key: &str) -> Option<&'static Metric> {
    METRICS.iter().find(|metric| metric.key == key)
}

/// A version code: plain ASCII digits only (no sign, no blanks), as `u64`.
pub fn version_code(text: &str) -> Option<u64> {
    let text = text.trim();
    (!text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}

pub const DATE_COLUMN: &str = "Date";
pub const PACKAGE_COLUMN: &str = "Package Name";
pub const VERSION_COLUMN: &str = "App Version Code";

/// Returns the decoded text and the detected encoding label. UTF-16 is
/// detected by BOM, or by the NUL pattern of an ASCII first character when the
/// BOM is absent; UTF-8 only when it validates.
pub fn decode(bytes: &[u8]) -> Result<(String, &'static str), String> {
    let utf16 = |body: &[u8], little: bool| -> Result<String, String> {
        if !body.len().is_multiple_of(2) {
            return Err("UTF-16 data has an odd byte length".to_string());
        }
        let units = body.as_chunks::<2>().0.iter().map(|pair| {
            if little {
                u16::from_le_bytes(*pair)
            } else {
                u16::from_be_bytes(*pair)
            }
        });
        char::decode_utf16(units)
            .collect::<Result<String, _>>()
            .map_err(|error| format!("invalid UTF-16: {error}"))
    };
    match bytes {
        [0xFF, 0xFE, rest @ ..] => Ok((utf16(rest, true)?, "UTF-16LE (BOM)")),
        [0xFE, 0xFF, rest @ ..] => Ok((utf16(rest, false)?, "UTF-16BE (BOM)")),
        [0xEF, 0xBB, 0xBF, rest @ ..] => std::str::from_utf8(rest)
            .map(|text| (text.to_string(), "UTF-8 (BOM)"))
            .map_err(|error| format!("invalid UTF-8: {error}")),
        [first, 0, ..] if *first != 0 => Ok((utf16(bytes, true)?, "UTF-16LE (no BOM)")),
        [0, second, ..] if *second != 0 => Ok((utf16(bytes, false)?, "UTF-16BE (no BOM)")),
        _ => std::str::from_utf8(bytes)
            .map(|text| (text.to_string(), "UTF-8"))
            .map_err(|_| "unrecognized encoding: no BOM and not valid UTF-8".to_string()),
    }
}

/// All CSV records as string cells, header first, in source order. Quoted
/// fields and embedded newlines are handled by the `csv` crate; ragged rows are
/// returned as-is for the caller to judge.
pub fn records(text: &str) -> Result<Vec<Vec<String>>, String> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(text.as_bytes())
        .records()
        .map(|record| {
            record
                .map(|record| record.iter().map(str::to_string).collect())
                .map_err(|error| format!("CSV error: {error}"))
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct InstallRow {
    pub row_number: usize,
    pub date: NaiveDate,
    pub version: u64,
    /// Requested metric key -> value; `None` is a blank source cell, never zero.
    pub values: BTreeMap<&'static str, Option<i64>>,
}

#[derive(Debug, Default)]
pub struct ParsedInstalls {
    pub headers: Vec<String>,
    /// Metrics mapped for this file: the requested ones, or every known
    /// metric whose column is present when none were requested.
    pub metrics: Vec<&'static Metric>,
    pub rows: Vec<InstallRow>,
    /// Rows that were rejected, each with a reason; never silently dropped.
    pub issues: Vec<Value>,
}

#[derive(Debug)]
pub struct SchemaError {
    pub message: String,
    pub headers: Vec<String>,
}

/// Maps a decoded `app_version` export onto install rows. Columns are found by
/// header name (case-insensitive), so reordered or added columns are tolerated;
/// a missing required or requested column is a schema error.
pub fn parse_installs(
    text: &str,
    package: &str,
    requested: Option<&[&'static Metric]>,
) -> Result<ParsedInstalls, SchemaError> {
    let all = records(text).map_err(|message| SchemaError {
        message,
        headers: Vec::new(),
    })?;
    let mut rows = all.into_iter();
    let headers: Vec<String> = rows
        .next()
        .unwrap_or_default()
        .into_iter()
        .map(|header| header.trim_start_matches('\u{feff}').trim().to_string())
        .collect();
    let index: HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(position, header)| (header.to_ascii_lowercase(), position))
        .collect();
    let column = |name: &str| index.get(&name.to_ascii_lowercase()).copied();
    let metrics: Vec<&'static Metric> = match requested {
        Some(requested) => requested.to_vec(),
        None => METRICS
            .iter()
            .filter(|metric| column(metric.column).is_some())
            .collect(),
    };
    // A repeated header leaves no way to tell which column holds the value;
    // reject it when it is one we read (unrelated duplicates are tolerated).
    let used: Vec<&str> = [DATE_COLUMN, PACKAGE_COLUMN, VERSION_COLUMN]
        .into_iter()
        .chain(metrics.iter().map(|metric| metric.column))
        .collect();
    let duplicated: Vec<&str> = used
        .iter()
        .copied()
        .filter(|name| {
            headers
                .iter()
                .filter(|header| header.eq_ignore_ascii_case(name))
                .count()
                > 1
        })
        .collect();
    if !duplicated.is_empty() {
        return Err(SchemaError {
            message: format!("ambiguous duplicate columns: {}", duplicated.join(", ")),
            headers,
        });
    }
    let missing: Vec<&str> = [DATE_COLUMN, PACKAGE_COLUMN, VERSION_COLUMN]
        .into_iter()
        .chain(metrics.iter().map(|metric| metric.column))
        .filter(|name| column(name).is_none())
        .collect();
    if !missing.is_empty() {
        return Err(SchemaError {
            message: format!("missing columns: {}", missing.join(", ")),
            headers,
        });
    }
    let (date_at, package_at, version_at) = (
        column(DATE_COLUMN).unwrap(),
        column(PACKAGE_COLUMN).unwrap(),
        column(VERSION_COLUMN).unwrap(),
    );
    let metric_at: Vec<(&Metric, usize)> = metrics
        .iter()
        .map(|metric| (*metric, column(metric.column).unwrap()))
        .collect();

    let mut parsed = ParsedInstalls {
        headers: headers.clone(),
        metrics: metrics.clone(),
        ..Default::default()
    };
    let mut keys: HashMap<(NaiveDate, u64), Vec<usize>> = HashMap::new();
    for (offset, cells) in rows.enumerate() {
        let row_number = offset + 1;
        let issue = |reason: String| json!({"rowNumber": row_number, "reason": reason});
        if cells.len() < headers.len() {
            parsed.issues.push(issue(format!(
                "row has {} cells; header has {}",
                cells.len(),
                headers.len()
            )));
            continue;
        }
        let row_package = cells[package_at].trim();
        if row_package != package {
            parsed
                .issues
                .push(issue(format!("package mismatch: {row_package:?}")));
            continue;
        }
        let Ok(date) = NaiveDate::parse_from_str(cells[date_at].trim(), "%Y-%m-%d") else {
            parsed
                .issues
                .push(issue(format!("invalid date: {:?}", cells[date_at])));
            continue;
        };
        let Some(version) = version_code(&cells[version_at]) else {
            parsed.issues.push(issue(format!(
                "invalid version code: {:?}",
                cells[version_at]
            )));
            continue;
        };
        let mut values = BTreeMap::new();
        let mut invalid = None;
        for (metric, position) in &metric_at {
            let cell = cells[*position].trim();
            if cell.is_empty() {
                values.insert(metric.key, None);
            } else if let Ok(value) = cell.parse::<i64>() {
                values.insert(metric.key, Some(value));
            } else {
                invalid = Some(format!("invalid integer in {:?}: {cell:?}", metric.column));
                break;
            }
        }
        if let Some(reason) = invalid {
            parsed.issues.push(issue(reason));
            continue;
        }
        keys.entry((date, version))
            .or_default()
            .push(parsed.rows.len());
        parsed.rows.push(InstallRow {
            row_number,
            date,
            version,
            values,
        });
    }
    // A repeated (date, version) key has no defined meaning; keeping either copy
    // would be a guess and summing both would double count. Reject all copies.
    let mut duplicate_rows = Vec::new();
    for ((date, version), positions) in &keys {
        if positions.len() > 1 {
            let row_numbers: Vec<usize> = positions
                .iter()
                .map(|position| parsed.rows[*position].row_number)
                .collect();
            parsed.issues.push(json!({
                "rowNumbers": row_numbers,
                "reason": format!("duplicate row key date={date} version={version}")
            }));
            duplicate_rows.extend(row_numbers);
        }
    }
    parsed
        .rows
        .retain(|row| !duplicate_rows.contains(&row.row_number));
    Ok(parsed)
}
