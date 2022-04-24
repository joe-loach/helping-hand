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
- dual registers: {consecutive, even-numbered, not R14}

## Warnings:

- mixed case opcodes
- labels with ':' in argument position
- ident arguments that 'look' like registers but aren't

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
            ARG => arg(&mut errors, Arg(node.clone())),
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
            LITERAL => {
                let lit = Literal(node.clone());
                if let LiteralKind::Number(n) = lit.kind() {
                    if let Err(e) = n.value() {
                        push(
                            &mut errors,
                            Error,
                            format!("couldn't parse int, {}", e),
                            lit.node().clone(),
                        )
                    }
                }
            }
            _ => (),
        }
    }

    errors
}

fn arg(errors: &mut Vec<Error>, arg: Arg) {
    // REGISTER LOOK-A-LIKE
    {
        use std::num::IntErrorKind::*;
        if let ArgKind::Label(lbl) = arg.kind() {
            let id = lbl.name().ident();
            if let Some(rest) = id.text().strip_prefix(['R', 'r']) {
                let is_num = match rest.parse::<u32>() {
                    Ok(_) => true,
                    // could still be a number, just too large / small
                    Err(e) => matches!(e.kind(), PosOverflow | NegOverflow),
                };
                if is_num {
                    push(
                        errors,
                        Warn,
                        "ident looks like a register but is being treated as a label",
                        lbl.name().node().clone(),
                    )
                }
            }
        }
    }
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
    // DUAL REGISTERS
    {
        use syntax::Opcode::*;

        let code = op.code();

        if matches!(code.syntax(), LDRD | STRD) {
            // find the instruction its in
            let instr = op.node().ancestors().find_map(Instruction::cast).unwrap();

            // get the first two registers from the arguments
            if let Some(args) = instr.args() {
                let regs = args
                    .take(2)
                    .filter_map(|a| {
                        if let ArgKind::Register(r) = a.kind() {
                            Some(r)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let rt = regs.get(0);
                let rt2 = regs.get(1);
                if let (Some(rt), Some(rt2)) = (rt, rt2) {
                    let rtv = rt.syntax().value();
                    let rt2v = rt2.syntax().value();
                    if rtv % 2 != 0 {
                        push(
                            errors,
                            Error,
                            "first dual register must be even-numbered",
                            rt.node().clone(),
                        )
                    } else if rtv == 14 {
                        push(
                            errors,
                            Error,
                            "first dual register mustn't be R14 or LR",
                            rt.node().clone(),
                        )
                    } else if rtv + 1 != rt2v {
                        push(
                            errors,
                            Error,
                            format!("dual registers must be consecutive, should be R{}", rtv + 1),
                            rt2.node().clone(),
                        )
                    }
                }
            }
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
