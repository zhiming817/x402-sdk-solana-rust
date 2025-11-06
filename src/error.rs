use std::fmt;

#[derive(Debug)]
pub enum X402Error {
    InvalidInput(String),
    NotFound(String),
    Unauthorized(String),
    InternalError(String),
    NetworkError(String),
    PaymentRequired(String),
    PaymentVerificationFailed(String),
    PaymentAmountExceeded { expected: u64, got: u64 },
    InvalidSignature(String),
    SerializationError(String),
    DeserializationError(String),
    SolanaError(String),
    HttpError(String),
    NotImplemented(String),
}

impl fmt::Display for X402Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X402Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            X402Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            X402Error::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            X402Error::InternalError(msg) => write!(f, "Internal error: {}", msg),
            X402Error::NetworkError(msg) => write!(f, "Network error: {}", msg),
            X402Error::PaymentRequired(msg) => write!(f, "Payment required: {}", msg),
            X402Error::PaymentVerificationFailed(msg) => {
                write!(f, "Payment verification failed: {}", msg)
            }
            X402Error::PaymentAmountExceeded { expected, got } => {
                write!(f, "Payment amount exceeded: expected {}, got {}", expected, got)
            }
            X402Error::InvalidSignature(msg) => write!(f, "Invalid signature: {}", msg),
            X402Error::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            X402Error::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            X402Error::SolanaError(msg) => write!(f, "Solana error: {}", msg),
            X402Error::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            X402Error::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl std::error::Error for X402Error {}

impl From<serde_json::Error> for X402Error {
    fn from(err: serde_json::Error) -> Self {
        X402Error::SerializationError(err.to_string())
    }
}

impl From<reqwest::Error> for X402Error {
    fn from(err: reqwest::Error) -> Self {
        X402Error::HttpError(err.to_string())
    }
}

impl From<solana_sdk::signature::SignerError> for X402Error {
    fn from(err: solana_sdk::signature::SignerError) -> Self {
        X402Error::InvalidSignature(err.to_string())
    }
}

impl From<solana_client::client_error::ClientError> for X402Error {
    fn from(err: solana_client::client_error::ClientError) -> Self {
        X402Error::SolanaError(err.to_string())
    }
}