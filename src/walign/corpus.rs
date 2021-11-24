use crate::vocabulary::Vocabulary;
use anyhow::{Context, Result};
use std::io::BufRead;

/// Parallel corpus.
#[derive(Debug)]
pub struct ParallelCorpus {
    /// Source sentences.
    /// Every element should be in `0..source_vocab.len()`.
    pub source_sents: Vec<Vec<u32>>,

    /// Target sentences.
    /// Every element should be in `0..target_vocab.len()`.
    pub target_sents: Vec<Vec<u32>>,
}

/// Loads parallel corpus and vocabularies from fast-align format file.
///
/// # Returns
///
/// Tuple of following values:
///
/// - source_vocab: `Vocabulary`
/// - target_vocab: `Vocabulary`
/// - corpus: `Corpus`
pub fn load(
    reader: impl BufRead,
) -> Result<(Vocabulary, Vocabulary, ParallelCorpus)> {
    const SEPARATOR: &'static str = "|||";

    let mut source_vocab = Vocabulary::new();
    let mut target_vocab = Vocabulary::new();
    let mut source_sents = Vec::new();
    let mut target_sents = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.context("Some input error occurred.")?;
        let words: Vec<String> =
            line.split_whitespace().map(|w| w.into()).collect();
        let sep_index = words.iter().position(|w| w == SEPARATOR).context(
            format!("Separator \"|||\" not found in line {}", i + 1),
        )?;
        source_sents.push(
            words[..sep_index]
                .iter()
                .map(|w| source_vocab.get_or_add_id(w))
                .collect(),
        );
        target_sents.push(
            words[sep_index + 1..]
                .iter()
                .map(|w| target_vocab.get_or_add_id(w))
                .collect(),
        )
    }

    Ok((
        source_vocab,
        target_vocab,
        ParallelCorpus {
            source_sents,
            target_sents,
        },
    ))
}
