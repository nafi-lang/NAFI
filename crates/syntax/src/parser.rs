use {
    crate::{AstKind, AstNode, Lexer, NafiLanguage, SyntaxKind, SyntaxNode},
    num_traits::ToPrimitive,
    rowan::{GreenNodeBuilder, Language as _},
    std::marker::PhantomData,
};

#[cfg(test)]
macro_rules! test {
    ( $Node:ident: $($input:expr),* $(,)? ) => {
        ::paste::paste! {
            #[test]
            #[allow(non_snake_case)]
            fn [< test_ $Node >]() {
                $({
                    let input: &str = $input;
                    let output = format!("{:#?}", $Node::parse(input).syntax());
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

macro_rules! parse {
    ( $Node:ident: $parse:expr ) => {
        use crate::{ParseAst, Parser, SyntaxKind};
        impl ParseAst for $Node {
            fn parse_with(p: &mut Parser<'_>) {
                if p.peek() == Some(SyntaxKind::$Node) {
                    p.bump()
                } else {
                    $parse(p)
                }
            }
        }
    };
    ( $Node:ident!: $parse:expr ) => {
        use crate::{ParseAst, Parser, SyntaxKind};
        impl ParseAst for $Node {
            fn parse_with(p: &mut Parser<'_>) {
                $parse(p)
            }
        }
    };
}

mod expr;
mod source_file;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'static>,
    trivia: Vec<(SyntaxKind, &'a str)>,
}

pub trait ParseAst {
    fn parse_with(p: &mut Parser<'_>);
}

#[derive(Clone)]
pub struct ParseResult<Kind: AstKind> {
    green: rowan::GreenNode,
    tag: PhantomData<Kind>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            builder: GreenNodeBuilder::new(),
            trivia: Vec::new(),
        }
    }

    pub fn parse<Kind: AstKind>(mut self) -> ParseResult<Kind> {
        Kind::parse_with(&mut self);
        ParseResult {
            green: self.builder.finish(),
            tag: PhantomData,
        }
    }
}

impl<Kind: AstKind> ParseResult<Kind> {
    pub fn syntax(self) -> AstNode<Kind> {
        AstNode {
            syntax: SyntaxNode::new_root(self.green),
            tag: self.tag,
        }
    }
}

impl<'a> Parser<'a> {
    // #Todo: figure out a principled way of attaching trivia meaningfully
    fn bump_trivia(&mut self) {
        self.peek(); // eat ws into buffer
        for (kind, src) in self.trivia.drain(..) {
            self.builder.token(NafiLanguage::kind_to_raw(kind), src);
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
        self.builder.token(NafiLanguage::kind_to_raw(kind), src);
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

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.peek_full().map(|(kind, _src)| kind)
    }

    fn peek_src(&mut self) -> Option<&'a str> {
        self.peek_full().map(|(_kind, src)| src)
    }

    fn peek_full(&mut self) -> Option<(SyntaxKind, &'a str)> {
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
}
