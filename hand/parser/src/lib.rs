mod grammar;
mod parser;
mod step;

use lexer::LexedStr;
use parser::{Parser, Source};
use step::StrStep;
use syntax::{
    rowan::{GreenNode, GreenNodeBuilder},
    SyntaxNode,
};

pub struct Parse {
    node: GreenNode,
    pub errors: Vec<String>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.node.clone())
    }
}

pub fn parse(text: &LexedStr) -> Parse {
    let steps = {
        let source = Source::new(text);
        let mut parser = Parser::new(source);
        grammar::root(&mut parser);
        parser.finish()
    };

    let (node, errors) = {
        let mut errors = Vec::new();
        let mut builder = GreenNodeBuilder::new();
        step::attach_trivia(text, steps, &mut |step| match step {
            StrStep::Start { kind } => builder.start_node(kind.into()),
            StrStep::Token { kind, text } => builder.token(kind.into(), text),
            StrStep::Finish => builder.finish_node(),
            StrStep::Error { msg, pos: _ } => errors.push(msg),
        });
        (builder.finish(), errors)
    };

    Parse { node, errors }
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
    let p = parse(&s);

    let syn = p.syntax();
    println!("{syn:#?}");
}
