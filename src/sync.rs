use std::{
    fs::{self, DirEntry},
    process::{Command, Stdio},
};

use thiserror::Error;

use crate::{
    config::get_home_path,
    errors::{pacman_install_error, pacman_unknown_error},
    manifest::{
        Manifest, Package,
        PackageManager::{Pacman, Yay},
        load_manifest,
    },
    successes::{dry_run_package_install_output, package_sync_success},
    warnings::dotfiles_repo_not_set,
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

    match clone_dotfiles(repo, dry_run) {
        Ok(_) => install_dotfiles(symlink, verbose),
        Err(error) => println!("{error}"),
    }
}

fn install_dotfiles(symlink: bool, verbose: bool) {
    let home_path = get_home_path();
    let dotfiles_path = home_path.join("dotfiles");

    let Ok(dotfiles_dir) = fs::read_dir(dotfiles_path) else {
        panic!("Error reading the dotfiles directory");
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

    if !check_binary_availability("stow") {
        panic!("stow is required to symlink dotfiles");
    }

    let dotfiles_path = get_home_path().join("dotfiles");

    let mut symlink_command = Command::new("stow");
    symlink_command.current_dir(dotfiles_path);
    symlink_command.arg("-S");
    symlink_command.arg(entry.file_name());

    // TODO: Update errors so they're unique per scenario
    let Ok(symlink_result) = symlink_command.output() else {
        panic!("stow was unable to install {:?}", entry.file_name());
    };

    if let Ok(stdout) = String::from_utf8(symlink_result.stdout) {
        println!("{stdout:?}");
    }

    if let Ok(stderr) = String::from_utf8(symlink_result.stderr) {
        println!("{stderr:?}");
    }

    let Some(status_code) = symlink_result.status.code() else {
        panic!("stow was unable to install {:?}", entry.file_name());
    };

    if status_code != 0 {
        panic!("stow was unable to install {:?}", entry.file_name());
    }
}

fn copy_config(_entry: DirEntry, verbose: bool) {
    unimplemented!();
}

// TODO: handle dry_run bool
fn clone_dotfiles(repo: &str, dry_run: bool) -> Result<(), RestoreError> {
    let mut clone_command = Command::new("git");

    let dotfiles_path = get_home_path().join("dotfiles");

    clone_command.arg("clone").arg(repo).arg(dotfiles_path);
    let Ok(cmd_result) = clone_command.output() else {
        // TODO: FIgure out how to message this

        return Err(RestoreError::DotfileClone);
    };

    if let Ok(stdout) = String::from_utf8(cmd_result.stdout) {
        println!("{stdout}");
    }

    if let Ok(stderr) = String::from_utf8(cmd_result.stderr) {
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
