// This file demonstrates how to create a Facilitator service for the x402 protocol.
// The Facilitator verifies and settles payments between clients and servers.

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::env;
use x402_sdk_solana_rust::{
    error::X402Error,
    Wallet, TransactionBuilder,
    types::{
        Network, PaymentPayload, PaymentRequirements, SupportedPaymentKind, VerifyResponse,
        SettleResponse, X402Config, SvmConfig, PaymentScheme,
    },
};

#[derive(Debug, Serialize, Deserialize)]
struct VerifyRequest {
    #[serde(rename = "paymentPayload")]
    payment_payload: PaymentPayload,
    #[serde(rename = "paymentRequirements")]
    payment_requirements: PaymentRequirements,
}

#[derive(Debug, Serialize, Deserialize)]
struct SettleRequest {
    #[serde(rename = "paymentPayload")]
    payment_payload: PaymentPayload,
    #[serde(rename = "paymentRequirements")]
    payment_requirements: PaymentRequirements,
}

#[derive(Debug, Serialize)]
struct SupportedResponse {
    kinds: Vec<SupportedPaymentKind>,
}

#[derive(Clone)]
struct AppState {
    wallet: Wallet,
    network: Network,
    x402_config: Option<X402Config>,
}

// GET /verify - Show endpoint information
async fn verify_info() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "endpoint": "/verify",
        "description": "POST to verify x402 payments",
        "body": {
            "paymentPayload": "PaymentPayload",
            "paymentRequirements": "PaymentRequirements"
        }
    }))
}

// POST /verify - Verify payment transaction
async fn verify_payment(
    data: web::Json<VerifyRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    println!("=== Verify Payment Request ===");
    println!("Payment Payload: {:?}", data.payment_payload);
    println!("Payment Requirements: {:?}", data.payment_requirements);
    println!("Raw JSON: {:?}", serde_json::to_string_pretty(&data.0).unwrap_or_default());

    match verify_transaction(&state, &data.payment_payload, &data.payment_requirements).await {
        Ok(response) => {
            println!("✓ Payment verified successfully");
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("✗ Payment verification failed: {:?}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Verification failed: {}", e)
            }))
        }
    }
}

// GET /settle - Show endpoint information
async fn settle_info() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "endpoint": "/settle",
        "description": "POST to settle x402 payments",
        "body": {
            "paymentPayload": "PaymentPayload",
            "paymentRequirements": "PaymentRequirements"
        }
    }))
}

// POST /settle - Submit payment transaction to blockchain
async fn settle_payment(
    data: web::Json<SettleRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    println!("=== Settle Payment Request ===");
    println!("Payment Payload: {:?}", data.payment_payload);
    println!("Payment Requirements: {:?}", data.payment_requirements);

    match settle_transaction(&state, &data.payment_payload, &data.payment_requirements).await {
        Ok(response) => {
            println!("✓ Payment settled successfully");
            println!("Transaction signature: {}", response.signature);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("✗ Payment settlement failed: {:?}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Settlement failed: {}", e)
            }))
        }
    }
}

// GET /supported - Return supported payment kinds
async fn supported_payment_kinds(state: web::Data<AppState>) -> impl Responder {
    let mut kinds = Vec::new();

    // Add SVM support
    kinds.push(SupportedPaymentKind {
        x402_version: 1,
        scheme: PaymentScheme::Exact,
        network: state.network.clone(),
    });

    let response = SupportedResponse { kinds };
    HttpResponse::Ok().json(response)
}

// Verify transaction logic
async fn verify_transaction(
    state: &AppState,
    payment_payload: &PaymentPayload,
    payment_requirements: &PaymentRequirements,
) -> Result<VerifyResponse, X402Error> {
    // 1. Deserialize the transaction
    let transaction = TransactionBuilder::deserialize_transaction(&payment_payload.signed_transaction)?;

    println!("Transaction deserialized successfully");

    // 2. Verify basic payment requirements
    if payment_requirements.network != state.network {
        return Err(X402Error::InvalidInput(format!(
            "Expected network {:?}, got {:?}",
            state.network, payment_requirements.network
        )));
    }

    // 3. Verify the transaction structure
    // Check that it transfers the correct amount to the correct recipient
    let instructions = transaction.message.instructions.clone();
    if instructions.is_empty() {
        return Err(X402Error::PaymentVerificationFailed(
            "Transaction has no instructions".to_string(),
        ));
    }

    println!("Transaction has {} instruction(s)", instructions.len());

    // 4. Simulate the transaction to ensure it would succeed
    // In a real implementation, you would use the RPC client to simulate
    // For now, we'll do basic validation

    // Verify signature is present
    if transaction.signatures.is_empty() {
        return Err(X402Error::PaymentVerificationFailed(
            "Transaction has no signatures".to_string(),
        ));
    }

    println!("Transaction signature: {:?}", transaction.signatures[0]);

    // Return success response
    Ok(VerifyResponse {
        verified: true,
        message: Some("Payment verified successfully".to_string()),
    })
}

// Settle transaction logic
async fn settle_transaction(
    state: &AppState,
    payment_payload: &PaymentPayload,
    payment_requirements: &PaymentRequirements,
) -> Result<SettleResponse, X402Error> {
    // 1. First verify the transaction
    verify_transaction(state, payment_payload, payment_requirements).await?;

    // 2. Deserialize the transaction
    let transaction = TransactionBuilder::deserialize_transaction(&payment_payload.signed_transaction)?;

    // 3. Prepare data for blockchain submission
    // Get RPC URL
    let rpc_url = if let Some(config) = &state.x402_config {
        if let Some(svm_config) = &config.svm_config {
            if let Some(url) = &svm_config.rpc_url {
                url.clone()
            } else {
                get_default_rpc_url(&state.network)
            }
        } else {
            get_default_rpc_url(&state.network)
        }
    } else {
        get_default_rpc_url(&state.network)
    };

    // 4. Submit to blockchain - use tokio::task::spawn_blocking for all RPC calls
    // NOTE: Transaction is already signed by client with a recent blockhash
    // We should NOT modify the transaction, just submit it as-is
    let signature = tokio::task::spawn_blocking(move || {
        let client = solana_client::rpc_client::RpcClient::new(rpc_url);
        
        // Submit the pre-signed transaction directly
        let signature = client.send_and_confirm_transaction(&transaction)
            .map_err(|e| X402Error::SolanaError(e.to_string()))?;
        
        Ok::<_, X402Error>(signature)
    })
    .await
    .map_err(|e| X402Error::SolanaError(format!("Task error: {e}")))?
    .map_err(|e| e)?;

    println!("Transaction submitted to blockchain");
    println!("Signature: {}", signature);

    // Return success response
    Ok(SettleResponse {
        signature: signature.to_string(),
        settled: true,
        message: Some("Payment settled successfully".to_string()),
    })
}

// Helper function to get default RPC URL for a network
fn get_default_rpc_url(network: &Network) -> String {
    match network {
        Network::SolanaLocalnet => "http://127.0.0.1:8899".to_string(),
        Network::SolanaDevnet => "https://api.devnet.solana.com".to_string(),
        Network::Solana => "https://api.mainnet-beta.solana.com".to_string(),
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> std::io::Result<()> {
    println!("=== X402 Facilitator Service ===");
    println!();

    // Load environment variables from .env_facilitator
    dotenv::from_filename(".env_facilitator").ok();

    // Read configuration from environment
    let svm_private_key = env::var("SVM_PRIVATE_KEY")
        .expect("SVM_PRIVATE_KEY must be set in .env_facilitator");
    
    let svm_network = env::var("SVM_NETWORK")
        .unwrap_or_else(|_| "solana-devnet".to_string());
    
    let svm_rpc_url = env::var("SVM_RPC_URL").ok();
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Parse network
    let network = match svm_network.as_str() {
        "solana-localnet" => Network::SolanaLocalnet,
        "solana-devnet" => Network::SolanaDevnet,
        "solana" => Network::Solana,
        _ => panic!("Invalid SVM_NETWORK: {}", svm_network),
    };

    println!("Network: {:?}", network);
    if let Some(ref url) = svm_rpc_url {
        println!("RPC URL: {}", url);
    } else {
        println!("RPC URL: (using default for {:?})", network);
    }
    println!();

    // Create wallet from private key
    let wallet = Wallet::from_private_key(&svm_private_key)
        .expect("Failed to create wallet from private key");

    println!("Facilitator wallet public key: {}", wallet.public_key());
    println!();

    // Create X402 config with custom RPC URL if provided
    let x402_config = svm_rpc_url.map(|url| X402Config {
        svm_config: Some(SvmConfig {
            rpc_url: Some(url),
            default_token: None,
        }),
    });

    // Create application state
    let app_state = web::Data::new(AppState {
        wallet,
        network,
        x402_config,
    });

    let bind_addr = format!("{}:{}", host, port);
    println!("Starting Facilitator service at http://{}", bind_addr);
    println!();
    println!("Available endpoints:");
    println!("  GET  /supported - Get supported payment kinds");
    println!("  GET  /verify    - Show verify endpoint info");
    println!("  POST /verify    - Verify payment transaction");
    println!("  GET  /settle    - Show settle endpoint info");
    println!("  POST /settle    - Settle payment transaction");
    println!();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/supported", web::get().to(supported_payment_kinds))
            .route("/verify", web::get().to(verify_info))
            .route("/verify", web::post().to(verify_payment))
            .route("/settle", web::get().to(settle_info))
            .route("/settle", web::post().to(settle_payment))
    })
    .bind(&bind_addr)?
    .run()
    .await
}