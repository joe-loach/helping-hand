opcodes! {
    ADC     ; // Add with carry
    ADCS    ;
    ADD     ; // Add
    ADDS    ;
    ADR     ; // Address of
    AND     ; // Bitwise AND
    ASR     ; // Arithmetic Shift Right
    ASRS    ;
    B       ; // Branch to target address
    BFC     ; // Bit Field Clear
    BFI     ; // Bit Field Insert
    BIC     ; // Bitwise Bit Clear
    BICS    ;
    BL      ; // Branch with Link
    BX      ; // Branch and Exchange
    CLZ     ; // Count Leading Zeros
    CMN     ; // Compare Negative
    CMP     ; // Compare
    EOR     ; // Bitwise Exclusive OR
    EORS    ;
    LDA     ; // Load-Acquire Word
    LDAB    ; // Load-Acquire Byte
    LDAH    ; // Load-Acquire Halfword
    LDM     ; // Load Multiple Increment After
    LDMIA   ; // Load Multiple Increment After
    LDMFD   ; // Load Multiple Full Descending
    LDMDA   ; // Load Multiple Decrement After
    LDMFA   ; // Load Multiple Full Ascending
    LDMDB   ; // Load Multiple Decrement Before
    LDMEA   ; // Load Multiple Empty Ascending
    LDMIB   ; // Load Multiple Increment Before
    LDMED   ; // Load Multiple Empty Descending
    LDR     ; // Load Register
    LDRB    ; // Load Register Byte
    LDRD    ; // Load Register Dual
    LDRH    ; // Load Register Halfword
    LDRSB   ; // Load Register Signed Byte
    LDRSH   ; // Load Register Signed Halfword
    LSL     ; // Logical Shift Left
    LSLS    ;
    LSR     ; // Logical Shift Right
    LSRS    ; // Logical Shift Right
    MLA     ; // Multiply Accumulate
    MLAS    ;
    MLS     ; // Multiply and Subtract
    MOV     ; // Move
    MOVS    ;
    MOVT    ; // Move Top
    MUL     ; // Multiply
    MULS    ;
    MVN     ; // Bitwise NOT
    MVNS    ;
    NOP     ; (args = false) // No Operation
    ORR     ; // Bitwise OR
    OORS    ;
    POP     ; // Pop Multiple Registers from Stack
    PUSH    ; // Push Mutliple Registers to Stack
    RBIT    ; // Reverse Bits
    REV     ; // Byte-Reverse Word
    ROR     ; // Rotate Right
    RORS    ;
    RRX     ; // Rotate Right with Extend
    RRXS    ;
    RSB     ; // Reverse Subtract
    RSBS    ;
    RSC     ; // Reverse Subtract with Carry
    RSCS    ;
    SBC     ; // Subtract with Carry
    SBCS    ;
    SDIV    ; // Signed Divide
    STM     ; // Store Mutliple
    STMIA   ; // Store Mutliple Increment After
    STMEA   ; // Store Mutliple Empty Ascending
    STMDA   ; // Store Multiple Decrement After
    STMED   ; // Store Multiple Empty Descending
    STMDB   ; // Store Multiple Decrement Before
    STMFD   ; // Store Multiple Full Descending
    STMIB   ; // Store Mutliple Increment Before
    STMFA   ; // Store Mutliple Full Ascending
    STR     ; // Store Register
    STRB    ; // Store Register Byte
    STRD    ; // Store Register Dual
    STRH    ; // Store Register Halfword
    SUB     ; // Subtract
    SUBS    ;
    SVC     ; // Supervisor Call
    TEQ     ; // Test Equivalence (Bitwise XOR)
    TST     ; // Test (Bitwise AND)
    UDIV    ; // Unsigned Divide
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
            $opid:ident; $((args = $args:expr))?
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
