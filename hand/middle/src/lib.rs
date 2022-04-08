//! # Middle Representation Layer

mod cursor;
mod ir;
mod lower;
mod higher;

pub use ir::*;
pub use lower::*;

pub use higher::*;

pub fn lower(root: ast::Root) -> IR {
    let labels = lower::labels(&root);
    lower::ir(root, &labels)
}
