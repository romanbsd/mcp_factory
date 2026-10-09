//! Analytics report segments are not JSON: the segment resource carries a
//! short-lived download URL for a gzip-compressed CSV, plus its MD5 checksum.

use std::io::Read;
use std::sync::Arc;

use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, ProxyError, ReadOnlyToolInvoker, ToolHints,
    ToolResult,
};
use md5::{Digest, Md5};
use serde_json::{json, Value};

use super::{check_transfer_url, failure, json_of, proxy_error_result, success, transfer_client};

/// Hard cap on the compressed and decompressed segment size.
const MAX_SEGMENT_BYTES: u64 = 256 * 1024 * 1024;
const DEFAULT_MAX_CHARS: usize = 200_000;
const MAX_CHARS: usize = 10_000_000;

pub(super) fn tool(http: Result<reqwest::Client, String>) -> CustomToolSpec {
    CustomToolSpec {
        name: "analytics_segment_download".to_string(),
        description: "Download one analytics report segment (from \
`analyticsReportInstances_segments_getToManyRelated`): fetches its URL, verifies the MD5 \
checksum, decompresses the gzip CSV and returns the text, truncated to `maxChars`."
            .to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "segmentId": {"type": "string", "minLength": 1},
                "maxChars": {"type": "integer", "minimum": 1, "maximum": MAX_CHARS,
                    "description": "Return at most this many characters (default 200000)."}
            },
            "required": ["segmentId"],
            "additionalProperties": false
        }),
        hints: ToolHints {
            title: Some("Download analytics segment".to_string()),
            read_only: Some(true),
            destructive: Some(false),
            idempotent: Some(true),
            open_world: Some(true),
            ..Default::default()
        },
        handler: Arc::new(SegmentDownload { http }),
    }
}

struct SegmentDownload {
    http: Result<reqwest::Client, String>,
}

#[async_trait]
impl CustomToolHandler for SegmentDownload {
    async fn call(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        Ok(match self.run(invoker, &arguments).await {
            Ok(result) | Err(result) => result,
        })
    }
}

impl SegmentDownload {
    async fn run(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        args: &Value,
    ) -> Result<ToolResult, ToolResult> {
        let segment_id = args["segmentId"].as_str().unwrap_or_default();
        let max_chars = args["maxChars"]
            .as_u64()
            .map_or(DEFAULT_MAX_CHARS, |value| {
                usize::try_from(value).unwrap_or(MAX_CHARS)
            })
            .min(MAX_CHARS);

        let segment = invoker
            .invoke_read_only(
                "analyticsReportSegments_getInstance",
                json!({"id": segment_id}),
            )
            .await
            .map_err(|error| proxy_error_result("lookup", error))
            .and_then(|result| json_of(result, "lookup"))?;
        let attributes = &segment["data"]["attributes"];
        let url = check_transfer_url(attributes["url"].as_str().unwrap_or_default())
            .map_err(|message| failure("lookup", &message, Value::Null))?;

        let http = transfer_client(&self.http, "download")?;
        let mut response = http
            .get(url)
            .send()
            .await
            .map_err(|error| failure("download", &error.to_string(), Value::Null))?;
        if !response.status().is_success() {
            return Err(failure(
                "download",
                &format!("download returned {}", response.status()),
                json!({"hint": "segment URLs expire; fetch the segment again for a fresh URL"}),
            ));
        }
        // Enforce the cap while streaming: Content-Length may be absent.
        // Content-Length is untrusted: preallocate at most 1 MiB and let
        // genuinely large segments grow as bytes arrive.
        let capacity = response.content_length().unwrap_or(0).min(1 << 20) as usize;
        let mut compressed = Vec::with_capacity(capacity);
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| failure("download", &error.to_string(), Value::Null))?
        {
            if (compressed.len() + chunk.len()) as u64 > MAX_SEGMENT_BYTES {
                return Err(failure(
                    "download",
                    "segment exceeds the size limit",
                    Value::Null,
                ));
            }
            compressed.extend_from_slice(&chunk);
        }

        // Hashing, decompression and truncation are CPU-bound on up to
        // MAX_SEGMENT_BYTES; keep them off the async workers.
        let expected = attributes["checksum"].as_str().map(str::to_string);
        let (content, total_chars, truncated) = tokio::task::spawn_blocking(move || {
            let text = verify_and_decode(compressed, expected, MAX_SEGMENT_BYTES)?;
            Ok::<_, ToolResult>(truncate_chars(text, max_chars))
        })
        .await
        .map_err(|error| failure("decompress", &error.to_string(), Value::Null))??;
        Ok(success(json!({
            "ok": true,
            "segmentId": segment_id,
            "checksumVerified": attributes["checksum"].is_string(),
            "totalChars": total_chars,
            "truncated": truncated,
            "content": content,
        })))
    }
}

/// Check the MD5 (only when Apple supplied one) and decompress gzip
/// segments, refusing output beyond `max_bytes` instead of truncating it.
fn verify_and_decode(
    compressed: Vec<u8>,
    expected: Option<String>,
    max_bytes: u64,
) -> Result<String, ToolResult> {
    if let Some(expected) = expected {
        let digest: String = Md5::digest(&compressed)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        if !expected.eq_ignore_ascii_case(&digest) {
            return Err(failure(
                "verify",
                "checksum mismatch",
                json!({"expected": expected, "actual": digest}),
            ));
        }
    }
    let bytes = if compressed.starts_with(&[0x1f, 0x8b]) {
        let mut decoded = Vec::new();
        flate2::read::MultiGzDecoder::new(&compressed[..])
            .take(max_bytes + 1)
            .read_to_end(&mut decoded)
            .map_err(|error| failure("decompress", &error.to_string(), Value::Null))?;
        if decoded.len() as u64 > max_bytes {
            return Err(failure(
                "decompress",
                "decompressed segment exceeds the size limit",
                Value::Null,
            ));
        }
        decoded
    } else {
        compressed
    };
    String::from_utf8(bytes).map_err(|_| {
        failure(
            "decompress",
            "segment is neither gzip nor UTF-8",
            Value::Null,
        )
    })
}

/// `(content, total_chars, truncated)`, truncating in place.
fn truncate_chars(text: String, max_chars: usize) -> (String, usize, bool) {
    match text.char_indices().nth(max_chars) {
        Some((cut, _)) => {
            let total = max_chars + text[cut..].chars().count();
            let mut text = text;
            text.truncate(cut);
            (text, total, true)
        }
        None => {
            let total = text.chars().count();
            (text, total, false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_rejects_oversized_or_mismatched_segments() {
        use std::io::Write;
        let gzip = |bytes: &[u8]| {
            let mut encoder =
                flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
            encoder.write_all(bytes).unwrap();
            encoder.finish().unwrap()
        };
        let gz = gzip(b"a,b\n1,2\n");
        let md5: String = Md5::digest(&gz)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(
            verify_and_decode(gz.clone(), Some(md5), 1024).unwrap(),
            "a,b\n1,2\n"
        );
        let bad = verify_and_decode(gz, Some("0".repeat(32)), 1024).unwrap_err();
        assert_eq!(bad.structured.unwrap()["stage"], "verify");
        let binary = verify_and_decode(vec![0xff, 0xfe], None, 1024).unwrap_err();
        assert_eq!(binary.structured.unwrap()["stage"], "decompress");

        // A small payload that expands past the cap is refused, not truncated.
        let bomb = gzip(&[b'a'; 10_000]);
        assert!(bomb.len() < 100);
        let refused = verify_and_decode(bomb.clone(), None, 9_999).unwrap_err();
        let details = refused.structured.unwrap();
        assert_eq!(details["stage"], "decompress");
        assert!(details["message"].as_str().unwrap().contains("size limit"));
        assert_eq!(verify_and_decode(bomb, None, 10_000).unwrap().len(), 10_000);
    }

    #[test]
    fn truncate_counts_characters_not_bytes() {
        assert_eq!(truncate_chars("héllo".into(), 2), ("hé".into(), 5, true));
        assert_eq!(
            truncate_chars("héllo".into(), 5),
            ("héllo".into(), 5, false)
        );
        assert_eq!(truncate_chars(String::new(), 3), (String::new(), 0, false));
    }
}
