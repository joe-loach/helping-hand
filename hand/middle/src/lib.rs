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

pub fn validate(ir: &IR) -> Vec<String> {
    let mut errors = Vec::new();
    let mut i = 0;
    while let Some(stmt) = ir.stmt(i) {
        match validation::shape(&mut Cursor::new(stmt)) {
            validation::Shape::Unknown => {
                errors.push(String::from("Unkown shape for Instruction"))
            },
            shape => {
                let _ = shape;
            }
        }
        i += 1;
    }
    errors
}
