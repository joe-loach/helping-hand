use middle::Cursor;
use middle::higher;
use middle::AtomKind::*;

use crate::LabelValue;

pub(crate) fn variant<T>(
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

pub(crate) fn shift_imm(args: &mut Cursor) -> Option<(syntax::Shift, u32)> {
    use syntax::Shift;
    let data = if let Some(shift) = args.eat(Shift) {
        let shift = unsafe { higher::<Shift>(shift) };
        let sign = sign(args);
        let value = args.eat(Number);
        if value.is_some() && sign?.is_negative() {
            return None;
        }
        match (shift, value) {
            // RRX (= ROR with no value)
            (Shift::ROR, None) => (Shift::RRX, 0),
            // LSL | ROR
            (Shift::LSL | Shift::ROR, Some(x)) if (1..=31).contains(&x) => (shift, x),
            // LSR | ASR
            (Shift::LSR | Shift::ASR, Some(x)) if (1..=32).contains(&x) => (shift, x % 32),
            // Any shift when value = 0
            (_, Some(0)) => (Shift::LSL, 0),
            _ => return None,
        }
    } else {
        (Shift::LSL, 0) // LSL #0
    };
    Some(data)
}

pub(crate) fn shift_reg(args: &mut Cursor) -> Option<(syntax::Shift, syntax::Register)> {
    unsafe {
        let shift = higher(args.eat(Shift)?);
        let reg = register(args)?;
        Some((shift, reg))
    }
}

pub(crate) fn address(args: &mut Cursor) -> Option<(syntax::AddressKind, syntax::Register)> {
    let addr = unsafe { higher(args.eat(Address)?) };
    let reg = register(args)?;
    Some((addr, reg))
}

/// (u32, bool): (imm, u)
pub(crate) fn value_offset(args: &mut Cursor) -> Option<(u32, bool)> {
    let data = if let Some(off) = offset(args) {
        if !off.is_value() {
            return None;
        }
        let (s, imm) = number(args)?;
        let u = s.is_positive();
        (imm, u)
    } else {
        (0, true)
    };
    Some(data)
}

pub(crate) fn register_offset(args: &mut Cursor) -> Option<(u32, bool)> {
    let offset = offset(args)?;
    if !offset.is_register() {
        return None;
    }
    let u = sign(args)?.is_positive();
    let reg = args.eat(Register)?;
    Some((reg, u))
}

pub(crate) fn label_value(args: &mut Cursor, labels: &crate::LabelMap) -> Option<u32> {
    let lbl = args.eat(Label)?;
    let val = labels[&lbl];
    match val {
        LabelValue::Offset(x) => Some(x),
        _ => None,
    }
}

pub(crate) fn signed_number(args: &mut Cursor, sign: syntax::Sign) -> Option<u32> {
    let (s, num) = number(args)?;
    if s != sign {
        return None;
    }
    Some(num)
}

pub(crate) fn register_bang(args: &mut Cursor) -> Option<(syntax::Register, bool)> {
    unsafe {
        let data = args.eat(Register)?;
        let syn = higher(data);
        let bang = data & 0x10 != 0;
        Some((syn, bang))
    }
}

pub(crate) fn register(args: &mut Cursor) -> Option<syntax::Register> {
    unsafe { Some(higher(args.eat(Register)?)) }
}

pub(crate) fn offset(args: &mut Cursor) -> Option<syntax::OffsetKind> {
    unsafe { Some(higher(args.eat(Offset)?)) }
}

pub(crate) fn number(args: &mut Cursor) -> Option<(syntax::Sign, u32)> {
    unsafe {
        let sign = higher(args.eat(Sign)?);
        let number = args.eat(Number)?;
        Some((sign, number))
    }
}

pub(crate) fn sign(args: &mut Cursor) -> Option<syntax::Sign> {
    unsafe { Some(higher(args.eat(Sign)?)) }
}
