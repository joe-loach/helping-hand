/// The mode of the address
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Address {
    Offset,
    PreInc,
    PostInc,
}
/// The mode of offset
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Offset {
    Val,
    Reg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Atom {
    Instruction,
    Condition,
    Shift,
    Register,
    Label,
    Value,
    Address,
    Offset,
    Sign,
    RegisterList,
}

use std::collections::HashMap;

use crate::IR;
use ast::Token;

/// Outputs IR in the form:
///
/// LABEL (INSTRUCTION CONDITION ARGS?)?
pub(super) fn ir(root: ast::Root, labels: HashMap<String, u32>) -> IR {
    use Atom::*;

    let mut ir = IR::new();

    let mut pos = 0;
    for stmt in root.program().statements() {
        // LABEL
        ir.push(Label, pos);
        if let Some(instr) = stmt.instruction() {
            // INSTRUCTION
            ir.push(Instruction, instr.op().code().syntax() as u32);
            // CONDITION
            ir.push(
                Condition,
                instr
                    .op()
                    .condition()
                    .map(|cond| cond.syntax())
                    .unwrap_or_default() as u32,
            );
            // ARGS
            if let Some(args) = instr.args() {
                for arg in args {
                    match arg.kind() {
                        ast::ArgKind::Register(reg) => {
                            ir.push(Register, reg.syntax().value() as u32);
                        }
                        ast::ArgKind::Shift(shift) => {
                            use syntax::Opcode::*;
                            let value = match shift.op().code().syntax() {
                                LSL => 0b00,
                                LSR => 0b01,
                                ASR => 0b10,
                                RRX | ROR => 0b11,
                                _ => unreachable!(),
                            };
                            ir.push(Shift, value);
                            if let Some(data) = shift.data() {
                                match data {
                                    ast::ShiftData::Register(reg) => {
                                        ir.push(Register, reg.syntax().value() as u32);
                                    }
                                    ast::ShiftData::Immediate(imm) => {
                                        ir.push(
                                            Sign,
                                            match imm.sign().is_positive() {
                                                true => crate::Sign::Positive,
                                                false => crate::Sign::Negative,
                                            } as u32,
                                        );
                                        ir.push(Value, imm.value());
                                    }
                                }
                            }
                        }
                        ast::ArgKind::Label(lbl) => {
                            if let Some(&pos) = labels.get(lbl.name().ident().text()) {
                                ir.push(Label, pos);
                            } else {
                                ir.push(Label, u32::MAX);
                            }
                        }
                        ast::ArgKind::Immediate(imm) => {
                            ir.push(
                                Sign,
                                match imm.sign().is_positive() {
                                    true => (crate::Sign::Positive),
                                    false => (crate::Sign::Negative),
                                } as u32,
                            );
                            ir.push(Value, imm.value())
                        }
                        ast::ArgKind::Address(addr) => {
                            let offset = match addr.kind() {
                                ast::AddrKind::Offset(a) => {
                                    ir.push(Address, (crate::Address::Offset) as u32);
                                    a.offset()
                                }
                                ast::AddrKind::PreInc(a) => {
                                    ir.push(Address, (crate::Address::PreInc) as u32);
                                    Some(a.offset())
                                }
                                ast::AddrKind::PostInc(a) => {
                                    ir.push(Address, (crate::Address::PostInc) as u32);
                                    Some(a.offset())
                                }
                            };
                            ir.push(Register, addr.base().syntax().value() as u32);
                            if let Some(offset) = offset {
                                match offset.kind() {
                                    ast::OffsetKind::Immediate(o) => {
                                        let imm = o.immediate();
                                        ir.push(
                                            Sign,
                                            match imm.sign().is_positive() {
                                                true => (crate::Sign::Positive),
                                                false => (crate::Sign::Negative),
                                            } as u32,
                                        );
                                        ir.push(Value, imm.value())
                                    }
                                    ast::OffsetKind::Register(o) => {
                                        ir.push(
                                            Sign,
                                            match o.sign().is_positive() {
                                                true => crate::Sign::Positive,
                                                false => crate::Sign::Negative,
                                            } as u32,
                                        );
                                        ir.push(Register, o.base().syntax().value() as u32);
                                        if let Some(shift) = o.shift() {
                                            use syntax::Opcode::*;
                                            let value = match shift.op().code().syntax() {
                                                LSL => 0b00,
                                                LSR => 0b01,
                                                ASR => 0b10,
                                                RRX | ROR => 0b11,
                                                _ => unreachable!(),
                                            };
                                            ir.push(Shift, value);
                                            if let Some(data) = shift.data() {
                                                let (atom, data) = match data {
                                                    ast::ShiftData::Register(reg) => {
                                                        (Register, reg.syntax().value() as u32)
                                                    }
                                                    ast::ShiftData::Immediate(imm) => {
                                                        ir.push(
                                                            Sign,
                                                            match imm.sign().is_positive() {
                                                                true => crate::Sign::Positive,
                                                                false => crate::Sign::Negative,
                                                            }
                                                                as u32,
                                                        );
                                                        (Value, imm.value())
                                                    }
                                                };
                                                ir.push(atom, data);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ast::ArgKind::RegList(r_list) => {
                            let mut bits: u16 = 0;
                            for reg in r_list.iter() {
                                let value = reg.syntax().value() as u16;
                                bits |= 1 << value;
                            }
                            ir.push(RegisterList, bits as u32)
                        }
                    }
                }
            }
        }
        // move onto the next statement
        ir.finish();
        pos += 4;
    }
    ir
}

pub fn labels(root: &ast::Root) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let mut pos = 0;
    for stmt in root.program().statements() {
        let label = stmt.label();
        if let Some(label) = label {
            map.insert(label.name().ident().text().to_owned(), pos);
        }
        pos += 4;
    }
    map
}
