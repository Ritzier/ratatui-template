use clap::Parser;
use client::{App, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Cli::parse();
    App::new(args.frame, args.tick)?.run().await
}
