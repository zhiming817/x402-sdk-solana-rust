// This file demonstrates how to use the facilitator functionality of the x402 SDK for Solana.

use x402_sdk_solana_rust::facilitator::Handler;
use x402_sdk_solana_rust::types::Request;

fn main() {
    // Create a new handler instance
    let handler = Handler::new();

    // Create a sample request
    let request = Request {
        request_id: "req_123".to_string(),
        data: "Sample data".to_string(),
    };

    // Handle the request
    match handler.handle_request(request) {
        Ok(response) => {
            println!("Request handled successfully: {:?}", response);
        }
        Err(e) => {
            eprintln!("Error handling request: {:?}", e);
        }
    }
}