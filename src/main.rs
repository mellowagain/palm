use std::collections::HashMap;
use std::sync::Arc;
use anyhow::{Context, Result};
use poem::listener::TcpListener;
use poem::Server;
use crate::config::{Item, load_config};
use crate::routes::all_routes;

mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let config: Arc<HashMap<String, Vec<Item>>> = Arc::new(load_config().await?);
    let port = std::env::var("PORT").unwrap_or_else(|_| "7123".to_string());
    let address = format!("0.0.0.0:{port}");

    println!("starting server on {address}");
    Server::new(TcpListener::bind(address)).run(all_routes(config)).await.context("failed to start server")
}
