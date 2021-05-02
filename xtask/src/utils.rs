use {
    crate::*,
    std::path::PathBuf,
    xshell::{cmd, pushenv},
};

// These don't catch 100% of edge cases but are close enough for most use cases
pub static TARGET_DIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("CARGO_TARGET_DIR")
        .unwrap_or_else(|_| env!("CARGO_TARGET_DIR").to_owned())
        .into()
});
pub static MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(|| {
    std::env::var("CARGO_MANIFEST_DIR")
        .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned())
        .into()
});
pub static WORKSPACE_DIR: Lazy<PathBuf> =
    Lazy::new(|| MANIFEST_DIR.ancestors().nth(1).unwrap().to_owned());

const PREAMBLE: &str = "Generated file, do not edit by hand; see `xtask/src/codegen`";

fn ensure_rustfmt() -> Result<()> {
    let out = cmd!("rustfmt --version").read()?;
    if !out.contains("stable") {
        bail!("Failed to run stable rustfmt");
    }
    Ok(())
}

pub fn rustfmt(code: &str) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let rustfmt_toml = WORKSPACE_DIR.join("rustfmt.toml");
    let stdout = cmd!("rustfmt --config-path {rustfmt_toml}")
        .stdin(code)
        .read()?;
    Ok(format!("//! {}\n\n{}", PREAMBLE, stdout))
}
