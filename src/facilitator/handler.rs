use crate::error::X402Error;
use crate::types::{
    PaymentPayload, PaymentRequirements, SettleResponse, VerifyResponse,
    SupportedPaymentKindsResponse, FacilitatorConfig,
};
use reqwest::Client;
use serde_json::json;

const DEFAULT_FACILITATOR_URL: &str = "https://x402.org/facilitator";

/// Facilitator Handler for payment verification and settlement
pub struct Handler {
    client: Client,
    config: Option<FacilitatorConfig>,
}

impl Handler {
    /// Create a new Handler with optional facilitator configuration
    pub fn new(config: Option<FacilitatorConfig>) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Get the facilitator URL
    fn get_url(&self) -> &str {
        self.config
            .as_ref()
            .map(|c| c.url.as_str())
            .unwrap_or(DEFAULT_FACILITATOR_URL)
    }

    /// Verify a payment payload against payment requirements
    pub async fn verify(
        &self,
        payload: &PaymentPayload,
        requirements: &PaymentRequirements,
    ) -> Result<VerifyResponse, X402Error> {
        let url = format!("{}/verify", self.get_url());
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        // Add custom auth headers if provided
        if let Some(config) = &self.config {
            if let Some(create_headers) = &config.create_auth_headers {
                let auth_headers = create_headers();
                if let Some(verify_headers) = auth_headers.verify {
                    for (key, value) in verify_headers {
                        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&value) {
                                headers.insert(header_name, header_value);
                            }
                        }
                    }
                }
            }
        }

        let body = json!({
            "paymentPayload": payload,
            "paymentRequirements": requirements,
        });

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(X402Error::HttpError(format!(
                "Verify request failed with status: {}",
                response.status()
            )));
        }

        let verify_response: VerifyResponse = response.json().await?;
        Ok(verify_response)
    }

    /// Settle a payment by submitting the transaction to the blockchain
    pub async fn settle(
        &self,
        payload: &PaymentPayload,
        requirements: &PaymentRequirements,
    ) -> Result<SettleResponse, X402Error> {
        let url = format!("{}/settle", self.get_url());
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        // Add custom auth headers if provided
        if let Some(config) = &self.config {
            if let Some(create_headers) = &config.create_auth_headers {
                let auth_headers = create_headers();
                if let Some(settle_headers) = auth_headers.settle {
                    for (key, value) in settle_headers {
                        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&value) {
                                headers.insert(header_name, header_value);
                            }
                        }
                    }
                }
            }
        }

        let body = json!({
            "paymentPayload": payload,
            "paymentRequirements": requirements,
        });

        let response = self.client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(X402Error::HttpError(format!(
                "Settle request failed with status: {}",
                response.status()
            )));
        }

        let settle_response: SettleResponse = response.json().await?;
        Ok(settle_response)
    }

    /// Get supported payment kinds from the facilitator
    pub async fn supported(&self) -> Result<SupportedPaymentKindsResponse, X402Error> {
        let url = format!("{}/supported", self.get_url());
        
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            "application/json".parse().unwrap(),
        );

        // Add custom auth headers if provided
        if let Some(config) = &self.config {
            if let Some(create_headers) = &config.create_auth_headers {
                let auth_headers = create_headers();
                if let Some(supported_headers) = auth_headers.supported {
                    for (key, value) in supported_headers {
                        if let Ok(header_name) = reqwest::header::HeaderName::from_bytes(key.as_bytes()) {
                            if let Ok(header_value) = reqwest::header::HeaderValue::from_str(&value) {
                                headers.insert(header_name, header_value);
                            }
                        }
                    }
                }
            }
        }

        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(X402Error::HttpError(format!(
                "Supported request failed with status: {}",
                response.status()
            )));
        }

        let supported_response: SupportedPaymentKindsResponse = response.json().await?;
        Ok(supported_response)
    }
}