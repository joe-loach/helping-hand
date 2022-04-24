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
pub use lower::*;

pub fn lower(root: ast::Root) -> IR {
    let labels = lower::labels(&root);
    lower::ir(root, labels)
}
