#[derive(PartialEq, Eq)]
pub enum Conditional {
    /// Equal (Z == 1)
    EQ = 0b0000,
    /// Not equal (Z == 0)
    NE = 0b0001,
    /// Carry set (C == 1)
    /// Also called "HS" (unsigned higher or same)
    CS = 0b0010,
    /// Carry clear (C == 0)
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

impl Conditional {
    /// Alias for "CS"
    pub const HS: Self = Self::CS;
    /// Alias for "LO"
    pub const LO: Self = Self::CC;
}
