use std::{io::Error, sync::LazyLock};

use colored::{ColoredString, Colorize};

static ERROR: LazyLock<ColoredString> = LazyLock::new(|| "[ERROR]".red().bold());

pub fn unsupported_package_manager(package_manager: &str) -> ! {
    panic!(
        "{} {} {}",
        &*ERROR,
        "Unsupported package manager:".red().dimmed(),
        package_manager.white().bold(),
    );
}

pub fn manifest_serialization_error() -> ! {
    panic!(
        "{} {}",
        &*ERROR,
        "Error serializing metl manifest file".red().dimmed(),
    )
}

pub fn packages_list_error(error: Option<Error>) -> ! {
    panic!(
        "{} {}\n\t{:?}",
        &*ERROR,
        "Could not get package list".red().dimmed(),
        error
    )
}

pub fn packages_parsing_error(manager: &str) -> ! {
    panic!(
        "{} {} {}",
        &*ERROR,
        "Could not parse package output bytes from".white().dimmed(),
        manager.white().bold()
    );
}
