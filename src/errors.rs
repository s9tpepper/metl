use core::panic;
use std::{io::Error, path::PathBuf, sync::LazyLock};

use colored::{ColoredString, Colorize};

use crate::sync::RestoreError;

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

pub fn failed_reading_manifest(error: std::io::Error, manifest_path: PathBuf) -> ! {
    panic!(
        "{} {} {}\n\t{}",
        &*ERROR,
        "Could not read manifest at path:".white().dimmed(),
        manifest_path.to_string_lossy().white().bold(),
        error.to_string().cyan().dimmed()
    );
}

pub fn manifest_parsing_error(error: &toml::de::Error, manifest_path: PathBuf) -> ! {
    panic!(
        "{} {} {:?}\n{}",
        &*ERROR,
        "Failed to parse the manifest:".white().dimmed(),
        manifest_path,
        error.to_string().cyan().dimmed()
    );
}

pub fn pacman_install_error(error: std::io::Error) -> ! {
    panic!(
        "{} {} {}",
        &*ERROR,
        "'pacman -S ...' failed\n".white().dimmed(),
        error.to_string().cyan()
    );
}

pub fn pacman_unknown_error() -> ! {
    panic!("{} {}", &*ERROR, "'pacman -S ...' failed".white().dimmed(),);
}

pub fn dotfiles_clone_error(error: RestoreError, verbose: bool) -> ! {
    if verbose {
        panic!(
            "{} {}\n{}",
            &*ERROR,
            "dotfiles could not be cloned".white().dimmed(),
            error.to_string().white().bold()
        )
    } else {
        panic!(
            "{} {}",
            &*ERROR,
            "dotfiles could not be cloned".white().dimmed()
        )
    }
}

pub fn dotfiles_dir_read_error(dotfiles_path: PathBuf, error: std::io::Error, verbose: bool) -> ! {
    if verbose {
        panic!(
            "{} {} {}\n{}",
            &*ERROR,
            "dotfiles path could not be read:".white().dimmed(),
            dotfiles_path.to_string_lossy().white().bold(),
            error.to_string().cyan().bold(),
        );
    } else {
        panic!(
            "{} {} {}",
            &*ERROR,
            "dotfiles path could not be read:".white().dimmed(),
            dotfiles_path.to_string_lossy().white().bold(),
        );
    }
}

pub fn package_install_failed(package: &str, error: std::io::Error) -> ! {
    panic!(
        "{} {} {}\n{}",
        &*ERROR,
        package.white().bold(),
        "failed to install:".white().dimmed(),
        error.to_string().cyan().bold(),
    );
}

pub fn install_failed(installed: &str, code: i32) {
    println!(
        "{} {} {}, code: {}",
        &*ERROR,
        "failed to install:".white().dimmed(),
        installed.white().bold(),
        code.to_string().cyan().bold(),
    );
}

pub fn remove_failed(installed: &str, code: i32) {
    println!(
        "{} {} {}, code: {}",
        &*ERROR,
        "failed to remove:".white().dimmed(),
        installed.white().bold(),
        code.to_string().cyan().bold(),
    );
}
