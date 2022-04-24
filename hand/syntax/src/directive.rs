directives! {
    ALIGN (args = false); /// Aligns the current location to a word boundary by padding with zeros.
    DCB; /// Allocates one or more bytes of memory, and defines initial runtime contents of the memory.
    DEFB; /// Alias of DCB.
    DCD; /// Allocates one or more words of memory, aligned on four-byte boundaries, and defines the initial runtime contents of the memory.
    DEFW; /// Alias of DCD.
    SPACE; /// Reserves a zeroed block of memory.
    DEFS; /// Alias of SPACE.
    EQU; /// Gives a symbolic name to a numeric constant
}

impl core::str::FromStr for Directive {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        DIRECTIVES
            .iter()
            .find(|(_, text)| *text == s)
            .map(|&(dir, _)| dir)
            .ok_or(())
    }
}

macro_rules! directives {
    (
        $(
            $did:ident $((args = $args:expr))?; $(#[$meta:meta])*
        )*
        $(;)?
    ) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub enum Directive {
            $(
                $(#[$meta])*
                $did,
            )*
        }

        impl Directive {
            pub fn as_str(&self) -> &str {
                match self {
                    $(Directive::$did => stringify!($did),)*
                }
            }

            pub fn has_args(&self) -> bool {
                match self {
                    $(Directive::$did => true $(&& $args)?,)*
                }
            }
        }

        pub const DIRECTIVES: &[(Directive, &str)] = &[
            $((Directive::$did, stringify!($did)),)*
        ];
    };
}

pub(self) use directives;
