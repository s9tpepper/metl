use clap::{Parser, Subcommand};

use crate::{generate::generate, install::install, sync::sync};

mod config;
mod errors;
mod generate;
mod install;
mod manifest;
mod successes;
mod sync;
mod warnings;

/// Does all the things for a new system.
#[derive(Subcommand)]
enum Commands {
    /// Install a package
    #[command(visible_alias = "i")]
    Install {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Remove a package
    #[command(visible_alias = "r")]
    Remove,

    /// Generate a package manifest
    #[command(visible_alias = "g")]
    Generate,

    /// Search for a package
    #[command(visible_alias = "q")]
    Search,

    /// Sync things
    #[command(visible_alias = "s")]
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
        Commands::Install { args } => install(args),
        Commands::Remove => todo!(),
        Commands::Generate => generate(),
        Commands::Search => todo!(),
        Commands::Sync { dry_run, verbose } => sync(dry_run, verbose),
    }
}
