#[cfg(test)]
mod tests;

mod cond;
mod kind;
mod macros;
mod ops;
mod shift;

pub use cond::*;
pub use kind::*;
pub use ops::*;
pub use shift::*;

pub extern crate rowan;

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
