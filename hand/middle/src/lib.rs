//! # Middle Representation Layer

mod cursor;
mod ir;
mod lowering;
mod validation;

pub use cursor::Cursor;
pub use ir::*;
pub use lowering::{Address, Atom, Offset, Sign};

pub fn lower(root: ast::Root) -> IR {
    let labels = lowering::labels(&root);
    lowering::ir(root, labels)
}

pub fn validate(ir: &IR) -> bool {
    let mut i = 0;
    while let Some(stmt) = ir.stmt(i) {
        let _stmt = validation::shape(&mut Cursor::new(stmt));
        i += 1;
    }
    true
}
