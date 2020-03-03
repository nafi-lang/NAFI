pub use anyhow::Result;

pub mod clean;

use once_cell::sync::Lazy;

static TARGET: Lazy<std::path::PathBuf> = Lazy::new(|| {
    std::env::var("CARGO_TARGET_DIR").map_or_else(|_| "./target".into(), |it| it.into())
});

mod fs {
    use {
        anyhow::{Context, Result},
        std::{fs, path::Path},
    };

    pub fn remove_file<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref();
        fs::remove_file(p).with_context(|| format!("Failed to remove file {}", p.display()))
    }

    pub fn remove_dir<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref();
        fs::remove_dir_all(p).with_context(|| format!("Failed to remove dir {}", p.display()))
    }

    pub fn remove<P: AsRef<Path>>(p: P) -> Result<()> {
        let p = p.as_ref();
        if p.exists() {
            if p.is_file() {
                remove_file(p)
            } else {
                remove_dir(p)
            }
        } else {
            Ok(())
        }
    }
}
