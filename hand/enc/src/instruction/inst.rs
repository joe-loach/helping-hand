/// Produces a 32 bit value from user definied values.
/// 
/// # How it works
/// This macro is an Incremental tt muncher.
/// It builds the expression `x` based on values that you give it.
/// Once the input is exhausted, the value is returned as an Option.
/// If 32 values weren't given to the macro, it will warn you and return None.
macro_rules! inst {
    // empty
    (@inner; $pos:expr; $x:expr;) => {
        if $pos == 32 {
            Some($x)
        } else {
            eprintln!("Instruction '{:032b}' is not 32 bits wide; is {} bits wide", $x, $pos);
            None
        }
    };
    // [id:width] (fill with id for width)
    (@inner; $pos:expr; $x:expr; [$id:ident : $width:expr] $($t:tt)*) => {
        inst!(@inner; $pos + $width; ($x).with_bits((32 - $pos - $width)..(32 - $pos), $id & !(!0 << $width)); $($t)*)
    };
    // 0 (do nothing)
    (@inner; $pos:expr; $x:expr; 0 $($t:tt)*) => {
        inst!(@inner; $pos + 1; $x; $($t)*)
    };
    // 1 (set bit at position)
    (@inner; $pos:expr; $x:expr; 1 $($t:tt)*) => {
        inst!(@inner; $pos + 1; ($x | (1 << (32 - $pos - 1))); $($t)*)
    };
    // id (width = 1)
    (@inner; $pos:expr; $x:expr; $id:ident $($t:tt)*) => {
        inst!(@inner; $pos + 1; ($x | (($id as u32) << (32 - $pos - 1))); $($t)*)
    };
    // | (visual separator)
    (@inner; $pos:expr; $x:expr; | $($t:tt)*) => {
        inst!(@inner; $pos; $x; $($t)*)
    };
    // entrance
    ($($t:tt)*) => {
        inst!(@inner; 0_u32; 0_u32; $($t)*)
    };
}

pub(crate) use inst;
