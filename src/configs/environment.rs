use lazy_static::lazy_static;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
};
use tracing::level_filters::LevelFilter;

use crate::configs::{var, var_opt, Environment};

lazy_static! {
    pub static ref HOSTNAME: IpAddr =
        var_opt("HOSTNAME").unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    pub static ref PORT: u16 = var_opt("PORT").unwrap_or(3000);
    pub static ref ENV: Environment =
        var_opt("ENV").unwrap_or(Environment::PROD);
    pub static ref STDOUT_LOG_SEVERITY: LevelFilter =
        var_opt("STDOUT_LOG_SEVERITY").unwrap_or(LevelFilter::INFO);
    pub static ref LOG_DIRECTORY: PathBuf = var_opt("LOG_DIRECTORY")
        .unwrap_or_else(|| PathBuf::from("/var/log/cheesecake"));
    pub static ref DATABASE_URL: String = var("DATABASE_URL");
    pub static ref MAX_CONN_POOL: u32 = var_opt("MAX_CONN_POOL").unwrap_or(100);
}
