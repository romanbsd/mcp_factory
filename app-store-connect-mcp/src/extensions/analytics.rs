//! Analytics report segments are not JSON: the segment resource carries a
//! short-lived download URL for a gzip-compressed CSV, plus its MD5 checksum.

use std::io::{Read, Seek};

use std::sync::Arc;

use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, OperationInvoker, ProxyError, ToolHints,
    ToolResult,
};

use serde_json::{json, Value};

use super::{failure, json_of, proxy_error_result, reports::ReportService, success};

/// Hard cap on the compressed and decompressed segment size.
const MAX_SEGMENT_BYTES: u64 = mcp_factory_core::TransferLimits::REPORT_BYTES;
const DEFAULT_MAX_CHARS: usize = 8_000;
const MAX_CHARS: usize = 10_000_000;

pub(super) fn tool_with_service(service: Arc<ReportService>) -> CustomToolSpec {
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
                    "description": "Return at most this many characters (default 8000)."}
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
        handler: Arc::new(SegmentDownload { service }),
    }
}

struct SegmentDownload {
    service: Arc<ReportService>,
}

#[async_trait]
impl CustomToolHandler for SegmentDownload {
    async fn call(
        &self,
        invoker: &dyn OperationInvoker,
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
        invoker: &dyn OperationInvoker,
        args: &Value,
    ) -> Result<ToolResult, ToolResult> {
        let permit = self
            .service
            .transfers
            .clone()
            .acquire_owned()
            .await
            .map_err(|e| failure("download", &e.to_string(), Value::Null))?;
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
        let mut spool = self.service.download(attributes).await?;
        let (content, total_chars, truncated) =
            super::reports::blocking_with_permit(permit, move || {
                decode_preview(&mut spool, MAX_SEGMENT_BYTES, max_chars)
            })
            .await?;
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

/// Decode all bytes for validation/counting, retaining only the requested preview.
/// At most a 64 KiB buffer, three trailing UTF-8 bytes and the preview are kept.
fn decode_preview<R: Read + Seek>(
    source: &mut R,
    max_bytes: u64,
    max_chars: usize,
) -> Result<(String, usize, bool), ToolResult> {
    collect_preview(
        mcp_factory_core::decoded_reader(source).map_err(decode_error)?,
        max_bytes,
        max_chars,
    )
}

fn decode_error(error: std::io::Error) -> ToolResult {
    failure("decompress", &error.to_string(), Value::Null)
}

fn collect_preview(
    mut reader: impl Read,
    max_bytes: u64,
    max_chars: usize,
) -> Result<(String, usize, bool), ToolResult> {
    let mut buffer = vec![0; 64 * 1024 + 4];
    let mut carry = 0;
    let mut bytes = 0u64;
    let mut total = 0usize;
    let mut preview = String::new();
    loop {
        let read = reader.read(&mut buffer[carry..]).map_err(decode_error)?;
        bytes += read as u64;
        if bytes > max_bytes {
            return Err(failure(
                "decompress",
                "decompressed segment exceeds the size limit",
                Value::Null,
            ));
        }
        let len = carry + read;
        if read == 0 {
            if carry != 0 {
                return Err(failure("decompress", "segment is not UTF-8", Value::Null));
            }
            break;
        }
        let valid = match std::str::from_utf8(&buffer[..len]) {
            Ok(_) => len,
            Err(error) if error.error_len().is_none() => error.valid_up_to(),
            Err(_) => return Err(failure("decompress", "segment is not UTF-8", Value::Null)),
        };
        for ch in std::str::from_utf8(&buffer[..valid])
            .expect("validated prefix")
            .chars()
        {
            if total < max_chars {
                preview.push(ch);
            }
            total += 1;
        }
        carry = len - valid;
        buffer.copy_within(valid..len, 0);
    }
    Ok((preview, total, total > max_chars))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    fn gzip(bytes: &[u8]) -> Vec<u8> {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(bytes).unwrap();
        encoder.finish().unwrap()
    }

    #[test]
    fn streaming_preview_counts_unicode_and_checks_the_entire_segment() {
        let mut input = vec![b'a'; 65539]; // split a multibyte character across buffer reads
        input.extend_from_slice("é🙂tail".as_bytes());
        let expected = "a".repeat(3);
        assert_eq!(
            decode_preview(&mut Cursor::new(gzip(&input)), 100000, 3).unwrap(),
            (expected, 65545, true)
        );
        assert_eq!(
            decode_preview(&mut Cursor::new("héllo".as_bytes()), 6, 2).unwrap(),
            ("hé".into(), 5, true)
        );
        input.push(0xff);
        let bad = decode_preview(&mut Cursor::new(gzip(&input)), 100000, 3).unwrap_err();
        assert_eq!(bad.structured.unwrap()["stage"], "decompress");
    }

    #[test]
    fn decode_refuses_expansion_past_cap_even_for_short_preview() {
        let compressed = gzip(&vec![b'a'; 10000]);
        let bad = decode_preview(&mut Cursor::new(&compressed), 9999, 1).unwrap_err();
        assert!(bad.into_text().contains("size limit"));
        assert_eq!(
            decode_preview(&mut Cursor::new(&compressed), 10000, 1).unwrap(),
            ("a".into(), 10000, true)
        );
        assert!(decode_preview(&mut Cursor::new([0xc3]), 10000, 1).is_err());
        assert_eq!(
            decode_preview(&mut Cursor::new([]), 0, 1).unwrap(),
            ("".into(), 0, false)
        );
    }
}
