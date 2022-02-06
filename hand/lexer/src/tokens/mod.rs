#[cfg(test)]
mod tests;

mod cursor;

use cursor::Cursor;
use syntax::SyntaxKind;

#[derive(Debug)]
pub(crate) struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
    pub error: Option<String>,
}

pub(crate) fn tokenise(mut text: &str) -> impl Iterator<Item = Token> + '_ {
    std::iter::from_fn(move || {
        if text.is_empty() {
            // no more text to lex
            return None;
        }
        // make the next token
        let tok = Cursor::new(text).next();
        // remove the lexed text
        text = &text[tok.len..];
        Some(tok)
    })
}

impl<'a> Cursor<'a> {
    fn kind(&mut self) -> SyntaxKind {
        use unicode_xid::UnicodeXID;
        use SyntaxKind::*;

        let first = self.eat().expect("text should never be empty");
        match first {
            // Whitespace
            c if c.is_whitespace() => {
                self.eat_while(char::is_whitespace);
                WHITESPACE
            }
            // Single line comment
            ';' => {
                self.eat_while(|c| c != '\n');
                COMMENT
            }
            '/' => {
                match self.peek() {
                    // Single line comment
                    Some('/') => {
                        self.eat(); // eat second '/'
                        self.eat_while(|c| c != '\n');
                        COMMENT
                    }
                    // Multi line comment
                    Some('*') => {
                        self.eat(); // eat star
                        let mut level = 1;
                        while let Some(c) = self.eat() {
                            match c {
                                // inc level
                                '/' if self.peek() == Some('*') => {
                                    self.eat();
                                    level += 1;
                                }
                                // dec level
                                '*' if self.peek() == Some('/') => {
                                    self.eat();
                                    level -= 1;
                                    if level == 0 {
                                        break;
                                    }
                                }
                                _ => (),
                            }
                        }
                        if level != 0 {
                            self.error("Multi-line comment is not terminated")
                        }
                        COMMENT
                    }
                    _ => SLASH,
                }
            }
            // Ident
            c if c.is_xid_start() => {
                self.eat_while(UnicodeXID::is_xid_continue);
                IDENT
            }
            // Literal
            c @ '0'..='9' => {
                // check for base prefix
                if c == '0' {
                    if let Some('x' | '&') = self.peek() {
                        self.eat(); // eat prefix
                        let digits =
                            self.eat_while(|c| matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z'));
                        if digits == 0 {
                            self.error("Missing digits after base prefix");
                        }
                    }
                }
                self.eat_while(|c| matches!(c, '0'..='9'));
                LITERAL
            }
            '!' => BANG,
            ',' => COMMA,
            ':' => COLON,
            '#' => HASH,
            '-' => MINUS,
            '+' => PLUS,
            '[' => OPEN_SQUARE,
            ']' => CLOSE_SQUARE,
            '{' => OPEN_CURLY,
            '}' => CLOSE_CURLY,
            // Any other characters are unknown to the lexer
            c => {
                self.error(format!("Unknown character '{c}'"));
                UNKNOWN
            }
        }
    }
}
