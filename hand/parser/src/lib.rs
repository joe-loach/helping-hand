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
            StrStep::Error { msg, .. } => errors.push(msg),
        });
        (builder.finish(), errors)
    };

    Parse { node, errors }
}
