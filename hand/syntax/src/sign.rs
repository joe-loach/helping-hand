#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn as_str(&self) -> &str {
        match self {
            Sign::Positive => "+",
            Sign::Negative => "-",
        }
    }
}