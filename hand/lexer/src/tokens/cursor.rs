use std::str::Chars;

use super::Token;

pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    pos: u32,
    error: Option<String>,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(text: &'a str) -> Self {
        assert!(!text.is_empty());
        Self {
            chars: text.chars(),
            pos: 0,
            error: None,
        }
    }

    pub(crate) fn next(mut self) -> Token {
        let kind = self.kind();
        let len = self.consumed() as usize;
        let error = self.error;
        Token { kind, len, error }
    }

    pub(crate) fn consumed(&self) -> u32 {
        self.pos
    }

    pub(crate) fn peek(&mut self) -> Option<char> {
        self.peek_nth(0)
    }

    pub(crate) fn peek_nth(&mut self, n: usize) -> Option<char> {
        self.chars.clone().nth(n)
    }

    pub(crate) fn eat(&mut self) -> Option<char> {
        self.chars.next().map(|c| {
            self.pos += 1;
            c
        })
    }

    pub(crate) fn eat_while<P>(&mut self, pred: P) -> u32
    where
        P: Fn(char) -> bool,
    {
        let mut ate = 0;
        while let Some(c) = self.peek() {
            if pred(c) {
                self.eat();
                ate += 1;
            } else {
                break;
            }
        }
        ate
    }

    pub(crate) fn error(&mut self, msg: impl Into<String>) {
        let msg = msg.into();
        self.error.replace(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_text() {
        // no text to go through :(
        // panics
        Cursor::new("");
    }

    #[test]
    fn eating() {
        let text = "frobnicator";
        let len = text.len() as u32;

        let mut c = Cursor::new(text);
        // peek and eat the first char
        c.peek().unwrap();
        c.eat().unwrap();
        // eat rest of the text
        let rest = c.eat_while(|_| true);
        assert_eq!(rest, len - 1);
        // nothing left!
        assert!(c.eat().is_none());
    }

    #[test]
    fn counting() {
        let text = "flippers";
        let len = text.len() as u32;

        let mut c = Cursor::new(text);
        assert_eq!(c.pos, 0);
        // eat everything
        c.eat_while(|_| true);
        assert_eq!(c.consumed(), len);
        // count shouldn't change if there's nothing to eat
        c.eat();
        assert_eq!(c.consumed(), len);
    }
}
