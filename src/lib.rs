// Please, let's never use unsafe code.
#![forbid(unsafe_code)]
#![deny(
  // We like future proofing and being up to date with the latest and greatest c:
  future_incompatible,
  keyword_idents,
  rust_2018_compatibility,
  rust_2018_idioms,
  rust_2021_compatibility,
  rust_2024_compatibility,

  // Clean + safe + efficient code is great.
  let_underscore,
  nonstandard_style,
  refining_impl_trait,
  clippy::all,
)]
#![warn(
  // Only here till we implement proper documentation...
  clippy::missing_errors_doc,
  clippy::missing_panics_doc,

  // We should be a little lenient on ourselves, right?
  // Also, be careful when using allow(unused):
  // https://www.reddit.com/r/rust/comments/1enkpxh/pro_tip_use_expectunused_upcoming_181_release/
  unused,
)]
// Not this lenient tho, lol
#![deny(unused_imports)]

// Lint groups for tracking:
// https://doc.rust-lang.org/rustc/lints/groups.html
// https://rust-lang.github.io/rust-clippy/master/index.html

pub mod business;
pub mod configs;
pub mod types;
pub mod utils;
pub mod views;
