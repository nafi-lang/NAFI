//! See https://github.com/matklad/cargo-xtask/.
//!
//! This binary defines various auxiliary build commands,
//! which are not expressible with just `cargo`.

use xtask::*;

fn main() -> Result<()> {
    let _d = xshell::pushd(&*WORKSPACE_DIR)?;

    let flags = flags::Xtask::from_env()?;
    match flags.subcommand {
        flags::XtaskCmd::Help(cmd) => cmd.run(),
        flags::XtaskCmd::Clean(cmd) => cmd.run(),
    }
}
