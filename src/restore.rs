use std::process::{Command, Stdio};

use crate::manifest::{
    Package,
    PackageManager::{Pacman, Yay},
    load_manifest,
};

pub fn restore(dry_run: bool) {
    let manifest = load_manifest();

    manifest.managers.iter().for_each(|manager| match manager {
        Pacman { packages, .. } => {
            install_arch_packages("pacman", packages, manifest.locked_versions, dry_run)
        }

        Yay { packages, .. } => {
            install_arch_packages("yay", packages, manifest.locked_versions, dry_run);
        }
    });
}

fn install_arch_packages(manager: &str, packages: &[Package], locked: bool, dry_run: bool) {
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
        // TODO: Make this output pretty.
        println!("{manager} -Sp {}", package_list.join(" "));
        return;
    }

    let mut command = Command::new(manager);
    command.arg("-S");
    command.arg("--needed");
    command.arg("--no-confirm");

    package_list.iter().for_each(|p| {
        command.arg(p);
    });

    command.stdout(Stdio::piped());
    let Ok(command_result) = command.output() else {
        panic!("Error running pacman install");
    };

    if !command_result.status.success() {
        let stdout = String::from_utf8(command_result.stdout);
        let stderr = String::from_utf8(command_result.stderr);

        println!("{stdout:?}");
        println!("{stderr:?}");

        panic!("Error running pacman install");
    }
}
