mod addr;
mod arg;
mod reg;
mod shift;

use syntax::SyntaxKind::*;
use syntax::SyntaxNode;

use crate::support::*;
use crate::token::*;

pub use addr::*;
pub use arg::*;
pub use reg::*;
pub use shift::*;

pub trait Node: Sized {
    fn node(&self) -> &SyntaxNode;
    fn cast(node: SyntaxNode) -> Option<Self>;
}

node! { Root(ROOT) }
node! { Program(PROGRAM) }
node! { Statement(STATEMENT) }
node! { Instruction(INSTR) }
node! { Meta(META) }
node! { Op(OP) }
node! { Immediate(IMMEDIATE) }
node! { Literal(LITERAL) }
node! { Label(LABEL) }
node! { Name(NAME) }
node! { Sign(SIGN) }

impl Program {
    pub fn statements(&self) -> impl Iterator<Item = Statement> {
        children(self.node())
    }
}

pub enum StmtBody {
    Instruction(Instruction),
    Meta(Meta),
}

impl Statement {
    pub fn label(&self) -> Option<Label> {
        child(self.node())
    }

    pub fn body(&self) -> Option<StmtBody> {
        self.instruction()
            .map(StmtBody::Instruction)
            .or_else(|| self.meta().map(StmtBody::Meta))
    }

    pub fn instruction(&self) -> Option<Instruction> {
        child(self.node())
    }

    pub fn meta(&self) -> Option<Meta> {
        child(self.node())
    }
}

impl Meta {
    pub fn directive(&self) -> Directive {
        token(self.node()).unwrap()
    }

    pub fn arg_list(&self) -> Option<ArgList> {
        child(self.node())
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Arg>> {
        self.arg_list().map(|list| list.iter())
    }
}

impl Instruction {
    pub fn op(&self) -> Op {
        child(self.node()).unwrap()
    }

    pub fn arg_list(&self) -> Option<ArgList> {
        child(self.node())
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Arg>> {
        self.arg_list().map(|list| list.iter())
    }
}

impl Op {
    pub fn code(&self) -> Opcode {
        token(self.node()).unwrap()
    }

    pub fn condition(&self) -> Option<Condition> {
        token(self.node())
    }
}

impl Opcode {
    pub fn syntax(&self) -> syntax::Opcode {
        self.text().parse().unwrap()
    }
}

impl Condition {
    pub fn syntax(&self) -> syntax::Condition {
        self.text().parse().unwrap()
    }
}

impl Label {
    pub fn name(&self) -> Name {
        child(self.node()).unwrap()
    }

    pub fn colon(&self) -> Option<Colon> {
        token(self.node())
    }
}

impl Immediate {
    pub fn sign(&self) -> Sign {
        child(self.node()).unwrap()
    }

    pub fn literal(&self) -> Literal {
        child(self.node()).unwrap()
    }
}

pub enum LiteralKind {
    Number(Num),
    String(Str),
    Char(Char),
    Bool(bool),
}

impl Literal {
    pub fn kind(&self) -> LiteralKind {
        self.number()
            .map(LiteralKind::Number)
            .or_else(|| self.string().map(LiteralKind::String))
            .or_else(|| self.char().map(LiteralKind::Char))
            .or_else(|| self.bool().map(LiteralKind::Bool))
            .unwrap()
    }

    pub fn number(&self) -> Option<Num> {
        token(self.node())
    }

    pub fn string(&self) -> Option<Str> {
        token(self.node())
    }

    pub fn char(&self) -> Option<Char> {
        token(self.node())
    }

    pub fn bool(&self) -> Option<bool> {
        if let Some(True(_)) = token(self.node()) {
            Some(true)
        } else if let Some(False(_)) = token(self.node()) {
            Some(false)
        } else {
            None
        }
    }
}

impl Name {
    pub fn ident(&self) -> Ident {
        self.node()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(Token::cast)
            .unwrap()
    }
}

impl Sign {
    pub fn plus(&self) -> Option<Plus> {
        token(self.node())
    }

    pub fn minus(&self) -> Option<Minus> {
        token(self.node())
    }

    pub fn syntax(&self) -> syntax::Sign {
        match self.is_positive() {
            true => syntax::Sign::Positive,
            false => syntax::Sign::Negative,
        }
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_negative(&self) -> bool {
        self.minus().is_some()
    }
}

macro_rules! node {
    ($name:ident ($kind:ident)) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(pub(crate) SyntaxNode);

        impl Node for $name {
            fn node(&self) -> &SyntaxNode {
                &self.0
            }

            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

pub(self) use node;
