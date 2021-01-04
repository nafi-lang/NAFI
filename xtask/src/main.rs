use xtask::*;

const USAGE: &str = "\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <command>

COMMANDS:
  clean                      Clean $CARGO_TARGET of workspace-local artifacts
";

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "clean" => clean::run(args),
        _ => {
            eprint!("{}", USAGE);
            Ok(())
        }
    }
}
