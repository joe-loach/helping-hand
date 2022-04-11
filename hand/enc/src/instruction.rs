mod helper;
mod inst;
mod ir;

use helper::*;
use inst::inst;
use ir::ir;

use std::cmp::Ordering;

use intbits::Bits;
use middle::{consts::*, higher};
use middle::{Atom::*, Cursor};

pub(crate) fn encode(args: &mut Cursor, op: u32) -> Option<u32> {
    let op = higher::<syntax::Opcode>(op);
    let cond = args.bump(Condition);

    use syntax::Opcode::*;

    let encoded = match op {
        ADC | ADCS => {
            let s = op == ADCS;
            variant(args, |args| {
                let (rd, rn) = ir!("{R} R")(args)?;
                ir!("+")(args)?;
                // TODO: do imm12 parsing fn(u32) -> Option<u32>
                let imm = args.eat(Value)?;
                inst!([cond:4] 0 0 1 0 | 1 0 1 | s | [rn:4] [rd: 4] [imm:12])
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rn, rm) = ir!("{R} R R")(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 0 0 0 | 1 0 1 | s | [rn:4] [rd:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rn, rm) = ir!("{R} R R")(args)?;
                    let shift = args.eat(Shift)?;
                    let rs = args.eat(Register)?;
                    inst!([cond:4] 0 0 0 0 | 1 0 1 | s | [rn:4] [rd:4] [rs:4] 0 [shift:2] 1 [rm:4])
                })
            })
        }
        ADR => variant(args, |args| {
            let rd = ir!("R")(args)?;
            let imm = args.eat(Label)?;
            // TODO: there are two different encodings for this,
            // based on if the offset is positive or negative
            // TODO: should be offset from Align(PC, 4)
            inst!([cond:4] 0 0 1 0 | 1 0 0 | 0 | 1 1 1 1 [rd:4] [imm:12])
        }),
        B => variant(args, |args| {
            let imm = args.eat(Label)?;
            inst!([cond:4] 1 0 1 | 0 [imm:24])
        }),
        BFC => variant(args, |args| {
            let rd = ir!("R")(args)?;
            ir!("+")(args)?;
            let lsb = args.eat(Value)?;
            if !(0..=31).contains(&lsb) {
                return None;
            }
            ir!("+")(args)?;
            let width = args.eat(Value)?;
            if !(1..=(32 - lsb)).contains(&width) {
                return None;
            }
            let msb = lsb + width - 1;
            inst!([cond:4] 0 1 1 1 1 1 0 [msb:5] [rd:4] [lsb:5] 0 0 1 | 1 1 1 1)
        }),
        BFI => variant(args, |args| {
            let (rd, rn) = ir!("R R")(args)?;
            ir!("+")(args)?;
            let lsb = args.eat(Value)?;
            if !(0..=31).contains(&lsb) {
                return None;
            }
            ir!("+")(args)?;
            let width = args.eat(Value)?;
            if !(1..=(32 - lsb)).contains(&width) {
                return None;
            }
            let msb = lsb + width - 1;
            inst!([cond:4] 0 1 1 1 1 1 0 [msb:5] [rd:4] [lsb:5] 0 0 1 [rn:4])
        }),
        // BIC(S)
        BL => variant(args, |args| {
            let imm = args.eat(Label)?;
            inst!([cond:4] 1 0 1 | 1 | [imm:24])
        }),
        BX => variant(args, |args| {
            let rm = ir!("R")(args)?;
            inst!([cond:4] 0 0 0 1 0 0 1 0 | 1 1 1 1 | 1 1 1 1 | 1 1 1 1 | 0 0 0 1 | [rm:4])
        }),
        CLZ => variant(args, |args| {
            let (rd, rm) = ir!("R R")(args)?;
            inst!([cond:4] 0 0 0 1 0 1 1 0 | 1 1 1 1 | [rd:4] | 1 1 1 1 | 0 0 0 1 | [rm:4])
        }),
        CMN | CMP => {
            let s = op == CMN;
            variant(args, |args| {
                let rn = ir!("R")(args)?;
                let sign = args.eat(Sign)?;
                let imm = args.eat(Value)?;
                let s = if sign == sign::NEGATIVE { !s } else { s };
                inst!([cond:4] 0 0 1 1 0 | 1 s | 1 | [rn:4] | 0 0 0 0 | [imm:12])
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rn, rm) = ir!("R R")(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 0 0 1 0 | 1 s | 1 | [rn:4] | 0 0 0 0 | [imm:5] [shift:2] 0 [rm:4])
                })
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rn, rm) = ir!("R R")(args)?;
                    let shift = args.eat(Shift)?;
                    let rs = ir!("R")(args)?;
                    inst!([cond:4] 0 0 0 1 0 | 1 s | 1 | [rn:4] | 0 0 0 0 | [rs:4] 0 [shift:2] 1 [rm:4])
                })
            })
        }
        // EOR(S)
        LDA | LDAB | LDAH => variant(args, |args| {
            let rt = ir!("R")(args)?;
            if args.eat(Address)? != address::OFFSET {
                return None;
            }
            let rn = ir!("R")(args)?;
            let width = match op {
                LDA => 0b00,
                LDAB => 0b10,
                LDAH => 0b11,
                _ => unreachable!(),
            };
            inst!([cond:4] 0 0 0 1 1 [width:2] 1 [rn:4] [rt:4] 1 1 0 0 | 1 0 0 1 | 1 1 1 1)
        }),
        LDM | LDMIA | LDMFD => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 1 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMDA | LDMFA => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 0 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMDB | LDMEA => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 0 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMIB | LDMED => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 1 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDR | LDRB => {
            let b = op == LDRB;
            variant(args, |args| {
                let rt = ir!("R")(args)?;
                let (p, w, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                inst!([cond: 4] 0 1 0 | p | u | b | w | 1 | [rn:4] [rt:4] [imm:12])
            }).or_else(|| {
                variant(args, |args| {
                    let rt = ir!("R")(args)?;
                    let imm = args.eat(Label)? * 4;
                    let u = 1; // TODO: once we calculate offset properly, we'll know the sign
                    inst!([cond:4] 0 1 0 | 1 | u | b | 0 | 1 | 1 1 1 1 [rt:4] [imm:12])
                })
            }).or_else(|| {
                variant(args, |args| {
                    let rt = ir!("R")(args)?;
                    let (p, w, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 1 1 | p | u | b | w | 1 | [rn:4] [rt:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
        }
        LDRD => variant(args, |args| {
            let (rt, rt2) = ir!("R R")(args)?;
            if (rt + 1) != rt2 {
                return None;
            }
            let (p, w, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = imm.bits(4..8);
            let imml = imm.bits(0..4);
            inst!([cond:4] 0 0 0 | p | u | 1 | w | 0 [rn:4] [rt:4] [immh:4] 1 | 1 0 | 1 [imml:4])
        }).or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt + 1) != rt2 {
                    return None;
                }
                let imm = args.eat(Label)? * 4;
                let immh = imm.bits(4..8);
                let imml = imm.bits(0..4);
                let u = 1; // TODO: once we calculate offset properly, we'll know the sign
                inst!([cond:4] 0 0 0 | 1 | u | 1 | 0 | 0 | 1 1 1 1 | [rt:4] [immh:4] 1 | 1 0 | 1 [imml:4])
            })
        }).or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt + 1) != rt2 {
                    return None;
                }
                let (p, w, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | p | u | 0 | w | 0 | [rn:4] [rt:4] 0 0 0 0 | 1 | 1 0 | 1 [rm:4])
            })
        }),
        LDRH | LDRSB | LDRSH => {
            let h = matches!(op, LDRH | LDRSH);
            let s = matches!(op, LDRSB | LDRSH);
            variant(args, |args| {
                let rt = ir!("R")(args)?;
                let (p, w, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                let immh = imm.bits(4..8);
                let imml = imm.bits(0..4);
                inst!([cond:4] 0 0 0 | p | u | 1 | w | 1 | [rn:4] [rt:4] [immh:4] 1 | s h | 1 | [imml:4])
            }).or_else(|| {
                variant(args, |args| {
                    let rt = ir!("R")(args)?;
                    let imm = args.eat(Label)? * 4;
                    let immh = imm.bits(4..8);
                    let imml = imm.bits(0..4);
                    let u = 1; // TODO: once we calculate offset properly, we'll know the sign
                    inst!([cond:4] 0 0 0 | 1 | u | 1 | 0 | 1 | 1 1 1 1 [rt:4] [immh:4] 1 | s h | 1 [imml:4])
                })
            }).or_else(|| {
                variant(args, |args| {
                    let rt = ir!("R")(args)?;
                    let (p, w, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    inst!([cond:4] 0 0 0 | p | u | 0 | w | 1 | [rn:4] [rt:4] 0 0 0 0 | 1 | s h | 1 | [rm:4])
                })
            })
        },
        LSL | LSLS => {
            let s = op == LSLS;
            variant(args, |args| {
                let (rd, rm) = ir!("{R} R")(args)?;
                ir!("+")(args)?;
                let imm = args.eat(Value)?;
                if !(0..=31).contains(&imm) {
                    return None;
                }
                inst!([cond:4] 0 0 0 1 1 | 0 1 | s | 0 0 0 0 [rd:4] [imm:5] 0 0 | 0 | [rm:4])
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rm, rs) = ir!("{R} R R")(args)?;
                    inst!([cond:4] 0 0 0 1 1 | 0 1 | s | 0 0 0 0 [rd:4] [rs:4] 0 | 0 0 | 1 | [rm:4])
                })
            })
        }
        LSR | LSRS => {
            let s = op == LSRS;
            variant(args, |args| {
                let (rd, rm) = ir!("{R} R")(args)?;
                ir!("+")(args)?;
                let imm = args.eat(Value)?;
                if !(1..=32).contains(&imm) {
                    return None;
                }
                let imm = imm % 32;
                inst!([cond:4] 0 0 0 1 1 | 0 1 | s | 0 0 0 0 [rd:4] [imm:5] 0 1 | 0 | [rm:4])
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rm, rs) = ir!("{R} R R")(args)?;
                    inst!([cond:4] 0 0 0 1 1 | 0 1 | s | 0 0 0 0 [rd:4] [rs:4] 0 | 0 1 | 1 | [rm:4])
                })
            })
        }
        MLA | MLAS | MLS => {
            let s = op == MLAS;
            variant(args, |args| {
                let (rd, rn, rm, ra) = ir!("R R R R")(args)?;
                let sub = op == MLS;
                inst!([cond:4] 0 0 0 0 | 0 sub 1 | s | [rd:4] [ra:4] [rm:4] 1 0 0 1 [rn:4])
            })
        }
        MOV | MOVS | MVN | MVNS => {
            let s = matches!(op, MOVS | MVNS);
            let n = matches!(op, MVN | MVNS);
            variant(args, |args| {
                let rd = ir!("R")(args)?;
                let sign = args.eat(Sign)?;
                let imm = args.eat(Value)?;
                let n = if sign == sign::NEGATIVE { !n } else { n };
                inst!([cond:4] 0 0 1 1 1 | n 1 | s | 0 0 0 0 | [rd:4] [imm:12])
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rm) = ir!("R R")(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 0 0 1 1 | n 1 | s | 0 0 0 0 | [rd:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
            .or_else(|| {
                variant(args, |args| {
                    let (rd, rm) = ir!("R R")(args)?;
                    let shift = args.eat(Shift)?;
                    let rs = args.eat(Register)?;
                    inst!([cond:4] 0 0 0 1 1 | n 1 | s | 0 0 0 0 | [rd:4] [rs:4] 0 [shift:2] 1 [rm:4])
                })
            })
        }
        MOVT => variant(args, |args| {
            let rd = ir!("R")(args)?;
            args.eat(Sign).is(sign::POSITIVE)?;
            let imm = args.eat(Value)?;
            let top = imm.bits(12..16);
            let bottom = imm.bits(0..12);
            inst!([cond:4] 0 0 1 1 0 | 1 | 0 0 | [top:4] [rd:4] [bottom:12])
        }),
        // MUL | MULS => {}
        NOP => inst!([cond:4] 0 0 1 1 0 | 0 | 1 0 | 0 0 | 0 0 | 1 1 1 1 | 0 0 0 0 0 0 0 0 0 0 0 0),
        // ORR | ORRS => {}
        POP => variant(args, |args| {
            let list = args.eat(RegisterList)?;
            let n = list.count_ones();
            match n.cmp(&1) {
                Ordering::Less => None,
                Ordering::Equal => {
                    // NOTE: this only works because there is only one '1' in the list
                    // eg:  0000_0000_0000_0001 => 0
                    //      0000_1000_0000_0000 => 11
                    let rt = list.trailing_zeros();
                    let imm = 0b0000_0000_0100;
                    inst!([cond:4] 0 1 0 | 0 | 1 0 0 1 | 1 1 0 1 | [rt:4] [imm:12])
                }
                Ordering::Greater => {
                    inst!([cond:4] 1 0 0 | 0 | 1 0 1 1 | 1 1 0 1 | [list:16])
                }
            }
        }),
        PUSH => variant(args, |args| {
            let list = args.eat(RegisterList)?;
            let n = list.count_ones();
            match n.cmp(&1) {
                Ordering::Less => None,
                Ordering::Equal => {
                    // NOTE: this only works because there is only one '1' in the list
                    // eg:  0000_0000_0000_0001 => 0
                    //      0000_1000_0000_0000 => 11
                    let rt = list.trailing_zeros();
                    let imm = 0b0000_0000_0100;
                    inst!([cond:4] 0 1 0 | 1 | 0 0 1 0 | 1 1 0 1 | [rt:4] [imm:12])
                }
                Ordering::Greater => {
                    inst!([cond:4] 1 0 0 | 1 | 0 0 1 0 | 1 1 0 1 | [list:16])
                }
            }
        }),
        // RBIT => todo!(),
        // REV => todo!(),
        // ROR => todo!(),
        // RORS => todo!(),
        // RRX => todo!(),
        // RRXS => todo!(),
        // RSB => todo!(),
        // RSBS => todo!(),
        // RSC => todo!(),
        // RSCS => todo!(),
        // SBC => todo!(),
        // SBCS => todo!(),
        // SDIV => todo!(),
        STM | STMIA | STMEA => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 1 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMDA | STMED => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 0 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMDB | STMFD => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 0 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMIB | STMFA => variant(args, |args| {
            let rn = ir!("R")(args)?;
            let w = rn & 0x10 != 0;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 1 | 0 | w | 0 [rn:4] [list:16])
        }),
        STR | STRB => {
            let b = op == STRB;
            variant(args, |args| {
                let rt = ir!("R")(args)?;
                let (p, w, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                inst!([cond:4] 0 1 0 | p | u | b | w | 0 | [rn:4] [rt:4] [imm:12])
            })
            .or_else(|| {
                variant(args, |args| {
                    let rt = ir!("R")(args)?;
                    let (p, w, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 1 1 | p | u | b | w | 0 | [rn:4] [rt:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
        }
        STRD => variant(args, |args| {
            let (rt, rt2) = ir!("R R")(args)?;
            if (rt + 1) != rt2 {
                return None;
            }
            let (p, w, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = imm.bits(4..8);
            let imml = imm.bits(0..4);
            inst!([cond:4] 0 0 0 | p | u | 1 | w | 0 | [rn:4] [rt:4] [immh:4] 1 | 1 1 | 1 [imml:4])
        })
        .or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt + 1) != rt2 {
                    return None;
                }
                let (p, w, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | p | u | 0 | w | 0 | [rn:4] [rt:4] | 0 0 0 0 | 1 | 1 1 | 1 [rm:4])
            })
        }),
        STRH => variant(args, |args| {
            let rt = ir!("R")(args)?;
            let (p, w, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = imm.bits(4..8);
            let imml = imm.bits(0..4);
            inst!([cond:4] 0 0 0 | p | u | 1 | w | 0 | [rn:4] [rt:4] [immh:4] 1 | 0 1 | 1 [imml:4])
        }).or_else(|| {
            variant(args, |args| {
                let rt = ir!("R")(args)?;
                let (p, w, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | p | u | 0 | w | 0 | [rn:4] [rt:4] 0 0 0 0 | 1 | 0 1 | 1 [rm:4])
            })
        }),
        // SUB => todo!(),
        // SUBS => todo!(),
        // SVC => todo!(),
        // TEQ => todo!(),
        // TST => todo!(),
        // UDIV => todo!(),
        _ => None,
    };

    if args.finished() {
        encoded
    } else {
        None
    }
}

