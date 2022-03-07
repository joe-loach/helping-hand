use super::*;

node! { Shift(SHIFT) }

pub enum ShiftData {
    Register(Register),
    Immediate(Immediate),
}

impl Shift {
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
