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
tok! { Literal(LITERAL) }
tok! { Opcode(OPCODE) }
tok! { Condition(COND) }

tok! { Colon(COLON) }
tok! { Bang(BANG) }
tok! { Minus(MINUS) }
tok! { Plus(PLUS) }

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
