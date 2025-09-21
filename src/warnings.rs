use std::sync::LazyLock;

use colored::{ColoredString, Colorize};

static WARNING: LazyLock<ColoredString> = LazyLock::new(|| "[WARNING]".yellow().bold());

pub fn warn_package_output(package_output: &str) {
    println!(
        "{} {} {}",
        &*WARNING,
        "Invalid package format:".white().dimmed(),
        package_output.magenta().bold()
    );
}

pub fn dotfiles_repo_not_set() {
    println!(
        "{} {}",
        &*WARNING,
        "dotfiles repo has not been configured".white().dimmed(),
    );
}
