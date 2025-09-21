use clap::{Parser, Subcommand};

use crate::{generate::generate, restore::restore};

mod config;
mod errors;
mod generate;
mod manifest;
mod restore;
mod warnings;

/// Does all the things for a new system.
#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install,

    /// Remove a package
    Remove,

    /// Generate a package manifest
    Generate,

    /// Search for a package
    Search,

    /// Sync things
    Restore {
        #[arg(long, short = 'd')]
        dry_run: bool,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Install => todo!(),
        Commands::Remove => todo!(),
        Commands::Generate => generate(),
        Commands::Search => todo!(),
        Commands::Restore { dry_run } => restore(dry_run),
    }
}
