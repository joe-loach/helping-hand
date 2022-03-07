mod validation;

use syntax::SyntaxKind::*;
use syntax::SyntaxNode;
use syntax::SyntaxToken;

pub trait Node: Sized {
    fn node(&self) -> &SyntaxNode;
    fn cast(node: SyntaxNode) -> Option<Self>;
}

pub trait Token: Sized {
    fn token(&self) -> &SyntaxToken;
    fn cast(token: SyntaxToken) -> Option<Self>;
    fn text(&self) -> &str {
        self.token().text()
    }
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
            ADDR_OFF | ADDR_POST | ADDR_PRE => ArgKind::Address(Address(node)),
            REG_LIST => ArgKind::RegList(RegList(node)),
            _ => return None,
        };
        Some(res)
    }
}

node! { RegList(REG_LIST) }
node! { Register(REGISTER) }

node! { Op(OP) }

pub enum ShiftData {
    Register(Register),
    Immediate(Immediate),
}

node! { Shift(SHIFT) }
node! { Immediate(IMMEDIATE) }

pub enum AddrKind {
    Offset(AddrOffset),
    PreInc(AddrPre),
    PostInc(AddrPost),
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Address(SyntaxNode);

node! { AddrOffset (ADDR_OFF) }
node! { AddrPost (ADDR_POST) }
node! { AddrPre (ADDR_PRE) }

pub enum OffsetKind {
    Immediate(OffsetImm),
    Register(OffsetReg),
}

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Offset(SyntaxNode);

node! { OffsetImm(OFFSET_IMM) }
node! { OffsetReg(OFFSET_REG) }

node! { Label(LABEL) }
node! { Name(NAME) }
node! { Sign(SIGN) }

tok! { Ident(IDENT) }
tok! { Literal(LITERAL) }
tok! { Opcode(OPCODE) }
tok! { Condition(COND) }

tok! { Colon(COLON) }
tok! { Bang(BANG) }
tok! { Minus(MINUS) }
tok! { Plus(PLUS) }

pub fn ast(parse: parser::Parse) -> Root {
    Root(parse.syntax())
}

impl Root {
    pub fn program(&self) -> Program {
        self.node().first_child().and_then(Node::cast).unwrap()
    }

    pub fn validate(&self) -> Vec<validation::Error> {
        validation::validate(self)
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

    pub fn arg_list(&self) -> Option<ArgList> {
        child(self.node())
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Arg>> {
        self.arg_list().map(|list| list.iter())
    }
}

impl Op {
    pub fn code(&self) -> Opcode {
        token(self.node()).unwrap()
    }

    pub fn condition(&self) -> Option<Condition> {
        token(self.node())
    }
}

impl Opcode {
    pub fn syntax(&self) -> syntax::Opcode {
        self.text().parse().unwrap()
    }
}

impl Condition {
    pub fn syntax(&self) -> syntax::Condition {
        self.text().parse().unwrap()
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
    pub fn bang(&self) -> Option<Bang> {
        token(self.node())
    }

    pub fn token(&self) -> SyntaxToken {
        self.node()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind().is_register())
            .unwrap()
    }

    pub fn syntax(&self) -> syntax::Register {
        self.token().text().parse().unwrap()
    }
}

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

impl Label {
    pub fn name(&self) -> Name {
        child(self.node()).unwrap()
    }

    pub fn colon(&self) -> Option<Colon> {
        token(self.node())
    }
}

impl Immediate {
    pub fn sign(&self) -> Sign {
        child(self.node()).unwrap()
    }

    pub fn literal(&self) -> Literal {
        token(self.node()).unwrap()
    }

    pub fn value(&self) -> Option<u64> {
        let lit = self.literal();
        let number = lit.text().trim_start_matches("0x");
        number.parse().ok()
    }
}

impl Name {
    pub fn ident(&self) -> Ident {
        self.node()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(Token::cast)
            .unwrap()
    }
}

impl Sign {
    pub fn plus(&self) -> Option<Plus> {
        token(self.node())
    }

    pub fn minus(&self) -> Option<Minus> {
        token(self.node())
    }

    pub fn is_positive(&self) -> bool {
        !self.is_negative()
    }

    pub fn is_negative(&self) -> bool {
        self.minus().is_some()
    }
}

fn children<N: Node>(parent: &SyntaxNode) -> impl Iterator<Item = N> {
    parent.children().filter_map(Node::cast)
}

fn child<N: Node>(parent: &SyntaxNode) -> Option<N> {
    parent.children().find_map(Node::cast)
}

fn token<T: Token>(parent: &SyntaxNode) -> Option<T> {
    parent
        .children_with_tokens()
        .filter_map(|it| it.into_token())
        .find_map(Token::cast)
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

macro_rules! tok {
    ($name:ident ($kind:ident)) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(SyntaxToken);

        impl Token for $name {
            fn token(&self) -> &SyntaxToken {
                &self.0
            }

            fn cast(token: SyntaxToken) -> Option<Self> {
                if token.kind() == $kind {
                    Some(Self(token))
                } else {
                    None
                }
            }
        }
    };
}

pub(self) use {node, tok};

#[test]
fn api() {
    let src = r##"
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

    NOP NOP NOP NOP

    labelA:
    labelB:

    adr r2, data:
    LDR     r6, [r2]
    lDrB    r2, [r2, #2]
    LDR     r2, [r2, #4]!
    LDRB    r2, [r2], #4
    mov r3, r2, LSL #1

    POP {R1, R1}

    "##;

    let s = lexer::lex(src);
    let p = parser::parse(&s);

    let ast = ast(p);

    for error in ast.validate() {
        match error.level {
            validation::Level::Error => eprintln!("error: {}", error.msg),
            validation::Level::Warn => eprintln!("warning: {}", error.msg),
        }
        let orig = error.element;
        if let Some(stmt) = orig.ancestors().find_map(Statement::cast) {
            let stmt_text = stmt.node().text().to_string();
            let stmt_text_sl = stmt_text.replace('\n', "").replace('\r', "");
            println!("{}", stmt_text_sl);
            let offset = orig.text_range().start() - stmt.node().text_range().start();
            let offset: usize = offset.try_into().unwrap();
            let offset = offset - (stmt_text.len() - stmt_text_sl.len());
            eprintln!(
                "{}{}",
                " ".repeat(offset),
                "^".repeat(orig.text_range().len().try_into().unwrap()),
            )
        } else {
            panic!("all errors occur inside statements");
        }
    }

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
        println!();
    }

    fn print_label(l: Label) {
        print!("{}: ", l.name().ident().text())
    }

    fn print_instr(i: Instruction) {
        let op = i.op();
        print!("{:?}", op.code().syntax());
        if let Some(cond) = op.condition() {
            print!("{:?}", cond.syntax())
        }
        if let Some(args) = i.args() {
            print!(" ");
            let mut args = args.peekable();
            while let Some(arg) = args.next() {
                print_arg(arg.kind());
                if args.peek().is_some() {
                    print!(", ");
                }
            }
        }
    }

    fn print_arg(kind: ArgKind) {
        match kind {
            ArgKind::Register(reg) => {
                print_register(reg);
            }
            ArgKind::Shift(sft) => {
                print_shift(sft);
            }
            ArgKind::Label(lbl) => {
                print!("{}", lbl.name().ident().text())
            }
            ArgKind::Immediate(imm) => {
                print_imm(imm);
            }
            ArgKind::Address(adr) => {
                print!("[");
                print_register(adr.base());
                match adr.kind() {
                    AddrKind::Offset(a) => {
                        if let Some(offset) = a.offset() {
                            print!(", ");
                            print_offset(offset);
                        }
                        print!("]");
                    }
                    AddrKind::PreInc(a) => {
                        print!(", ");
                        print_offset(a.offset());
                        print!("]!");
                    }
                    AddrKind::PostInc(a) => {
                        print!("], ");
                        print_offset(a.offset());
                    }
                }
            }
            ArgKind::RegList(reg_list) => {
                print!("{{");
                let mut regs = reg_list.iter().peekable();
                while let Some(reg) = regs.next() {
                    print_register(reg);
                    if regs.peek().is_some() {
                        print!(", ");
                    }
                }
                print!("}}");
            }
        }
    }

    fn print_offset(off: Offset) {
        match off.kind() {
            OffsetKind::Immediate(o) => {
                print_imm(o.immediate());
            }
            OffsetKind::Register(o) => {
                print_register(o.base());
                if let Some(sft) = o.shift() {
                    print!(" , ");
                    print_shift(sft);
                }
            }
        }
    }

    fn print_register(reg: Register) {
        print!(
            "{:?}{}",
            reg.syntax(),
            if reg.bang().is_some() { "!" } else { "" }
        );
    }

    fn print_shift(sft: Shift) {
        print!("{} ", sft.op().code().text());
        if let Some(data) = sft.data() {
            match data {
                ShiftData::Register(reg) => print_register(reg),
                ShiftData::Immediate(imm) => print_imm(imm),
            }
        }
    }

    fn print_imm(imm: Immediate) {
        if let Some(value) = imm.value() {
            print!(
                "#{}{}",
                if imm.sign().is_positive() { "" } else { "-" },
                value
            )
        }
    }
}
