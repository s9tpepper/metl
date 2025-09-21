use std::{io::Write, process::Command};

use crate::{errors::package_install_failed, generate::generate};

pub fn pacman_proxy<S, F>(args: Vec<String>, default_args: Vec<&str>, success: S, failed: F)
where
    S: Fn(&str),
    F: Fn(&str, i32),
{
    // TODO: update this after refactoring config to use only a single defined package manager
    let mut command = Command::new("paru");
    let mut proxied_cmd = String::new();

    if args.len() == 1 {
        default_args.iter().for_each(|arg| {
            command.arg(arg);
        });

        command.arg(args[0].clone());

        proxied_cmd.push_str(&args[0]);
    } else {
        args.iter().for_each(|arg| {
            command.arg(arg);
        });

        proxied_cmd.push_str(&args.join(" "));
    }

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => package_install_failed(&proxied_cmd, error),
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
        success(&proxied_cmd);
        generate();
    } else {
        failed(&proxied_cmd, code);
    }
}

fn has_verbose(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "-v" || arg == "--verbose")
}
