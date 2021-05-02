pub use self::generated::*;
use {
    crate::SyntaxNode,
    std::{fmt, marker::PhantomData},
};

macro_rules! Node {
    ($Name:ident) => {
        pub enum $Name {}
        impl AstKind for $Name {
            fn name() -> &'static str {
                stringify!($Name)
            }
        }
    };
}

mod generated;

pub struct AstNode<Kind: AstKind> {
    pub(crate) syntax: SyntaxNode,
    pub(crate) tag: PhantomData<Kind>,
}

pub trait AstKind {
    fn name() -> &'static str;
}

impl<Kind: AstKind> fmt::Debug for AstNode<Kind> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", &self.syntax)
    }
}
