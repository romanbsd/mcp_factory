//! Independent fixtures for typed upload identities and resume reconciliation.
use std::sync::{Arc, Mutex};
use std::time::Duration;

use mcp_factory_core::{async_trait, OperationInvoker, ProxyConfig, ProxyError, ToolResult};
use serde_json::{json, Value};
use tokio::sync::Semaphore;

use super::asset_upload::AssetUpload;
use super::upload_session::{
    AssetState, FileIdentity, PreparedFile, ResumeCheckpoint, UploadRange,
};

const HELLO_MD5: &str = "5d41402abc4b2a76b9719d911017c592";
const TWO_GIB: u64 = 2 * 1024 * 1024 * 1024;

fn identity() -> FileIdentity {
    FileIdentity {
        file: "nested/hello.txt".into(),
        file_name: "hello.txt".into(),
        file_size: 5,
        checksum: HELLO_MD5.into(),
    }
}
fn checkpoint() -> Value {
    json!({"assetId": "shot-1", "fileSize": 5,
        "sourceFileChecksum": "5D41402ABC4B2A76B9719D911017C592", "completedRanges": []})
}
fn error_body(error: ToolResult) -> Value {
    assert!(error.is_error);
    error.structured.expect("structured upload error")
}

#[test]
fn legacy_checkpoint_accepts_known_identity_and_case_insensitive_md5() {
    let value = checkpoint();
    let parsed = ResumeCheckpoint::parse(&value).unwrap();
    parsed
        .validate_file("appScreenshot", "set-1", &identity())
        .unwrap();
    assert_eq!(parsed.asset_id, "shot-1");
    assert!(parsed.kind.is_none());
    assert_eq!(serde_json::to_value(&parsed).unwrap(), value);
    for (key, wrong) in [
        ("assetId", json!("")),
        ("fileSize", json!(6)),
        (
            "sourceFileChecksum",
            json!("00000000000000000000000000000000"),
        ),
        ("kind", json!("appPreview")),
        ("parentId", json!("other-set")),
        ("file", json!("other.txt")),
        ("fileName", json!("other.txt")),
    ] {
        let mut invalid = value.clone();
        invalid[key] = wrong;
        let parsed = ResumeCheckpoint::parse(&invalid).unwrap();
        let error = error_body(
            parsed
                .validate_file("appScreenshot", "set-1", &identity())
                .unwrap_err(),
        );
        assert_eq!(error["stage"], "validate", "identity constraint {key}");
        assert!(error["message"]
            .as_str()
            .unwrap()
            .contains("does not match"));
    }
}

#[test]
fn present_optional_identity_and_checkpoint_fields_reject_null_and_wrong_types() {
    for (key, wrong_type) in [
        ("kind", json!(1)),
        ("parentId", json!(false)),
        ("file", json!([])),
        ("fileName", json!({})),
        ("state", json!(7)),
        ("committed", json!("true")),
        ("pendingRanges", json!("0")),
        ("ranges", json!({"offset": 0, "length": 5})),
    ] {
        for invalid in [Value::Null, wrong_type] {
            let mut value = checkpoint();
            value[key] = invalid;
            let error = error_body(ResumeCheckpoint::parse(&value).unwrap_err());
            assert_eq!(error["stage"], "validate", "provided field {key}");
            assert_eq!(error["message"], "invalid resume checkpoint");
        }
    }
}

#[test]
fn checkpoint_parser_requires_identity_and_structurally_valid_range_indices() {
    for key in [
        "assetId",
        "fileSize",
        "sourceFileChecksum",
        "completedRanges",
    ] {
        let mut value = checkpoint();
        value.as_object_mut().unwrap().remove(key);
        assert_eq!(
            error_body(ResumeCheckpoint::parse(&value).unwrap_err())["message"],
            "invalid resume checkpoint"
        );
    }
    for completed in [
        json!([-1]),
        json!(["0"]),
        json!([null]),
        json!([0.5]),
        json!({}),
    ] {
        let mut value = checkpoint();
        value["completedRanges"] = completed;
        assert!(ResumeCheckpoint::parse(&value).is_err());
    }
    for ranges in [
        json!([{"offset": -1, "length": 5}]),
        json!([{"offset": "0", "length": 5}]),
        json!([{"offset": 0, "length": -5}]),
    ] {
        let mut value = checkpoint();
        value["ranges"] = ranges;
        assert!(ResumeCheckpoint::parse(&value).is_err());
    }
}

#[test]
fn apple_range_constructor_and_validation_enforce_file_and_transfer_limits() {
    let valid = UploadRange::from_operation(&json!({"offset": 2, "length": 3})).unwrap();
    assert_eq!(valid.validate(5).unwrap(), (2, 3));
    assert_eq!(
        UploadRange::from_operation(&json!({"length": 5}))
            .unwrap()
            .validate(5)
            .unwrap(),
        (0, 5)
    );
    for offset in [json!(-1), json!(null), json!("2"), json!(1.5)] {
        assert!(
            UploadRange::from_operation(&json!({"offset": offset, "length": 3}))
                .unwrap_err()
                .contains("invalid offset")
        );
    }
    for operation in [
        json!({"offset": 0}),
        json!({"offset": 0, "length": null}),
        json!({"offset": 0, "length": -1}),
        json!({"offset": 0, "length": "5"}),
    ] {
        assert!(UploadRange::from_operation(&operation)
            .unwrap()
            .validate(5)
            .unwrap_err()
            .contains("no length"));
    }
    for (operation, size) in [
        (json!({"offset": 2, "length": 4}), 5),
        (json!({"offset": u64::MAX, "length": 1}), 5),
        (json!({"offset": 0, "length": TWO_GIB + 1}), TWO_GIB),
        (json!({"offset": 0, "length": 5}), TWO_GIB + 1),
    ] {
        assert!(UploadRange::from_operation(&operation)
            .unwrap()
            .validate(size)
            .is_err());
    }
    assert_eq!(
        UploadRange::from_operation(&json!({"offset": 0, "length": TWO_GIB}))
            .unwrap()
            .validate(TWO_GIB)
            .unwrap(),
        (0, TWO_GIB)
    );
}

#[test]
fn delivery_states_preserve_wire_names_and_only_committed_states_are_committed() {
    for (wire, expected, committed) in [
        ("AWAITING_UPLOAD", AssetState::AwaitingUpload, false),
        ("UPLOAD_COMPLETE", AssetState::UploadComplete, true),
        ("PROCESSING", AssetState::Processing, true),
        ("COMPLETE", AssetState::Complete, true),
        ("FAILED", AssetState::Failed, false),
        (
            "APPLE_NEW_STATE",
            AssetState::Unknown("APPLE_NEW_STATE".into()),
            false,
        ),
    ] {
        let state = AssetState::from_resource(
            &json!({"attributes": {"assetDeliveryState": {"state": wire}}}),
        );
        assert_eq!(state, expected);
        assert_eq!(state.committed(), committed, "state {wire}");
        assert_eq!(state.as_str(), wire);
        assert_eq!(serde_json::to_value(&state).unwrap(), json!(wire));
    }
    for resource in [
        json!({}),
        json!({"attributes": {"assetDeliveryState": {"state": null}}}),
    ] {
        assert_eq!(
            AssetState::from_resource(&resource),
            AssetState::Unknown("UNKNOWN".into())
        );
    }
}

#[tokio::test]
async fn prepared_file_inspection_hashes_known_bytes_and_validates_root_and_size() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("media");
    std::fs::create_dir_all(root.join("nested")).unwrap();
    std::fs::write(root.join("nested/hello.txt"), b"hello").unwrap();
    std::fs::write(dir.path().join("outside.txt"), b"outside").unwrap();
    let prepared = PreparedFile::inspect(Some(&root), "nested/hello.txt", None)
        .await
        .unwrap();
    assert_eq!(prepared.identity.file, "nested/hello.txt");
    assert_eq!(prepared.identity.file_name, "hello.txt");
    assert_eq!(prepared.identity.file_size, 5);
    assert_eq!(prepared.identity.checksum, HELLO_MD5);
    assert_eq!(
        prepared.path,
        root.join("nested/hello.txt").canonicalize().unwrap()
    );
    let renamed = PreparedFile::inspect(Some(&root), "nested/hello.txt", Some("store-name.txt"))
        .await
        .unwrap();
    assert_eq!(renamed.identity.file_name, "store-name.txt");
    assert_eq!(renamed.identity.checksum, HELLO_MD5);
    let outside = match PreparedFile::inspect(Some(&root), "../outside.txt", None).await {
        Err(error) => error_body(error),
        Ok(_) => panic!("outside-root file was accepted"),
    };
    assert_eq!(outside["stage"], "validate");
    let sparse = std::fs::File::create(root.join("oversized.bin")).unwrap();
    sparse.set_len(TWO_GIB + 1).unwrap();
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        PreparedFile::inspect(Some(&root), "oversized.bin", None),
    )
    .await
    .expect("oversized file must be rejected before hashing 2 GiB");
    let error = match result {
        Err(error) => error_body(error),
        Ok(_) => panic!("oversized file was accepted"),
    };
    assert_eq!(error["message"], "file exceeds the 2 GiB upload limit");
    assert_eq!(error["details"]["fileSize"], TWO_GIB + 1);
}

struct ResumeReader {
    resource: Value,
    reads: Mutex<Vec<(String, Value)>>,
}
#[async_trait]
impl OperationInvoker for ResumeReader {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.reads.lock().unwrap().push((name.to_owned(), args));
        Ok(super::success(json!({"data": self.resource})))
    }
    async fn invoke_mutating(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
        panic!("resume inspection must remain read-only")
    }
}
fn remote() -> Value {
    json!({"id": "shot-1", "type": "appScreenshots",
        "relationships": {"appScreenshotSet": {"data": {"id": "set-1", "type": "appScreenshotSets"}}},
        "attributes": {"fileName": "hello.txt", "fileSize": 5, "sourceFileChecksum": HELLO_MD5,
            "assetDeliveryState": {"state": "AWAITING_UPLOAD"},
            "uploadOperations": [{"url": "https://upload.example.com/part", "method": "PUT", "offset": 0, "length": 5}]}})
}

#[tokio::test]
async fn shared_resume_inspector_checks_remote_identity_state_and_range_plan_read_only() {
    let upload = AssetUpload::new(
        Ok(reqwest::Client::new()),
        &ProxyConfig::default(),
        Arc::new(Semaphore::new(2)),
    );
    let reader = ResumeReader {
        resource: remote(),
        reads: Mutex::new(vec![]),
    };
    let inspected = upload
        .inspect_resume(
            &reader,
            "appScreenshot",
            "set-1",
            &identity(),
            &checkpoint(),
        )
        .await
        .unwrap();
    assert_eq!(inspected.resource["id"], "shot-1");
    assert_eq!(
        *reader.reads.lock().unwrap(),
        vec![(
            "appScreenshots_getInstance".into(),
            json!({"id": "shot-1", "include": ["appScreenshotSet"]})
        )]
    );
    let cases = [
        (
            "/relationships/appScreenshotSet/data/id",
            json!("other-set"),
            "resume",
        ),
        (
            "/attributes/assetDeliveryState/state",
            json!("FUTURE_STATE"),
            "resume",
        ),
        (
            "/attributes/assetDeliveryState/state",
            json!("FAILED"),
            "processing",
        ),
        ("/attributes/uploadOperations/0/offset", json!(-1), "resume"),
        ("/attributes/uploadOperations/0/length", json!(6), "resume"),
    ];
    for (pointer, replacement, stage) in cases {
        let mut resource = remote();
        *resource.pointer_mut(pointer).unwrap() = replacement;
        let reader = ResumeReader {
            resource,
            reads: Mutex::new(vec![]),
        };
        let result = upload
            .inspect_resume(
                &reader,
                "appScreenshot",
                "set-1",
                &identity(),
                &checkpoint(),
            )
            .await;
        let error = match result {
            Err(error) => error_body(error),
            Ok(_) => panic!("invalid remote fixture accepted: {pointer}"),
        };
        assert_eq!(error["stage"], stage, "fixture {pointer}");
        assert_eq!(error["resume"], checkpoint());
        assert_eq!(reader.reads.lock().unwrap().len(), 1);
    }
    for (completed, ranges) in [
        (json!([0, 0]), json!([{"offset": 0, "length": 5}])),
        (json!([1]), json!([{"offset": 0, "length": 5}])),
        (json!([0]), json!([{"offset": 1, "length": 4}])),
    ] {
        let mut value = checkpoint();
        value["completedRanges"] = completed;
        value["ranges"] = ranges;
        let reader = ResumeReader {
            resource: remote(),
            reads: Mutex::new(vec![]),
        };
        assert!(upload
            .inspect_resume(&reader, "appScreenshot", "set-1", &identity(), &value)
            .await
            .is_err());
        assert_eq!(reader.reads.lock().unwrap().len(), 1);
    }
}
