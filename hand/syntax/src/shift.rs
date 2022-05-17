#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shift {
    LSL = 0b00,
    LSR = 0b01,
    ASR = 0b10,
    ROR = 0b11,
    RRX,
}

impl Shift {
    pub fn value(&self) -> u32 {
        use Shift::*;
        match self {
            LSL => 0b00,
            LSR => 0b01,
            ASR => 0b10,
            ROR | RRX => 0b11,
        }
    }
}