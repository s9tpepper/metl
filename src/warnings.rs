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
