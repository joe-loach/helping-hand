use super::*;

node! { ArgList(ARG_LIST) }
node! { Arg(ARG) }

pub enum ArgKind {
    Register(Register),
    Shift(Shift),
    Label(Label),
    Immediate(Immediate),
    Address(Address),
    RegList(RegList),
    Literal(Literal),
}

impl Node for ArgKind {
    fn node(&self) -> &SyntaxNode {
        use ArgKind::*;
        match self {
            Register(node) => node.node(),
            Shift(node) => node.node(),
            Label(node) => node.node(),
            Immediate(node) => node.node(),
            Address(node) => node.node(),
            RegList(node) => node.node(),
            Literal(node) => node.node(),
        }
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            REGISTER => ArgKind::Register(Register(node)),
            SHIFT => ArgKind::Shift(Shift(node)),
            LABEL => ArgKind::Label(Label(node)),
            IMMEDIATE => ArgKind::Immediate(Immediate(node)),
            ADDR_OFF | ADDR_POST | ADDR_PRE => ArgKind::Address(Address(node)),
            REG_LIST => ArgKind::RegList(RegList(node)),
            LITERAL => ArgKind::Literal(Literal(node)),
            _ => return None,
        };
        Some(res)
    }
}

impl ArgList {
    pub(crate) fn iter(&self) -> impl Iterator<Item = Arg> {
        children(self.node())
    }
}

impl Arg {
    pub fn kind(&self) -> ArgKind {
        child(self.node()).unwrap()
    }
}
