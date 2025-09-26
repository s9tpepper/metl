use std::{fs::read_to_string, path::PathBuf};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::{errors::missing_metl_config, manifest::PackageManager};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub locked_versions: bool,
    pub package_manager: PackageManager,
    pub dotfiles_repo: String,
    pub dotfiles_symlink: bool,
    pub manifest_repo: String,
}

pub fn get_home_path() -> PathBuf {
    let Some(user_dirs) = UserDirs::new() else {
        panic!("Can not find user directory while loading metl config.");
    };

    user_dirs.home_dir().to_path_buf()
}

pub fn get_config_path() -> PathBuf {
    let home_dir = get_home_path();

    home_dir.join(".config").join("metl")
}

pub fn load_config() -> Config {
    let metl_config_path = get_config_path().join("config");
    let Ok(toml_str) = read_to_string(&metl_config_path) else {
        missing_metl_config(metl_config_path);
    };

    let parse_error = format!("Error parsing metl config file at {metl_config_path:?}");
    toml::from_str::<Config>(&toml_str).expect(&parse_error)
}

#[test]
fn test_load_config() {
    let toml = r#"
package_manager = "pacman"
locked_versions = true
dotfiles_repo = "repo_url"
dotfiles_symlink = true
manifest_repo = "repo_url"
"#;

    let Ok(config) = toml::from_str::<Config>(toml) else {
        panic!("Error parsing config toml");
    };

    assert_eq!(
        config,
        Config {
            package_manager: PackageManager::Pacman,
            locked_versions: true,
            dotfiles_repo: "repo_url".into(),
            dotfiles_symlink: true,
            manifest_repo: "manifest repo".into(),
        }
    );
}
