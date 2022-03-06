mod validation;

use syntax::SyntaxKind::{self, *};
use syntax::SyntaxNode;
use syntax::SyntaxToken;

pub trait Node: Sized {
    fn node(&self) -> &SyntaxNode;
    fn cast(node: SyntaxNode) -> Option<Self>;
}

node! { Root(ROOT) }
node! { Program(PROGRAM) }
node! { Statement(STATEMENT) }
node! { Instruction(INSTR) }

node! { ArgList(ARG_LIST) }
node! { Arg(ARG) }

pub enum ArgKind {
    Register(Register),
    Shift(Shift),
    Label(Label),
    Immediate(Immediate),
    Address(Address),
    RegList(RegList),
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
        }
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        let res = match node.kind() {
            REGISTER => ArgKind::Register(Register(node)),
            SHIFT => ArgKind::Shift(Shift(node)),
            LABEL => ArgKind::Label(Label(node)),
            IMMEDIATE => ArgKind::Immediate(Immediate(node)),
            ADDRESS => ArgKind::Address(Address(node)),
            REG_LIST => ArgKind::RegList(RegList(node)),
            _ => return None,
        };
        Some(res)
    }
}

node! { RegList(REG_LIST) }
node! { Register(REGISTER) }

node! { Op(OP) }
node! { Shift(SHIFT) }
node! { Immediate(IMMEDIATE) }
node! { Address(ADDRESS) }
node! { Offset(OFFSET) }

node! { Label(LABEL) }
node! { Name(NAME) }
node! { Sign(SIGN) }

pub fn ast(parse: parser::Parse) -> Root {
    Root(parse.syntax())
}

impl Root {
    pub fn program(&self) -> Program {
        self.node().first_child().and_then(Node::cast).unwrap()
    }

    pub fn validate(&self) {
        todo!()
    }
}

impl Program {
    pub fn statements(&self) -> impl Iterator<Item = Statement> {
        children(self.node())
    }
}

pub enum StmtKind {
    Label(Label),
    Instruction(Instruction),
    Both {
        label: Label,
        instruction: Instruction,
    },
}

impl Statement {
    pub fn kind(&self) -> StmtKind {
        match (self.label(), self.instruction()) {
            (Some(label), None) => StmtKind::Label(label),
            (None, Some(instruction)) => StmtKind::Instruction(instruction),
            (Some(label), Some(instruction)) => StmtKind::Both { label, instruction },
            (None, None) => unreachable!(),
        }
    }

    pub fn label(&self) -> Option<Label> {
        child(self.node())
    }

    pub fn instruction(&self) -> Option<Instruction> {
        child(self.node())
    }
}

impl Instruction {
    pub fn op(&self) -> Op {
        child(self.node()).unwrap()
    }

    pub fn args(&self) -> impl Iterator<Item = Arg> {
        self.node()
            .children()
            .find_map(ArgList::cast)
            .unwrap()
            .iter()
    }
}

impl Op {
    pub fn code(&self) -> syntax::Opcode {
        token(self.node(), OPCODE).unwrap().text().parse().unwrap()
    }

    pub fn condition(&self) -> syntax::Condition {
        token(self.node(), COND)
            .and_then(|t| t.text().parse().ok())
            .unwrap_or(syntax::Condition::AL)
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

impl RegList {
    pub fn iter(&self) -> impl Iterator<Item = Register> {
        children(self.node())
    }
}

impl Register {
    pub fn has_bang(&self) -> bool {
        token(self.node(), BANG).is_some()
    }

    pub fn syntax(&self) -> syntax::Register {
        self.node()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind().is_register())
            .and_then(|t| t.text().parse().ok())
            .unwrap()
    }
}

impl Shift {
    pub fn op(&self) -> Op {
        child(self.node()).unwrap()
    }
}

impl Label {
    pub fn name(&self) -> Name {
        child(self.node()).unwrap()
    }
}

impl Immediate {
    pub fn sign(&self) -> Sign {
        child(self.node()).unwrap()
    }

    pub fn value(&self) -> Option<u64> {
        token(self.node(), LITERAL).and_then(|t| {
            t.text()
                .strip_prefix("0x")
                .map(|number| number.parse().unwrap())
        })
    }
}

impl Address {
    pub fn register(&self) -> Register {
        child(self.node()).unwrap()
    }
    pub fn offset(&self) -> Offset {
        child(self.node()).unwrap()
    }
}

impl Offset {}

impl Name {
    pub fn text(&self) -> String {
        token(self.node(), IDENT).unwrap().text().to_owned()
    }
}

impl Sign {
    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_negative(&self) -> bool {
        token(self.node(), MINUS).is_some()
    }
}

fn children<N: Node>(parent: &SyntaxNode) -> impl Iterator<Item = N> {
    parent.children().filter_map(Node::cast)
}

fn child<N: Node>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(Node::cast)
}

fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    parent
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find(|it| it.kind() == kind)
}

macro_rules! node {
    ($name:ident ($kind:ident)) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(SyntaxNode);

        impl Node for $name {
            fn node(&self) -> &SyntaxNode {
                &self.0
            }

            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

pub(self) use node;

#[test]
fn api() {
    let s = lexer::lex(
        r##"
        /* A HAND program.
         * /* Nested comments */ are supported
         */

        start:
        STMDBAL SP!, {R0, R1}   ; save r0 and r1 (equivalent to PUSH {R0, R1})
        ; operator case doesn't matter either
        cmp r0, r1      // compare r0 and r1
        // hex base is supported
        ADDEQ r0, #0x1  // if r0 == r1: r0 += 1
        BEQ end         // if r0 == r1: goto end
        MUL r1, r0, #3  // r1 = r0 * 3
        ; pop the saved registers
        end
        POP {R0, R1}

        adr r2, data
        ldrb r2, [r2, #2]
        mov r3, r2, LSL #1

        "##,
    );
    let p = parser::parse(&s);

    let ast = ast(p);
    let stmts = ast.program().statements();
    for s in stmts {
        match s.kind() {
            StmtKind::Label(l) => print_label(l),
            StmtKind::Instruction(i) => print_instr(i),
            StmtKind::Both { label, instruction } => {
                print_label(label);
                print_instr(instruction);
            }
        }
        println!(";");
    }

    fn print_label(l: Label) {
        print!("{}: ", l.name().text())
    }

    fn print_instr(i: Instruction) {
        let op = i.op();
        print!("{:?}", op.code());
        match op.condition() {
            syntax::Condition::AL => (),
            cond => print!("{:?}", cond),
        }
        for a in i.args() {
            print!(" ");
            print_arg(a.kind());
        }
    }

    fn print_arg(a: ArgKind) {
        match a {
            ArgKind::Register(reg) => {
                print_register(reg);
            }
            ArgKind::Shift(sft) => {
                println!("{:?}", sft.op().code());
            },
            ArgKind::Label(lbl) => {
                print!("{}", lbl.name().text())
            }
            ArgKind::Immediate(imm) => {
                if let Some(value) = imm.value() {
                    println!(
                        "{}{}",
                        if imm.sign().is_positive() { "" } else { "-" },
                        value
                    )
                }
            }
            ArgKind::Address(adr) => {
                print!("[ ");
                print_register(adr.register());
                print!(" ]");
            },
            ArgKind::RegList(reg_list) => {
                print!("{{ ");
                for reg in reg_list.iter() {
                    print_register(reg);
                    print!(", ")
                }
                print!(" }}");
            }
        }
    }

    fn print_register(r: Register) {
        print!("{:?}{}", r.syntax(), if r.has_bang() { "!" } else { "" });
    }
}
