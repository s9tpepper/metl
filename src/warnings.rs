use std::{ffi::OsString, path::PathBuf, sync::LazyLock};

use colored::{ColoredString, Colorize};

use crate::manifest::PackageManager;

static WARNING: LazyLock<ColoredString> = LazyLock::new(|| "[WARNING]".yellow().bold());

pub fn warn_package_output(package_output: &str) {
    println!(
        "{} {} {}",
        &*WARNING,
        "Invalid package format:".white().dimmed(),
        package_output.magenta().bold()
    );
}

pub fn warn_dotfiles_symlink_failed(name: OsString, error: std::io::Error) {
    println!(
        "{} {} {}\n{}",
        &*WARNING,
        name.to_string_lossy().white().bold(),
        "could not be stowed".white().dimmed(),
        error.to_string().cyan().bold(),
    );
}

pub fn warn_dotfiles_symlink_signal_exit(name: OsString) {
    println!(
        "{} {} {}",
        &*WARNING,
        name.to_string_lossy().white().bold(),
        "could not be stowed, exited because of signal"
            .white()
            .dimmed(),
    );
}

pub fn warn_dotfiles_symlink_non_zero(name: OsString, code: i32) {
    println!(
        "{} {} {} {}",
        &*WARNING,
        name.to_string_lossy().white().bold(),
        "could not be stowed, exit code:".white().dimmed(),
        code.to_string().white().bold()
    );
}

pub fn dotfiles_copy_failed(name: OsString, to: PathBuf, error: std::io::Error) {
    println!(
        "{} {} {} {}\n{}",
        &*WARNING,
        name.to_string_lossy().white().bold(),
        "could not be copied to".white(),
        to.to_string_lossy().white().bold(),
        error.to_string().cyan().bold(),
    );
}

pub fn warn_failed_installs(
    manager: &PackageManager,
    install_errors: &[(&String, Option<std::io::Error>)],
) {
    install_errors
        .iter()
        .for_each(|(package, error)| match error {
            Some(err) => {
                println!(
                    "{} {} {} {}\n{}",
                    &*WARNING,
                    manager.to_string().white().bold(),
                    "failed to install".white().dimmed(),
                    package.white().bold(),
                    err.to_string().cyan().bold(),
                );
            }

            None => {
                println!(
                    "{} {} {} {}",
                    &*WARNING,
                    manager.to_string().white().bold(),
                    "failed to install".white().dimmed(),
                    package.white().bold(),
                );
            }
        });
}

pub fn warn_git_add_metl_manifest_failed(error: Option<std::io::Error>) {
    match error {
        Some(err) => {
            println!(
                "{} {}\n{}",
                &*WARNING,
                "failed to git add metl manifest files".white().dimmed(),
                err.to_string().cyan().bold(),
            );
        }
        None => {
            println!(
                "{} {}",
                &*WARNING,
                "failed to git add metl manifest files".white().dimmed(),
            );
        }
    }
}

pub fn warn_git_add_metl_manifest_code(code: i32) {
    println!(
        "{} {} {}",
        &*WARNING,
        "failed to git add metl manifest files, code:"
            .white()
            .dimmed(),
        code.to_string().cyan().bold(),
    );
}

pub fn warn_metl_manifest_commit_failed(error: Option<std::io::Error>) {
    match error {
        Some(err) => {
            println!(
                "{} {}\n{}",
                &*WARNING,
                "failed to git commit metl manifest files".white().dimmed(),
                err.to_string().cyan().bold(),
            );
        }
        None => {
            println!(
                "{} {}",
                &*WARNING,
                "failed to git commit metl manifest files".white().dimmed(),
            );
        }
    }
}

pub fn warn_metl_manifest_commit_code(code: i32) {
    println!(
        "{} {} {}",
        &*WARNING,
        "failed to git commit metl manifest files, code:"
            .white()
            .dimmed(),
        code.to_string().cyan().bold(),
    );
}

pub fn warn_git_push_metl_manifest_failed(error: Option<std::io::Error>, code: Option<i32>) {
    match (error, code) {
        (None, None) => {
            println!(
                "{} {}",
                &*WARNING,
                "failed to git push metl manifest files, code:"
                    .white()
                    .dimmed(),
            );
        }
        (None, Some(code)) => {
            println!(
                "{} {} {}",
                &*WARNING,
                "failed to git push metl manifest files, code:"
                    .white()
                    .dimmed(),
                code.to_string().cyan(),
            );
        }
        (Some(error), None) => {
            println!(
                "{} {} {}",
                &*WARNING,
                "failed to git push metl manifest files, error:"
                    .white()
                    .dimmed(),
                error.to_string().cyan(),
            );
        }
        (Some(error), Some(code)) => {
            println!(
                "{} {} {}\n{}",
                &*WARNING,
                "failed to git push metl manifest files, code:"
                    .white()
                    .dimmed(),
                code.to_string().cyan(),
                error.to_string().cyan(),
            );
        }
    }
}
