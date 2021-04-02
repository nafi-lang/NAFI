pub use anyhow::{anyhow, Result};

pub mod clean;

use once_cell::sync::Lazy;

static TARGET: Lazy<std::path::PathBuf> = Lazy::new(|| {
    // Good enough. Misses configured build.target-dir and CLI --target-dir.
    std::env::var("CARGO_TARGET_DIR").map_or_else(|_| "./target".into(), |it| it.into())
});

mod fs {
    pub use std::fs::*;
    use {
        anyhow::{Context, Result},
        std::{
            fs,
            path::{Path, PathBuf},
        },
    };

    pub fn remove_file<P: AsRef<Path>>(p: P) -> Result<()> {
        fn _impl(path: &Path) -> Result<()> {
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove file {}", path.display()))
        }

        _impl(p.as_ref())
    }

    pub fn remove_dir<P: AsRef<Path>>(p: P) -> Result<()> {
        fn _impl(path: &Path) -> Result<()> {
            fs::remove_dir_all(path)
                .with_context(|| format!("Failed to remove dir {}", path.display()))
        }

        _impl(p.as_ref())
    }

    pub fn remove<P: AsRef<Path>>(p: P) -> Result<()> {
        fn _impl(path: &Path) -> Result<()> {
            if path.exists() {
                if path.is_file() {
                    remove_file(path)
                } else {
                    remove_dir(path)
                }
            } else {
                Ok(())
            }
        }

        _impl(p.as_ref())
    }

    pub fn read_dir(
        path: impl AsRef<Path>,
        cond: impl Fn(&fs::FileType) -> bool,
    ) -> Result<Vec<PathBuf>> {
        fn _impl(path: &Path, cond: &dyn Fn(&fs::FileType) -> bool) -> Result<Vec<PathBuf>> {
            let mut res = vec![];
            for entry in path
                .read_dir()
                .with_context(|| format!("Failed to read dir {}", path.display()))?
                .flatten()
            {
                let path = entry.path();
                let ftype = entry
                    .file_type()
                    .with_context(|| format!("Failed to type file {}", path.display()))?;
                if cond(&ftype) {
                    res.push(path);
                }
            }
            Ok(res)
        }

        _impl(path.as_ref(), &cond)
    }
}
