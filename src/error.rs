use std::fmt;

#[derive(Debug)]
pub enum AnalyzerError {
    SolanaClientError(String),
    StorageError(String),
    ParseError(String),
    NetworkError(String),
    DataError(String),
}

impl std::error::Error for AnalyzerError {}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalyzerError::SolanaClientError(msg) => write!(f, "Solana client error: {}", msg),
            AnalyzerError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AnalyzerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AnalyzerError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AnalyzerError::DataError(msg) => write!(f, "Data error: {}", msg),
        }
    }
}

impl From<solana_client::client_error::ClientError> for AnalyzerError {
    fn from(error: solana_client::client_error::ClientError) -> Self {
        AnalyzerError::SolanaClientError(error.to_string())
    }
}

impl From<redis::RedisError> for AnalyzerError {
    fn from(error: redis::RedisError) -> Self {
        AnalyzerError::StorageError(error.to_string())
    }
}

impl From<serde_json::Error> for AnalyzerError {
    fn from(error: serde_json::Error) -> Self {
        AnalyzerError::ParseError(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AnalyzerError>;