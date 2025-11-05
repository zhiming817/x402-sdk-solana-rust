use serde::{Deserialize, Serialize};
use super::{Network, PaymentScheme};

/// Payment Requirements returned by the server in 402 response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequirements {
    pub x402_version: u8,
    pub scheme: PaymentScheme,
    pub network: Network,
    pub max_amount_required: String,
    pub pay_to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_decimals: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// Payment Payload sent by client with payment proof
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPayload {
    pub x402_version: u8,
    pub scheme: PaymentScheme,
    pub network: Network,
    pub signed_transaction: String, // Base64 encoded signed transaction
    pub from: String, // Sender's public key
}

/// Verify Response from facilitator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    pub verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Settle Response from facilitator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettleResponse {
    pub signature: String,
    pub settled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Supported Payment Kind
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedPaymentKind {
    pub x402_version: u8,
    pub scheme: PaymentScheme,
    pub network: Network,
}

/// Supported Payment Kinds Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedPaymentKindsResponse {
    pub kinds: Vec<SupportedPaymentKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub amount: u64,
    pub recipient: String,
}