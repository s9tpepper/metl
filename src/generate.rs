use std::{
    fs,
    process::{self, Command},
};

use crate::{
    config::{Config, get_config_path, load_config},
    manifest::{Manifest, Package, PackageManager},
};

pub fn generate() {
    println!("Generating manifest...");

    let config = load_config();

    let Config {
        package_managers,
        locked_versions,
    } = config;

    let mut manifest = Manifest {
        managers: vec![],
        locked_versions,
    };

    package_managers
        .iter()
        .for_each(|package_manager| match package_manager.as_str() {
            "pacman" => get_arch_packages("pacman", &mut manifest, locked_versions),
            "yay" => get_arch_packages("yay", &mut manifest, locked_versions),

            _ => {}
        });

    write_manifest(manifest);
}

fn write_manifest(manifest: Manifest) {
    // let Ok(manifest_output) = serde_json::to_string_pretty(&manifest) else {
    let Ok(manifest_output) = toml::to_string_pretty(&manifest) else {
        panic!("Error serializing metl manifest file");
    };

    // let manifest_path = get_config_path().join("metl-manifest.json");
    let manifest_path = get_config_path().join("metl-manifest.toml");
    let _ = fs::write(manifest_path, manifest_output);
}

fn get_arch_packages(manager: &str, manifest: &mut Manifest, locked_versions: bool) {
    let flags = match manager {
        "pacman" => "-Qne",
        "yay" => "-Qme",
        _ => panic!("Unsupported Arch package manager: {manager}"),
    };

    let mut command = Command::new(manager);
    command.arg(flags);

    // TODO: Add pretty colorized error messages
    let Ok(output) = command
        .stdout(process::Stdio::piped())
        .stderr(process::Stdio::piped())
        .output()
    else {
        panic!("Error getting list of {manager} packages");
    };

    if !output.status.success() {
        panic!("Error getting list of {manager} packages");
    }

    let Ok(packages) = String::from_utf8(output.stdout) else {
        panic!("Error getting list of {manager} packages");
    };

    let packages: Vec<Package> = packages
        .split('\n')
        .filter(|p| !p.trim().is_empty())
        .map(|p| {
            println!("splitting: {p}");
            let Some((name, version)) = p.split_once(" ") else {
                panic!("Error the package output is incorrect");
            };

            Package {
                name: name.into(),
                version: if locked_versions {
                    Some(version.into())
                } else {
                    None
                },
            }
        })
        .collect();

    manifest.managers.push(match manager {
        "pacman" => PackageManager::Pacman {
            name: manager.into(),
            packages,
        },

        "yay" => PackageManager::Yay {
            name: manager.into(),
            packages,
        },

        _ => {
            panic!("Unsupported package manager: {manager}");
        }
    })
}
