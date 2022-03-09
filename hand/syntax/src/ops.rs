// TODO: finish
opcodes! {
    ADC     = 0b0000_0000_1010_0000_0000_0000_0000_0000; // Add with carry
    ADCS    = 0b0000_0000_1011_0000_0000_0000_0000_0000;
    ADD     = 0b0000_0000_1000_0000_0000_0000_0000_0000; // Add
    ADDS    = 0b0000_0000_1001_0000_0000_0000_0000_0000;
    ADR     = 0b0000_0000_1000_0000_0000_0000_0000_0000; // Address of
    AND     = 0b0000_0000_0000_0000_0000_0000_0000_0000; // Bitwise AND
    ASR     = 0b0000_0001_1010_0000_0000_0000_0100_0000; // Arithmetic Shift Right
    ASRS    = 0b0000_0001_1011_0000_0000_0000_0100_0000;
    B       = 0b0000_1010_0000_0000_0000_0000_0000_0000; // Branch to target address
    BFC     = 0b0000_0111_1110_0000_0000_0000_0001_1111; // Bit Field Clear
    BFI     = 0b0000_0111_1100_0000_0000_0000_0001_0000; // Bit Field Insert
    BIC     = 0b0000_0001_1100_0000_0000_0000_0000_0000; // Bitwise Bit Clear
    BICS    = 0b0000_0001_1101_0000_0000_0000_0000_0000;
    BL      = 0b0000_1011_0000_0000_0000_0000_0000_0000; // Branch with Link
    BX      = 0b0000_0001_0010_1111_1111_1111_0001_0000; // Branch and Exchange
    CLZ     = 0b0000_0001_0110_1111_0000_1111_0001_0000; // Count Leading Zeros
    CMN     = 0b0000_0001_0111_0000_0000_0000_0000_0000; // Compare Negative
    CMP     = 0b0000_0001_0101_0000_0000_0000_0000_0000; // Compare
    EOR     = 0b0000_0000_0010_0000_0000_0000_0000_0000; // Bitwise Exclusive OR
    EORS    = 0b0000_0000_0011_0000_0000_0000_0000_0000;
    LDA     = 0b0000_0001_1001_0000_0000_1100_1001_1111; // Load-Acquire Word
    LDAB    = 0b0000_0001_1101_0000_0000_1100_1001_1111; // Load-Acquire Byte
    LDAH    = 0b0000_0001_1111_0000_0000_1100_0000_1111; // Load-Acquire Halfword
    LDM     = 0b0000_1000_1001_0000_0000_0000_0000_0000; // Load Multiple Increment After
    LDMIA   = 0b0000_1000_1001_0000_0000_0000_0000_0000; // Load Multiple Increment After
    LDMFD   = 0b0000_1000_1001_0000_0000_0000_0000_0000; // Load Multiple Full Descending
    LDMDA   = 0b0000_1000_0001_0000_0000_0000_0000_0000; // Load Multiple Decrement After
    LDMFA   = 0b0000_1000_0001_0000_0000_0000_0000_0000; // Load Multiple Full Ascending
    LDMDB   = 0b0000_1001_0001_0000_0000_0000_0000_0000; // Load Multiple Decrement Before
    LDMEA   = 0b0000_1001_0001_0000_0000_0000_0000_0000; // Load Multiple Empty Ascending
    LDMIB   = 0b0000_1001_1001_0000_0000_0000_0000_0000; // Load Multiple Increment Before
    LDMED   = 0b0000_1001_1001_0000_0000_0000_0000_0000; // Load Multiple Empty Descending
    LDR     = 0b0000_0100_0001_0000_0000_0000_0000_0000; // Load Register
    LDRB    = 0b0000_0100_0101_0000_0000_0000_0000_0000; // Load Register Byte
    LDRD    = 0b0000_0000_0000_0000_0000_0000_1101_0000; // Load Register Dual
    LDRH    = 0b0000_0000_0001_0000_0000_0000_1011_0000; // Load Register Halfword
    LDRSB   = 0b0000_0000_0001_0000_0000_0000_1101_0000; // Load Register Signed Byte
    LDRSH   = 0b0000_0000_0001_0000_0000_0000_1111_0000; // Load Register Signed Halfword
    LSL     = 0b0000_0001_1010_0000_0000_0000_0000_0000; // Logical Shift Left
    LSLS    = 0b0000_0001_1011_0000_0000_0000_0000_0000;
    LSR     = 0b0000_0001_1010_0000_0000_0000_0010_0000; // Logical Shift Right
    LSRS    = 0b0000_0001_1011_0000_0000_0000_0010_0000; // Logical Shift Right
    MLA     = 0b0000_0000_0010_0000_0000_0000_1001_0000; // Multiply Accumulate
    MLAS    = 0b0000_0000_0011_0000_0000_0000_1001_0000;
    MLS     = 0b0000_0000_0110_0000_0000_0000_1001_0000; // Multiply and Subtract
    MOV     = 0b0000_0001_1010_0000_0000_0000_0000_0000; // Move
    MOVS    = 0b0000_0001_1011_0000_0000_0000_0000_0000;
    MOVT    = 0b0000_0011_0100_0000_0000_0000_0000_0000; // Move Top
    MUL     = 0b0000_0000_0000_0000_0000_0000_1001_0000; // Multiply
    MULS    = 0b0000_0000_0001_0000_0000_0000_1001_0000;
    MVN     = 0b0000_0001_1110_0000_0000_0000_0000_0000; // Bitwise NOT
    MVNS    = 0b0000_0001_1111_0000_0000_0000_0000_0000;
    NOP     = 0b0000_0011_0010_1111_0000_0000_0000_0000; (args = false) // No Operation
    ORR     = 0b0000_0001_1000_0000_0000_0000_0000_0000; // Bitwise OR
    OORS    = 0b0000_0001_1001_0000_0000_0000_0000_0000;
    POP     = 0b0000_0000_1001_1101_0000_0000_0000_0000; // Pop Multiple Registers from Stack
    PUSH    = 0b0000_0001_0010_1101_0000_0000_0000_0000; // Push Mutliple Registers to Stack
    RBIT    = 0b0000_0110_1111_1111_0000_1111_0011_0000; // Reverse Bits
    REV     = 0b0000_0110_1011_1111_0000_1111_0011_0000; // Byte-Reverse Word
    ROR     = 0b0000_0001_1010_0000_0000_0000_0110_0000; // Rotate Right
    RORS    = 0b0000_0001_1011_0000_0000_0000_0110_0000;
    RRX     = 0b0000_0001_1010_0000_0000_0000_0110_0000; // Rotate Right with Extend
    RRXS    = 0b0000_0001_1011_0000_0000_0000_0110_0000;
    RSB     = 0b0000_0000_0110_0000_0000_0000_0000_0000; // Reverse Subtract
    RSBS    = 0b0000_0000_0111_0000_0000_0000_0000_0000;
    RSC     = 0b0000_0000_1110_0000_0000_0000_0000_0000; // Reverse Subtract with Carry
    RSCS    = 0b0000_0000_1111_0000_0000_0000_0000_0000;
    SBC     = 0b0000_0000_1100_0000_0000_0000_0000_0000; // Subtract with Carry
    SBCS    = 0b0000_0000_1101_0000_0000_0000_0000_0000;
    SDIV    = 0b0000_0111_0001_0000_1111_0000_0001_0000; // Signed Divide
    STM     = 0b0000_1000_0000_0000_0000_0000_0000_0000; // Store Mutliple
    STMIA   = 0b0000_1000_0000_0000_0000_0000_0000_0000; // Store Mutliple Increment After
    STMEA   = 0b0000_1000_0000_0000_0000_0000_0000_0000; // Store Mutliple Empty Ascending
    STMDB   = 0b0000_1001_0000_0000_0000_0000_0000_0000; // Store Multiple Decrement Before
    STMFD   = 0b0000_1001_0000_0000_0000_0000_0000_0000; // Store Multiple Full Descending
    STMID   = 0b0000_1001_1000_0000_0000_0000_0000_0000; // Store Mutliple Increment Before
    STMFA   = 0b0000_1001_1000_0000_0000_0000_0000_0000; // Store Mutliple Full Ascending
    STR     = 0b0000_0100_0000_0000_0000_0000_0000_0000; // Store Register
    STRB    = 0b0000_0100_0100_0000_0000_0000_0000_0000; // Store Register Byte
    STRD    = 0b0000_0000_0000_0000_0000_0000_1111_0000; // Store Register Dual
    STRH    = 0b0000_0000_0000_0000_0000_0000_1011_0000; // Store Register Halfword
    SUB     = 0b0000_0000_0100_0000_0000_0000_0000_0000; // Subtract
    SUBS    = 0b0000_0000_0101_0000_0000_0000_0000_0000;
    SVC     = 0b0000_1111_0000_0000_0000_0000_0000_0000; // Supervisor Call
    TEQ     = 0b0000_0001_0011_0000_0000_0000_0000_0000; // Test Equivalence (Bitwise XOR)
    TST     = 0b0000_0001_0001_0000_0000_0000_0000_0000; // Test (Bitwise AND)
    UDIV    = 0b0000_0111_0011_0000_1111_0000_0001_0000; // Unsigned Divide
}

impl core::str::FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        OPCODES
            .iter()
            .find(|(_, text)| *text == s)
            .map(|&(code, _)| code)
            .ok_or(())
    }
}

macro_rules! opcodes {
    (
        $(
            $(#[$meta:meta])*
            $opid:ident = $value:expr; $((args = $args:expr))?
        )*
        $(;)?
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Opcode {
            $(
                $(#[$meta])*
                $opid,
            )*
        }

        impl Opcode {
            pub fn as_str(&self) -> &str {
                match self {
                    $(Opcode::$opid => stringify!($opid),)*
                }
            }

            pub fn value(&self) -> u32 {
                match self {
                    $(Opcode::$opid => $value,)*
                }
            }

            pub fn has_args(&self) -> bool {
                match self {
                    $(Opcode::$opid => true $(&& $args)?,)*
                }
            }
        }

        pub const OPCODES: &[(Opcode, &str)] = &[
            $((Opcode::$opid, stringify!($opid)),)*
        ];
    };
}

pub(self) use opcodes;
