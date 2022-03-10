use super::*;

pub enum AddrKind {
    Offset(AddrOffset),
    PreInc(AddrPre),
    PostInc(AddrPost),
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(pub(crate) SyntaxNode);

node! { AddrOffset (ADDR_OFF) }
node! { AddrPost (ADDR_POST) }
node! { AddrPre (ADDR_PRE) }

pub enum OffsetKind {
    Immediate(OffsetImm),
    Register(OffsetReg),
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Offset(pub(crate) SyntaxNode);

node! { OffsetImm(OFFSET_IMM) }
node! { OffsetReg(OFFSET_REG) }

impl Node for Address {
    fn node(&self) -> &SyntaxNode {
        &self.0
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        if AddrOffset::cast(node.clone()).is_some()
            || AddrPre::cast(node.clone()).is_some()
            || AddrPost::cast(node.clone()).is_some()
        {
            Some(Address(node))
        } else {
            None
        }
    }
}

impl Address {
    pub fn kind(&self) -> AddrKind {
        AddrOffset::cast(self.0.clone())
            .map(AddrKind::Offset)
            .or_else(|| AddrPre::cast(self.0.clone()).map(AddrKind::PreInc))
            .or_else(|| AddrPost::cast(self.0.clone()).map(AddrKind::PostInc))
            .unwrap()
    }

    pub fn base(&self) -> Register {
        child(self.node()).unwrap()
    }
}

impl AddrOffset {
    pub fn offset(&self) -> Option<Offset> {
        child(self.node())
    }
}

impl AddrPre {
    pub fn offset(&self) -> Offset {
        child(self.node()).unwrap()
    }
}

impl AddrPost {
    pub fn offset(&self) -> Offset {
        child(self.node()).unwrap()
    }
}

impl Node for Offset {
    fn node(&self) -> &SyntaxNode {
        &self.0
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        if OffsetImm::cast(node.clone()).is_some() || OffsetReg::cast(node.clone()).is_some() {
            Some(Offset(node))
        } else {
            None
        }
    }
}

impl Offset {
    pub fn kind(&self) -> OffsetKind {
        OffsetImm::cast(self.0.clone())
            .map(OffsetKind::Immediate)
            .or_else(|| OffsetReg::cast(self.0.clone()).map(OffsetKind::Register))
            .unwrap()
    }
}

impl OffsetImm {
    pub fn immediate(&self) -> Immediate {
        child(self.node()).unwrap()
    }
}

impl OffsetReg {
    pub fn sign(&self) -> Sign {
        child(self.node()).unwrap()
    }

    pub fn base(&self) -> Register {
        child(self.node()).unwrap()
    }

    pub fn shift(&self) -> Option<Shift> {
        child(self.node())
    }
}
