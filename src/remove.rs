use crate::{errors::remove_failed, proxies::pacman_proxy, successes::remove_successful};

pub fn remove(args: Vec<String>) {
    // TODO: read config and call the correct proxy function for the configured package manager
    pacman_proxy(
        args,
        vec!["-R", "--noconfirm"],
        |proxied| {
            remove_successful(proxied);
        },
        |proxied, code| {
            remove_failed(proxied, code);
        },
    );
}
