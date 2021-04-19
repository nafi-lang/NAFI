use {
    crate::*,
    std::path::{Path, PathBuf},
    xshell::{read_dir, rm_rf},
};

impl flags::Clean {
    pub fn run(self) -> Result<()> {
        #![allow(clippy::needless_collect)] // rust-lang/rust-clippy#6164

        // Delete all final artifacts
        for path in read_dir_files(TARGET_DIR.join("debug"))? {
            // But cannot delete self on Windows
            if cfg!(not(windows)) || !path.ends_with("xtask.exe") {
                rm_rf(path)?;
            }
        }

        // Delete intermediate artifacts for workspace-local crates
        let to_delete = read_dir_files(WORKSPACE_DIR.join("crates"))?
            .into_iter()
            .map(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .replace("-", "_")
            })
            .chain(Some("xtask".into()))
            .collect::<Vec<_>>();

        for &target_subdir in ["debug/deps", "debug/.fingerprint"].iter() {
            for path in read_dir(TARGET_DIR.join(target_subdir))? {
                // But cannot delete self on Windows
                if cfg!(windows) && path.ends_with("xtask.exe") {
                    continue;
                }

                let fname = path.file_name().unwrap().to_string_lossy();
                // Strip hash disambiguator
                match rsplit_one(&fname, '-') {
                    (Some(stem), _) => {
                        let stem = stem.replace('-', "_");
                        // Delete if local
                        if to_delete.contains(&stem) {
                            rm_rf(path)?;
                        }
                    }
                    (None, _) => {
                        rm_rf(path)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn read_dir_files(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    fn _impl(path: &Path) -> Result<Vec<PathBuf>> {
        let mut res = vec![];
        for entry in path.read_dir()?.flatten() {
            if entry.file_type()?.is_file() {
                res.push(entry.path());
            }
        }
        Ok(res)
    }

    _impl(path.as_ref())
}

fn rsplit_one(s: &str, pat: char) -> (Option<&str>, &str) {
    let mut split = s.rsplitn(2, pat);
    let suffix = split.next().unwrap();
    let prefix = split.next();
    (prefix, suffix)
}
