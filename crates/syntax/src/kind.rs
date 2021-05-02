use {
    logos::Logos,
    num_derive::{FromPrimitive, ToPrimitive},
    num_traits::{FromPrimitive, ToPrimitive},
    serde::{Deserialize, Serialize},
};

// This implementation folows UAX31-R1 (Default Identifiers),
// the Unicode Consortium recommended definition for code identifiers.
// Specifically, we use a profile where
//   <Identifier> := <Start> <Continue>* (<Medial> <Continue>+)*
//   <Start>    := XID_Start,    where characters are in one of the Recommended Scripts (UAX31-T5)
//   <Continue> := XID_Continue, where characters are in one of the Recommended Scripts (UAX31-T5)
//   <Medial>   := empty
// The allowed scripts were last updated for Unicode 13.0.0, UAX31 revision 33 (2020-02-13).
// The contents of the sets are those as provided by the regex crate.

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Logos, Serialize, Deserialize, FromPrimitive, ToPrimitive)]
#[logos(subpattern allowed_scripts = r#"\p{Zyyy}\p{Zinh}\p{Arab}\p{Armn}\p{Beng}\p{Bopo}\p{Cyrl}\p{Deva}\p{Ethi}\p{Geor}\p{Grek}\p{Gujr}\p{Gujr}\p{Guru}\p{Hani}\p{Hang}\p{Hebr}\p{Hira}\p{Knda}\p{Kana}\p{Khmr}\p{Laoo}\p{Latn}\p{Mlym}\p{Mymr}\p{Orya}\p{Sinh}\p{Taml}\p{Telu}\p{Thaa}\p{Thai}\p{Tibt}"#)]
pub enum SyntaxKind {
    #[error]
    #[allow(clippy::upper_case_acronyms)]
    ERROR = u16::MAX,

    // terminals
    #[regex(r#"\p{Pattern_White_Space}+"#)]
    Whitespace = 0,
    #[regex(r#"[\p{XID_Start}&&(?&allowed_scripts)][\p{XID_Continue}&&(?&allowed_scripts)]*"#)]
    Identifier,
    #[regex(r#"[0-9]+"#)]
    Digits,
    #[regex(r#"\p{Pattern_Syntax}"#)]
    Syntax,

    // NB: `｢` is _not_ in the lexical grammar, so is a safe debug tool here

    // quasi-terminals (i.e. glued by parser, no introspection)
    #[cfg_attr(test, token("｢Op｣"))]
    Op,
    #[cfg_attr(test, token("｢Number｣"))]
    Number,

    // items
    #[cfg_attr(test, token("｢SourceFile｣"))]
    SourceFile,

    // expressions
    #[cfg_attr(test, token("｢Expr｣"))]
    Expr,
    #[cfg_attr(test, token("｢LiteralExpr｣"))]
    LiteralExpr,
    #[cfg_attr(test, token("｢InfixExpr｣"))]
    InfixExpr,
    #[cfg_attr(test, token("｢PrefixExpr｣"))]
    PrefixExpr,
    #[cfg_attr(test, token("｢SuffixExpr｣"))]
    SuffixExpr,
    #[cfg_attr(test, token("｢CircumfixExpr｣"))]
    CircumfixExpr,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::Whitespace)
    }
}

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
