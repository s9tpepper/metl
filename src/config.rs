use std::{fs::read_to_string, path::PathBuf};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub locked_versions: bool,
    // pub dotfiles_path: String,
    // pub dotfiles_symlink: bool,
    pub package_managers: Vec<String>,
}

pub fn get_config_path() -> PathBuf {
    let Some(user_dirs) = UserDirs::new() else {
        panic!("Can not find user directory while loading metl config.");
    };

    let home_dir = user_dirs.home_dir();

    home_dir.join(".config")
}

pub fn load_config() -> Config {
    let metl_config_path = get_config_path().join("metl");
    let Ok(toml_str) = read_to_string(&metl_config_path) else {
        panic!("Can not find metl config toml at {metl_config_path:?}");
    };

    let parse_error = format!("Error parsing metl config file at {metl_config_path:?}");
    toml::from_str::<Config>(&toml_str).expect(&parse_error)
}

#[test]
fn test_load_config() {
    let toml = r#"
package_managers = ["pacman", "yay"]
locked_versions = true
"#;

    let Ok(config) = toml::from_str::<Config>(toml) else {
        panic!("Error parsing config toml");
    };

    assert_eq!(
        config,
        Config {
            package_managers: vec!["pacman".into(), "yay".into()],
            locked_versions: true,
        }
    );
}
