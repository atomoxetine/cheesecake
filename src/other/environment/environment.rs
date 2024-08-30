use lazy_static::lazy_static;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
};
use tracing::level_filters::LevelFilter;

use crate::{owned_var_or, try_leak, var, var_or, var_or_else};

lazy_static! {
    pub static ref HOSTNAME: IpAddr =
        owned_var_or("HOSTNAME", IpAddr::V4(Ipv4Addr::LOCALHOST));
    pub static ref PORT: u16 = owned_var_or("PORT", 3000);
    pub static ref STDOUT_LOG_SEVERITY: LevelFilter =
        owned_var_or("STDOUT_LOG_SEVERITY", LevelFilter::INFO);
    pub static ref LOG_DIRECTORY: &'static Path = var_or_else(
        "LOG_DIRECTORY",
        || PathBuf::from("/var/log/cheesecake")
    );
    pub static ref DATABASE_URL: &'static str =
        var::<String, _>("DATABASE_URL");
    pub static ref DB_CONN_POOL_MAX: u32 =
        owned_var_or("DB_CONN_POOL_MAX", 100);
    pub static ref DOMAIN: &'static str =
        var_or::<String, _>("DOMAIN", "localhost");
}

#[cfg(debug_assertions)]
lazy_static! {
    pub static ref WORKSPACE_DIR: &'static Path = {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path =
            Path::new(std::str::from_utf8(&output).unwrap().trim());
        try_leak(cargo_path.parent().unwrap().to_path_buf()).unwrap()
    };
}

#[cfg(not(debug_assertions))]
lazy_static! {
    pub static ref WORKSPACE_DIR: &'static Path =
        try_leak(Path::new(".")).unwrap();
}
