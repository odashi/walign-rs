/// Alignment Position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position(pub u32);

/// Alignment.
#[derive(Debug)]
pub struct Alignment {
    /// Source word position (0-origin). None represents null alignment.
    pub source: Option<Position>,

    /// Target word position (0-origin).
    pub target: Position,
}

impl Alignment {
    /// Creates a new Alignment object.
    pub fn new(source: Option<Position>, target: Position) -> Self {
        Self { source, target }
    }
}
