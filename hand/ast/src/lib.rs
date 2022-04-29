mod node;
mod token;

mod validation;

pub use node::*;
pub use token::*;

pub use validation::{Error, Level};

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
