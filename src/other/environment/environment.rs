//! In this file, we define the environment variables that are used multiple times
//! in the program, and therefore are better left as static variables.
//!
//! There are other environment variables that are only used once:
//! - `LOG_SEVERITY` - Severity level for the log file and stdout.
//!   * Defaults to `INFO`. Used at `utils::init_logging`.
//! - `LOG_DIRECTORY` - The directory where the log files are stored.
//!   * Defaults to `/var/log/LunarParfait/conecta`. Used at `utils::init_logging`.
//! - `DATABASE_URL` - The URL to the database.
//!   * Does not have a default. Program will panic if undefined. Used at `Database::init`.
//! - `DB_CONN_POOL_MAX` - The maximum number of connections to the database.
//!   * Defaults to `100`. Used at `Database::init`.

use anyhow::anyhow;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
};

use crate::{owned_var_or, try_leak, var_or, EnvLock};

pub struct Environment {
    pub hostname: IpAddr,
    pub port: u16,
    pub domain: &'static str,
    pub workspace_dir: &'static Path,
}

impl Environment {
    /// # Panics
    /// Will panic if it fails to parse the environment variables
    #[must_use]
    pub fn new(workspace_dir: &'static Path) -> Self {
        Self {
            hostname: owned_var_or("HOSTNAME", IpAddr::V4(Ipv4Addr::LOCALHOST)),
            port: owned_var_or("PORT", 3000),
            domain: var_or::<String, _>("DOMAIN", "localhost"),
            workspace_dir,
        }
    }
}

pub static ENV: EnvLock = EnvLock::new();

/// # Panics
/// will panic if it fails to get workspace dir
#[must_use]
pub fn get_workspace_dir() -> &'static Path {
    if cfg!(not(debug_assertions)) {
        try_leak(Path::new(".")).unwrap()
    } else {
        (|| {
            let child_path_u8 = std::process::Command::new(env!("CARGO"))
                .arg("locate-project")
                .arg("--workspace")
                .arg("--message-format=plain")
                .output()?
                .stdout;
            let child_path_str = std::str::from_utf8(&child_path_u8)?.trim();
            let final_path =
                Path::new(child_path_str).parent().ok_or_else(|| {
                    anyhow!(
                        "Couldn't find the parent directory of the workspace"
                    )
                })?;
            Ok::<&Path, anyhow::Error>(try_leak(final_path.to_path_buf())?)
        })()
        .map_err(|e| panic!("Failed to set WORKSPACE_DIR: {e}"))
        .unwrap()
    }
}
