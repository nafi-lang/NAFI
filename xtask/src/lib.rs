pub use {
    crate::utils::{rustfmt, TARGET_DIR, WORKSPACE_DIR},
    anyhow::{anyhow, bail, Result},
    once_cell::sync::Lazy,
};

pub mod flags;

mod clean;
mod codegen;
mod help;

mod utils;
