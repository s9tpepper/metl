use std::{
    fs::{self, DirEntry},
    process::{Command, Stdio},
};

use thiserror::Error;

use crate::{
    config::get_home_path,
    errors::{
        dotfiles_clone_error, dotfiles_dir_read_error, pacman_install_error, pacman_unknown_error,
    },
    manifest::{
        Manifest, Package,
        PackageManager::{Pacman, Yay},
        load_manifest,
    },
    successes::{
        dry_run_dotfiles_clone, dry_run_package_install_output, package_sync_success, stow_success,
    },
    warnings::{
        dotfiles_repo_not_set, warn_dotfiles_symlink_failed, warn_dotfiles_symlink_non_zero,
        warn_dotfiles_symlink_signal_exit,
    },
};

#[derive(Debug, Error)]
pub enum RestoreError {
    #[error("Failed to clone dotfiles repository")]
    DotfileClone,
}

pub fn sync(dry_run: bool, verbose: bool) {
    let manifest = load_manifest();

    restore_packages(&manifest, dry_run, verbose);
    restore_dotfiles(&manifest, dry_run, verbose);
}

fn restore_dotfiles(manifest: &Manifest, dry_run: bool, verbose: bool) {
    let Some(repo) = &manifest.dotfiles_repo else {
        dotfiles_repo_not_set();
        return;
    };

    let symlink = manifest.dotfiles_symlink.unwrap_or(false);

    match clone_dotfiles(repo, dry_run, verbose) {
        Ok(_) => install_dotfiles(symlink, verbose),
        Err(error) => dotfiles_clone_error(error, verbose),
    }
}

fn install_dotfiles(symlink: bool, verbose: bool) {
    let home_path = get_home_path();
    let dotfiles_path = home_path.join("dotfiles");

    let dotfiles_dir = match fs::read_dir(&dotfiles_path) {
        Ok(dir) => dir,
        Err(error) => dotfiles_dir_read_error(dotfiles_path, error, verbose),
    };

    dotfiles_dir
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .for_each(|entry| match symlink {
            true => symlink_config(entry, verbose),
            false => copy_config(entry, verbose),
        });
}

fn check_binary_availability(binary_name: &str) -> bool {
    let mut which_command = Command::new("which");
    which_command.arg(binary_name);

    let Ok(command_result) = which_command.output() else {
        return false;
    };

    let Some(status_code) = command_result.status.code() else {
        return false;
    };

    status_code == 0
}

fn symlink_config(entry: DirEntry, verbose: bool) {
    if entry.file_name().into_string().expect("").starts_with(".") {
        return;
    }

    // TODO: Move this to an earlier spot to check all deps up front
    if !check_binary_availability("stow") {
        panic!("stow is required to symlink dotfiles");
    }

    let dotfiles_path = get_home_path().join("dotfiles");

    let mut symlink_command = Command::new("stow");
    symlink_command.current_dir(dotfiles_path);
    symlink_command.arg("-S");
    symlink_command.arg(entry.file_name());

    let symlink_result = match symlink_command.output() {
        Ok(result) => result,
        Err(error) => {
            warn_dotfiles_symlink_failed(entry.file_name(), error);

            return;
        }
    };

    if verbose
        && let Ok(stdout) = String::from_utf8(symlink_result.stdout)
        && !stdout.is_empty()
    {
        println!("stdout {stdout:?}");
    }

    if verbose
        && let Ok(stderr) = String::from_utf8(symlink_result.stderr)
        && !stderr.is_empty()
    {
        println!("stderr {stderr:?}");
    }

    let status_code = match symlink_result.status.code() {
        Some(code) => code,
        None => {
            warn_dotfiles_symlink_signal_exit(entry.file_name());
            return;
        }
    };

    if status_code != 0 {
        warn_dotfiles_symlink_non_zero(entry.file_name(), status_code);
        return;
    }

    stow_success(entry.file_name());
}

fn copy_config(_entry: DirEntry, verbose: bool) {
    unimplemented!();
}

fn clone_dotfiles(repo: &str, dry_run: bool, verbose: bool) -> Result<(), RestoreError> {
    let dotfiles_path = get_home_path().join("dotfiles");

    if dry_run {
        dry_run_dotfiles_clone(repo, dotfiles_path);
        return Ok(());
    }

    let mut clone_command = Command::new("git");
    clone_command.arg("clone").arg(repo).arg(dotfiles_path);

    let Ok(cmd_result) = clone_command.output() else {
        return Err(RestoreError::DotfileClone);
    };

    if verbose && let Ok(stdout) = String::from_utf8(cmd_result.stdout) {
        println!("{stdout}");
    }

    if verbose && let Ok(stderr) = String::from_utf8(cmd_result.stderr) {
        println!("{stderr}");
    }

    Ok(())
}

fn restore_packages(manifest: &Manifest, dry_run: bool, verbose: bool) {
    manifest.managers.iter().for_each(|manager| match manager {
        Pacman { packages, .. } => install_arch_packages(
            "pacman",
            packages,
            manifest.locked_versions,
            dry_run,
            verbose,
        ),

        Yay { packages, .. } => {
            install_arch_packages("yay", packages, manifest.locked_versions, dry_run, verbose);
        }
    });
}

fn install_arch_packages(
    manager: &str,
    packages: &[Package],
    locked: bool,
    dry_run: bool,
    verbose: bool,
) {
    let package_list: Vec<String> = packages
        .iter()
        .map(|p| {
            if locked && p.version.is_some() {
                format!("{}={}", p.name, p.version.as_ref().unwrap())
            } else {
                p.name.clone()
            }
        })
        .collect();

    if dry_run {
        dry_run_package_install_output(manager, &package_list);
        return;
    }

    let mut command = Command::new(manager);
    command.arg("-S");
    command.arg("--needed");
    command.arg("--noconfirm");
    command.arg("--color");
    command.arg("always");

    package_list.iter().for_each(|p| {
        command.arg(p);
    });

    command.stdout(Stdio::piped());
    let command_result = match command.output() {
        Ok(result) => result,
        Err(error) => pacman_install_error(error),
    };

    if verbose {
        let stdout = String::from_utf8(command_result.stdout);
        println!("{stdout:?}");
    }

    if !command_result.status.success() {
        if verbose {
            let stderr = String::from_utf8(command_result.stderr);
            println!("{stderr:?}");
        }

        pacman_unknown_error();
    }

    package_sync_success(manager, &package_list);
}
