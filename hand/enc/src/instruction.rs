use crate::bits::{self, ToWord};
use crate::cursor::*;
use crate::LabelValue;

use std::cmp::Ordering;

use middle::higher;
use middle::{AtomKind::*, Cursor};

fn pc(x: u32) -> u32 {
    align(x + 8, 4)
}

fn align(x: u32, to: u32) -> u32 {
    x + (x % to)
}

pub(crate) fn encode(
    args: &mut Cursor,
    labels: &crate::LabelMap,
    curr: u32,
    op: u32,
) -> Option<u32> {
    let op = unsafe { higher::<syntax::Opcode>(op) };
    let cond = args.bump(Condition);

    use syntax::Opcode::*;

    let off = match labels[&curr] {
        LabelValue::Offset(x) => x,
        _ => unreachable!(),
    };
    let pc = pc(off);

    let encoded = match op {
        ADC | ADCS => {
            let s = op == ADCS;
            variant(args, |args| {
                let (rd, rn) = ir!("{R} R")(args)?;
                // TODO: do imm12 parsing fn(u32) -> Option<u32>
                let imm = signed_number(args, syntax::Sign::Positive)?;
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
                    let (shift, rs) = shift_reg(args)?;
                    inst!([cond:4] 0 0 0 0 | 1 0 1 | s | [rn:4] [rd:4] [rs:4] 0 [shift:2] 1 [rm:4])
                })
            })
        }
        ADR => variant(args, |args| {
            let rd = register(args)?;
            let pos = label_value(args, labels)?;
            let (imm, p, n) = if pos >= pc {
                (pos - pc, 1, 0)
            } else {
                (pc - pos, 0, 1)
            };
            inst!([cond:4] 0 0 1 0 | p n 0 | 0 | 1 1 1 1 [rd:4] [imm:12])
        }),
        B => variant(args, |args| {
            let pos = label_value(args, labels)?;
            let imm = if pos >= pc {
                (pos - pc) / 4
            } else {
                u32::MAX - ((pc - pos) / 4) + 1
            };
            inst!([cond:4] 1 0 1 | 0 [imm:24])
        }),
        BFC => variant(args, |args| {
            let rd = register(args)?;
            let lsb = signed_number(args, syntax::Sign::Positive)?;
            if !(0..=31).contains(&lsb) {
                return None;
            }
            let width = signed_number(args, syntax::Sign::Positive)?;
            if !(1..=(32 - lsb)).contains(&width) {
                return None;
            }
            let msb = lsb + width - 1;
            inst!([cond:4] 0 1 1 1 1 1 0 [msb:5] [rd:4] [lsb:5] 0 0 1 | 1 1 1 1)
        }),
        BFI => variant(args, |args| {
            let (rd, rn) = ir!("R R")(args)?;
            let lsb = signed_number(args, syntax::Sign::Positive)?;
            if !(0..=31).contains(&lsb) {
                return None;
            }
            let width = signed_number(args, syntax::Sign::Positive)?;
            if !(1..=(32 - lsb)).contains(&width) {
                return None;
            }
            let msb = lsb + width - 1;
            inst!([cond:4] 0 1 1 1 1 1 0 [msb:5] [rd:4] [lsb:5] 0 0 1 [rn:4])
        }),
        // BIC(S)
        BL => variant(args, |args| {
            let pos = label_value(args, labels)?;
            let imm = if pos >= pc {
                (pos - pc) / 4
            } else {
                u32::MAX - ((pc - pos) / 4) + 1
            };
            inst!([cond:4] 1 0 1 | 1 | [imm:24])
        }),
        BX => variant(args, |args| {
            let rm = register(args)?;
            inst!([cond:4] 0 0 0 1 0 0 1 0 | 1 1 1 1 | 1 1 1 1 | 1 1 1 1 | 0 0 0 1 | [rm:4])
        }),
        CLZ => variant(args, |args| {
            let (rd, rm) = ir!("R R")(args)?;
            inst!([cond:4] 0 0 0 1 0 1 1 0 | 1 1 1 1 | [rd:4] | 1 1 1 1 | 0 0 0 1 | [rm:4])
        }),
        CMN | CMP => {
            let s = op == CMN;
            variant(args, |args| {
                let rn = register(args)?;
                let (sign, imm) = number(args)?;
                let s = if sign.is_negative() { !s } else { s };
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
                    let rs = register(args)?;
                    inst!([cond:4] 0 0 0 1 0 | 1 s | 1 | [rn:4] | 0 0 0 0 | [rs:4] 0 [shift:2] 1 [rm:4])
                })
            })
        }
        // EOR(S)
        LDA | LDAB | LDAH => variant(args, |args| {
            let rt = register(args)?;
            let (addr, rn) = address(args)?;
            if !addr.is_offset() {
                return None;
            }
            let width = match op {
                LDA => 0b00,
                LDAB => 0b10,
                LDAH => 0b11,
                _ => unreachable!(),
            };
            inst!([cond:4] 0 0 0 1 1 [width:2] 1 [rn:4] [rt:4] 1 1 0 0 | 1 0 0 1 | 1 1 1 1)
        }),
        LDM | LDMIA | LDMFD => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 1 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMDA | LDMFA => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 0 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMDB | LDMEA => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 0 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDMIB | LDMED => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 1 | 0 | w | 1 | [rn:4] [list:16])
        }),
        LDR | LDRB => {
            let b = op == LDRB;
            variant(args, |args| {
                let rt = register(args)?;
                let (addr, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                inst!([cond: 4] 0 1 0 | {addr.p()} | u | b | {addr.w()} | 1 | [rn:4] [rt:4] [imm:12])
            }).or_else(|| {
                variant(args, |args| {
                    let rt = register(args)?;
                    let pos = label_value(args, labels)?;
                    let (imm, u) = if pos >= pc {
                        (pos - pc, 1)
                    } else {
                        (pc - pos, 0)
                    };
                    inst!([cond:4] 0 1 0 | 1 | u | b | 0 | 1 | 1 1 1 1 [rt:4] [imm:12])
                })
            }).or_else(|| {
                variant(args, |args| {
                    let rt = register(args)?;
                    let (addr, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 1 1 | {addr.p()} | u | b | {addr.w()} | 1 | [rn:4] [rt:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
        }
        LDRD => variant(args, |args| {
            let (rt, rt2) = ir!("R R")(args)?;
            if rt.value() % 2 != 0 || rt.value() == 14 {
                return None;
            }
            if (rt.value() + 1) != rt2.value() {
                return None;
            }
            let (addr, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = bits::get(imm, 4..8);
            let imml = bits::get(imm, 0..4);
            inst!([cond:4] 0 0 0 | {addr.p()} | u | 1 | {addr.w()} | 0 [rn:4] [rt:4] [immh:4] 1 | 1 0 | 1 [imml:4])
        }).or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt.value() + 1) != rt2.value() {
                    return None;
                }
                let pos = label_value(args, labels)?;
                let (imm, u) = if pos >= pc {
                    (pos - pc, 1)
                } else {
                    (pc - pos, 0)
                };
                let immh = bits::get(imm, 4..8);
                let imml = bits::get(imm, 0..4);
                inst!([cond:4] 0 0 0 | 1 | u | 1 | 0 | 0 | 1 1 1 1 | [rt:4] [immh:4] 1 | 1 0 | 1 [imml:4])
            })
        }).or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt.value() + 1) != rt2.value() {
                    return None;
                }
                let (addr, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | {addr.p()} | u | 0 | {addr.w()} | 0 | [rn:4] [rt:4] 0 0 0 0 | 1 | 1 0 | 1 [rm:4])
            })
        }),
        LDRH | LDRSB | LDRSH => {
            let h = matches!(op, LDRH | LDRSH);
            let s = matches!(op, LDRSB | LDRSH);
            variant(args, |args| {
                let rt = register(args)?;
                let (addr, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                let immh = bits::get(imm, 4..8);
                let imml = bits::get(imm, 0..4);
                inst!([cond:4] 0 0 0 | {addr.p()} | u | 1 | {addr.w()} | 1 | [rn:4] [rt:4] [immh:4] 1 | s h | 1 | [imml:4])
            }).or_else(|| {
                variant(args, |args| {
                    let rt = register(args)?;
                    let pos = label_value(args, labels)?;
                    let (imm, u) = if pos >= pc {
                        (pos - pc, 1)
                    } else {
                        (pc - pos, 0)
                    };
                    let immh = bits::get(imm, 4..8);
                    let imml = bits::get(imm, 0..4);
                    inst!([cond:4] 0 0 0 | 1 | u | 1 | 0 | 1 | 1 1 1 1 [rt:4] [immh:4] 1 | s h | 1 [imml:4])
                })
            }).or_else(|| {
                variant(args, |args| {
                    let rt = register(args)?;
                    let (addr, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    inst!([cond:4] 0 0 0 | {addr.p()} | u | 0 | {addr.w()} | 1 | [rn:4] [rt:4] 0 0 0 0 | 1 | s h | 1 | [rm:4])
                })
            })
        },
        LSL | LSLS => {
            let s = op == LSLS;
            variant(args, |args| {
                let (rd, rm) = ir!("{R} R")(args)?;
                if !sign(args)?.is_positive() {
                    return None;
                }
                let imm = args.eat(Number)?;
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
                if !sign(args)?.is_positive() {
                    return None;
                }
                let imm = args.eat(Number)?;
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
                let rd = register(args)?;
                let sign = sign(args)?;
                let imm = args.eat(Number)?;
                let n = if sign.is_negative() { !n } else { n };
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
            let rd = register(args)?;
            if !sign(args)?.is_positive() {
                return None;
            }
            let imm = args.eat(Number)?;
            let top = bits::get(imm, 12..16);
            let bottom = bits::get(imm, 0..12);
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
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 1 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMDA | STMED => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 0 | 0 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMDB | STMFD => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 0 | 0 | w | 0 [rn:4] [list:16])
        }),
        STMIB | STMFA => variant(args, |args| {
            let (rn, w) = register_bang(args)?;
            let list = args.eat(RegisterList)?;
            inst!([cond:4] 1 0 0 | 1 | 1 | 0 | w | 0 [rn:4] [list:16])
        }),
        STR | STRB => {
            let b = op == STRB;
            variant(args, |args| {
                let rt = register(args)?;
                let (addr, rn) = address(args)?;
                let (imm, u) = value_offset(args)?;
                inst!([cond:4] 0 1 0 | {addr.p()} | u | b | {addr.w()} | 0 | [rn:4] [rt:4] [imm:12])
            })
            .or_else(|| {
                variant(args, |args| {
                    let rt = register(args)?;
                    let (addr, rn) = address(args)?;
                    let (rm, u) = register_offset(args)?;
                    let (shift, imm) = shift_imm(args)?;
                    inst!([cond:4] 0 1 1 | {addr.p()} | u | b | {addr.w()} | 0 | [rn:4] [rt:4] [imm:5] [shift:2] 0 [rm:4])
                })
            })
        }
        STRD => variant(args, |args| {
            let (rt, rt2) = ir!("R R")(args)?;
            if rt.value() % 2 != 0 || rt.value() == 14 {
                return None;
            }
            if (rt.value() + 1) != rt2.value() {
                return None;
            }
            let (addr, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = bits::get(imm, 4..8);
            let imml = bits::get(imm, 0..4);
            inst!([cond:4] 0 0 0 | {addr.p()} | u | 1 | {addr.w()} | 0 | [rn:4] [rt:4] [immh:4] 1 | 1 1 | 1 [imml:4])
        })
        .or_else(|| {
            variant(args, |args| {
                let (rt, rt2) = ir!("R R")(args)?;
                if (rt.value() + 1) != rt2.value() {
                    return None;
                }
                let (addr, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | {addr.p()} | u | 0 | {addr.w()} | 0 | [rn:4] [rt:4] | 0 0 0 0 | 1 | 1 1 | 1 [rm:4])
            })
        }),
        STRH => variant(args, |args| {
            let rt = register(args)?;
            let (addr, rn) = address(args)?;
            let (imm, u) = value_offset(args)?;
            let immh = bits::get(imm, 4..8);
            let imml = bits::get(imm, 0..4);
            inst!([cond:4] 0 0 0 | {addr.p()} | u | 1 | {addr.w()} | 0 | [rn:4] [rt:4] [immh:4] 1 | 0 1 | 1 [imml:4])
        }).or_else(|| {
            variant(args, |args| {
                let rt = register(args)?;
                let (addr, rn) = address(args)?;
                let (rm, u) = register_offset(args)?;
                inst!([cond:4] 0 0 0 | {addr.p()} | u | 0 | {addr.w()} | 0 | [rn:4] [rt:4] 0 0 0 0 | 1 | 0 1 | 1 [rm:4])
            })
        }),
        // SUB => todo!(),
        // SUBS => todo!(),
        SVC => variant(args, |args| {
            args.eat(Sign);
            let imm = args.eat(Number)?;
            inst!([cond:4] 1 1 1 1 | [imm:24])
        }),
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

/// Produces a 32 bit value from user definied values.
///
/// # How it works
/// This macro is an Incremental tt muncher.
/// It builds the expression `x` based on values that you give it.
/// Once the input is exhausted, the value is returned as an Option.
/// If 32 values weren't given to the macro, it will warn you and return None.
macro_rules! inst {
    // empty
    (@inner; $pos:expr; $x:expr;) => {
        if $pos == 0 {
            Some($x)
        } else {
            None
        }
    };
    // [{expr}:width] (fill with expr for width)
    (@inner; $pos:expr; $x:expr; [{$ex:expr} : $width:expr] $($t:tt)*) => {
        // inst!(@inner; $pos - $width; ($x).with_bits(($pos - $width)..($pos), ($ex).word() & !(!0 << $width)); $($t)*)
        inst!(@inner; $pos - $width; bits::set($x, ($pos - $width)..($pos), ($ex).word() & !(!0 << $width)); $($t)*)
    };
    // [id:width] (fill with id for width)
    (@inner; $pos:expr; $x:expr; [$id:ident : $width:expr] $($t:tt)*) => {
        // inst!(@inner; $pos - $width; ($x).with_bits(($pos - $width)..($pos), ($id).word() & !(!0 << $width)); $($t)*)
        inst!(@inner; $pos - $width; bits::set($x, ($pos - $width)..($pos), ($id).word() & !(!0 << $width)); $($t)*)
    };
    // 0 (do nothing)
    (@inner; $pos:expr; $x:expr; 0 $($t:tt)*) => {
        inst!(@inner; $pos - 1; $x; $($t)*)
    };
    // 1 (set bit at position)
    (@inner; $pos:expr; $x:expr; 1 $($t:tt)*) => {
        inst!(@inner; $pos - 1; ($x | (1 << ($pos - 1))); $($t)*)
    };
    // expr (width = 1)
    (@inner; $pos:expr; $x:expr; {$ex:expr} $($t:tt)*) => {
        inst!(@inner; $pos - 1; ($x | (($ex).word() << ($pos - 1))); $($t)*)
    };
    // id (width = 1)
    (@inner; $pos:expr; $x:expr; $id:ident $($t:tt)*) => {
        inst!(@inner; $pos - 1; ($x | (($id).word() << ($pos - 1))); $($t)*)
    };
    // | (visual separator)
    (@inner; $pos:expr; $x:expr; | $($t:tt)*) => {
        inst!(@inner; $pos; $x; $($t)*)
    };
    // entrance
    ($($t:tt)*) => {
        inst!(@inner; 32_u32; 0_u32; $($t)*)
    };
}

/// Shortcuts for common closures that eat the IR.
macro_rules! ir {
    ("R") => {
        register
    };
    ("R R") => {
        |args: &mut Cursor| -> Option<(syntax::Register, syntax::Register)> {
            let a = register(args)?;
            let b = register(args)?;
            Some((a, b))
        }
    };
    ("R R R") => {
        |args: &mut Cursor| -> Option<(syntax::Register, syntax::Register, syntax::Register)> {
            let a = register(args)?;
            let b = register(args)?;
            let c = register(args)?;
            Some((a, b, c))
        }
    };
    ("R R R R") => {
        |args: &mut Cursor| -> Option<(
            syntax::Register,
            syntax::Register,
            syntax::Register,
            syntax::Register,
        )> {
            let a = register(args)?;
            let b = register(args)?;
            let c = register(args)?;
            let d = register(args)?;
            Some((a, b, c, d))
        }
    };
    ("{R} R") => {
        |args: &mut Cursor| -> Option<(syntax::Register, syntax::Register)> {
            let a = register(args)?;
            Some(if let Some(b) = register(args) {
                (a, b)
            } else {
                (a, a)
            })
        }
    };
    ("{R} R R") => {
        |args: &mut Cursor| -> Option<(syntax::Register, syntax::Register, syntax::Register)> {
            let a = register(args)?;
            let b = register(args)?;
            Some(if let Some(c) = register(args) {
                (a, b, c)
            } else {
                (a, a, b)
            })
        }
    };
}

pub(self) use inst;
pub(self) use ir;
