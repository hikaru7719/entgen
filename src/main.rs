pub mod cli;
pub mod config;
pub mod db;
pub mod runner;
pub mod template;

#[tokio::main]
async fn main() {
    runner::run_with_result().await;
}
