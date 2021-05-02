use anyhow::Context;

use {
    crate::*,
    quote::{format_ident, quote},
    std::path::Path,
    ungrammar::Grammar,
    xshell::{read_file, write_file},
};

static NODE_GENERATED: Lazy<&Path> = Lazy::new(|| Path::new("crates/syntax/src/ast/generated.rs"));
static SYNTAX_GRAMMAR: Lazy<&Path> = Lazy::new(|| Path::new("crates/syntax/src/nafi.ungram"));

pub fn codegen() -> Result<bool> {
    let grammar = {
        let s = read_file(&*SYNTAX_GRAMMAR)?;
        s.parse::<Grammar>()
            .context("while parsing nafi ungrammar")?
    };

    let generated = {
        let mut generated = quote! { use super::*; };

        for node in grammar.iter().map(|node| &grammar[node]) {
            let name = format_ident!("{}", node.name);
            // #Todo: generate all the fun accessors and stuff
            generated.extend(quote! { Node!(#name); });
        }

        rustfmt(&generated.to_string())? + "\n"
    };

    let existing = read_file(&*NODE_GENERATED).unwrap_or_default();

    if generated == existing {
        Ok(false)
    } else {
        write_file(&*NODE_GENERATED, generated)?;
        Ok(true)
    }
}
