mod binary;
mod bits;
mod cursor;
mod directive;
mod instruction;

use crate::binary::Binary;

pub(crate) type LabelMap = std::collections::HashMap<u32, LabelValue>;

#[derive(Debug, Clone, Copy)]
pub enum LabelValue {
    Offset(u32),
    Expr(u32),
}

pub fn encode(ir: middle::IR) -> Vec<u8> {
    use middle::AtomKind::*;

    // label -> value
    let mut labels = LabelMap::new();

    // PASS 1:
    // calculate label offsets
    {
        let mut pos = 0_u32;
        for stmt in ir.iter() {
            let mut cursor = stmt.cursor();

            let lbl = cursor.bump(Label);

            if cursor.eat(Instruction).is_some() {
                // instructions are always a word
                pos += 4;
            } else if let Some(op) = cursor.eat(Directive) {
                // HACK: encode to a fake binary
                let mut test = Binary::new();
                directive::encode(&mut test, &mut cursor, &mut labels, lbl, op);
                let size = test.len() as u32;
                pos += size;
            } else {
                // a label by itself, one word
                pos += 4;
            };

            labels.insert(lbl, LabelValue::Offset(pos));
        }
    }

    // PASS 2:
    // actually encode the binary
    let mut binary = Binary::new();
    for stmt in ir.iter() {
        let mut cursor = stmt.cursor();

        let lbl = cursor.bump(Label);

        if let Some(op) = cursor.eat(Instruction) {
            let enc = if let Some(instr) = instruction::encode(&mut cursor, &labels, lbl, op) {
                instr
            } else {
                // TODO: Errors
                u32::MAX
            };
            binary.push(enc);
        } else if let Some(op) = cursor.eat(Directive) {
            if directive::encode(&mut binary, &mut cursor, &mut labels, lbl, op).is_none() {
                binary.push(u32::MAX);
            }
        } else {
            binary.push(0);
        };
    }

    binary.into_vec()
}
