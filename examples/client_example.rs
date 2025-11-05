// examples/client_example.rs

use x402_sdk_solana_rust::client::Fetcher;

fn main() {
    let fetcher = Fetcher::new();
    match fetcher.fetch_data() {
        Ok(data) => {
            println!("Fetched data: {:?}", data);
        }
        Err(e) => {
            eprintln!("Error fetching data: {:?}", e);
        }
    }
}