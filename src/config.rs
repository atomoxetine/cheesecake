use std::str::FromStr;

use lazy_static::lazy_static;

fn var<T: FromStr>(name: &'static str) -> T {
    std::env::var(name)
        .expect(format!("Couldn't find env variable {}", name).as_str())
        .parse::<T>()
        .ok()
        .expect(format!("Couldn't parse env variable {}", name).as_str())
}

#[allow(dead_code)]
fn var_opt<T: FromStr>(name: &'static str) -> Option<T> {
    std::env::var(name).ok()?.parse::<T>().ok()
}

lazy_static! {
    pub static ref PORT: u16 = var("PORT");
}
