use crate::config::{Environment, ENV};


pub fn is_dev() -> bool {
    *ENV == Environment::DEV
}
