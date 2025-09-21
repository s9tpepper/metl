use std::{io::Write, process::Command};

use crate::generate::generate;

pub fn install(args: Vec<String>) {
    // TODO: update this after refactoring config to use only a single defined package manager
    let mut command = Command::new("paru");
    args.iter().for_each(|arg| {
        command.arg(arg);
    });

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => panic!("{error:?}"),
    };

    if !output.stdout.is_empty() {
        let _ = std::io::stdout().write_all(&output.stdout);
    }

    if !output.stderr.is_empty() {
        let _ = std::io::stderr().write_all(&output.stderr);
    }

    generate();
}
