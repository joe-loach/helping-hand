use super::*;

#[allow(dead_code)]
pub enum Shape {
    Unknown,
    Normal,
    Immediate,
    Literal,
    Register,
    Rsr,
    ImmediateSp,
    ImmediatePc,
    RegisterSp,
    MultipleRegisters,
    SingleRegisters,
}

pub(super) fn shape(cursor: &mut Cursor) -> Shape {
    use Atom::*;

    fn err_rewind<T>(cursor: &mut Cursor, f: impl FnOnce(&mut Cursor) -> Option<T>) -> Option<T> {
        let c = cursor.checkpoint();
        let res = f(cursor);
        if res.is_none() {
            cursor.rewind(c);
        }
        res
    }

    cursor.bump(Label);

    if let Some(op) = cursor.eat(Instruction) {
        let op = syn::<syntax::Opcode>(op);
        cursor.bump(Condition);

        use syntax::Opcode::*;

        let shape = match op {
            ADC => {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?; // {<Rd>}
                    cursor.eat(Register); // <Rn>
                    if cursor.eat(Sign)? != crate::Sign::Positive as u32 {
                        return None;
                    }
                    cursor.eat(Value)?; // #<const>
                    Some(Shape::Immediate)
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // {<Rd>}
                        cursor.eat(Register); // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        if let Some(shift) = cursor.eat(Shift) {
                            cursor.eat(Sign)?;
                            match (shift, cursor.eat(Value)) {
                                (0b11, None) => {}                                    // RRX
                                (0b00 | 0b11, Some(x)) if (1..=31).contains(&x) => {} // LSL | ROR
                                (0b01 | 0b10, Some(x)) if (1..=32).contains(&x) => {} // LSR | ASR
                                _ => return None,
                            }
                        }
                        Some(Shape::Register)
                    })
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // {<Rd>}
                        cursor.eat(Register); // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        cursor.eat(Shift)?; // <shift>
                        cursor.eat(Register)?; // <Rs>
                        Some(Shape::Rsr)
                    })
                })
            }
            ADCS => {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?; // {<Rd>}
                    cursor.eat(Register); // <Rn>
                    if cursor.eat(Sign)? != crate::Sign::Positive as u32 {
                        return None;
                    }
                    cursor.eat(Value)?; // #<const>
                    Some(Shape::Immediate)
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // {<Rd>}
                        cursor.eat(Register); // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        if let Some(shift) = cursor.eat(Shift) {
                            cursor.eat(Sign)?;
                            match (shift, cursor.eat(Value)) {
                                (0b11, None) => {}                                    // RRX
                                (0b00 | 0b11, Some(x)) if (1..=31).contains(&x) => {} // LSL | ROR
                                (0b01 | 0b10, Some(x)) if (1..=32).contains(&x) => {} // LSR | ASR
                                _ => return None,
                            }
                        }
                        Some(Shape::Register)
                    })
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // {<Rd>}
                        cursor.eat(Register); // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        cursor.eat(Shift)?; // <shift>
                        cursor.eat(Register)?; // <Rs>
                        Some(Shape::Rsr)
                    })
                })
            }
            ADD => {
                // TODO: add is weird, it has special cases for PC and SP
                // chaining only works if it is defined in specificity:
                // going from MOST -> LEAST
                todo!()
            }
            ADDS => todo!(),
            ADR => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Label)?;
                Some(Shape::Normal)
            }),
            AND => todo!(),
            ASR => todo!(),
            ASRS => todo!(),
            B => todo!(),
            BFC => todo!(),
            BFI => todo!(),
            BIC => todo!(),
            BICS => todo!(),
            BL => todo!(),
            BX => todo!(),
            CLZ => todo!(),
            CMN => todo!(),
            CMP => todo!(),
            EOR => todo!(),
            EORS => todo!(),
            LDA => todo!(),
            LDAB => todo!(),
            LDAH => todo!(),
            LDM => todo!(),
            LDMIA => todo!(),
            LDMFD => todo!(),
            LDMDA => todo!(),
            LDMFA => todo!(),
            LDMDB => todo!(),
            LDMEA => todo!(),
            LDMIB => todo!(),
            LDMED => todo!(),
            LDR => todo!(),
            LDRB => todo!(),
            LDRD => todo!(),
            LDRH => todo!(),
            LDRSB => todo!(),
            LDRSH => todo!(),
            LSL => todo!(),
            LSLS => todo!(),
            LSR => todo!(),
            LSRS => todo!(),
            MLA => todo!(),
            MLAS => todo!(),
            MLS => todo!(),
            MOV => todo!(),
            MOVS => todo!(),
            MOVT => todo!(),
            MUL => todo!(),
            MULS => todo!(),
            MVN => todo!(),
            MVNS => todo!(),
            NOP => Some(Shape::Normal),
            ORR => todo!(),
            OORS => todo!(),
            POP => todo!(),
            PUSH => todo!(),
            RBIT => todo!(),
            REV => todo!(),
            ROR => todo!(),
            RORS => todo!(),
            RRX => todo!(),
            RRXS => todo!(),
            RSB => todo!(),
            RSBS => todo!(),
            RSC => todo!(),
            RSCS => todo!(),
            SBC => todo!(),
            SBCS => todo!(),
            SDIV => todo!(),
            STM => todo!(),
            STMIA => todo!(),
            STMEA => todo!(),
            STMDB => todo!(),
            STMFD => todo!(),
            STMID => todo!(),
            STMFA => todo!(),
            STR => todo!(),
            STRB => todo!(),
            STRD => todo!(),
            STRH => todo!(),
            SUB => todo!(),
            SUBS => todo!(),
            SVC => todo!(),
            TEQ => todo!(),
            TST => todo!(),
            UDIV => todo!(),
        };

        // cursor might have more arguments to parse,
        // therefore the shape doesnt fit
        if cursor.finished() {
            shape.unwrap_or(Shape::Unknown)
        } else {
            Shape::Unknown
        }
    } else {
        // OK
        Shape::Normal
    }
}

fn syn<T: FromRaw>(x: u32) -> T {
    unsafe { FromRaw::from(x) }
}

use core::mem;

/// # Safety
/// We know how the conversions to u32 are made,
/// so we can convert them back.
unsafe trait FromRaw {
    unsafe fn from(x: u32) -> Self;
}

unsafe impl FromRaw for syntax::Opcode {
    unsafe fn from(x: u32) -> Self {
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Condition {
    unsafe fn from(x: u32) -> Self {
        mem::transmute(x as u8)
    }
}
