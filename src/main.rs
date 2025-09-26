use clap::{Parser, Subcommand};

use crate::{generate::generate, install::install, remove::remove, sync::sync};

mod commits;
mod config;
mod errors;
mod generate;
mod install;
mod manifest;
mod proxies;
mod remove;
mod successes;
mod sync;
mod warnings;

#[derive(Subcommand)]
enum Commands {
    /// Install a package, update manifest file, and push to git repo
    #[command(visible_alias = "i")]
    Install {
        /// Flags for your configured package manager as defined by the package manager
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Remove a package, update manifest file, and push to git repo
    #[command(visible_alias = "r")]
    Remove {
        /// Flags for your configured package manager as defined by the package manager
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Generate a package manifest at ~/.config/metl/manifest.toml
    #[command(visible_alias = "g")]
    Generate,

    /// Sync system with all packages in manifest file at ~/.config/metl/manifest.toml
    #[command(visible_alias = "s")]
    Sync {
        /// Run a dry run without modifying the system
        #[arg(long, short = 'd')]
        dry_run: bool,

        /// Enable verbose output
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
        Commands::Remove { args } => remove(args),
        Commands::Generate => generate(),
        Commands::Sync { dry_run, verbose } => sync(dry_run, verbose),
    }
}
