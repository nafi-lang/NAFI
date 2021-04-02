use std::writeln;

use {
    std::io::{self, prelude::*},
    syntax::SourceFile,
};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        write!(stdout, "✎ ")?;
        stdout.flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        let source = input.replace("\r\n", "\n");
        if let Some(source) = source.strip_suffix('\n') {
            let parse = SourceFile::parse(source);
            writeln!(stdout, "{:#?}", parse.syntax_tree())?;
        } else {
            break;
        }
    }

    Ok(())
}
