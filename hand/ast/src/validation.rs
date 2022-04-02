/*!

# AST Validation Layer

This doesn't validate arguments for specific operators, only individual form.

## Categories

- *Error* (E)
- *Warn* (W)

## Errors:

- repeated registers in a register list
- no registers in a register list
- invalid bangs attached to registers

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

use Level::*;

pub(super) fn validate(root: &Root) -> Vec<Error> {
    let mut errors = Vec::new();

    for node in root.node().descendants() {
        match node.kind() {
            INSTR => (),
            OP => op(&mut errors, Op(node)),
            REG_LIST => reg_list(&mut errors, RegList(node.clone())),
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
            REGISTER => reg(&mut errors, Register(node.clone())),
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

fn reg(errors: &mut Vec<Error>, register: Register) {
    // INVALID BANGS
    {
        use syntax::Opcode::*;

        let has_bang = register.bang().is_some();

        // find the instruction its in
        let instr = register
            .node()
            .ancestors()
            .find_map(Instruction::cast)
            .unwrap();
        let code = instr.op().code();

        // if the register can have a bang, skip the check
        if matches!(
            code.syntax(),
            LDM | LDMIA
                | LDMFD
                | LDMDA
                | LDMFA
                | LDMDB
                | LDMEA
                | LDMIB
                | LDMED
                | STM
                | STMIA
                | STMEA
                | STMDA
                | STMED
                | STMDB
                | STMFD
                | STMIB
                | STMFA
        ) {
            // find the position of the register in the arguments
            if let Some(0) = instr.args().unwrap().position(|arg| {
                if let ArgKind::Register(reg) = arg.kind() {
                    reg.node() == register.node()
                } else {
                    false
                }
            }) {
                return;
            }
        }

        if has_bang {
            push(
                errors,
                Error,
                "register shouldn't have a bang",
                register.node().clone(),
            );
        }
    }
}

fn op(errors: &mut Vec<Error>, op: Op) {
    // CASE CONSISTENCY
    {
        let code = op.code();
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
                errors,
                Warn,
                "opcode casing should be consistent",
                code.token().clone(),
            );
        }
    }
}

fn reg_list(errors: &mut Vec<Error>, list: RegList) {
    let counts = list.iter().fold(HashMap::with_capacity(16), |mut c, reg| {
        let reg = reg.syntax().to_numbered();
        *c.entry(reg).or_insert(0) += 1;
        c
    });

    // REPEATS
    {
        let repeated = counts
            .iter()
            .filter_map(|(r, &c)| if c > 1 { Some(r) } else { None });
        if repeated.count() != 0 {
            push(
                errors,
                Error,
                "register list mustn't contain repeated registers",
                list.node().clone(),
            )
        }
    }

    // COUNT
    {
        if counts.keys().len() == 0 {
            push(
                errors,
                Error,
                "register list must contain at least one register",
                list.node().clone(),
            )
        }
    }
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
