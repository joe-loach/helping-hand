pub fn set(x: u32, range: std::ops::Range<u32>, bits: u32) -> u32 {
    let width = range.end - range.start;
    intbits::Bits::with_bits(x, range, bits & !(!0 << width))
}

pub fn get(x: u32, range: std::ops::Range<u32>) -> u32 {
    intbits::Bits::bits(x, range)
}

pub trait ToWord {
    fn word(&self) -> u32;
}

impl ToWord for u32 {
    fn word(&self) -> u32 {
        *self
    }
}

impl ToWord for bool {
    fn word(&self) -> u32 {
        *self as u32
    }
}

impl ToWord for syntax::Register {
    fn word(&self) -> u32 {
        self.value()
    }
}

impl ToWord for syntax::Shift {
    fn word(&self) -> u32 {
        self.value()
    }
}
