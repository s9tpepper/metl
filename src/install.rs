use crate::{errors::install_failed, proxies::pacman_proxy, successes::install_successful};

pub fn install(args: Vec<String>) {
    // TODO: read config and call the correct proxy function for the configured package manager
    pacman_proxy(
        args,
        vec!["-S", "--noconfirm"],
        |proxied| {
            install_successful(proxied);
        },
        |proxied, code| {
            install_failed(proxied, code);
        },
    );
}
