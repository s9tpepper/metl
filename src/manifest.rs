use std::fs;

use serde::{Deserialize, Serialize};

use crate::{
    config::get_config_path,
    errors::{failed_reading_manifest, manifest_parsing_error},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub packages: Vec<Package>,
}

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PackageManager {
    #[default]
    #[serde(rename(serialize = "pacman", deserialize = "pacman"))]
    Pacman,

    #[serde(rename(serialize = "paru", deserialize = "paru"))]
    Paru,

    #[serde(rename(serialize = "yay", deserialize = "yay"))]
    Yay,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for PackageManager {
    fn to_string(&self) -> String {
        match self {
            PackageManager::Paru => "paru".to_string(),
            PackageManager::Pacman => "pacman".to_string(),
            PackageManager::Yay => "yay".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: Option<String>,
}

pub fn load_manifest() -> Manifest {
    let manifest_path = get_config_path().join("manifest.toml");

    let manifest_contents = match fs::read_to_string(&manifest_path) {
        Ok(contents) => contents,
        Err(error) => failed_reading_manifest(error, manifest_path),
    };

    match toml::from_str::<Manifest>(manifest_contents.trim()) {
        Ok(manifest) => manifest,
        Err(error) => manifest_parsing_error(&error, manifest_path),
    }
}
