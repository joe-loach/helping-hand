use super::*;

node! { Shift(SHIFT) }

pub enum ShiftData {
    Register(Register),
    Immediate(Immediate),
}

impl Shift {
    pub fn syntax(&self) -> syntax::Shift {
        use syntax::Opcode;
        match self.op().code().syntax() {
            Opcode::LSL => syntax::Shift::LSL,
            Opcode::LSR => syntax::Shift::LSR,
            Opcode::ASR => syntax::Shift::ASR,
            Opcode::RRX | Opcode::ROR => syntax::Shift::ROR,
            _ => unreachable!()
        }
    }

    pub fn op(&self) -> Op {
        child(self.node()).unwrap()
    }

    pub fn register(&self) -> Option<Register> {
        child(self.node())
    }

    pub fn immediate(&self) -> Option<Immediate> {
        child(self.node())
    }

    pub fn data(&self) -> Option<ShiftData> {
        self.register()
            .map(ShiftData::Register)
            .or_else(|| self.immediate().map(ShiftData::Immediate))
    }
}
