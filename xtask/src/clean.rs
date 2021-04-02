use crate::*;

pub fn run() -> Result<()> {
    #![allow(clippy::needless_collect)] // rust-lang/rust-clippy#6164

    // Delete all final artifacts
    for path in fs::read_dir(TARGET.join("debug"), fs::FileType::is_file)? {
        // But cannot delete self on Windows
        if cfg!(not(windows)) || !path.ends_with("xtask.exe") {
            fs::remove(path)?;
        }
    }

    // Delete intermediate artifacts for workspace-local crates
    let to_delete = fs::read_dir("./crates", fs::FileType::is_dir)?
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
        for path in fs::read_dir(TARGET.join(target_subdir), |_| true)? {
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
                        fs::remove(path)?;
                    }
                }
                (None, _) => {
                    fs::remove(path)?;
                }
            }
        }
    }

    Ok(())
}

fn rsplit_one(s: &str, pat: char) -> (Option<&str>, &str) {
    let mut split = s.rsplitn(2, pat);
    let suffix = split.next().unwrap();
    let prefix = split.next();
    (prefix, suffix)
}
