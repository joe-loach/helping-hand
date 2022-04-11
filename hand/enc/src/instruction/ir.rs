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
