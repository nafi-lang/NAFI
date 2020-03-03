use xtask::*;

fn main() -> Result<()> {
    let mut args = pico_args::Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "clean" => {
            args.finish()?;
            clean::run()
        }
        _ => {
            eprint!("\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <command>

COMMANDS:
    clean                              Clean target directory of workspace-local artifacts
");
            Ok(())
        }
    }
}
