#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    SP,
    LR,
    PC,
}

impl Register {
    pub fn value(&self) -> u32 {
        use Register::*;
        match self {
            R0 => 0b0000,
            R1 => 0b0001,
            R2 => 0b0010,
            R3 => 0b0011,
            R4 => 0b0100,
            R5 => 0b0101,
            R6 => 0b0110,
            R7 => 0b0111,
            R8 => 0b1000,
            R9 => 0b1001,
            R10 => 0b1010,
            R11 => 0b1011,
            R12 => 0b1100,
            R13 | SP => 0b1101,
            R14 | LR => 0b1110,
            R15 | PC => 0b1111,
        }
    }

    #[must_use]
    pub fn to_numbered(self) -> Self {
        match self {
            Register::SP => Register::R13,
            Register::LR => Register::R14,
            Register::PC => Register::R15,
            _ => self
        }
    }
}

impl core::str::FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Register::*;
        let s = s.to_ascii_uppercase();
        let res = match s.as_str() {
            "R0" => R0,
            "R1" => R1,
            "R2" => R2,
            "R3" => R3,
            "R4" => R4,
            "R5" => R5,
            "R6" => R6,
            "R7" => R7,
            "R8" => R8,
            "R9" => R9,
            "R10" => R10,
            "R11" => R11,
            "R12" => R12,
            "R13" => R13,
            "R14" => R14,
            "R15" => R15,
            "SP" => SP,
            "LR" => LR,
            "PC" => PC,
            _ => return Err(()),
        };
        Ok(res)
    }
}

pub const REGISTERS: &[&str] = &[
    "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "R10", "R11", "R12", "R13", "R14",
    "R15", "SP", "LR", "PC",
];
