use crate::consts::*;

use super::*;

#[allow(dead_code)]
pub enum Shape {
    Unknown,
    Implied,
    Immediate,
    Literal,
    Register,
    Rsr,
    ImmediateSp,
    ImmediatePc,
    RegisterSp,
    MultipleRegisters,
    SingleRegister,
    Label,
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
            ADC | ADCS => {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?; // {<Rd>}
                    cursor.eat(Register); // <Rn>
                    if cursor.eat(Sign)? != sign::POSITIVE {
                        return None;
                    }
                    cursor.eat(Value)?; // #<const>
                    Some(Shape::Immediate)
                })
                .or_else(|| {
                    err_rewind(cursor, |cursor| {
                        cursor.eat(Register)?; // {<Rd>}
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register); // <Rm>
                        if let Some(shift) = cursor.eat(Shift) {
                            cursor.eat(Sign);
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
                        cursor.eat(Register)?; // <Rn>
                        cursor.eat(Register); // <Rm>
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
                Some(Shape::Implied)
            }),
            AND => todo!(),
            ASR => todo!(),
            ASRS => todo!(),
            B => err_rewind(cursor, |cursor| {
                cursor.eat(Label)?;
                Some(Shape::Implied)
            }),
            BFC => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Sign).is(sign::POSITIVE)?;
                let lsb = cursor.eat(Value)?;
                if !(0..=31).contains(&lsb) {
                    return None;
                }
                cursor.eat(Sign).is(sign::POSITIVE)?;
                let width = cursor.eat(Value)?;
                if !(1..=(32 - lsb)).contains(&width) {
                    return None;
                }
                Some(Shape::Implied)
            }),
            BFI => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                cursor.eat(Sign).is(sign::POSITIVE)?;
                let lsb = cursor.eat(Value)?;
                if !(0..=31).contains(&lsb) {
                    return None;
                }
                cursor.eat(Sign).is(sign::POSITIVE)?;
                let width = cursor.eat(Value)?;
                if !(1..=(32 - lsb)).contains(&width) {
                    return None;
                }
                Some(Shape::Implied)
            }),
            BIC => todo!(),
            BICS => todo!(),
            BL => err_rewind(cursor, |cursor| {
                cursor.eat(Label)?;
                Some(Shape::Implied)
            }),
            BX => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                Some(Shape::Implied)
            }),
            CLZ => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                Some(Shape::Implied)
            }),
            CMN | CMP => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Sign)?;
                cursor.eat(Value)?;
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    if let Some(shift) = cursor.eat(Shift) {
                        cursor.eat(Sign);
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
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    cursor.eat(Shift)?;
                    cursor.eat(Register)?;
                    Some(Shape::Rsr)
                })
            }),
            EOR => todo!(),
            EORS => todo!(),
            LDA | LDAB | LDAH => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                if cursor.eat(Address)? != address::OFFSET {
                    return None;
                }
                cursor.eat(Register)?;
                Some(Shape::Implied)
            }),
            LDM | LDMIA | LDMFD | LDMDA | LDMFA | LDMDB | LDMEA | LDMIB | LDMED => {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(RegisterList)?;
                    Some(Shape::Implied)
                })
            }
            LDR | LDRB | LDRD | LDRH | LDRSB | LDRSH => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                let adr = cursor.eat(Address)?;
                let base = cursor.eat(Register)?;
                if adr == address::OFFSET && base == 0b1111 {
                    return None;
                }
                cursor.eat(Offset).is(offset::VALUE)?;
                cursor.eat(Sign)?;
                cursor.eat(Value)?;
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    match cursor.current()? {
                        Label => {
                            cursor.bump(Label);
                        }
                        Address => {
                            cursor.bump(Address).is(address::OFFSET)?;
                            cursor.bump(Register).is(0b1111)?;
                            cursor.eat(Offset).is(offset::VALUE)?;
                            cursor.eat(Sign)?;
                            cursor.eat(Value)?;
                        }
                        _ => return None,
                    }
                    Some(Shape::Literal)
                })
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Address)?;
                    cursor.eat(Register)?;
                    cursor.eat(Offset)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Register)?;
                    let shift = cursor.eat(Shift)?;
                    cursor.eat(Sign);
                    match (shift, cursor.eat(Value)) {
                        (0b11, None) => {}                                    // RRX
                        (0b00 | 0b11, Some(x)) if (1..=31).contains(&x) => {} // LSL | ROR
                        (0b01 | 0b10, Some(x)) if (1..=32).contains(&x) => {} // LSR | ASR
                        _ => return None,
                    }
                    Some(Shape::Register)
                })
            }),
            LSL | LSLS | LSR | LSRS => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Register);
                cursor.eat(Sign);
                cursor.eat(Value)?;
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    cursor.eat(Register);
                    Some(Shape::Register)
                })
            }),
            MLA | MLAS | MLS => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                Some(Shape::Implied)
            }),
            MOV | MOVS | MVN | MVNS => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Sign)?;
                cursor.eat(Value)?;
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    if let Some(shift) = cursor.eat(Shift) {
                        cursor.eat(Sign);
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
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    cursor.eat(Shift)?;
                    cursor.eat(Register)?;
                    Some(Shape::Rsr)
                })
            }),
            MOVT => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Sign).is(sign::POSITIVE)?;
                cursor.eat(Value)?;
                Some(Shape::Implied)
            }),
            MUL => todo!(),
            MULS => todo!(),
            NOP => Some(Shape::Implied),
            ORR => todo!(),
            OORS => todo!(),
            POP | PUSH => err_rewind(cursor, |cursor| {
                let n = cursor.eat(RegisterList)?.count_ones();
                match n.cmp(&1) {
                    std::cmp::Ordering::Less => None,
                    std::cmp::Ordering::Equal => Some(Shape::SingleRegister),
                    std::cmp::Ordering::Greater => Some(Shape::MultipleRegisters),
                }
            }),
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
            STM | STMIA | STMEA | STMDA | STMED | STMDB | STMFD | STMIB | STMFA => {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(RegisterList)?;
                    Some(Shape::Implied)
                })
            }
            STR | STRB => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Address)?;
                cursor.bump(Register);
                if let Some(off) = cursor.eat(Offset) {
                    off.is(offset::VALUE)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Value)?;
                }
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Address)?;
                    cursor.bump(Register);
                    cursor.eat(Offset).is(offset::REGISTER)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Register)?;
                    if cursor.eat(Shift).is_some() {
                        cursor.eat(Sign)?;
                        cursor.eat(Value)?;
                    }
                    Some(Shape::Register)
                })
            }),
            STRD => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Register)?;
                cursor.eat(Address)?;
                cursor.bump(Register);
                if let Some(off) = cursor.eat(Offset) {
                    off.is(offset::VALUE)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Value)?;
                }
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Register)?;
                    cursor.eat(Address)?;
                    cursor.bump(Register);
                    cursor.eat(Offset).is(offset::REGISTER)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Register)?;
                    Some(Shape::Register)
                })
            }),
            STRH => err_rewind(cursor, |cursor| {
                cursor.eat(Register)?;
                cursor.eat(Address)?;
                cursor.bump(Register);
                if let Some(off) = cursor.eat(Offset) {
                    off.is(offset::VALUE)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Value)?;
                }
                Some(Shape::Immediate)
            })
            .or_else(|| {
                err_rewind(cursor, |cursor| {
                    cursor.eat(Register)?;
                    cursor.eat(Address)?;
                    cursor.bump(Register);
                    cursor.eat(Offset).is(offset::REGISTER)?;
                    cursor.eat(Sign)?;
                    cursor.eat(Register)?;
                    Some(Shape::Register)
                })
            }),
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
        Shape::Label
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

trait Is {
    fn is(&self, x: u32) -> Option<()>;
}

trait IsNot {
    fn is_not(&self, x: u32) -> Option<()>;
}

impl Is for Option<u32> {
    fn is(&self, x: u32) -> Option<()> {
        if let Some(y) = self {
            if x == *y {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl IsNot for Option<u32> {
    fn is_not(&self, x: u32) -> Option<()> {
        if let Some(y) = self {
            if x == *y {
                None
            } else {
                Some(())
            }
        } else {
            None
        }
    }
}

impl Is for u32 {
    fn is(&self, x: u32) -> Option<()> {
        if *self == x {
            Some(())
        } else {
            None
        }
    }
}

impl IsNot for u32 {
    fn is_not(&self, x: u32) -> Option<()> {
        if *self == x {
            None
        } else {
            Some(())
        }
    }
}
