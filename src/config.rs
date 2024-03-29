use std::env::var;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PORT: u16 = var("PORT").unwrap().parse::<u16>().unwrap();
}
