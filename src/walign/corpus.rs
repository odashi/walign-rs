use crate::vocabulary::Vocabulary;
use anyhow::{Context, Result};
use std::io::BufRead;

/// Word ID.
pub type WordID = u32;

/// Sentence.
pub struct Sentence {
    /// List of word IDs.
    pub words: Vec<WordID>,
}

/// Snetence pair.
pub struct SentencePair {
    /// Source sentence.
    pub source: Sentence,

    /// Target sentence.
    pub target: Sentence,
}

/// Loads parallel corpus and vocabularies from fast-align format file.
///
/// # Returns
///
/// Tuple of following values:
///
/// - source_vocab: `Vocabulary`
/// - target_vocab: `Vocabulary`
/// - corpus: `Vec<SentencePair>`
pub fn load(
    reader: impl BufRead,
) -> Result<(Vocabulary, Vocabulary, Vec<SentencePair>)> {
    const SEPARATOR: &'static str = "|||";

    let mut source_vocab = Vocabulary::new();
    let mut target_vocab = Vocabulary::new();
    let mut corpus = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.context("Some input error occurred.")?;
        let words: Vec<String> =
            line.split_whitespace().map(|w| w.into()).collect();
        let sep_index = words.iter().position(|w| w == SEPARATOR).context(
            format!("Separator \"|||\" not found in line {}", i + 1),
        )?;
        let source = Sentence { words: words[..sep_index]
            .iter()
            .map(|w| source_vocab.get_or_add_id(w))
            .collect()};
        let target = Sentence { words: words[sep_index + 1..]
            .iter()
            .map(|w| target_vocab.get_or_add_id(w))
            .collect()};
        corpus.push(SentencePair { source, target });
    }

    Ok((source_vocab, target_vocab, corpus))
}
