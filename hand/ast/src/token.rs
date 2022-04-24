use syntax::SyntaxKind::*;
use syntax::SyntaxToken;

pub trait Token: Sized {
    fn token(&self) -> &SyntaxToken;
    fn cast(token: SyntaxToken) -> Option<Self>;
    fn text(&self) -> &str {
        self.token().text()
    }
}

tok! { Ident(IDENT) }
tok! { Num(NUMBER) }
tok! { Str(STRING) }
tok! { Char(CHAR) }
tok! { Opcode(OPCODE) }
tok! { Directive(DIRECTIVE) }
tok! { Condition(COND) }

tok! { Colon(COLON) }
tok! { Bang(BANG) }
tok! { Minus(MINUS) }
tok! { Plus(PLUS) }

tok! { True(TRUE) }
tok! { False(FALSE) }

impl Directive {
    pub fn syntax(&self) -> syntax::Directive {
        self.text().parse().unwrap()
    }
}

impl Num {
    pub fn value(&self) -> Result<u32, std::num::ParseIntError> {
        let number = self.text().trim_start_matches("0x");
        number.parse()
    }
}

impl Str {
    pub fn value(&self) -> &str {
        self.text().trim_matches('"')
    }
}

impl Char {
    pub fn value(&self) -> char {
        self.text().trim_matches('"').chars().next().unwrap()
    }
}

macro_rules! tok {
    ($name:ident ($kind:ident)) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(pub(crate) SyntaxToken);

        impl Token for $name {
            fn token(&self) -> &SyntaxToken {
                &self.0
            }

            fn cast(token: SyntaxToken) -> Option<Self> {
                if token.kind() == $kind {
                    Some(Self(token))
                } else {
                    None
                }
            }
        }
    };
}

pub(self) use tok;
