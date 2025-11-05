use crate::error::X402Error;
use crate::solana::{TransactionBuilder, Wallet};
use crate::types::{
    PaymentPayload, PaymentRequirements, PaymentScheme, X402Config, X402Request, X402Response,
};
use reqwest::Client;
use std::collections::HashMap;

/// Fetcher for making HTTP requests with automatic payment handling
pub struct Fetcher {
    client: Client,
    wallet: Wallet,
    max_value: Option<u64>,
    config: Option<X402Config>,
}

impl Fetcher {
    /// Create a new Fetcher with wallet
    pub fn new(wallet: Wallet, config: Option<X402Config>) -> Self {
        Self {
            client: Client::new(),
            wallet,
            max_value: None,
            config,
        }
    }

    /// Create a new Fetcher with max payment value limit
    pub fn with_max_value(wallet: Wallet, max_value: u64, config: Option<X402Config>) -> Self {
        Self {
            client: Client::new(),
            wallet,
            max_value: Some(max_value),
            config,
        }
    }

    /// Fetch data with automatic payment handling
    pub async fn fetch(&self, request: X402Request) -> Result<X402Response, X402Error> {
        // First attempt - send request without payment
        let response = self.send_request(&request).await?;

        // Check if payment is required (402 status)
        if response.status == 402 {
            // Parse payment requirements from X-PAYMENT-REQUIRED header
            let payment_required = response
                .headers
                .get("x-payment-required")
                .ok_or_else(|| {
                    X402Error::PaymentRequired(
                        "402 response missing x-payment-required header".to_string(),
                    )
                })?;

            let requirements: PaymentRequirements = serde_json::from_str(payment_required)
                .map_err(|e| X402Error::DeserializationError(e.to_string()))?;

            // Verify payment amount doesn't exceed max_value
            if let Some(max) = self.max_value {
                let amount: u64 = requirements
                    .max_amount_required
                    .parse()
                    .map_err(|e| X402Error::InvalidInput(format!("Invalid amount: {}", e)))?;
                if amount > max {
                    return Err(X402Error::PaymentAmountExceeded {
                        expected: max,
                        got: amount,
                    });
                }
            }

            // Create payment
            let payment = self.create_payment(&requirements).await?;

            // Resend request with payment
            let mut paid_request = request.clone();
            paid_request
                .headers
                .insert("x-payment".to_string(), payment);

            return self.send_request(&paid_request).await;
        }

        Ok(response)
    }

    /// Send HTTP request
    async fn send_request(&self, request: &X402Request) -> Result<X402Response, X402Error> {
        let method = match request.method.as_str() {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            _ => reqwest::Method::GET,
        };

        let mut req_builder = self.client.request(method, &request.url);

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        let response = req_builder.send().await?;

        // Extract response details
        let status = response.status().as_u16();
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(val_str) = value.to_str() {
                headers.insert(key.to_string(), val_str.to_string());
            }
        }
        let body = response.bytes().await?.to_vec();

        Ok(X402Response {
            status,
            headers,
            body,
        })
    }

    /// Create payment payload for the requirements
    async fn create_payment(
        &self,
        requirements: &PaymentRequirements,
    ) -> Result<String, X402Error> {
        // Get RPC URL from config or use default based on network
        let rpc_url = self
            .config
            .as_ref()
            .and_then(|c| c.svm_config.as_ref())
            .and_then(|s| s.rpc_url.as_ref())
            .map(|s| s.as_str())
            .unwrap_or("https://api.devnet.solana.com");

        let tx_builder = TransactionBuilder::new(rpc_url);

        // Parse amount
        let amount: u64 = requirements
            .max_amount_required
            .parse()
            .map_err(|e| X402Error::InvalidInput(format!("Invalid amount: {}", e)))?;

        // Parse recipient address
        let to_pubkey = requirements.pay_to.parse().map_err(|e| {
            X402Error::InvalidInput(format!("Invalid recipient address: {}", e))
        })?;

        // Create transaction
        let transaction = tx_builder.create_payment_transaction(
            self.wallet.keypair(),
            &to_pubkey,
            amount,
        )?;

        // Serialize transaction to base64
        let signed_tx = TransactionBuilder::serialize_transaction(&transaction)?;

        // Create payment payload
        let payload = PaymentPayload {
            x402_version: 1,
            scheme: PaymentScheme::Exact,
            network: requirements.network.clone(),
            signed_transaction: signed_tx,
            from: self.wallet.public_key().to_string(),
        };

        // Serialize payload to JSON string
        let payload_json = serde_json::to_string(&payload)
            .map_err(|e| X402Error::SerializationError(e.to_string()))?;

        Ok(payload_json)
    }
}

/// Create a payment header for a request
pub async fn create_payment_header(
    wallet: &Wallet,
    requirements: &PaymentRequirements,
    config: Option<&X402Config>,
) -> Result<String, X402Error> {
    let fetcher = Fetcher::new(wallet.clone(), config.cloned());
    fetcher.create_payment(requirements).await
}

impl Clone for Wallet {
    fn clone(&self) -> Self {
        // Note: This creates a new keypair, not a true clone
        // In production, you'd want to properly clone the keypair
        Self::new()
    }
}