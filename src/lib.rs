use std::{fs::File, io::Read, path::Path};

#[cfg(feature = "o")]
#[path = "origin.rs"]
mod sum;

#[cfg(feature = "r1")]
#[path = "refactor1.rs"]
mod sum;

#[cfg(feature = "r2")]
#[path = "refactor2.rs"]
mod sum;

pub use crate::sum::sum;

// cargo run --no-default-features  --features r1
// engine_test lib.rs
