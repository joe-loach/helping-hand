use intbits::Bits;

#[test]
fn test_decode() {
    decode(16818402);
}

pub fn decode(word: u32) -> Option<()> {
    let cond = word.bits(28..32);
    let op0 = word.bits(25..28);
    let op1 = word.bit(4) as u8;

    let family = match (cond, bitarr(op0), op1) {
        (c, [0, 0, _], _) if c != 0b1111 => data,
        (c, [0, 1, 0], _) if c != 0b1111 => ls_imm,
        (c, [0, 1, 1], 0) if c != 0b1111 => ls_reg,
        (c, [0, 1, 1], 1) if c != 0b1111 => media,
        (_, [1, 0, _], _) => branch,
        (_, [1, 1, _], _) => system,
        (0b1111, [0, _, _], _) => unconditional,
        _ => return None,
    };

    family(word)?;

    Some(())
}

/// Data-processing and miscellaneous instructions
fn data(word: u32) -> Option<()> {
    let op0 = word.bit(25) as u32;
    let op1 = word.bits(20..25);
    let op2 = word.bit(7) as u32;
    let op3 = word.bits(5..7);
    let op4 = word.bit(4) as u32;

    match (op0, bitarr(op1), op2, op3, op4) {
        // extra load/store
        (0, _, 1, op3, 1) if op3 != 0b00 => {}
        (0, [0, _, _, _, _], 1, 0b00, 1) => {}
        (0, [1, _, _, _, _], 1, 0b00, 1) => {}
        (0, [1, 0, _, _, 0], 0, _, _) => {}
        (0, [1, 0, _, _, 0], 1, _, 0) => {}
        (0, op1, _, _, 0) if !matches!(op1, [1, 0, _, _, 0]) => {}
        (0, op1, 0, _, 1) if !matches!(op1, [1, 0, _, _, 0]) => {}
        (1, _, _, _, _) => {}
        _ => return None,
    }
    Some(())
}

/// Load/Store Word, Unsigned Byte (immediate, literal)
fn ls_imm(word: u32) -> Option<()> {
    Some(())
}

/// Load/Store Word, Unsigned Byte (register)
fn ls_reg(word: u32) -> Option<()> {
    Some(())
}

/// Media instructions
fn media(word: u32) -> Option<()> {
    Some(())
}

/// Branch, branch with link, and block data transfer
fn branch(word: u32) -> Option<()> {
    Some(())
}

/// System register access, Advanced SIMD, floating-point, and Supervisor call
fn system(word: u32) -> Option<()> {
    Some(())
}

/// Unconditional instructions
fn unconditional(word: u32) -> Option<()> {
    Some(())
}

fn bitarr<const N: usize>(x: u32) -> [u8; N] {
    let mut arr = [0; N];
    for (i, bit) in arr.iter_mut().enumerate().take(N) {
        *bit = x.bit(i) as u8;
    }
    arr
}
