use clap::{Parser, Subcommand};

use crate::{generate::generate, sync::sync};

mod config;
mod errors;
mod generate;
mod manifest;
mod successes;
mod sync;
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
    Sync {
        #[arg(long, short = 'd')]
        dry_run: bool,

        #[arg(long, short = 'v')]
        verbose: bool,
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
        Commands::Sync { dry_run, verbose } => sync(dry_run, verbose),
    }
}
