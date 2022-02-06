use lexer::LexedStr;
use syntax::SyntaxKind;

#[derive(Debug)]
pub enum Step {
    Start { kind: SyntaxKind },
    Token { kind: SyntaxKind },
    Finish,
    Error { msg: String },
}

impl Step {
    pub fn tombstone() -> Self {
        Self::Start {
            kind: SyntaxKind::TOMBSTONE,
        }
    }
}

use core::mem;

pub enum StrStep<'a> {
    Start { kind: SyntaxKind },
    Token { kind: SyntaxKind, text: &'a str },
    Finish,
    Error { msg: String, pos: u32 },
}

pub fn attach_trivia(text: &LexedStr, steps: Vec<Step>, sink: &mut dyn FnMut(StrStep)) {
    let mut builder = Builder {
        lexed: text,
        pos: 0,
        state: State::PendingStart,
        sink,
    };

    for step in steps {
        match step {
            Step::Start { kind } => builder.start(kind),
            Step::Token { kind } => builder.token(kind),
            Step::Finish => builder.finish(),
            Step::Error { msg } => {
                let pos = text.start(builder.pos);
                (builder.sink)(StrStep::Error { msg, pos })
            }
        }
    }

    match mem::replace(&mut builder.state, State::Normal) {
        State::PendingFinish => {
            builder.eat_trivias();
            (builder.sink)(StrStep::Finish);
        }
        State::PendingStart | State::Normal => unreachable!(),
    }
}

struct Builder<'a, 'b> {
    lexed: &'a LexedStr<'a>,
    pos: usize,
    state: State,
    sink: &'b mut dyn FnMut(StrStep<'_>),
}

enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

impl Builder<'_, '_> {
    fn token(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => (self.sink)(StrStep::Finish),
            State::Normal => (),
        }
        self.eat_trivias();
        self.do_token(kind);
    }

    fn start(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                (self.sink)(StrStep::Start { kind });
                return;
            }
            State::PendingFinish => (self.sink)(StrStep::Finish),
            State::Normal => (),
        }
        self.eat_trivias();
        (self.sink)(StrStep::Start { kind });
        self.eat_trivias();
    }

    fn finish(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => (self.sink)(StrStep::Finish),
            State::Normal => (),
        }
    }

    fn eat_trivias(&mut self) {
        while self.pos < self.lexed.len() {
            let kind = self.lexed.kind(self.pos);
            if !kind.is_trivia() {
                break;
            }
            self.do_token(kind);
        }
    }

    fn do_token(&mut self, kind: SyntaxKind) {
        let text = &self.lexed.text(self.pos);
        self.pos += 1;
        (self.sink)(StrStep::Token { kind, text });
    }
}
