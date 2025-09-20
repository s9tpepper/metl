use std::fs;

use serde::{Deserialize, Serialize};

use crate::config::get_config_path;

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

    let Ok(manifest_contents) = fs::read_to_string(manifest_path) else {
        panic!("Error getting manifest contents");
    };

    println!("manifest_contents: {manifest_contents:?}");
    // let Ok(manifest) = toml::from_str::<Manifest>(manifest_contents.trim()) else {
    //     panic!("Error parsing manifest contents");
    // };

    let manifest_result = toml::from_str::<Manifest>(manifest_contents.trim());
    if let Err(error) = &manifest_result {
        println!("error: {error:?}");
    }

    manifest_result.unwrap()
}
