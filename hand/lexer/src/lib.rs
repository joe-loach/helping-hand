mod tokens;

use syntax::SyntaxKind::{self, *};

pub fn lex(text: &str) -> LexedStr {
    LexedStr::new(text)
}

struct LexError {
    msg: String,
    token: u32,
}

pub struct LexedStr<'t> {
    text: &'t str,
    kind: Vec<SyntaxKind>,
    start: Vec<u32>,
    error: Vec<LexError>,
}

impl<'t> LexedStr<'t> {
    fn new(text: &'t str) -> LexedStr<'t> {
        let mut res = LexedStr {
            text,
            kind: Vec::new(),
            start: Vec::new(),
            error: Vec::new(),
        };

        let mut pos = 0;
        for token in tokens::tokenise(text) {
            let token_text = &text[pos..][..token.len];

            let kind = match token.kind {
                IDENT => SyntaxKind::from_keyword(token_text).unwrap_or(IDENT),
                kind => kind,
            };
            res.push(kind, pos);
            pos += token.len;

            if let Some(msg) = token.error {
                let token = res.len() as u32;
                res.error.push(LexError { msg, token });
            }
        }
        res.push(EOF, pos);
        res
    }

    pub fn as_str(&self) -> &str {
        self.text
    }

    pub fn len(&self) -> usize {
        self.kind.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self, i: usize) -> SyntaxKind {
        assert!(i < self.len());
        self.kind[i]
    }

    pub fn text(&self, i: usize) -> &str {
        assert!(i < self.len());
        let lo = self.start[i] as usize;
        let hi = self.start[i + 1] as usize;
        &self.text[lo..hi]
    }

    pub fn start(&self, i: usize) -> u32 {
        self.start[i]
    }

    pub fn error(&self, i: usize) -> Option<&str> {
        assert!(i < self.len());
        let err = self
            .error
            .binary_search_by_key(&(i as u32), |i| i.token)
            .ok()?;
        Some(self.error[err].msg.as_str())
    }

    fn push(&mut self, kind: SyntaxKind, offset: usize) {
        self.kind.push(kind);
        self.start.push(offset as u32);
    }
}
