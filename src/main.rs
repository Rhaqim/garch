extern crate tokio;

mod core;
mod cmd;

#[tokio::main]
async fn main() {
    core::cli::garch_cli::parse().await;
}