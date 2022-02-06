pub mod ast;

mod event;
mod grammar;
mod parser;

use event::StrStep;
use lexer::LexedStr;
use parser::{Parser, Source};
use syntax::{
    rowan::{GreenNode, GreenNodeBuilder},
    SyntaxNode,
};

pub struct Parse {
    node: GreenNode,
    errors: Vec<String>,
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
        event::attach_trivia(text, steps, &mut |step| match step {
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
fn it_works() {
    let s = lexer::lex(
        "; Hello world
label add r0, r0, #1 ; adds two numbers together
mov r2, #2
mvn r2, #2",
    );
    let p = parse(&s);
    let syn = p.syntax();
    println!("{syn:#?}");
}
