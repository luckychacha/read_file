use std::{fs::File, io::Read, path::Path};

#[cfg(feature = "o")]
#[path = "origin.rs"]
mod sum;

#[cfg(feature = "r1")]
#[path = "refactor1.rs"]
mod sum;

#[cfg(all(not(feature = "o"), not(feature = "r1")))]
#[path = "refactor2.rs"]
mod sum;

pub use crate::sum::sum;
