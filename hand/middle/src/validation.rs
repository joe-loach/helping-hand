use super::*;

pub(super) fn shape(cursor: &mut Cursor) -> bool {
    use Atom::*;

    cursor.bump(Label);

    fn err_rewind<T>(cursor: &mut Cursor, f: impl FnOnce(&mut Cursor) -> Option<T>) -> Option<T> {
        let c = cursor.checkpoint();
        let res = f(cursor);
        if res.is_none() {
            cursor.rewind(c);
        }
        res
    }

    if let Some(op) = cursor.eat(Instruction) {
        let op = syn::<syntax::Opcode>(op);
        cursor.bump(Condition);

        use syntax::Opcode::*;

        // TODO: for the love of god write a macro to make this smaller
        // and easier to maintain!
        // please Joe
        match op {
            ADC => {
                cursor.eat(Register); // {<Rd>}
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?; // <Rn>
                    cursor.eat(Value)?; // #<const>
                    Some(())
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        match cursor.eat(Shift) {
                            // RRX
                            Some(0b11) => (),
                            // <shift> #<amount>
                            Some(_) => {
                                cursor.eat(Value)?;
                            }
                            // {}
                            None => (),
                        }
                        Some(())
                    })
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        cursor.eat(Shift)?; // <shift>
                        cursor.eat(Register)?; // <Rs>
                        Some(())
                    })
                })
                .is_some()
            }
            ADCS => {
                cursor.eat(Register); // {<Rd>}
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?; // <Rn>
                    cursor.eat(Value)?; // #<const>
                    Some(())
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        match cursor.eat(Shift) {
                            // RRX
                            Some(0b11) => (),
                            // <shift> #<amount>
                            Some(_) => {
                                cursor.eat(Value)?;
                            }
                            // {}
                            None => (),
                        }
                        Some(())
                    })
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register)?; // <Rm>
                        cursor.eat(Shift)?; // <shift>
                        cursor.eat(Register)?; // <Rs>
                        Some(())
                    })
                })
                .is_some()
            },
            ADD => {
                // TODO: add is weird, it has special cases for PC and SP
                // chaining only works if it is defined in specificity:
                // going from MOST -> LEAST
                true
            },
            ADDS => todo!(),
            ADR => todo!(),
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
            NOP => true,
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
        }
    } else {
        // OK
        true
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
