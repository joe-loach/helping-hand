#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive = 0,
    Negative = 1,
}

impl Sign {
    pub fn as_str(&self) -> &str {
        match self {
            Sign::Positive => "+",
            Sign::Negative => "-",
        }
    }

    /// Returns `true` if the sign is [`Positive`].
    ///
    /// [`Positive`]: Sign::Positive
    #[must_use]
    pub fn is_positive(&self) -> bool {
        matches!(self, Self::Positive)
    }

    /// Returns `true` if the sign is [`Negative`].
    ///
    /// [`Negative`]: Sign::Negative
    #[must_use]
    pub fn is_negative(&self) -> bool {
        matches!(self, Self::Negative)
    }
}
