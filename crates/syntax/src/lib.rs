use {
    num_traits::{FromPrimitive, ToPrimitive},
    std::fmt,
};

mod lexer;
pub use crate::lexer::*;

mod parser;
pub use crate::parser::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NafiLanguage;
impl rowan::Language for NafiLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from_u16(raw.0).unwrap_or(SyntaxKind::ERROR)
    }

    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind.to_u16().unwrap())
    }
}

pub type SyntaxNode = rowan::SyntaxNode<NafiLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<NafiLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<NafiLanguage>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<NafiLanguage>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<NafiLanguage>;

#[derive(Clone)]
pub struct SourceFile {
    tree: rowan::GreenNode,
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceFile")
            .field("green_tree", &SyntaxNode::new_root(self.tree.clone()))
            .finish()
    }
}

impl SourceFile {
    pub fn parse(input: &str) -> SourceFile {
        Parser::new(input).parse()
    }

    pub fn syntax_tree(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.tree.clone())
    }
}
