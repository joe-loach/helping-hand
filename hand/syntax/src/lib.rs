mod cond;
mod kind;
mod ops;
mod reg;

pub use cond::*;
pub use kind::*;
pub use ops::*;
pub use reg::*;

pub extern crate rowan;

// TODO: DIRECTIVES IMPL
// https://developer.arm.com/documentation/101754/0617/armclang-Reference/armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0742/k/Migrating-from-armasm-to-the-armclang-Integrated-Assembler/Data-definition-directives?lang=en
// https://developer.arm.com/documentation/dui0802/b/Directives-Reference/Alphabetical-list-of-directives

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Hand {}

impl rowan::Language for Hand {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < SyntaxKind::LAST as u16);
        unsafe { core::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<Hand>;
pub type SyntaxToken = rowan::SyntaxToken<Hand>;
pub type SyntaxElement = rowan::SyntaxElement<Hand>;
