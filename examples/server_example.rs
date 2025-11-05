use actix_web::{web, App, HttpServer, Responder};
use std::sync::Arc;

async fn health_check() -> impl Responder {
    "Server is running"
}

async fn handle_request(data: web::Json<String>) -> impl Responder {
    format!("Received request with data: {}", data)
}

pub async fn run_server() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/request", web::post().to(handle_request))
    });

    let addr = "127.0.0.1:8080";
    println!("Starting server at {}", addr);
    server.bind(addr)?.run().await
}

fn main() {
    let server_future = run_server();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(server_future).unwrap();
}