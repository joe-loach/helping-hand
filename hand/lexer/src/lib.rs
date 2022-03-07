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
            let token_text = &text[pos..][..token.len].to_ascii_uppercase();

            let mut has_args = true;
            let mut split_id = None;
            let kind = match token.kind {
                IDENT => {
                    if let Some(kind) = SyntaxKind::from_register(token_text) {
                        kind
                    } else if let Some((op, cond)) = SyntaxKind::from_opcode(token_text) {
                        has_args = op.has_args();
                        split_id = cond;
                        OPCODE
                    } else {
                        IDENT
                    }
                }
                kind => kind,
            };
            res.push(kind, pos);

            if let Some((offset, kind)) = split_id {
                pos += offset;
                res.push(kind, pos);
                pos += token.len - offset;
            } else {
                pos += token.len;
            }

            if kind == OPCODE && has_args {
                res.push(HAS_ARGS, pos);
            }

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
