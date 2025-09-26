use crate::{
    config::load_config,
    errors::install_failed,
    manifest::PackageManager::{Pacman, Paru, Yay},
    proxies::pacman_compatible_proxy,
    successes::install_successful,
};

pub fn install(args: Vec<String>) {
    let config = load_config();

    match config.package_manager {
        Pacman | Paru | Yay => {
            pacman_compatible_proxy(
                &config.package_manager,
                &args,
                vec!["-S", "--noconfirm"],
                |proxied| {
                    install_successful(&config.package_manager, proxied);
                },
                |proxied, code| {
                    install_failed(&config.package_manager, proxied, code);
                },
            );
        }
    }
}
