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

/// Alignment list.
pub struct AlignmentList {
    /// Alignments.
    pub alignments: Vec<Alignment>,
}

impl AlignmentList {
    /// Creates a new AlignmentList.
    pub fn new(alignments: Vec<Alignment>) -> Self {
        Self { alignments }
    }
}

impl fmt::Display for AlignmentList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.alignments.len() > 0 {
            self.alignments[0].fmt(f)?;
            for a in self.alignments.iter().skip(1) {
                write!(f, " ")?;
                a.fmt(f)?;
            }
        }
        Ok(())
    }
}
