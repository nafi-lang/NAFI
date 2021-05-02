# cargo xtask

This directory implements the [cargo xtask workflow](https://github.com/matklad/cargo-xtask/) for this repository.

cargo-xtask is way to add free-form automation to a Rust project, a-la `make`, `npm run` or bespoke bash scripts.

The two distinguishing features of xtask are:

* It doesn't require any other binaries besides `cargo` and `rustc`, it fully bootstraps from them
* Unlike bash, it can more easily be cross platform, as it doesn't use the shell.

## Our tasks

```man
cargo xtask
  Run custom build command

OPTIONS:
    -h, --help
      Print help information

SUBCOMANDS:

xtask clean
  Clean $CARGO_TARGET of workspace-local artifacts


xtask codegen
  Regenerate nuts & bolts of generated code (automatically done by `cargo test`)

  OPTIONS:
    -s, --syntax
      Regenerate syntax types (ast, etc)

    -a, --all
      Regenerate all generated code
```
