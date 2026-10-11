//! Lightweight generated capabilities. Schemas are initialized on first use,
//! independently of which operations are exposed over MCP.
use crate::{ProxyError, ToolSpec};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub struct OperationDescriptor {
    pub name: &'static str,
    pub build: fn() -> ToolSpec,
}
struct OperationEntry {
    build: fn() -> ToolSpec,
    spec: OnceLock<ToolSpec>,
    validator: OnceLock<Result<jsonschema::Validator, String>>,
}
#[derive(Default)]
pub(crate) struct OperationRegistry {
    entries: HashMap<&'static str, OperationEntry>,
}
impl OperationRegistry {
    pub fn register(&mut self, descriptors: &[OperationDescriptor]) -> Result<(), ProxyError> {
        for descriptor in descriptors {
            if self.entries.contains_key(descriptor.name) {
                return Err(ProxyError::DuplicateTool(descriptor.name.into()));
            }
            self.entries.insert(
                descriptor.name,
                OperationEntry {
                    build: descriptor.build,
                    spec: OnceLock::new(),
                    validator: OnceLock::new(),
                },
            );
        }
        Ok(())
    }
    pub fn names(&self) -> impl Iterator<Item = &&'static str> {
        self.entries.keys()
    }
    pub fn get(&self, name: &str) -> Option<&ToolSpec> {
        self.entries
            .get(name)
            .map(|entry| entry.spec.get_or_init(entry.build))
    }
    pub fn ensure_schema(&self, name: &str) -> Result<&jsonschema::Validator, ProxyError> {
        let entry = self
            .entries
            .get(name)
            .ok_or_else(|| ProxyError::ToolNotFound(name.into()))?;
        let spec = entry.spec.get_or_init(entry.build);
        if spec.name != name {
            return Err(ProxyError::Config(format!(
                "operation descriptor {name} constructed {}",
                spec.name
            )));
        }
        let validator = entry.validator.get_or_init(|| {
            jsonschema::validator_for(&spec.input_schema).map_err(|e| e.to_string())
        });
        match validator {
            Ok(validator) => Ok(validator),
            Err(error) => Err(ProxyError::Validation(format!(
                "invalid operation schema for {name}: {error}"
            ))),
        }
    }
    pub fn validate(&self, name: &str, args: &Value) -> Result<(), ProxyError> {
        self.ensure_schema(name)?
            .validate(args)
            .map_err(|e| ProxyError::Validation(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        async_trait, CustomToolHandler, CustomToolSpec, ExecutionKind, McpProxyServer,
        OperationInvoker, ProxyConfig, RestOperation, ToolHints, ToolResult,
    };
    use serde_json::json;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    static BUILDS: AtomicUsize = AtomicUsize::new(0);
    fn hidden() -> ToolSpec {
        BUILDS.fetch_add(1, Ordering::SeqCst);
        ToolSpec {
            name: "hidden_read".into(),
            description: "Read hidden operation".into(),
            input_schema: json!({"type":"object","properties":{"id":{"type":"string"}},"required":["id"]}),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".into(),
                path_template: "/hidden".into(),
                params: vec![],
                body_fields: vec![],
                content_type: None,
                raw_body: false,
                media: None,
            }),
            hints: ToolHints {
                read_only: Some(true),
                ..Default::default()
            },
        }
    }
    fn hidden_write() -> ToolSpec {
        let mut spec = hidden();
        spec.name = "hidden_write".into();
        spec.hints.read_only = Some(false);
        spec.execution = ExecutionKind::Rest(RestOperation {
            method: "POST".into(),
            path_template: "/hidden".into(),
            params: vec![],
            body_fields: vec![],
            content_type: None,
            raw_body: false,
            media: None,
        });
        spec
    }
    struct Probe;
    #[async_trait]
    impl CustomToolHandler for Probe {
        async fn call(
            &self,
            invoker: &dyn OperationInvoker,
            args: Value,
        ) -> Result<ToolResult, ProxyError> {
            if args["attemptStreamWrite"] == true {
                let response = invoker
                    .stream_read_only("hidden_write", json!({"id":"x"}))
                    .await?;
                return Ok(ToolResult::text(response.status().to_string()));
            }
            if args["attemptWrite"] == true {
                return invoker
                    .invoke_mutating("hidden_write", json!({"id":"x"}))
                    .await;
            }
            invoker.invoke_read_only("hidden_read", args).await
        }
    }
    #[tokio::test]
    async fn hidden_catalog_is_lazy_validated_and_not_public_and_readonly_scope_is_enforced() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};
        BUILDS.store(0, Ordering::SeqCst);
        let api = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/hidden"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"value":42})))
            .expect(1)
            .mount(&api)
            .await;
        let catalog = [
            OperationDescriptor {
                name: "hidden_read",
                build: hidden,
            },
            OperationDescriptor {
                name: "hidden_write",
                build: hidden_write,
            },
        ];
        let probe = CustomToolSpec {
            name: "probe".into(),
            description: "Read-only workflow".into(),
            input_schema: json!({"type":"object"}),
            hints: ToolHints {
                read_only: Some(true),
                ..Default::default()
            },
            handler: Arc::new(Probe),
        };
        let server = McpProxyServer::builder(ProxyConfig {
            base_url: api.uri(),
            profiles: Some(vec!["workflow".into()]),
            ..Default::default()
        })
        .operation_catalog(&catalog)
        .unwrap()
        .tool_profiles(&[
            ("hidden_read", &["raw"]),
            ("hidden_write", &["raw"]),
            ("probe", &["workflow"]),
        ])
        .unwrap()
        .custom_tools(&[probe])
        .unwrap()
        .build()
        .unwrap();
        assert_eq!(
            BUILDS.load(Ordering::SeqCst),
            0,
            "discovery must not construct hidden schemas"
        );
        assert_eq!(server.tool_names(), vec!["probe"]);
        assert!(matches!(
            server.invoke_tool("hidden_read", json!({"id":"x"})).await,
            Err(ProxyError::ToolNotFound(_))
        ));
        assert!(server
            .invoke_tool("probe", json!({}))
            .await
            .unwrap_err()
            .to_string()
            .contains("required"));
        assert_eq!(BUILDS.load(Ordering::SeqCst), 1);
        let value: Value = serde_json::from_str(
            &server
                .invoke_tool("probe", json!({"id":"x"}))
                .await
                .unwrap(),
        )
        .unwrap();
        assert_eq!(value["value"], 42);
        assert_eq!(
            BUILDS.load(Ordering::SeqCst),
            1,
            "operation/schema reused after validation"
        );
        assert!(server
            .invoke_tool("probe", json!({"attemptWrite":true}))
            .await
            .unwrap_err()
            .to_string()
            .contains("read-only"));
        assert!(server
            .invoke_tool("probe", json!({"attemptStreamWrite":true}))
            .await
            .unwrap_err()
            .to_string()
            .contains("read-only"));
        assert_eq!(api.received_requests().await.unwrap().len(), 1);
    }
}
