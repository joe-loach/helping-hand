// TODO: finish
opcodes! {
    ADC,
    ADD,
    ADR,
    B,
    CMP,
    LDR,
    LDRB,
    MOV,
    MUL,
    MVN,
    NOP false,
    POP,
    PUSH,
    ROR,
    RRX,
    STR,
    SUB,
    ASR,
    LSL,
    LSR,
    STMDB,
    STMFD,
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
            $opid:ident $($args:expr)?
        ),*
        $(,)?
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
