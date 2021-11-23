use anyhow::{Context, Result};
use std::collections::HashMap;
use std::io::{BufRead, Write};

/// Vocabulary
#[derive(Debug)]
pub struct Vocabulary {
    /// Word to ID map.
    stoi: HashMap<String, u32>,
}

impl Vocabulary {
    /// Creates a new Vocabulary.
    pub fn new() -> Self {
        Vocabulary {
            stoi: HashMap::new(),
        }
    }

    /// Saves vocabulary into file.
    ///
    /// # File format
    ///
    /// Data is stored as a text format.
    ///
    /// - Line 1: size: u32
    /// - Line 2: word[0]: String
    /// - Line 3: word[1]: String
    /// - ...
    /// - Line size + 1: word[size - 1]: String
    pub fn save(&self, writer: &mut impl Write) -> Result<()> {
        writeln!(writer, "{}", self.len())?;

        let mut kv: Vec<_> = self.stoi.iter().collect();
        kv.sort_by(|a, b| a.1.cmp(b.1));
        for word in kv.iter().map(|x| x.0) {
            writeln!(writer, "{}", word)?;
        }

        Ok(())
    }

    /// Obtains word ID.
    /// If the vocabulary don't have an entry for a given word,
    /// this function inserts a new entry.
    pub fn get_or_add_id(&mut self, word: &str) -> u32 {
        // NOTE(odashi): not using `try_insert()` to prevent unnecessary copy of `word`.
        match self.stoi.get(word) {
            Some(&id) => id,
            None => {
                let id = self.stoi.len() as u32;
                self.stoi.insert(word.into(), id);
                id
            }
        }
    }

    /// Obtains vocabulary size.
    pub fn len(&self) -> usize {
        self.stoi.len()
    }
}

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
