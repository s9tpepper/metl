use std::{ffi::OsString, path::PathBuf, process::Output, sync::LazyLock};

use colored::{ColoredString, Colorize};

static SUCCESS: LazyLock<ColoredString> = LazyLock::new(|| "[SUCCESS]".green().bold());

pub fn packages_retrieved_successfully(manager: &str) {
    println!(
        "{} {} {}",
        &*SUCCESS,
        manager.white().bold(),
        "packages saved.".white()
    );
}

pub fn dry_run_package_install_output(manager: &str, packages: &[String]) {
    println!(
        "{} {} {} {} {}",
        &*SUCCESS,
        "DRY RUN:".yellow(),
        manager.white().bold(),
        "-S --needed --noconfirm".white(),
        packages.join(" ").white().dimmed()
    );
}

pub fn package_sync_success(manager: &str, packages: &[String]) {
    println!(
        "{} {} {} {}",
        &*SUCCESS,
        manager.white().bold(),
        "-S --needed --noconfirm".white(),
        packages.join(" ").white().dimmed()
    );
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
        "{} {} {}",
        &*SUCCESS,
        "stow symlinked".white(),
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
