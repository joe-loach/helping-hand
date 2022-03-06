#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SyntaxKind {
    TOMBSTONE,
    
    UNKNOWN,
    EOF,
    WHITESPACE,
    ERROR,
    COMMENT,
    IDENT,
    LITERAL,
    STRING,

    BANG,
    COMMA,
    COLON,
    HASH,
    MINUS,
    PLUS,
    OPEN_SQUARE,
    CLOSE_SQUARE,
    OPEN_CURLY,
    CLOSE_CURLY,
    SLASH,

    ROOT,
    PROGRAM,
    STATEMENT,
    INSTR,
    OP,
    OPCODE,
    COND,
    ARG_LIST,
    REG_LIST,
    ARG,
    ADDRESS,
    OFFSET,
    SHIFT,
    IMMEDIATE,
    REGISTER,
    LABEL,
    NAME,
    SIGN,

    TRUE,
    FALSE,

    RN,
    SP,
    LR,
    PC,

    #[doc(hidden)]
    LAST,
}

use SyntaxKind::*;

use crate::{Condition, OPCODES, REGISTERS};

impl SyntaxKind {
    pub fn is_trivia(&self) -> bool {
        matches!(self, WHITESPACE | COMMENT)
    }

    pub fn is_register(&self) -> bool {
        matches!(self, RN | SP | LR | PC)
    }

    pub fn from_register(s: &str) -> Option<SyntaxKind> {
        let kind = match s {
            "SP" => SP,
            "LR" => LR,
            "PC" => PC,
            _ if REGISTERS.contains(&s) => RN,
            _ => return None,
        };
        Some(kind)
    }

    pub fn from_opcode(s: &str) -> Option<(SyntaxKind, Option<(usize, SyntaxKind)>)> {
        for &(_, code) in OPCODES {
            if let Some(rest) = s.strip_prefix(code) {
                if rest.is_empty() {
                    return Some((OPCODE, None));
                } else if rest.parse::<Condition>().is_ok() {
                    return Some((OPCODE, Some((code.len(), COND))));
                }
            }
        }
        None
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}
