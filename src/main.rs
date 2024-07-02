mod args;
mod error;
mod fzf;
mod gh;
use anyhow::bail;
use clap::{Parser, Subcommand};
use which::which_all_global;

fn main() -> anyhow::Result<()> {
    let is_fzf_installed = which_all_global("fzf").is_ok();
    let is_gh_installed = which_all_global("gh").is_ok();
    if !is_fzf_installed && !is_gh_installed {
        bail!(error::EasyGHError::FzfAndGHNotFound);
    } else if !is_fzf_installed {
        bail!(error::EasyGHError::FzfNotFound);
    } else if !is_gh_installed {
        bail!(error::EasyGHError::GHNotFound);
    }

    let cli = args::Cli::parse();
    cli.command.run().unwrap();
    Result::Ok(())
}
