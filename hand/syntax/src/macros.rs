macro_rules! args {
    (parse; [$($res:expr),*]; , $($tail:tt)*) => {
        args!(parse; [$($res),* ]; $($tail)*)
    };
    (parse; [$($res:expr),*]; { < $arg:ident > } $($tail:tt)*) => {
        args!(parse; [$($res,)* Arg { optional: true, kind: $arg }]; $($tail)*)
    };
    (parse; [$($res:expr),*]; < $arg:ident > $($tail:tt)*) => {
        args!(parse; [$($res,)* Arg { optional: false, kind: $arg }]; $($tail)*)
    };
    (parse; [$($res:expr),*]; ) => {
        [$($res),*]
    };
    ($($t:tt)*) => {
        args!(parse; []; $($t)*)
    };
}
pub(crate) use args;

macro_rules! str_enum {
    (
        $(#[$outer:meta])*
        $vis:vis enum $name:ident {
            $($(#[$inner:meta])* $variant:ident),*
            $(,)?
        }
    ) => {
        $(#[$outer])*
        $vis enum $name {
            $($(#[$inner])* $variant),*
        }

        impl ::core::str::FromStr for $name {
            type Err = $crate::macros::ParseEnumErr;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use $name::*;
                let var = match s {
                    $(stringify!($variant) => $variant,)*
                    _ => return Err($crate::macros::ParseEnumErr),
                };
                Ok(var)
            }
        }
    };
}

#[derive(Debug)]
pub struct ParseEnumErr;
impl ::std::error::Error for ParseEnumErr {}
impl ::std::fmt::Display for ParseEnumErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "string provided didn't match any enum variant")
    }
}

pub(crate) use str_enum;
