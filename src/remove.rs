use crate::{
    config::load_config,
    errors::remove_failed,
    manifest::PackageManager::{Pacman, Paru, Yay},
    proxies::pacman_compatible_proxy,
    successes::remove_successful,
};

pub fn remove(args: Vec<String>) {
    let config = load_config();

    match config.package_manager {
        Pacman | Paru | Yay => {
            pacman_compatible_proxy(
                &config.package_manager,
                &args,
                vec!["-R", "--noconfirm"],
                |proxied| {
                    remove_successful(&config.package_manager, proxied);
                },
                |proxied, code| {
                    remove_failed(&config.package_manager, proxied, code);
                },
            );
        }
    }
}
