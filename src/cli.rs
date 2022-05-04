use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "entgen")]
#[clap(author = "Hikaru Miyahara")]
#[clap(version = "0.0.1")]
#[clap(about = "Entity generator for sqlx", long_about = None)]
struct Cli {
    config: PathBuf,
}
