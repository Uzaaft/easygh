use std::default;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::{fzf, gh};

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Subcommands {
    Repos {
        #[arg(short, long)]
        delete: bool,

        #[clap(default_value = "30")]
        #[arg(short, long)]
        number: i32,
    },
}

/// Implement a method to check for subcommands for easy pattern matching
impl Subcommands {
    pub fn run(&self) -> Result<()> {
        match self {
            Subcommands::Repos { delete, number } => match delete {
                true => self.delete_repositories(number),
                false => panic!("Only delete repos is supported for now"),
            },
        }
    }

    pub fn delete_repositories(&self, n: &i32) -> Result<()> {
        let gh = std::process::Command::new("gh")
            .args([
                "repo",
                "list",
                "--json",
                "nameWithOwner",
                "--limit",
                n.to_string().as_str(),
            ])
            .output()
            .expect("failed to list repos");
        if !gh.status.success() {
            dbg!(&gh);
            panic!("failed to list repos");
        }
        let repos: Vec<gh::RepoWithNameOwner> =
            serde_json::from_slice(&gh.stdout).expect("failed to parse gh output");
        let mut fzf = fzf::Fzf::new().expect("failed to create fzf");
        fzf.write_to_stdin(
            &repos
                .iter()
                .map(|repo| repo.name_with_owner.clone())
                .collect::<Vec<String>>(),
        )
        .expect("failed to write to fzf");
        let selected_repos = fzf
            .get_selected(&repos)
            .expect("failed to get selected repos");

        // Print out and ask for confirmation
        println!("Selected repos:");
        for repo in &selected_repos {
            println!("{}", repo);
        }
        println!("Are you sure you want to delete these repos? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "y" {
            println!("Aborting");
            return Ok(());
        }

        for repo in selected_repos {
            let gh = std::process::Command::new("gh")
                .args(["repo", "delete", &repo.name_with_owner, "--yes"])
                .output()
                .expect("failed to delete repo");
            if !gh.status.success() {
                panic!("failed to delete repo");
            }
            println!("Deleted {}", repo);
        }
        Ok(())
    }
}
