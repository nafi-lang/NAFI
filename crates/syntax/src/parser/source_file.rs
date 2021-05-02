use crate::{Expr, ParseAst, Parser, SourceFile, SyntaxKind};

parse!(SourceFile: parse_source_file);

fn parse_source_file(p: &mut Parser<'_>) {
    p.start_node_strict(SyntaxKind::SourceFile);
    loop {
        match p.peek() {
            None => break,
            Some(SyntaxKind::Digits) | Some(SyntaxKind::Identifier) => Expr::parse_with(p),
            Some(SyntaxKind::ERROR) => p.bump(),
            Some(_) => p.bump_node(SyntaxKind::ERROR),
        }
    }
    p.bump_trivia();
    assert_eq!(p.peek(), None);
    p.finish_node();
}
