// examples/client_example.rs
// This example shows how to use the x402 SDK to make a request to a resource server that requires a payment.
//
// To run this example, you need to set the following environment variables in .env_client:
// - USER_SVM_PRIVATE_KEY: The private key of the signer (Base58 format)
// - SVM_NETWORK: The network to use (solana-localnet, solana-devnet, solana)
// - SVM_RPC_URL: The RPC URL (optional)

use x402_sdk_solana_rust::{
    client::Fetcher,
    solana::create_signer,
    types::{X402Request, SvmConfig, X402Config},
};
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env_client
    dotenv::from_filename(".env_client").ok();

    // Read environment variables
    let svm_private_key = env::var("USER_SVM_PRIVATE_KEY")
        .expect("USER_SVM_PRIVATE_KEY must be set in .env_client");
    let svm_network = env::var("SVM_NETWORK").unwrap_or_else(|_| "solana-devnet".to_string());
    let rpc_url = env::var("SVM_RPC_URL").ok();
    let need_pay_resource_url = env::var("NEED_PAY_RESOURCE_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:4021/weather".to_string());

    println!("=== X402 Client Example ===");
    println!("Network: {}", svm_network);
    println!("RPC URL: {}", rpc_url.as_deref().unwrap_or("(using default)"));
    println!("Resource URL: {}", need_pay_resource_url);
    println!();

    // Create signer from private key
    let wallet = create_signer(&svm_network, &svm_private_key)?;
    println!("Wallet created with public key: {}", wallet.public_key());

    // Configure X402
    let config = Some(X402Config {
        svm_config: Some(SvmConfig {
            rpc_url,
            default_token: None, // Use default USDC
        }),
    });

    // Create Fetcher with max payment value (0.1 USDC = 100,000 smallest units)
    let fetcher = Fetcher::with_max_value(wallet, 100_000, config);
    println!("Fetcher created with max payment: 0.1 USDC");
    println!();

    // Create request
    let request = X402Request {
        url: need_pay_resource_url.clone(),
        method: "GET".to_string(),
        headers: HashMap::new(),
        body: None,
    };

    println!("Sending request to {}...", need_pay_resource_url);
    //打印   request 详细信息
    println!("Request details: {:?}", request); 
    // Fetch with automatic payment handling
    // The function detects 402 status code, parses payment requirements from x-payment-required header
    // Creates and signs payment transaction, attaches payment info (x-payment header) and resends request
    match fetcher.fetch(request).await {
        Ok(response) => {
            println!("✓ Response received");
            println!("Status: {}", response.status);
            // 打印response 详细信息
            println!("Response details: {:?}", response);
            // Print headers for debugging
            if response.status != 200 {
                println!("Response headers:");
                for (key, value) in &response.headers {
                    println!("  {}: {}", key, value);
                }
            }
            
            // Parse response body
            let body = String::from_utf8_lossy(&response.body);
            println!("Body: {}", body);
            
            // Check for payment response
            if let Some(payment_response) = response.headers.get("x-payment-response") {
                println!();
                println!("Payment Response: {}", payment_response);
            }
            
            println!();
            println!("✓ Request completed successfully!");
        }
        Err(e) => {
            eprintln!("✗ Request failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}