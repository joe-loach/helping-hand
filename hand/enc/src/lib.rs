mod directive;
mod instruction;

// TODO: EQU directive
// the equ directive tells the assembler to define a label's value but emit no binary data

pub struct Binary {
    inner: Vec<u8>,
}

impl Binary {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(crate) fn push(&mut self, data: u32) {
        let bytes = data.to_le_bytes();
        self.inner.extend_from_slice(&bytes);
    }

    pub(crate) fn push_byte(&mut self, byte: u8) {
        self.inner.push(byte);
    }

    pub(crate) fn extend_with_n(&mut self, n: usize, byte: u8) {
        self.inner.extend((0..n).map(|_| byte));
    }

    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

pub fn encode(ir: middle::IR) -> Binary {
    use middle::Atom::*;

    let mut binary = Binary::new();

    for stmt in ir.iter() {
        let mut cursor = stmt.cursor();

        cursor.bump(Label);

        if let Some(op) = cursor.eat(Instruction) {
            let enc = if let Some(instr) = instruction::encode(&mut cursor, op) {
                instr
            } else {
                // TODO: Errors
                u32::MAX
            };
            binary.push(enc);
        } else if let Some(op) = cursor.eat(Directive) {
            if directive::encode(&mut binary, &mut cursor, op).is_none() {
                binary.push(u32::MAX);
            }
        } else {
            binary.push(0);
        };
    }

    binary
}

use middle::Cursor;

pub(crate) fn variant<T>(
    cursor: &mut Cursor,
    f: impl FnOnce(&mut Cursor) -> Option<T>,
) -> Option<T> {
    let c = cursor.checkpoint();
    let res = f(cursor);
    if res.is_none() {
        cursor.rewind(c);
    }
    res
}

pub(crate) trait Is {
    fn is(&self, x: u32) -> Option<()>;
}

impl Is for Option<u32> {
    fn is(&self, x: u32) -> Option<()> {
        if let Some(y) = self {
            if x == *y {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Is for u32 {
    fn is(&self, x: u32) -> Option<()> {
        if *self == x {
            Some(())
        } else {
            None
        }
    }
}
