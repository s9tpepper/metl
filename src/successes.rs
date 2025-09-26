use std::{ffi::OsString, path::PathBuf, process::Output, sync::LazyLock};

use colored::{ColoredString, Colorize};

use crate::{config::load_config, manifest::PackageManager};

static SUCCESS: LazyLock<ColoredString> = LazyLock::new(|| "[SUCCESS]".green().bold());

pub fn packages_retrieved_successfully(manager: PackageManager) {
    println!(
        "{} {} {}",
        &*SUCCESS,
        manager.to_string().white().bold(),
        "packages saved.".white()
    );
}

pub fn package_sync_success(
    manager: &PackageManager,
    packages: &[String],
    install_errors: &[(&String, Option<std::io::Error>)],
) {
    if install_errors.is_empty() {
        println!(
            "{} {} {} {}",
            &*SUCCESS,
            manager.to_string().white().bold(),
            "-S --needed --noconfirm".white().dimmed(),
            packages.join(" ").white().bold()
        );
    } else {
        let failed_packages =
            install_errors
                .iter()
                .fold(Vec::<String>::new(), |mut acc, (package, _)| {
                    acc.push(package.to_string());
                    acc
                });

        println!(
            "{} {} {} {}",
            &*SUCCESS,
            manager.to_string().white().bold(),
            "completed with some failures:".white().dimmed(),
            failed_packages.join(" ").white().bold()
        );
    }
}

pub fn dry_run_dotfiles_clone(repo: &str, dotfiles_path: PathBuf) {
    println!(
        "{} {} {} {} {}",
        &*SUCCESS,
        "DRY RUN:".yellow(),
        "git clone".white(),
        repo.truecolor(255, 255, 255).bold(),
        dotfiles_path
            .to_string_lossy()
            .truecolor(255, 255, 255)
            .bold()
    );
}

pub fn stow_success(name: OsString) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        "stow".white().bold(),
        "symlinked".white().dimmed(),
        name.to_string_lossy().white().bold(),
    );
}

pub fn dotfiles_copied_successfully(name: OsString, to: PathBuf, output: Output, verbose: bool) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        name.to_string_lossy().white().bold(),
        "copied successfully to".white(),
        to.to_string_lossy().white().bold(),
    );

    if verbose {
        if let Ok(stdout) = String::from_utf8(output.stdout)
            && !stdout.trim().is_empty()
        {
            println!("{stdout}");
        }

        if let Ok(stderr) = String::from_utf8(output.stderr)
            && !stderr.trim().is_empty()
        {
            println!("{stderr}");
        }
    }
}

pub fn pacman_dry_run_header() {
    println!(
        "{} {} {}",
        &*SUCCESS,
        "DRY RUN".yellow(),
        "pacman:".white().bold(),
    );
}

pub fn install_successful(package_manager: &PackageManager, installed: &str) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        "Installed with:".white().dimmed(),
        package_manager.to_string().white().bold(),
        installed.cyan(),
    );
}

pub fn remove_successful(package_manager: &PackageManager, installed: &str) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        "Removed:".white().dimmed(),
        package_manager.to_string().white().bold(),
        installed.white().cyan(),
    );
}

pub fn package_update_success(manager: &PackageManager, package: &str) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        manager.to_string().white().bold(),
        "updated:".white().dimmed(),
        package.white().bold(),
    );
}

pub fn git_metl_manifest_commit_success(proxied_command: &str) {
    let config = load_config();

    println!(
        "{} {} {} {}",
        &*SUCCESS,
        "metl manifest files committed after command: "
            .white()
            .dimmed(),
        config.package_manager.to_string().white().bold(),
        proxied_command.cyan(),
    );
}

pub fn git_push_metl_manifest_success() {
    println!(
        "{} {}",
        &*SUCCESS,
        "metl manifest files pushed to remote git".white().dimmed(),
    );
}
