use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub request_id: String,
    pub data: String,
}

/// HTTP Request with Payment Information
#[derive(Debug, Clone)]
pub struct X402Request {
    pub url: String,
    pub method: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

/// HTTP Response with Payment Information
#[derive(Debug, Clone)]
pub struct X402Response {
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Vec<u8>,
}