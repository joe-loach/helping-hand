use middle::AtomKind::*;
use middle::{higher, Cursor};
use syntax::Directive::{self, *};

use crate::{Binary, LabelValue};
use crate::cursor::*;

// https://developer.arm.com/documentation/101754/0617/armclang-Reference/armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0742/k/Migrating-from-armasm-to-the-armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0802/b/Directives-Reference/Alphabetical-list-of-directives

pub(super) fn encode(
    bin: &mut Binary,
    args: &mut Cursor,
    labels: &mut crate::LabelMap,
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
                let (sign, word) = number(args)?;
                if sign.is_negative() {
                    return None;
                }
                Some(word)
            })?;
            bin.push(word);
        }
        SPACE | DEFS => {
            let size = variant(args, |args| {
                let (sign, size) = number(args)?;
                if sign.is_negative() {
                    return None;
                }
                Some(size)
            })?;
            bin.extend_with_n(size as usize, 0);
        }
        EQU => {
            let expr = variant(args, |args| {
                let (sign, expr) = number(args)?;
                if sign.is_negative() {
                    return None;
                }
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
