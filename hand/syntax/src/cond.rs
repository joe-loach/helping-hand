#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    /// Equal (Z == 1)
    EQ = 0b0000,
    /// Not equal (Z == 0)
    NE = 0b0001,
    /// Carry set (C == 1)
    ///
    /// Also called "HS" (unsigned higher or same)
    CS = 0b0010,
    /// Carry clear (C == 0)
    ///
    /// Also called "LO" (unsigned lower)
    CC = 0b0011,
    /// Minus, negative (N == 1)
    MI = 0b0100,
    /// Plus, positive or zero (N == 0)
    PL = 0b0101,
    /// Overflow (V == 1)
    VS = 0b0110,
    /// No overflow (V == 0)
    VC = 0b0111,
    /// Unsigned higher (C == 1 and Z == 0)
    HI = 0b1000,
    /// Unsigned lower or same (C == 0 or Z == 1)
    LS = 0b1001,
    /// Signed greater than or equal (N == V)
    GE = 0b1010,
    /// Signed less than (N != V)
    LT = 0b1011,
    /// Signed greater than (Z == 0 and N == V)
    GT = 0b1100,
    /// Signed less than or equal (Z == 1 or N != V)
    LE = 0b1101,
    /// Always
    AL = 0b1110,
}

impl Condition {
    /// Alias for "CS"
    pub const HS: Self = Self::CS;
    /// Alias for "CC"
    pub const LO: Self = Self::CC;

    /// Returns the byte representing the condition in the lower 4 bits of the byte.
    pub fn value(&self) -> u32 {
        *self as u32
    }

    pub fn as_str(&self) -> &str {
        match self {
            Condition::EQ => "EQ",
            Condition::NE => "NE",
            Condition::CS => "CS",
            Condition::CC => "CC",
            Condition::MI => "MI",
            Condition::PL => "PL",
            Condition::VS => "VS",
            Condition::VC => "VC",
            Condition::HI => "HI",
            Condition::LS => "LS",
            Condition::GE => "GE",
            Condition::LT => "LT",
            Condition::GT => "GT",
            Condition::LE => "LE",
            Condition::AL => "AL",
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Condition::AL
    }
}

impl core::str::FromStr for Condition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cond = match s {
            "EQ" => Condition::EQ,
            "NE" => Condition::NE,
            "CS" => Condition::CS,
            "CC" => Condition::CC,
            "MI" => Condition::MI,
            "PL" => Condition::PL,
            "VS" => Condition::VS,
            "VC" => Condition::VC,
            "HI" => Condition::HI,
            "LS" => Condition::LS,
            "GE" => Condition::GE,
            "LT" => Condition::LT,
            "GT" => Condition::GT,
            "LE" => Condition::LE,
            "HS" => Condition::HS,
            "LO" => Condition::LO,
            "AL" => Condition::AL,
            _ => return Err(()),
        };
        Ok(cond)
    }
}
