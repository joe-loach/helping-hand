use middle::higher;
use middle::Atom::*;

pub struct Binary {
    inner: Vec<u32>,
}

impl Binary {
    pub(crate) fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub(crate) fn push(&mut self, data: u32) {
        self.inner.push(data);
    }

    pub fn as_words(&self) -> &[u32] {
        &self.inner
    }

    pub fn as_bytes(&self) -> &[u8] {
        let (_, bytes, _) = unsafe { self.inner.align_to() };
        bytes
    }
}

pub fn encode(ir: middle::IR) -> Binary {
    use syntax::Opcode;

    let mut binary = Binary::new();

    for stmt in ir.iter() {
        let mut cursor = stmt.cursor();

        // 0x0000_0000
        let mut enc = 0_u32;

        let _pos = cursor.bump(Label);

        if let Some(op) = cursor.eat(Instruction) {
            let op = higher::<Opcode>(op);
            let cond = cursor.bump(Condition); // 0x0 - 0xF

            let op_base = op.value();

            enc |= cond << (32 - 4);
            enc |= op_base;
        }

        binary.push(enc);
    }

    binary
}
