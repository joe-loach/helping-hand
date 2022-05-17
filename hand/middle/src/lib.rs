/*!
# Middle Representation Layer

The IR produced by the lowering functions is a flat vector of tuple pairs (kind, data).
As the kind is known, the data can always be "higher"ed back up again to its syntax representation.

The goal of the IR is to make it easy to eat through the data in the assembly whilst still preseving some type knowledge.
It should also reduce the space in memory as each pair is only a maximum of 64 bits wide.
*/

mod cursor;
mod higher;
mod ir;
mod lower;

pub use cursor::*;
pub use higher::*;
pub use ir::*;

#[derive(Debug, Clone, Copy)]
pub struct Atom {
    pub kind: AtomKind,
    data: u32,
}

impl Atom {
    pub fn new(kind: AtomKind, data: u32) -> Self {
        Self { kind, data }
    }

    pub fn raw(&self) -> u32 {
        self.data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomKind {
    Instruction,
    Directive,
    Condition,
    Shift,
    Register,
    Address,
    Sign,
    Label,
    Number,
    Char,
    Bool,
    Offset,
    RegisterList,
    Error,
}

// Assert the size is the same as a u64
const _: fn() = || {
    let _ = core::mem::transmute::<u64, Atom>;
};

pub fn lower(root: ast::Root) -> IR {
    let labels = lower::labels(&root);
    lower::ir(root, labels)
}
