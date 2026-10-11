//! Shared streaming transfer primitives. Callers retain their transfer permit and
//! enforce compressed/decompressed limits while consuming these readers.
use std::io::{self, Read, Seek, SeekFrom};

/// Default bounds shared by report transfers and diagnostic error responses.
pub struct TransferLimits;
impl TransferLimits {
    pub const CONCURRENT_TRANSFERS: usize = 2;
    pub const ERROR_PREFIX_BYTES: usize = 2 * 1024;
    pub const REPORT_BYTES: u64 = 256 * 1024 * 1024;
}

#[derive(Debug, PartialEq, Eq)]
pub struct ErrorPrefix {
    /// Lossy UTF-8 diagnostic text, limited in encoded bytes (not characters).
    pub text: String,
    /// The response or its lossy representation did not fit the byte bound.
    pub truncated: bool,
    /// URL-free diagnostic when a body read fails after a useful prefix arrived.
    pub read_error: Option<String>,
}

/// Read only a bounded response prefix, including one lookahead byte to
/// distinguish an exact-bound response from a truncated one. Invalid UTF-8 is
/// replaced, then clipped at a UTF-8 boundary so expansion cannot exceed limit.
pub async fn read_error_prefix(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<ErrorPrefix, reqwest::Error> {
    let bound = limit.saturating_add(1);
    let mut bytes = Vec::new();
    let mut read_error = None;
    while bytes.len() < bound {
        let chunk = match response.chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break,
            Err(error) if bytes.is_empty() => return Err(error),
            Err(error) => {
                read_error = Some(error.without_url().to_string());
                break;
            }
        };
        bytes.extend_from_slice(&chunk[..chunk.len().min(bound - bytes.len())]);
    }
    let mut truncated = bytes.len() > limit || read_error.is_some();
    bytes.truncate(limit);
    let mut text = String::from_utf8_lossy(&bytes).into_owned();
    if text.len() > limit {
        truncated = true;
        let mut end = limit;
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
    }
    Ok(ErrorPrefix {
        text,
        truncated,
        read_error,
    })
}

/// Sniff gzip magic and return an incremental decoder, including concatenated
/// gzip members. Corrupt gzip remains an I/O error during consumption. Plain
/// inputs are passed through without allocating their contents.
pub fn decoded_reader<'a, R: Read + Seek + 'a>(mut source: R) -> io::Result<Box<dyn Read + 'a>> {
    source.seek(SeekFrom::Start(0))?;
    let mut magic = [0u8; 2];
    let mut count = 0;
    while count < magic.len() {
        let read = source.read(&mut magic[count..])?;
        if read == 0 {
            break;
        }
        count += read;
    }
    source.seek(SeekFrom::Start(0))?;
    if count == 2 && magic == [0x1f, 0x8b] {
        Ok(Box::new(flate2::read::MultiGzDecoder::new(source)))
    } else {
        Ok(Box::new(source))
    }
}

/// Process-wide budget for expensive binary transfers, hashing and decoding.
/// JSON control-plane requests must not acquire it: workflows hold a permit
/// while using those requests to reserve/commit/reconcile their transfer.
pub fn shared_transfer_budget() -> std::sync::Arc<tokio::sync::Semaphore> {
    static BUDGET: std::sync::OnceLock<std::sync::Arc<tokio::sync::Semaphore>> =
        std::sync::OnceLock::new();
    BUDGET
        .get_or_init(|| {
            std::sync::Arc::new(tokio::sync::Semaphore::new(
                TransferLimits::CONCURRENT_TRANSFERS,
            ))
        })
        .clone()
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
    fn decoded_reader_handles_plain_gzip_members_and_corruption() {
        let mut members = gzip(b"first");
        members.extend(gzip(b"second"));
        for (bytes, expected) in [
            (b"plain".to_vec(), "plain"),
            (gzip(b"gzip"), "gzip"),
            (members, "firstsecond"),
            (vec![], ""),
        ] {
            let mut out = String::new();
            decoded_reader(Cursor::new(bytes))
                .unwrap()
                .read_to_string(&mut out)
                .unwrap();
            assert_eq!(out, expected);
        }
        let mut bad = gzip(b"broken");
        let len = bad.len();
        bad[len - 8] ^= 0xff; // corrupt CRC
        assert!(decoded_reader(Cursor::new(bad))
            .unwrap()
            .read_to_end(&mut Vec::new())
            .is_err());
        assert!(decoded_reader(Cursor::new([0x1f, 0x8b, 0]))
            .unwrap()
            .read_to_end(&mut Vec::new())
            .is_err());
    }
    #[test]
    fn decoded_reader_supports_borrowed_sources_and_short_reads() {
        struct Short<R>(R);
        impl<R: Read> Read for Short<R> {
            fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
                let count = out.len().min(1);
                self.0.read(&mut out[..count])
            }
        }
        impl<R: Seek> Seek for Short<R> {
            fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
                self.0.seek(pos)
            }
        }
        let mut source = Short(Cursor::new(gzip(b"borrowed")));
        source.seek(SeekFrom::Start(5)).unwrap();
        let mut reader = decoded_reader(&mut source).unwrap();
        let mut out = String::new();
        reader.read_to_string(&mut out).unwrap();
        assert_eq!(out, "borrowed");
    }
    #[tokio::test]
    async fn error_prefix_keeps_diagnostics_when_body_closes_early() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = [0; 1024];
            assert!(socket.read(&mut request).await.unwrap() > 0);
            socket
                .write_all(
                    b"HTTP/1.1 403 Forbidden\r\nContent-Length: 999\r\nConnection: close\r\n\r\n",
                )
                .await
                .unwrap();
            // Deliver a distinct chunk before the premature close so reqwest can
            // yield the useful prefix and then report the body framing error.
            socket
                .write_all(b"Apple vendor access denied")
                .await
                .unwrap();
            socket.flush().await.unwrap();
            tokio::time::sleep(std::time::Duration::from_millis(30)).await;
            socket.shutdown().await.unwrap();
        });
        let response = reqwest::Client::new()
            .get(format!("http://{address}/secret-token"))
            .send()
            .await
            .unwrap();
        let prefix = read_error_prefix(response, 100).await.unwrap();
        assert_eq!(prefix.text, "Apple vendor access denied");
        assert!(prefix.text.len() <= 100);
        assert!(prefix.truncated);
        let diagnostic = prefix.read_error.unwrap();
        assert!(!diagnostic.is_empty());
        assert!(!diagnostic.contains("secret-token"));
        assert!(!diagnostic.contains(&address.to_string()));
        peer.await.unwrap();
    }
    #[tokio::test]
    async fn error_prefix_bounds_lossy_utf8_and_detects_exact_boundary() {
        use wiremock::matchers::path;
        use wiremock::{Mock, MockServer, ResponseTemplate};
        let server = MockServer::start().await;
        let cases: &[(&str, &[u8], usize, &str, bool)] = &[
            ("ascii", b"abcd", 3, "abc", true),
            ("exact", b"abc", 3, "abc", false),
            ("utf8", "é🙂".as_bytes(), 3, "é", true),
            ("invalid", &[0xff, 0xff], 2, "", true),
            ("replacement", &[0xff], 3, "�", false),
            ("zero", b"x", 0, "", true),
            ("empty", b"", 0, "", false),
        ];
        for (name, bytes, limit, expected, truncated) in cases {
            Mock::given(path(format!("/{name}")))
                .respond_with(ResponseTemplate::new(403).set_body_bytes(bytes.to_vec()))
                .mount(&server)
                .await;
            let response = reqwest::Client::new()
                .get(format!("{}/{name}", server.uri()))
                .send()
                .await
                .unwrap();
            let prefix = read_error_prefix(response, *limit).await.unwrap();
            assert_eq!(
                prefix,
                ErrorPrefix {
                    text: expected.to_string(),
                    truncated: *truncated,
                    read_error: None,
                }
            );
            assert!(prefix.text.len() <= *limit);
        }
    }
}
