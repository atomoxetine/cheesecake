use std::str::FromStr;

use lazy_static::lazy_static;

#[derive(PartialEq, Eq)]
pub enum Environment {
    PROD,
    DEV,
}

impl FromStr for Environment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Environment::*;
        match s {
            "PROD" => Ok(PROD),
            "DEV" => Ok(DEV),
            _ => Err("Invalid environment string")
        }
    }
}

fn var<T: FromStr>(name: &'static str) -> T {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("Couldn't find env variable {}", name))
        .parse::<T>()
        .ok()
        .unwrap_or_else(|| panic!("Couldn't parse env variable {}", name))
}

#[allow(dead_code)]
fn var_opt<T: FromStr>(name: &'static str) -> Option<T> {
    std::env::var(name).ok()?.parse::<T>().ok()
}

lazy_static! {
    pub static ref PORT: u16 = var("PORT");
    pub static ref ENV: Environment = var_opt("ENV").unwrap_or(Environment::DEV);
}

// helper functions

pub fn is_dev() -> bool {
    *ENV == Environment::DEV
}
