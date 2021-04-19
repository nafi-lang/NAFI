pub use anyhow::{anyhow, bail, Result};

pub mod flags;

mod clean;
mod codegen;
mod help;

use once_cell::sync::Lazy;

// These don't catch 100% of edge cases but are close enough for most use cases
pub static TARGET_DIR: Lazy<std::path::PathBuf> = Lazy::new(|| {
    std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| env!("CARGO_TARGET_DIR").to_owned())
        .into()
});
pub static MANIFEST_DIR: Lazy<std::path::PathBuf> = Lazy::new(|| {
    std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned())
        .into()
});
pub static WORKSPACE_DIR: Lazy<std::path::PathBuf> =
    Lazy::new(|| MANIFEST_DIR.ancestors().nth(1).unwrap().to_owned());
