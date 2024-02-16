extern crate tokio;

mod cmd;
mod core;

#[tokio::main]
async fn main() {
    core::cli::garch_cli::parse().await;
}
