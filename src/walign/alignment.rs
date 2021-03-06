use crate::corpus::Corpus;
use crate::model::Model;
use std::fmt;
use std::io::Write;

/// Alignment Position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    /// Value of position.
    pub id: u32,
}

impl Position {
    /// Creates a new Position object.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Alignment edge.
#[derive(Debug)]
pub struct Edge {
    /// Source word position (0-origin).
    pub source: Position,

    /// Target word position (0-origin).
    pub target: Position,
}

impl Edge {
    /// Creates a new Edge object.
    pub fn new(source: Position, target: Position) -> Self {
        Self { source, target }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.source.id, self.target.id)
    }
}

/// Alignment for a sentence pair.
pub struct Alignment {
    /// Alignment edges.
    pub edges: Vec<Edge>,
}

impl Alignment {
    /// Creates a new Alignment.
    pub fn new(edges: Vec<Edge>) -> Self {
        Self { edges }
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.edges.len() > 0 {
            self.edges[0].fmt(f)?;
            for a in self.edges.iter().skip(1) {
                write!(f, " ")?;
                a.fmt(f)?;
            }
        }
        Ok(())
    }
}

/// Utility class to dump alignments.
pub struct AlignmentGenerator<'c, 'm, M: Model> {
    /// Corpus associated to this generator.
    corpus: &'c Corpus,

    /// Model associated to this generator.
    model: &'m M,
}

impl<'c, 'm, M: Model> AlignmentGenerator<'c, 'm, M> {
    /// Generates new AlignmentGenerator.
    pub fn new(corpus: &'c Corpus, model: &'m M) -> Self {
        Self {
            corpus: &corpus,
            model: &model,
        }
    }
}

impl<'c, 'm, M: Model> crate::io::Save for AlignmentGenerator<'c, 'm, M> {
    /// Generates alignments for each sentence pair and dump it into writer.
    fn save(&self, writer: &mut impl Write) -> std::io::Result<()> {
        for pair in &self.corpus.pairs {
            writeln!(writer, "{}", self.model.make_viterbi_alignment(&pair))?;
        }
        Ok(())
    }
}
