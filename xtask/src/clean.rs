use {crate::*, walkdir::WalkDir};

pub fn run(args: pico_args::Arguments) -> Result<()> {
    if !args.finish().is_empty() {
        return Err(anyhow!("Excess arguments to `cargo xtask clean`"));
    }

    // Delete all final artifacts
    for entry in WalkDir::new(TARGET.as_path()).max_depth(2) {
        let entry = entry?;
        if entry.file_type().is_file() {
            // But cannot delete self on Windows
            let path = entry.path();
            if cfg!(not(windows)) || !path.ends_with("xtask.exe") {
                fs::remove(path)?;
            }
        }
    }

    // Delete intermediate artifacts for workspace-local crates
    let to_delete = ["nafi", "xtask"];
    for entry in WalkDir::new(TARGET.as_path()).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        let disp = path.display().to_string();
        if to_delete.iter().any(|d| disp.contains(d)) {
            // But cannot delete self on Windows
            if cfg!(not(windows)) || !path.ends_with("xtask.exe") {
                fs::remove(path)?;
            }
        }
    }

    Ok(())
}
