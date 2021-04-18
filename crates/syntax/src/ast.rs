use {
    crate::{ParseAst, SyntaxNode},
    std::{fmt, marker::PhantomData},
};

pub struct AstNode<Kind: AstKind> {
    pub(crate) syntax: SyntaxNode,
    pub(crate) tag: PhantomData<Kind>,
}

pub trait AstKind: ParseAst {
    fn name() -> &'static str;
}

impl<Kind: AstKind> fmt::Debug for AstNode<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", &self.syntax)
    }
}

macro_rules! Node {
    ($Name:ident) => {
        pub enum $Name {}
        impl AstKind for $Name {
            fn name() -> &'static str {
                stringify!($Name)
            }
        }
        impl $Name {
            pub fn parse(input: &str) -> crate::ParseResult<Self> {
                crate::Parser::new(input).parse()
            }
        }
    };
}

// #Todo: generate these with all the fun accessors and stuff
pub mod node {
    use super::*;

    Node!(SourceFile);
    Node!(Expr);
}
