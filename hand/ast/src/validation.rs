/*!

# AST Validation Layer

This doesn't validate arguments for specific operators, only individual form.

## Categories

- *Error* (E)
- *Warn* (W)

## Errors:

- repeated registers in list

## Warnings:

- mixed case opcodes
- labels with ':' in argument position

## Todo:

 */

use std::collections::HashMap;

use super::*;
use syntax::{SyntaxElement, SyntaxKind::*};

pub enum Level {
    Error,
    Warn,
}

pub struct Error {
    pub element: SyntaxElement,
    pub msg: String,
    pub level: Level,
}

pub(super) fn validate(root: &Root) -> Vec<Error> {
    use Level::*;

    let mut errors = Vec::new();

    for node in root.node().descendants() {
        match node.kind() {
            OP => {
                let code = Op(node).code();
                let mut chars = code.text().chars();
                let first = chars.next().unwrap(); // must have 1 char to be an ident
                let consistent = if first.is_ascii_lowercase() {
                    // rest should be lowercase
                    chars.all(|c| c.is_ascii_lowercase())
                } else {
                    // rest should be uppercase
                    chars.all(|c| c.is_ascii_uppercase())
                };
                if !consistent {
                    push(
                        &mut errors,
                        Warn,
                        "opcode casing should be consistent",
                        code.token().clone(),
                    );
                }
            }
            ARG_LIST => {
                let args = ArgList(node.clone());
                let labels = args.iter().filter_map(|arg| match arg.kind() {
                    ArgKind::Label(lbl) => Some(lbl),
                    _ => None,
                });
                for lbl in labels {
                    if let Some(colon) = lbl.colon() {
                        push(
                            &mut errors,
                            Warn,
                            "labels in argument position shouldn't have colons",
                            colon.token().clone(),
                        )
                    }
                }
            }
            REG_LIST => {
                let regs = RegList(node.clone());
                let counts = regs.iter().fold(HashMap::with_capacity(16), |mut c, reg| {
                    let reg = reg.syntax().to_numbered();
                    *c.entry(reg).or_insert(0) += 1;
                    c
                });
                let repeated = counts
                    .into_iter()
                    .filter_map(|(r, c)| if c > 1 { Some(r) } else { None });
                if repeated.count() != 0 {
                    push(
                        &mut errors,
                        Error,
                        "register list mustn't contain repeated registers",
                        regs.node().clone(),
                    )
                }
            }
            IMMEDIATE => {
                let imm = Immediate(node.clone());
                if let Err(e) = imm.value() {
                    push(
                        &mut errors,
                        Error,
                        format!("couldn't parse int, {}", e),
                        imm.node().clone(),
                    )
                }
            }
            _ => (),
        }
    }

    errors
}

fn push(
    errors: &mut Vec<Error>,
    level: Level,
    msg: impl Into<String>,
    element: impl Into<SyntaxElement>,
) {
    errors.push(Error {
        element: element.into(),
        msg: msg.into(),
        level,
    })
}
