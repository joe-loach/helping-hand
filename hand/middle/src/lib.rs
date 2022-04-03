//! # Middle Representation Layer

mod cursor;
mod ir;
mod lower;
mod shape;
mod higher;

pub use cursor::Cursor;
pub use ir::*;
pub use lower::*;

pub use higher::*;

pub fn lower(root: ast::Root) -> IR {
    let labels = lower::labels(&root);
    lower::ir(root, &labels)
}

pub fn validate(ir: &IR) -> Vec<String> {
    let mut errors = Vec::new();
    for (i, stmt) in ir.iter().enumerate() {
        let cursor = &mut Cursor::new(&stmt);
        match shape::shape(cursor) {
            shape::Shape::Unknown => {
                errors.push(format!("Stmt {}: Unkown shape '{:?}'", i + 1, stmt.atoms()))
            }
            shape => {
                let _ = shape;
            }
        }
    }
    errors
}
