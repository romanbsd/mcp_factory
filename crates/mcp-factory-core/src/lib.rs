//! MCP Factory core runtime: proxy MCP tools to REST and GraphQL backends.

pub mod auth;
pub mod config;
pub mod error;
pub mod graphql;
pub mod resources;
pub mod rest;
pub mod server;
pub mod tools;
pub mod transport;

pub use auth::{oauth_logout, oauth_status, run_oauth_login, AuthProvider};
pub use config::{AuthConfig, ProxyConfig, TransportMode};
pub use error::ProxyError;
pub use graphql::{GraphQLOperation, GraphQLProxyExecutor};
pub use resources::ResourceSpec;
pub use rest::{ParamBinding, ParamLocation, RestOperation, RestProxyExecutor};
pub use server::McpProxyServer;
pub use tools::{ExecutionKind, ToolRegistry, ToolSpec};
