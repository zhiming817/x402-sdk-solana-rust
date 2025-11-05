mod payment;
mod request;

pub use payment::*;
pub use request::*;

use serde::{Deserialize, Serialize};

/// Supported networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Network {
    SolanaLocalnet,
    SolanaDevnet,
    Solana,
}

/// X402 Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X402Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svm_config: Option<SvmConfig>,
}

/// Solana VM Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_token: Option<TokenConfig>,
}

/// Token Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub address: String,
    pub decimals: u8,
    pub name: String,
}

/// Facilitator Configuration
pub struct FacilitatorConfig {
    pub url: String,
    pub create_auth_headers: Option<Box<dyn Fn() -> AuthHeaders + Send + Sync>>,
}

impl Clone for FacilitatorConfig {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            // Cannot clone function pointer, set to None
            create_auth_headers: None,
        }
    }
}

impl std::fmt::Debug for FacilitatorConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FacilitatorConfig")
            .field("url", &self.url)
            .field("create_auth_headers", &self.create_auth_headers.is_some())
            .finish()
    }
}

/// Authentication Headers
#[derive(Debug, Clone, Default)]
pub struct AuthHeaders {
    pub verify: Option<std::collections::HashMap<String, String>>,
    pub settle: Option<std::collections::HashMap<String, String>>,
    pub supported: Option<std::collections::HashMap<String, String>>,
}

/// Payment scheme
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentScheme {
    Exact,
}

/// Route Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub price: String,
    pub network: Network,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timeout_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
}

impl Default for X402Config {
    fn default() -> Self {
        Self {
            svm_config: None,
        }
    }
}