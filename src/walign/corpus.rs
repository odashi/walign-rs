use crate::vocabulary::Vocabulary;
use std::io::{BufRead, Error, ErrorKind, Result};

/// Word ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WordId {
    /// Value of ID.
    pub id: u32,
}

impl WordId {
    /// Creates a new WordId object.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Sentence.
#[derive(Debug)]
pub struct Sentence {
    /// List of word IDs.
    pub words: Vec<WordId>,
}

/// Snetence pair.
#[derive(Debug)]
pub struct SentencePair {
    /// Source sentence.
    pub source: Sentence,

    /// Target sentence.
    pub target: Sentence,
}

/// Corpus.
#[derive(Debug)]
pub struct Corpus {
    /// Vocabulary in the source language associated to this corpus.
    pub source_vocab: Vocabulary,

    /// Vocabulary in the target language associated to this corpus.
    pub target_vocab: Vocabulary,

    /// List of sentence pairs.
    pub pairs: Vec<SentencePair>,
}

impl crate::io::Load for Corpus {
    /// Loads parallel corpus and vocabularies from fast-align format file.
    ///
    /// # Returns
    ///
    /// Tuple of following values:
    ///
    /// - source_vocab: `Vocabulary`
    /// - target_vocab: `Vocabulary`
    /// - corpus: `Vec<SentencePair>`
    fn load(reader: &mut impl BufRead) -> Result<Self> {
        const SEPARATOR: &'static str = "|||";

        let mut source_vocab = Vocabulary::new();
        let mut target_vocab = Vocabulary::new();
        let mut pairs = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let words: Vec<String> =
                line?.split_whitespace().map(|w| w.into()).collect();
            let sep_index =
                words.iter().position(|w| w == SEPARATOR).ok_or(Error::new(
                    ErrorKind::InvalidData,
                    format!("Separator \"|||\" not found in line {}", i + 1),
                ))?;
            let source = Sentence {
                words: words[..sep_index]
                    .iter()
                    .map(|w| source_vocab.get_or_add_id(w))
                    .collect(),
            };
            let target = Sentence {
                words: words[sep_index + 1..]
                    .iter()
                    .map(|w| target_vocab.get_or_add_id(w))
                    .collect(),
            };
            pairs.push(SentencePair { source, target });
        }

        Ok(Self {
            source_vocab,
            target_vocab,
            pairs,
        })
    }
}
