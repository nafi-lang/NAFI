use {
    logos::Logos,
    num_derive::{FromPrimitive, ToPrimitive},
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
pub enum TokenKind {
    #[error]
    #[allow(clippy::upper_case_acronyms)]
    ERROR = u16::MAX,

    // leaf token nodes
    #[regex(r#"\p{Pattern_White_Space}+"#)]
    Whitespace = 0,
    #[regex(r#"[\p{XID_Start}&&(?&allowed_scripts)][\p{XID_Continue}&&(?&allowed_scripts)]*"#)]
    Identifier,
    #[regex(r#"[0-9]+"#)]
    LitDigits,
    #[regex(r#"\p{Pattern_Syntax}"#)]
    Syntax,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, TokenKind::Whitespace)
    }
}

pub struct Lexer<'a> {
    logos: logos::Lexer<'a, TokenKind>,
    next_kind: Option<TokenKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut logos = TokenKind::lexer(input);
        Self {
            next_kind: logos.next(),
            logos,
        }
    }

    pub fn peek(&self) -> Option<<Self as Iterator>::Item> {
        self.next_kind.map(|kind| (kind, self.logos.slice()))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (TokenKind, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peek();
        self.next_kind = self.logos.next();
        item
    }
}
