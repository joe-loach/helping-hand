use middle::consts::*;
use middle::{Atom::*, Cursor};

pub(super) fn shift_imm(args: &mut Cursor) -> Option<(u32, u32)> {
    let data = if let Some(shift) = args.eat(Shift) {
        let sign = args.eat(Sign);
        let value = args.eat(Value);
        if value.is_some() {
            sign.is(sign::POSITIVE)?;
        }
        match (shift, value) {
            // RRX
            (shift::RRX, None) => (shift::RRX, 0),
            // LSL | ROR
            (shift::LSL | shift::ROR, Some(x)) if (1..=31).contains(&x) => (shift, x),
            // LSR | ASR
            (shift::LSR | shift::ASR, Some(x)) if (1..=32).contains(&x) => (shift, x % 32),
            // No shift when value = 0
            (_, Some(0)) => (shift::LSL, 0),
            _ => return None,
        }
    } else {
        (shift::LSL, 0) // LSL #0
    };
    Some(data)
}

pub(super) fn address(args: &mut Cursor) -> Option<(bool, bool, u32)> {
    let addr = args.eat(Address)?;
    let reg = args.bump(Register);
    let p = (addr & 0b10) != 0;
    let w = (addr & 0b01) != 0;
    Some((p, w, reg))
}

pub(super) fn value_offset(args: &mut Cursor) -> Option<(u32, bool)> {
    let data = if let Some(off) = args.eat(Offset) {
        off.is(offset::VALUE)?;
        let u = args.eat(Sign)? == sign::POSITIVE;
        let imm = args.eat(Value)?;
        (imm, u)
    } else {
        (0, true)
    };
    Some(data)
}

pub(super) fn register_offset(args: &mut Cursor) -> Option<(u32, bool)> {
    args.eat(Offset).is(offset::REGISTER)?;
    let u = args.eat(Sign)? == sign::POSITIVE;
    let reg = args.eat(Register)?;
    Some((reg, u))
}

pub(super) fn variant<T>(
    cursor: &mut Cursor,
    f: impl FnOnce(&mut Cursor) -> Option<T>,
) -> Option<T> {
    let c = cursor.checkpoint();
    let res = f(cursor);
    if res.is_none() {
        cursor.rewind(c);
    }
    res
}

pub(super) trait Is {
    fn is(&self, x: u32) -> Option<()>;
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

impl Is for u32 {
    fn is(&self, x: u32) -> Option<()> {
        if *self == x {
            Some(())
        } else {
            None
        }
    }
}
