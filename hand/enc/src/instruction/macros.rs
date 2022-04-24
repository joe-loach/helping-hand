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

/// Shortcuts for common closures that eat the IR.
macro_rules! ir {
    ("R") => {
        |args: &mut Cursor| -> Option<u32> { args.eat(Register) }
    };
    ("R R") => {
        |args: &mut Cursor| -> Option<(u32, u32)> {
            let a = args.eat(Register)?;
            let b = args.eat(Register)?;
            Some((a, b))
        }
    };
    ("R R R") => {
        |args: &mut Cursor| -> Option<(u32, u32, u32)> {
            let a = args.eat(Register)?;
            let b = args.eat(Register)?;
            let c = args.eat(Register)?;
            Some((a, b, c))
        }
    };
    ("R R R R") => {
        |args: &mut Cursor| -> Option<(u32, u32, u32, u32)> {
            let a = args.eat(Register)?;
            let b = args.eat(Register)?;
            let c = args.eat(Register)?;
            let d = args.eat(Register)?;
            Some((a, b, c, d))
        }
    };
    ("{R} R") => {
        |args: &mut Cursor| -> Option<(u32, u32)> {
            let a = args.eat(Register)?;
            Some(if let Some(b) = args.eat(Register) {
                (a, b)
            } else {
                (a, a)
            })
        }
    };
    ("{R} R R") => {
        |args: &mut Cursor| -> Option<(u32, u32, u32)> {
            let a = args.eat(Register)?;
            let b = args.eat(Register)?;
            Some(if let Some(c) = args.eat(Register) {
                (a, b, c)
            } else {
                (a, a, b)
            })
        }
    };
    ("+") => {
        |args: &mut Cursor| -> Option<()> { args.eat(Sign).is(sign::POSITIVE) }
    };
}

pub(crate) use ir;
pub(crate) use inst;
