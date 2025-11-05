// This file demonstrates how to create a Server with payment protection using the x402 SDK.
// The server uses middleware to check payments before serving protected content.

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::env;
use x402_sdk_solana_rust::{
    check_payment, settle_payment,
    types::{Network, RouteConfig, TokenConfig},
    server::PaymentMiddlewareConfig,
};

#[derive(Clone)]
struct ServerConfig {
    facilitator_url: String,
    pay_to: String,
    network: Network,
    token_config: Option<TokenConfig>,
}

#[derive(Serialize)]
struct WeatherReport {
    weather: String,
    temperature: i32,
}

#[derive(Serialize)]
struct WeatherResponse {
    report: WeatherReport,
}

#[derive(Serialize)]
struct PremiumContent {
    content: String,
}

// Weather endpoint - requires payment
async fn weather_handler(
    req: HttpRequest,
    config: web::Data<ServerConfig>,
) -> impl Responder {
    println!("=== Weather endpoint called ===");

    // Create routes map
    let mut routes = std::collections::HashMap::new();
    routes.insert(
        "GET /weather".to_string(),
        RouteConfig {
            price: "1800".to_string(), // 1,800 micro-USDC = 0.0018 USDC (USDC has 6 decimals)
            network: config.network.clone(),
            description: Some("Weather information".to_string()),
            mime_type: Some("application/json".to_string()),
            max_timeout_seconds: Some(30),
            discoverable: Some(true),
        },
    );

    // Build X402 config with token if available
    let x402_config = config.token_config.as_ref().map(|token| {
        x402_sdk_solana_rust::types::X402Config {
            svm_config: Some(x402_sdk_solana_rust::types::SvmConfig {
                rpc_url: None,
                default_token: Some(token.clone()),
            }),
        }
    });

    // Create middleware config
    let middleware_config = PaymentMiddlewareConfig {
        pay_to: config.pay_to.clone(),
        routes,
        facilitator: Some(x402_sdk_solana_rust::types::FacilitatorConfig {
            url: config.facilitator_url.clone(),
            create_auth_headers: None,
        }),
        x402_config,
    };

    // Check payment
    match check_payment(&req, &middleware_config).await {
        Ok(Some(response)) => {
            // Payment not provided or verification failed, return the response
            println!("✗ Payment check failed, returning 402 or error");
            response
        }
        Ok(None) => {
            // Payment verified successfully
            println!("✓ Payment verified, settling...");

            // Settle payment
            if let Err(e) = settle_payment(&req, &middleware_config).await {
                eprintln!("✗ Payment settlement failed: {:?}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Payment settlement failed"
                }));
            }

            println!("✓ Payment settled successfully");

            // Return protected content
            let response = WeatherResponse {
                report: WeatherReport {
                    weather: "sunny".to_string(),
                    temperature: 70,
                },
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("✗ Payment check error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

// Premium content endpoint - requires higher payment
async fn premium_content_handler(
    req: HttpRequest,
    config: web::Data<ServerConfig>,
) -> impl Responder {
    println!("=== Premium content endpoint called ===");

    // Create routes map
    let mut routes = std::collections::HashMap::new();
    routes.insert(
        "GET /premium/content".to_string(),
        RouteConfig {
            price: "150000".to_string(), // 150,000 micro-USDC = 0.15 USDC (USDC has 6 decimals)
            network: config.network.clone(),
            description: Some("Premium content access".to_string()),
            mime_type: Some("application/json".to_string()),
            max_timeout_seconds: Some(60),
            discoverable: Some(true),
        },
    );

    // Build X402 config with token if available
    let x402_config = config.token_config.as_ref().map(|token| {
        x402_sdk_solana_rust::types::X402Config {
            svm_config: Some(x402_sdk_solana_rust::types::SvmConfig {
                rpc_url: None,
                default_token: Some(token.clone()),
            }),
        }
    });

    // Create middleware config
    let middleware_config = PaymentMiddlewareConfig {
        pay_to: config.pay_to.clone(),
        routes,
        facilitator: Some(x402_sdk_solana_rust::types::FacilitatorConfig {
            url: config.facilitator_url.clone(),
            create_auth_headers: None,
        }),
        x402_config,
    };

    // Check payment
    match check_payment(&req, &middleware_config).await {
        Ok(Some(response)) => {
            // Payment not provided or verification failed, return the response
            println!("✗ Payment check failed, returning 402 or error");
            response
        }
        Ok(None) => {
            // Payment verified successfully
            println!("✓ Payment verified, settling...");

            // Settle payment
            if let Err(e) = settle_payment(&req, &middleware_config).await {
                eprintln!("✗ Payment settlement failed: {:?}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Payment settlement failed"
                }));
            }

            println!("✓ Payment settled successfully");

            // Return protected content
            let response = PremiumContent {
                content: "This is premium content".to_string(),
            };

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("✗ Payment check error: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== X402 Payment-Protected Server ===");
    println!();

    // Load environment variables from .env_server
    dotenv::from_filename(".env_server").ok();

    // Read configuration from environment
    let facilitator_url = env::var("FACILITATOR_URL")
        .expect("FACILITATOR_URL must be set in .env_server");
    
    let pay_to = env::var("ADDRESS")
        .expect("ADDRESS must be set in .env_server");
    
    let network = env::var("NETWORK")
        .unwrap_or_else(|_| "solana-devnet".to_string());
    
    let token_mint_address = env::var("TOKEN_MINT_ADDRESS").ok();
    let token_decimals = env::var("TOKEN_DECIMALS").ok()
        .and_then(|s| s.parse::<u8>().ok());
    let token_name = env::var("TOKEN_NAME").ok();
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "4021".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    // Parse network
    let network = match network.as_str() {
        "solana-localnet" => Network::SolanaLocalnet,
        "solana-devnet" => Network::SolanaDevnet,
        "solana" => Network::Solana,
        _ => panic!("Invalid NETWORK: {}", network),
    };

    println!("Configuration:");
    println!("  Facilitator URL: {}", facilitator_url);
    println!("  Pay to address: {}", pay_to);
    println!("  Network: {:?}", network);

    // Build token config if all token parameters are provided
    let token_config = if let (Some(address), Some(decimals), Some(name)) = 
        (token_mint_address.as_ref(), token_decimals, token_name.as_ref()) 
    {
        println!("  Custom token config:");
        println!("    Address: {}", address);
        println!("    Decimals: {}", decimals);
        println!("    Name: {}", name);
        Some(TokenConfig {
            address: address.clone(),
            decimals,
            name: name.clone(),
        })
    } else {
        println!("  Using native SOL");
        None
    };

    println!();

    // Create server configuration
    let config = web::Data::new(ServerConfig {
        facilitator_url,
        pay_to,
        network,
        token_config,
    });

    let bind_addr = format!("{}:{}", host, port);
    println!("Starting payment-protected server at http://{}", bind_addr);
    println!();
    println!("Available endpoints:");
    println!("  GET /weather         - Weather information (1,800 micro-USDC = 0.0018 USDC)");
    println!("  GET /premium/content - Premium content (150,000 micro-USDC = 0.15 USDC)");
    println!();
    println!("Note: All endpoints require X-PAYMENT header with signed transaction");
    println!();

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .route("/weather", web::get().to(weather_handler))
            .route("/premium/content", web::get().to(premium_content_handler))
    })
    .bind(&bind_addr)?
    .run()
    .await
}