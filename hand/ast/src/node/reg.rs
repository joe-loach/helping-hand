use syntax::SyntaxToken;

use super::*;

node! { RegList(REG_LIST) }
node! { Register(REGISTER) }

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
