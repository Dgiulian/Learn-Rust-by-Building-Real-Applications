#![allow(dead_code)]
use website_handler::WebsiteHandler;

use crate::server::Server;

use std::env;
mod http;
mod server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let server = Server::new("127.0.0.1:8080".to_string());

    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    server.run(WebsiteHandler::new({ public_path }));
}
