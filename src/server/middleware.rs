use crate::facilitator::Handler;
use crate::types::{
    FacilitatorConfig, PaymentPayload, PaymentRequirements, PaymentScheme, RouteConfig,
    X402Config,
};
use actix_web::{Error, HttpRequest, HttpResponse};
use std::collections::HashMap;

/// Payment middleware configuration
pub struct PaymentMiddlewareConfig {
    pub pay_to: String,
    pub routes: HashMap<String, RouteConfig>,
    pub facilitator: Option<FacilitatorConfig>,
    pub x402_config: Option<X402Config>,
}

impl PaymentMiddlewareConfig {
    pub fn new(
        pay_to: String,
        routes: HashMap<String, RouteConfig>,
        facilitator: Option<FacilitatorConfig>,
        x402_config: Option<X402Config>,
    ) -> Self {
        Self {
            pay_to,
            routes,
            facilitator,
            x402_config,
        }
    }
}

/// Check if payment is required and validate payment for a request
pub async fn check_payment(
    req: &HttpRequest,
    config: &PaymentMiddlewareConfig,
) -> Result<Option<HttpResponse>, Error> {
    let method = req.method().as_str();
    let path = req.path();
    let route_key = format!("{} {}", method, path);

    // Check if route requires payment
    if let Some(route_config) = config.routes.get(&route_key) {
        // Check for X-PAYMENT header
        let payment_header = req.headers().get("x-payment");

        if payment_header.is_none() {
            // No payment provided, return 402 with payment requirements
            let requirements = PaymentRequirements {
                x402_version: 1,
                scheme: PaymentScheme::Exact,
                network: route_config.network.clone(),
                max_amount_required: route_config.price.clone(),
                pay_to: config.pay_to.clone(),
                token_address: None,
                token_decimals: None,
                token_name: None,
                memo: route_config.description.clone(),
                nonce: None,
            };

            let requirements_json = serde_json::to_string(&requirements)
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

            return Ok(Some(
                HttpResponse::PaymentRequired()
                    .insert_header(("x-payment-required", requirements_json))
                    .finish(),
            ));
        }

        // Payment provided, verify it
        if let Some(payment_value) = payment_header {
            let payment_str = payment_value
                .to_str()
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
            let payment_payload: PaymentPayload = serde_json::from_str(payment_str)
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

            // Verify payment with facilitator
            let handler = Handler::new(config.facilitator.clone());
            let requirements = PaymentRequirements {
                x402_version: 1,
                scheme: PaymentScheme::Exact,
                network: route_config.network.clone(),
                max_amount_required: route_config.price.clone(),
                pay_to: config.pay_to.clone(),
                token_address: None,
                token_decimals: None,
                token_name: None,
                memo: route_config.description.clone(),
                nonce: None,
            };

            match handler.verify(&payment_payload, &requirements).await {
                Ok(verify_response) => {
                    if !verify_response.verified {
                        return Ok(Some(
                            HttpResponse::PaymentRequired().body("Payment verification failed"),
                        ));
                    }
                    // Payment verified, allow request to proceed
                }
                Err(_) => {
                    return Ok(Some(
                        HttpResponse::PaymentRequired().body("Payment verification failed"),
                    ));
                }
            }
        }
    }

    // No payment required or payment verified
    Ok(None)
}

/// Settle payment after successful request
pub async fn settle_payment(
    req: &HttpRequest,
    config: &PaymentMiddlewareConfig,
) -> Result<(), Error> {
    let method = req.method().as_str();
    let path = req.path();
    let route_key = format!("{} {}", method, path);

    if let Some(route_config) = config.routes.get(&route_key) {
        if let Some(payment_value) = req.headers().get("x-payment") {
            let payment_str = payment_value
                .to_str()
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
            let payment_payload: PaymentPayload = serde_json::from_str(payment_str)
                .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

            let handler = Handler::new(config.facilitator.clone());
            let requirements = PaymentRequirements {
                x402_version: 1,
                scheme: PaymentScheme::Exact,
                network: route_config.network.clone(),
                max_amount_required: route_config.price.clone(),
                pay_to: config.pay_to.clone(),
                token_address: None,
                token_decimals: None,
                token_name: None,
                memo: route_config.description.clone(),
                nonce: None,
            };

            let _ = handler.settle(&payment_payload, &requirements).await;
        }
    }

    Ok(())
}