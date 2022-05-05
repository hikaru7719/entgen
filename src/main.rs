pub mod cli;
pub mod config;
pub mod db;
pub mod error;
pub mod runner;
pub mod template;

#[tokio::main]
async fn main() {
    runner::run().await;
}
