use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::error::ProxyError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
    pub content: &'static str,
}

#[derive(Debug, Default)]
pub struct ResourceRegistry {
    resources: HashMap<String, ResourceSpec>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, resource: ResourceSpec) -> Result<(), ProxyError> {
        if self.resources.contains_key(&resource.uri) {
            return Err(ProxyError::Other(format!(
                "duplicate resource uri: {}",
                resource.uri
            )));
        }
        self.resources.insert(resource.uri.clone(), resource);
        Ok(())
    }

    pub fn register_many(
        &mut self,
        resources: impl IntoIterator<Item = ResourceSpec>,
    ) -> Result<(), ProxyError> {
        for resource in resources {
            self.register(resource)?;
        }
        Ok(())
    }

    pub fn get(&self, uri: &str) -> Option<&ResourceSpec> {
        self.resources.get(uri)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ResourceSpec> {
        self.resources.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_registered_resource() {
        let mut registry = ResourceRegistry::new();
        registry
            .register(ResourceSpec {
                uri: "schema://openapi".to_string(),
                name: "openapi".to_string(),
                description: "schema".to_string(),
                mime_type: "application/yaml".to_string(),
                content: "openapi: 3.0.0",
            })
            .unwrap();
        assert_eq!(
            registry.get("schema://openapi").unwrap().content,
            "openapi: 3.0.0"
        );
    }
}
