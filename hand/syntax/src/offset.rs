pub enum OffsetKind {
    Value = 0,
    Register = 1,
}

impl OffsetKind {
    /// Returns `true` if the offset is [`Value`].
    ///
    /// [`Value`]: Offset::Value
    #[must_use]
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value)
    }

    /// Returns `true` if the offset is [`Register`].
    ///
    /// [`Register`]: Offset::Register
    #[must_use]
    pub fn is_register(&self) -> bool {
        matches!(self, Self::Register)
    }
}
