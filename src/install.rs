use std::{io::Write, process::Command};

use crate::{
    errors::{install_failed, package_install_failed},
    generate::generate,
    successes::install_successful,
};

pub fn install(args: Vec<String>) {
    // TODO: update this after refactoring config to use only a single defined package manager
    let mut command = Command::new("paru");
    let mut installed = String::new();

    if args.len() == 1 {
        command.arg("-S");
        command.arg("--noconfirm");
        command.arg(args[0].clone());

        installed.push_str(&args[0]);
    } else {
        args.iter().for_each(|arg| {
            command.arg(arg);
        });

        installed.push_str(&args.join(" "));
    }

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => package_install_failed(&installed, error),
    };

    let code = output.status.code().unwrap_or(1);
    let verbose = has_verbose(&args);

    if verbose && !output.stdout.is_empty() {
        let _ = std::io::stdout().write_all(&output.stdout);
    }

    if verbose && !output.stderr.is_empty() {
        let _ = std::io::stderr().write_all(&output.stderr);
    }

    if code == 0 {
        install_successful(&installed);
        generate();
    } else {
        install_failed(&installed, code);
    }
}

fn has_verbose(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-v" || arg == "--verbose")
}
