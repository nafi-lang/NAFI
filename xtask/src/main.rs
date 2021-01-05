//! See https://github.com/matklad/cargo-xtask/.
//!
//! This binary defines various auxiliary build commands,
//! which are not expressible with just `cargo`.

use xtask::*;

const USAGE: &str = "\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <command>

COMMANDS:
    clean                    Clean $CARGO_TARGET of workspace-local artifacts
";

fn finish(args: pico_args::Arguments, usage: &str) -> Result<()> {
    let leftover = args.finish();
    if leftover.is_empty() {
        Ok(())
    } else {
        eprint!("{}", usage);
        Err(anyhow!("unexpected arguments"))
    }
}

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()?.as_deref() {
        None => {
            finish(args, USAGE)?;
            eprint!("{}", USAGE);
            Ok(())
        }
        Some("clean") => {
            finish(
                args,
                "\
cargo xtask clean
Clean $CARGO_TARGET of workspace-local artifacts.

USAGE:
    cargo xtask clean
",
            )?;
            clean::run()
        }
        Some(x) => {
            eprint!("{}", USAGE);
            Err(anyhow!("unexpected subcommand `{}`", x))
        }
    }
}
