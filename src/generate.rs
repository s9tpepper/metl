use std::{
    fs,
    process::{self, Command},
};

use crate::{
    config::{Config, get_config_path, load_config},
    errors::{
        manifest_serialization_error, packages_list_error, packages_parsing_error,
        unsupported_package_manager,
    },
    manifest::{Manifest, Package, PackageManager},
    successes::packages_retrieved_successfully,
    warnings::warn_package_output,
};

pub fn generate() {
    let config = load_config();

    let Config {
        package_manager,
        locked_versions,
        dotfiles_repo,
        dotfiles_symlink,
    } = config;

    let mut manifest = Manifest {
        managers: vec![],
        locked_versions,
        dotfiles_repo,
        dotfiles_symlink,
    };

    match package_manager.as_str() {
        "pacman" => get_arch_packages("pacman", &mut manifest, locked_versions),
        "paru" => get_arch_packages("paru", &mut manifest, locked_versions),
        "yay" => get_arch_packages("yay", &mut manifest, locked_versions),
        _ => unsupported_package_manager(&package_manager),
    }

    write_manifest(manifest);
}

fn write_manifest(manifest: Manifest) {
    let Ok(manifest_output) = toml::to_string_pretty(&manifest) else {
        manifest_serialization_error();
    };

    let manifest_path = get_config_path().join("metl-manifest.toml");
    let _ = fs::write(manifest_path, manifest_output);
}

fn get_arch_packages(manager: &str, manifest: &mut Manifest, locked_versions: bool) {
    let flags = match manager {
        "pacman" | "yay" | "paru" => "-Qe",

        _ => unsupported_package_manager(manager),
    };

    let mut command = Command::new(manager);
    command.arg(flags);

    let list_cmd_result = command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output();

    let output = match list_cmd_result {
        Ok(output) => output,
        Err(error) => packages_list_error(Some(error)),
    };

    if !output.status.success() {
        packages_list_error(None);
    }

    let Ok(packages) = String::from_utf8(output.stdout) else {
        packages_parsing_error(manager);
    };

    let package_list: Vec<&str> = packages
        .split('\n')
        .filter(|p| !p.trim().is_empty())
        .collect();

    let mut packages: Vec<Package> = vec![];

    for p in package_list {
        let Some((name, version)) = p.split_once(" ") else {
            warn_package_output(p);
            continue;
        };

        packages.push(Package {
            name: name.into(),
            version: if locked_versions {
                Some(version.into())
            } else {
                None
            },
        })
    }

    manifest.managers.push(match manager {
        "pacman" => PackageManager::Pacman {
            name: manager.into(),
            packages,
        },

        "yay" => PackageManager::Yay {
            name: manager.into(),
            packages,
        },

        _ => unsupported_package_manager(manager),
    });

    packages_retrieved_successfully(manager);
}
