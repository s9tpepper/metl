use std::{path::PathBuf, process::Command};

use thiserror::Error;

use crate::{
    config::{get_config_path, load_config},
    successes::{git_metl_manifest_commit_success, git_push_metl_manifest_success},
    warnings::{
        warn_git_add_metl_manifest_code, warn_git_add_metl_manifest_failed,
        warn_git_push_metl_manifest_failed, warn_metl_manifest_commit_code,
        warn_metl_manifest_commit_failed,
    },
};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum CommitMetlManifestError {
    #[error("Git error staging files")]
    AddSigInt,

    #[error("Received sigint while running git add")]
    AddError { error: std::io::Error },

    #[error("Received non-zero code from git add")]
    AddFailed { code: i32 },
}

pub fn commit_manifest(proxied_cmd: &str) {
    let metl_config_path = get_config_path();

    match git_add_files(&metl_config_path) {
        Ok(_) => git_commit_metl_manifest(&metl_config_path, proxied_cmd),

        Err(git_add_error) => match git_add_error {
            CommitMetlManifestError::AddError { error } => {
                warn_git_add_metl_manifest_failed(Some(error))
            }
            CommitMetlManifestError::AddSigInt => warn_git_add_metl_manifest_failed(None),
            CommitMetlManifestError::AddFailed { code } => warn_git_add_metl_manifest_code(code),
        },
    }
}

fn git_commit_metl_manifest(metl_manifest_repo_path: &PathBuf, proxied_cmd: &str) {
    let config = load_config();
    let mut command = Command::new("git");
    command.current_dir(metl_manifest_repo_path);

    let commit_msg = format!(
        "Updated with: {} {}",
        config.package_manager.to_string(),
        proxied_cmd
    );
    command.arg("commit").arg("-m").arg(commit_msg);

    match command.output() {
        Ok(output) => match output.status.code() {
            Some(code) => match code {
                0 => {
                    git_metl_manifest_commit_success(proxied_cmd);
                    git_push_metl_manifest(metl_manifest_repo_path);
                }
                code => warn_metl_manifest_commit_code(code),
            },

            None => warn_metl_manifest_commit_failed(None),
        },

        Err(error) => warn_metl_manifest_commit_failed(Some(error)),
    }
}

pub fn git_push_metl_manifest(working_copy_path: &PathBuf) {
    let mut command = Command::new("git");
    command.current_dir(working_copy_path);
    command.arg("push").arg("origin").arg("master");

    match command.output() {
        Ok(output) => match output.status.code() {
            Some(code) => match code {
                0 => git_push_metl_manifest_success(),
                code => warn_git_push_metl_manifest_failed(None, Some(code)),
            },
            None => warn_git_push_metl_manifest_failed(None, None),
        },

        Err(error) => warn_git_push_metl_manifest_failed(Some(error), None),
    }
}

fn git_add_files(working_copy_path: &PathBuf) -> Result<(), CommitMetlManifestError> {
    let mut command = Command::new("git");
    command.current_dir(working_copy_path);
    command.arg("add").arg(".");

    match command.output() {
        Ok(output) => match output.status.code() {
            Some(code) => match code {
                0 => Ok(()),
                code => Err(CommitMetlManifestError::AddFailed { code }),
            },
            None => Err(CommitMetlManifestError::AddSigInt),
        },

        Err(error) => Err(CommitMetlManifestError::AddError { error }),
    }
}
