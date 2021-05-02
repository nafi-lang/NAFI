use {crate::SyntaxKind, logos::Logos};

pub struct Lexer<'a> {
    logos: logos::Lexer<'a, SyntaxKind>,
    next_kind: Option<SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut logos = SyntaxKind::lexer(input);
        Self {
            next_kind: logos.next(),
            logos,
        }
    }

    pub fn peek(&self) -> Option<<Self as Iterator>::Item> {
        self.next_kind.map(|kind| (kind, self.logos.slice()))
    }

    pub fn source(&self) -> &'a str {
        self.logos.source()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let item = self.peek();
        self.next_kind = self.logos.next();
        item
    }
}
