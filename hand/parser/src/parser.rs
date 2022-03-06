use lexer::LexedStr;
use syntax::SyntaxKind;

use crate::step::Step;

pub(crate) struct Parser<'s> {
    source: Source<'s>,
    pos: usize,
    steps: Vec<Step>,
}

impl<'s> Parser<'s> {
    /// Create a new parser from a stream of "trivia-less" tokens.
    pub(crate) fn new(source: Source<'s>) -> Self {
        Self {
            source,
            pos: 0,
            steps: Vec::new(),
        }
    }

    /// Consumes the parser and returns a list of steps.
    pub(crate) fn finish(self) -> Vec<Step> {
        self.steps
    }

    /// Start a new node and save a marker.
    pub(crate) fn start(&mut self) -> Marker {
        let pos = self.steps.len() as u32;
        self.steps.push(Step::tombstone());
        Marker::new(pos)
    }

    /// Returns the [`SyntaxKind`] at the current position in the token stream.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Returns the `n`th [`SyntaxKind`], offset from the position in the token stream.
    pub(crate) fn nth(&self, n: usize) -> SyntaxKind {
        self.source.kind(self.pos + n)
    }

    /// Returns the `n`th [`str`], offset from the position in the token stream.
    pub(crate) fn nth_str(&self, n: usize) -> Option<&'s str> {
        self.source.text(self.pos + n)
    }

    /// Returns true if at `kind`.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth(0) == kind
    }

    /// Try and consume a `kind` at the current position of the parse stream.
    /// If the parser isn't at `kind`, this returns false.
    pub(crate) fn eat(&mut self, kind: SyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }
        self.do_bump(kind);
        true
    }

    /// Consume a `kind`.
    /// Panics if not currently at `kind`.
    pub(crate) fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    /// Consume the next token, regardless of what it is.
    pub(crate) fn bump_any(&mut self) {
        self.eat(self.current());
    }

    /// Creates an error event with a message.
    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.steps.push(Step::Error { msg });
    }

    /// Expects to find a `kind` at current position.
    /// If not at `kind`, creates an error event.
    pub(crate) fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.error(format!("expected {:?}", kind));
        false
    }

    fn do_bump(&mut self, kind: SyntaxKind) {
        self.pos += 1;
        self.steps.push(Step::Token { kind })
    }
}

pub(crate) struct Marker {
    pos: u32,
}

impl Marker {
    fn new(pos: u32) -> Self {
        Self { pos }
    }

    pub(crate) fn finish(self, p: &mut Parser, kind: SyntaxKind) {
        let idx = self.pos as usize;
        match &mut p.steps[idx] {
            Step::Start { kind: slot } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        p.steps.push(Step::Finish);
    }
}

pub(crate) struct Source<'a> {
    kind: Vec<SyntaxKind>,
    text: Vec<&'a str>,
}

impl<'a> Source<'a> {
    pub(crate) fn new(lexed: &'a LexedStr) -> Self {
        let mut res = Source {
            kind: Vec::new(),
            text: Vec::new(),
        };
        for i in 0..lexed.len() {
            let kind = lexed.kind(i);
            if !kind.is_trivia() {
                let text = lexed.text(i);
                res.push(kind, text);
            }
        }
        res
    }

    pub(crate) fn kind(&self, idx: usize) -> SyntaxKind {
        self.kind.get(idx).copied().unwrap_or(SyntaxKind::EOF)
    }

    pub(crate) fn text(&self, idx: usize) -> Option<&'a str> {
        self.text.get(idx).copied()
    }

    fn push(&mut self, kind: SyntaxKind, text: &'a str) {
        self.kind.push(kind);
        self.text.push(text);
    }
}
