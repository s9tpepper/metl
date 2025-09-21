use std::sync::LazyLock;

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
