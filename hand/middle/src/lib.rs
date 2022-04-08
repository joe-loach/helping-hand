//! # Middle Representation Layer

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
    lower::ir(root, &labels)
}
