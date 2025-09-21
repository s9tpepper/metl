use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    config::get_config_path,
    errors::{failed_reading_manifest, manifest_parsing_error},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub managers: Vec<PackageManager>,
    pub locked_versions: bool,
    pub dotfiles_repo: Option<String>,
    pub dotfiles_symlink: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PackageManager {
    #[serde(rename(serialize = "pacman", deserialize = "pacman"))]
    Pacman {
        name: String,
        packages: Vec<Package>,
    },

    #[serde(rename(serialize = "yay", deserialize = "yay"))]
    Yay {
        name: String,
        packages: Vec<Package>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
}

pub fn load_manifest() -> Manifest {
    let manifest_path = get_config_path().join("metl-manifest.toml");

    let manifest_contents = match fs::read_to_string(&manifest_path) {
        Ok(contents) => contents,
        Err(error) => failed_reading_manifest(error, manifest_path),
    };

    match toml::from_str::<Manifest>(manifest_contents.trim()) {
        Ok(manifest) => manifest,
        Err(error) => manifest_parsing_error(&error, manifest_path),
    }
}
