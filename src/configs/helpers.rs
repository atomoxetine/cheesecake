use super::{environment::ENV, Environment};

#[must_use]
pub fn is_dev() -> bool {
    *ENV == Environment::DEV
}
