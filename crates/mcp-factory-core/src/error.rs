use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("tool not found: {0}")]
    ToolNotFound(String),

    #[error("duplicate tool name: {0}")]
    DuplicateTool(String),

    #[error("resource not found: {0}")]
    ResourceNotFound(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("invalid configuration: {0}")]
    Config(String),

    #[error("transport error: {0}")]
    Transport(String),

    #[error("{0}")]
    Other(String),
}

impl From<ProxyError> for rmcp::ErrorData {
    fn from(err: ProxyError) -> Self {
        // Client-caused errors map to invalid_params (-32602); everything else
        // is an internal error (-32603).
        match err {
            ProxyError::ToolNotFound(_)
            | ProxyError::ResourceNotFound(_)
            | ProxyError::Validation(_) => rmcp::ErrorData::invalid_params(err.to_string(), None),
            _ => rmcp::ErrorData::internal_error(err.to_string(), None),
        }
    }
}
