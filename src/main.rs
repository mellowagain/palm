use anyhow::{Context, Result};
use poem::listener::TcpListener;
use poem::Server;
use crate::config::load_config;
use crate::routes::all_routes;

mod config;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().await?;
    let address = "0.0.0.0:7123";

    println!("starting server on {address}");
    Server::new(TcpListener::bind(address)).run(all_routes()).await.context("failed to start server")
}
