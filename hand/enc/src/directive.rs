use std::collections::HashMap;

use middle::consts::*;
use middle::Atom::*;
use middle::{higher, Cursor};
use syntax::Directive::{self, *};

use crate::{variant, Binary, Is, LabelValue};

// https://developer.arm.com/documentation/101754/0617/armclang-Reference/armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0742/k/Migrating-from-armasm-to-the-armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0802/b/Directives-Reference/Alphabetical-list-of-directives

pub(super) fn encode(
    bin: &mut Binary,
    args: &mut Cursor,
    labels: &mut HashMap<u32, LabelValue>,
    lbl: u32,
    op: u32,
) -> Option<()> {
    let dir = unsafe { higher::<Directive>(op) };

    match dir {
        ALIGN => {
            const WORD_BOUNDARY: usize = 4;
            let bytes_to_align = 4 - (bin.len() % WORD_BOUNDARY);
            for _ in 0..bytes_to_align {
                bin.push_byte(0);
            }
        }
        DCB | DEFB => {
            let bytes = variant(args, |args| {
                Some({
                    let mut data = vec![];
                    while let Some(c) = args.current() {
                        match c {
                            Char | Number => data.push((args.bump(c) & 0xFF) as u8),
                            Sign => {
                                args.bump(Sign);
                            }
                            _ => return None,
                        }
                    }
                    data
                })
            })?;
            bin.extend_with(&bytes);
        }
        DCD | DEFW => {
            let word = variant(args, |args| {
                args.eat(Sign).is(sign::POSITIVE)?;
                let word = args.eat(Number)?;
                Some(word)
            })?;
            bin.push(word);
        }
        SPACE | DEFS => {
            let size = variant(args, |args| {
                args.eat(Sign).is(sign::POSITIVE)?;
                let size = args.eat(Number)?;
                Some(size)
            })?;
            bin.extend_with_n(size as usize, 0);
        }
        EQU => {
            let expr = variant(args, |args| {
                args.eat(Sign).is(sign::POSITIVE)?;
                let expr = args.eat(Number)?;
                Some(expr)
            })?;
            // change the lbl to be a expr value
            labels.insert(lbl, LabelValue::Expr(expr));
        }
    };

    if !args.finished() {
        None
    } else {
        Some(())
    }
}
