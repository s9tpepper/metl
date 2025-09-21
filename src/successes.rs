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
