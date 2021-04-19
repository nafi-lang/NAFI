use {
    crate::*,
    std::{
        fs::FileType,
        path::{Path, PathBuf},
    },
};

impl flags::Clean {
    pub fn run(self) -> Result<()> {
        #![allow(clippy::needless_collect)] // rust-lang/rust-clippy#6164

        // Delete all final artifacts
        for path in read_dir(TARGET_DIR.join("debug"), FileType::is_file)? {
            // But cannot delete self on Windows
            if cfg!(not(windows)) || !path.ends_with("xtask.exe") {
                xshell::rm_rf(path)?;
            }
        }

        // Delete intermediate artifacts for workspace-local crates
        let to_delete = read_dir(WORKSPACE_DIR.join("crates"), FileType::is_dir)?
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
            for path in xshell::read_dir(TARGET_DIR.join(target_subdir))? {
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
                            xshell::rm_rf(path)?;
                        }
                    }
                    (None, _) => {
                        xshell::rm_rf(path)?;
                    }
                }
            }
        }

        Ok(())
    }
}

fn read_dir(path: impl AsRef<Path>, cond: impl Fn(&FileType) -> bool) -> Result<Vec<PathBuf>> {
    fn _impl(path: &Path, cond: &dyn Fn(&FileType) -> bool) -> Result<Vec<PathBuf>> {
        let mut res = vec![];
        for entry in path.read_dir()?.flatten() {
            if cond(&entry.file_type()?) {
                res.push(entry.path());
            }
        }
        Ok(res)
    }

    _impl(path.as_ref(), &cond)
}

fn rsplit_one(s: &str, pat: char) -> (Option<&str>, &str) {
    let mut split = s.rsplitn(2, pat);
    let suffix = split.next().unwrap();
    let prefix = split.next();
    (prefix, suffix)
}
