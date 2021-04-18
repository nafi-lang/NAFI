use {
    crate::{Lexer, NafiLanguage, SourceFile, TokenKind},
    num_derive::{FromPrimitive, ToPrimitive},
    num_traits::{FromPrimitive, ToPrimitive},
    rowan::{GreenNodeBuilder, Language as _},
    serde::{Deserialize, Serialize},
};

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize, FromPrimitive, ToPrimitive)]
pub enum SyntaxKind {
    #[allow(clippy::upper_case_acronyms)]
    ERROR = u16::MAX,

    // terminals
    Whitespace = 0,
    Identifier,
    LitDigits,
    Syntax,

    // nonterminals
    SourceFile,
    Expr,
}

impl From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> Self {
        kind.to_u16().and_then(SyntaxKind::from_u16).unwrap()
    }
}

pub(crate) struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'static>,
    trivia: Vec<(TokenKind, &'a str)>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
            trivia: Vec::new(),
        }
    }

    // #Todo: figure out a principled way of attaching trivia meaningfully
    fn bump_trivia(&mut self) {
        self.peek(); // eat ws into buffer
        for (kind, src) in self.trivia.drain(..) {
            self.builder
                .token(NafiLanguage::kind_to_raw(kind.into()), src);
        }
    }

    fn start_node_strict(&mut self, kind: SyntaxKind) {
        assert!(
            kind.to_u16() >= SyntaxKind::SourceFile.to_u16(),
            "called start_node with terminal kind"
        );
        self.builder.start_node(NafiLanguage::kind_to_raw(kind))
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.bump_trivia();
        self.start_node_strict(kind);
    }

    fn bump(&mut self) {
        self.bump_trivia();
        let (kind, src) = self.lexer.next().expect("called bump on exhausted parser");
        self.builder
            .token(NafiLanguage::kind_to_raw(kind.into()), src);
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }

    fn bump_node(&mut self, kind: SyntaxKind) {
        self.start_node(kind);
        self.bump();
        self.finish_node();
    }

    fn checkpoint(&mut self) -> rowan::Checkpoint {
        self.bump_trivia();
        self.builder.checkpoint()
    }

    fn start_node_at(&mut self, checkpoint: rowan::Checkpoint, kind: SyntaxKind) {
        assert!(
            kind.to_u16() >= SyntaxKind::SourceFile.to_u16(),
            "called start_node_at with terminal kind"
        );
        self.builder
            .start_node_at(checkpoint, NafiLanguage::kind_to_raw(kind))
    }

    fn wrap_node_from(&mut self, checkpoint: rowan::Checkpoint, kind: SyntaxKind) {
        self.start_node_at(checkpoint, kind);
        self.finish_node();
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.peek_full().map(|(kind, _src)| kind)
    }

    fn peek_src(&mut self) -> Option<&'a str> {
        self.peek_full().map(|(_kind, src)| src)
    }

    fn peek_full(&mut self) -> Option<(TokenKind, &'a str)> {
        while self
            .lexer
            .peek()
            .filter(|(kind, _src)| kind.is_trivia())
            .is_some()
        {
            self.trivia.push(self.lexer.next().unwrap());
        }
        self.lexer.peek()
    }

    pub(crate) fn parse(mut self) -> SourceFile {
        parse_source_file(&mut self);

        SourceFile {
            tree: self.builder.finish(),
        }
    }

    #[cfg(test)]
    pub(crate) fn debug_parse(input: &'a str, parse: fn(&mut Self)) -> String {
        let mut p = Self::new(input);
        parse(&mut p);
        format!("{:#?}", crate::SyntaxNode::new_root(p.builder.finish()))
    }
}

#[cfg(test)]
macro_rules! test {
    ( $grammar:ident: $($input:expr),* $(,)? ) => {
        ::paste::paste! {
            #[test]
            fn [< test_ $grammar >]() {
                $({
                    let input: &str = $input;
                    let output = Parser::debug_parse(input, self::$grammar);
                    let input = String::from("✎ ") + $input; // mitsuhiko/insta#177
                    insta::assert_snapshot!(insta::internals::AutoName, output, &input);
                })*
            }
        }
    };
}
#[cfg(not(test))]
macro_rules! test {
    ($($tt:tt)*) => {};
}

fn parse_source_file(p: &mut Parser<'_>) {
    p.start_node_strict(SyntaxKind::SourceFile);
    loop {
        match p.peek() {
            None => break,
            Some(TokenKind::LitDigits) | Some(TokenKind::Identifier) => parse_expr(p),
            Some(TokenKind::ERROR) => p.bump(),
            Some(_) => p.bump_node(SyntaxKind::ERROR),
        }
    }
    p.bump_trivia();
    assert!(matches!(p.peek(), None));
    p.finish_node();
}

test!(parse_expr: "1+2+3+4", "1+2*3+4", "1*2+3*4");
fn parse_expr(p: &mut Parser<'_>) {
    parse_expr_(p, f32::NEG_INFINITY)
}

fn parse_expr_(p: &mut Parser<'_>, current_binding_power: f32) {
    let checkpoint = p.checkpoint();

    p.bump_node(SyntaxKind::Expr);

    while let Some(TokenKind::Syntax) = p.peek() {
        let (left_power, right_power) = op_binding_power(p.peek_src().unwrap());
        if left_power < current_binding_power {
            break;
        }
        p.bump();
        parse_expr_(p, right_power);
        p.wrap_node_from(checkpoint, SyntaxKind::Expr);
    }
}

fn op_binding_power(op: &str) -> (f32, f32) {
    match op {
        "+" | "-" => (10.0, 11.0),
        "*" | "/" => (20.0, 21.0),
        _ => (f32::NEG_INFINITY, f32::INFINITY),
    }
}
