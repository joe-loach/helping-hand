#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressKind {
    PostInc = 0b00,
    Offset = 0b10,
    PreInc = 0b11,
}

impl AddressKind {
    pub fn p(&self) -> bool {
        matches!(self, AddressKind::Offset | AddressKind::PreInc)
    }

    pub fn w(&self) -> bool {
        matches!(self, AddressKind::PreInc)
    }

    /// Returns `true` if the address kind is [`PostInc`].
    ///
    /// [`PostInc`]: AddressKind::PostInc
    #[must_use]
    pub fn is_post_inc(&self) -> bool {
        matches!(self, Self::PostInc)
    }

    /// Returns `true` if the address kind is [`Offset`].
    ///
    /// [`Offset`]: AddressKind::Offset
    #[must_use]
    pub fn is_offset(&self) -> bool {
        matches!(self, Self::Offset)
    }

    /// Returns `true` if the address kind is [`PreInc`].
    ///
    /// [`PreInc`]: AddressKind::PreInc
    #[must_use]
    pub fn is_pre_inc(&self) -> bool {
        matches!(self, Self::PreInc)
    }
}
