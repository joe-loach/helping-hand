mod node;
mod token;

mod validation;

use node::*;
use token::*;

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

pub(crate) mod support {
    use crate::{Node, Token};
    use syntax::SyntaxNode;

    pub(super) fn children<N: Node>(parent: &SyntaxNode) -> impl Iterator<Item = N> {
        parent.children().filter_map(Node::cast)
    }

    pub(super) fn child<N: Node>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(Node::cast)
    }

    pub(super) fn token<T: Token>(parent: &SyntaxNode) -> Option<T> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find_map(Token::cast)
    }
}

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
            let stmt_text_sl = stmt_text.replace(['\n', '\r'], "");
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
            print!(
                "#{}{}",
                if imm.sign().is_positive() { "" } else { "-" },
            imm.value()
            )
    }
}
