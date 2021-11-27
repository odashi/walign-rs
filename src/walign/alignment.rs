use std::fmt;

/// Alignment Position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position(pub u32);

/// Alignment.
#[derive(Debug)]
pub struct Alignment {
    /// Source word position (0-origin).
    pub source: Position,

    /// Target word position (0-origin).
    pub target: Position,
}

impl Alignment {
    /// Creates a new Alignment object.
    pub fn new(source: Position, target: Position) -> Self {
        Self { source, target }
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.source.0, self.target.0)
    }
}
