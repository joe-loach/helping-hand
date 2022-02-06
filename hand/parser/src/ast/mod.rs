use syntax::{SyntaxKind::*, SyntaxNode};

pub trait AstNode: Sized {
    fn syntax(&self) -> &SyntaxNode;
    fn cast(node: SyntaxNode) -> Option<Self>;
}

macro_rules! ast_node {
    ($ast:ident, $kind:ident) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $ast(SyntaxNode);
        impl AstNode for $ast {
            fn syntax(&self) -> &SyntaxNode {
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

ast_node!(Root, ROOT);
ast_node!(Program, PROGRAM);
ast_node!(Statement, STATEMENT);
ast_node!(Instruction, INSTR);
