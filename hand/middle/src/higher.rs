use core::mem;

/// # Safety
/// We know how the conversions to u32 are made,
/// so we can convert them back.
pub unsafe trait FromRaw {
    unsafe fn higher(x: u32) -> Self;
}

pub fn syn<T: FromRaw>(x: u32) -> T {
    unsafe { FromRaw::higher(x) }
}

unsafe impl FromRaw for syntax::Opcode {
    unsafe fn higher(x: u32) -> Self {
        debug_assert!(x < syntax::OPCODES.len() as u32);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Condition {
    unsafe fn higher(x: u32) -> Self {
        debug_assert!(x < 0b1111);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Sign {
    unsafe fn higher(x: u32) -> Self {
        debug_assert!(x <= 1);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Register {
    unsafe fn higher(x: u32) -> Self {
        debug_assert!(x <= 0b1111);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::RegisterList {
    unsafe fn higher(x: u32) -> Self {
        debug_assert!(x <= u16::MAX as u32);
        mem::transmute(x as u16)
    }
}