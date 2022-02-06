use crate::SyntaxKind;

crate::macros::str_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Shift {
        /// Logical shift left.
        LSL,
        /// Logical shift right.
        LSR,
        /// Arithmetic shift right.
        ASR,
        /// Rotate right.
        ROR,
        /// Rotate right one, with extend.
        RRX,
    }
}

impl Shift {
    pub fn syntax(&self) -> SyntaxKind {
        use SyntaxKind::*;
        match self {
            Shift::LSL => LSL,
            Shift::LSR => LSR,
            Shift::ASR => ASR,
            Shift::ROR => ROR,
            Shift::RRX => RRX,
        }
    }
}
