use core::mem;

/// # Safety
/// We know how the conversions to u32 are made,
/// so we can convert them back.
pub unsafe trait FromRaw {
    /// # Safety
    /// Transmutations are used internally,
    /// use asserts to make sure that everything works correctly.
    unsafe fn from(x: u32) -> Self;
}

/// # Safety
/// This function should only be called if you're sure that `data` was lowered down from `T`.
pub unsafe fn higher<T: FromRaw>(data: u32) -> T {
    FromRaw::from(data)
}

unsafe impl FromRaw for syntax::Directive {
    unsafe fn from(x: u32) -> Self {
        debug_assert!(x < syntax::DIRECTIVES.len() as u32);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Opcode {
    unsafe fn from(x: u32) -> Self {
        debug_assert!(x < syntax::OPCODES.len() as u32);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Condition {
    unsafe fn from(x: u32) -> Self {
        debug_assert!(x < 0b1111);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Sign {
    unsafe fn from(x: u32) -> Self {
        debug_assert!(x <= 1);
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::Register {
    unsafe fn from(x: u32) -> Self {
        let x = x & 0b1111;
        mem::transmute(x as u8)
    }
}

unsafe impl FromRaw for syntax::RegisterList {
    unsafe fn from(x: u32) -> Self {
        debug_assert!(x <= u16::MAX as u32);
        mem::transmute(x as u16)
    }
}
