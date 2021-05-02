use crate::{Number, Parser, SyntaxKind};

parse!(Number: parse_number);

fn parse_number(p: &mut Parser) {
    match p.peek() {
        Some(SyntaxKind::Digits) => (),
        kind => panic!("unexpected kind {:?} at begining of expr", kind),
    }

    p.bump_as(SyntaxKind::Number);

    if p.peek_src() == Some(".") {
        todo!("support floating point");
    }
}
