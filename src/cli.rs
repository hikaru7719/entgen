use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "entgen")]
#[clap(author = "Hikaru Miyahara")]
#[clap(version = "0.0.1")]
#[clap(about = "Entity generator for sqlx", long_about = None)]
pub struct Cli {
    #[clap(
        short,
        long,
        help = "Set entgen config file",
        default_value = "entgen.toml"
    )]
    pub file: PathBuf,
}

impl Cli {
    pub fn parse_opt() -> Self {
        Cli::parse()
    }
}
