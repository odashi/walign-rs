use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::BufRead;

/// Parallel corpus.
#[derive(Debug)]
pub struct Corpus {
    /// Source vocabulary.
    pub source_vocab: HashMap<String, u32>,

    /// Target vocabulary.
    pub target_vocab: HashMap<String, u32>,

    /// Source sentences.
    /// Every element should be in `0..source_vocab.len()`.
    pub source_sents: Vec<Vec<u32>>,

    /// Target sentences.
    /// Every element should be in `0..target_vocab.len()`.
    pub target_sents: Vec<Vec<u32>>,
}

/// Helper function to obtain word ID.
/// If the vocabulary don't have an entry for a given word, this function inserts a new entry.
fn stoi(word: &str, vocab: &mut HashMap<String, u32>) -> u32 {
    match vocab.get(word) {
        Some(&id) => id,
        None => {
            let id = vocab.len() as u32;
            vocab.insert(word.into(), id);
            id
        }
    }
}

impl Corpus {
    /// Load corpus data from fast-align format file.
    pub fn load(reader: impl BufRead) -> Result<Self> {
        const SEPARATOR: &'static str = "|||";

        let mut source_vocab = HashMap::new();
        let mut target_vocab = HashMap::new();
        let mut source_sents = Vec::new();
        let mut target_sents = Vec::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line.context("Some input error occurred.")?;
            let words: Vec<String> = line.split_whitespace().map(|w| w.into()).collect();
            let sep_index = words
                .iter()
                .position(|w| w == SEPARATOR)
                .context(format!("Separator \"|||\" not found in line {}", i + 1))?;
            source_sents.push(
                words[..sep_index]
                    .iter()
                    .map(|w| stoi(w, &mut source_vocab))
                    .collect(),
            );
            target_sents.push(
                words[sep_index + 1..]
                    .iter()
                    .map(|w| stoi(w, &mut target_vocab))
                    .collect(),
            )
        }

        Ok(Self {
            source_vocab,
            target_vocab,
            source_sents,
            target_sents,
        })
    }
}
