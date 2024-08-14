use std::str::FromStr;

pub mod environment;
pub mod helpers;

mod app;
pub use app::*;

mod shutdown_signal;
pub use shutdown_signal::*;

#[derive(PartialEq, Eq)]
pub enum Environment {
    PROD,
    DEV,
}

impl FromStr for Environment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Environment::{DEV, PROD};
        match s {
            "PROD" => Ok(PROD),
            "DEV" => Ok(DEV),
            _ => Err("Invalid environment string"),
        }
    }
}

fn var<T: FromStr>(name: &'static str) -> T {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("Couldn't find env variable {name}"))
        .parse::<T>()
        .ok()
        .unwrap_or_else(|| panic!("Couldn't parse env variable {name}"))
}

#[allow(dead_code)]
fn var_opt<T: FromStr>(name: &'static str) -> Option<T> {
    std::env::var(name).ok()?.parse::<T>().ok()
}
