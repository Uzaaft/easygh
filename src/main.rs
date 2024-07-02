mod args;
mod error;
mod fzf;
mod gh;
use anyhow::bail;
use clap::{Parser, Subcommand};

fn main() {
    let cli = args::Cli::parse();
    cli.command.run().unwrap();
}
