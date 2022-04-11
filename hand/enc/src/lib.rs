mod instruction;

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
    use middle::Atom::*;

    let mut binary = Binary::new();

    for stmt in ir.iter() {
        let mut cursor = stmt.cursor();

        let _pos = cursor.bump(Label);

        let enc = if let Some(op) = cursor.eat(Instruction) {
            if let Some(instr) = instruction::encode(&mut cursor, op) {
                instr
            } else {
                // TODO: Errors
                u32::MAX
            }
        } else {
            0
        };

        binary.push(enc);
    }

    binary
}
