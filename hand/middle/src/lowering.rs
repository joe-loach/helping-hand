pub mod consts {
    pub mod address {
        pub const OFFSET: u32 = 0;
        pub const PREINC: u32 = 1;
        pub const POSTINC: u32 = 2;
    }

    pub mod offset {
        pub const VALUE: u32 = 0;
        pub const REGISTER: u32 = 1;
    }

    pub mod sign {
        pub const POSITIVE: u32 = 1;
        pub const NEGATIVE: u32 = 0;
    }
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
    Error,
}

use std::collections::HashMap;

use crate::{IR, consts::*};
use ast::Token;

/// Outputs IR in the form:
///
/// LABEL (INSTRUCTION CONDITION ARGS?)?
pub(super) fn ir(root: ast::Root, labels: &HashMap<String, u32>) -> IR {
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
                            register(&mut ir, reg);
                        }
                        ast::ArgKind::Shift(sft) => {
                            shift(&mut ir, sft);
                        }
                        ast::ArgKind::Label(lbl) => {
                            if let Some(&pos) = labels.get(lbl.name().ident().text()) {
                                ir.push(Label, pos);
                            } else {
                                ir.error("Label is not defined");
                            }
                        }
                        ast::ArgKind::Immediate(imm) => {
                            immediate(&mut ir, imm);
                        }
                        ast::ArgKind::Address(addr) => {
                            let offset = match addr.kind() {
                                ast::AddrKind::Offset(a) => {
                                    ir.push(Address, address::OFFSET);
                                    a.offset()
                                }
                                ast::AddrKind::PreInc(a) => {
                                    ir.push(Address, address::PREINC);
                                    Some(a.offset())
                                }
                                ast::AddrKind::PostInc(a) => {
                                    ir.push(Address, address::POSTINC);
                                    Some(a.offset())
                                }
                            };
                            register(&mut ir, addr.base());
                            if let Some(offset) = offset {
                                match offset.kind() {
                                    ast::OffsetKind::Immediate(o) => {
                                        ir.push(Offset, offset::VALUE);
                                        immediate(&mut ir, o.immediate());
                                    }
                                    ast::OffsetKind::Register(o) => {
                                        ir.push(Offset, offset::REGISTER);
                                        sign(&mut ir, o.sign());
                                        register(&mut ir, o.base());
                                        if let Some(sft) = o.shift() {
                                            shift(&mut ir, sft);
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
    return ir;

    fn sign(ir: &mut IR, sign: ast::Sign) {
        ir.push(
            Sign,
            match sign.is_positive() {
                true => sign::POSITIVE,
                false => sign::NEGATIVE,
            },
        )
    }

    fn register(ir: &mut IR, reg: ast::Register) {
        ir.push(Register, reg.syntax().value() as u32);
    }

    fn immediate(ir: &mut IR, imm: ast::Immediate) {
        sign(ir, imm.sign());
        match imm.value() {
            Ok(value) => {
                ir.push(Value, value);
            }
            Err(e) => {
                ir.error(format!("Immmediate value couldn't be parsed, {e}"));
            }
        }
    }

    fn shift(ir: &mut IR, shift: ast::Shift) {
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
                    register(ir, reg);
                }
                ast::ShiftData::Immediate(imm) => {
                    immediate(ir, imm);
                }
            }
        }
    }
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
