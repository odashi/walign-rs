use std::collections::HashMap;
use std::io::BufRead;

/// Information for errors.
#[derive(Debug)]
pub struct Error {
    /// Error message.
    message: String,
}

/// Parallel corpus.
#[derive(Debug)]
pub struct Corpus {
    /// Source vocabulary.
    source_vocab: HashMap<String, u32>,

    /// Target vocabulary.
    target_vocab: HashMap<String, u32>,

    /// Source sentences.
    source_sents: Vec<Vec<u32>>,

    /// Target sentences.
    target_sents: Vec<Vec<u32>>,
}

/// Helper function to obtain word ID.
/// If the vocabulary don't have an entry for a given word, This function inserts a new entry.
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

/// Load corpus data from fast-align format file.
pub fn load(reader: impl BufRead) -> Result<Corpus, Error> {
    const SEPARATOR: &'static str = "|||";

    let mut source_vocab = HashMap::new();
    let mut target_vocab = HashMap::new();
    let mut source_sents = Vec::new();
    let mut target_sents = Vec::new();

    for (i, line) in reader.lines().map(|x| x.unwrap()).enumerate() {
        let words: Vec<String> = line.split_whitespace().map(|w| w.into()).collect();
        match words.iter().position(|w| w == SEPARATOR) {
            Some(index) => {
                source_sents.push(
                    words[..index]
                        .iter()
                        .map(|w| stoi(w, &mut source_vocab))
                        .collect(),
                );
                target_sents.push(
                    words[index + 1..]
                        .iter()
                        .map(|w| stoi(w, &mut target_vocab))
                        .collect(),
                )
            }
            None => {
                return Err(Error {
                    message: format!("Separator \"|||\" not found in line {}.", i + 1),
                })
            }
        }
    }

    Ok(Corpus {
        source_vocab,
        target_vocab,
        source_sents,
        target_sents,
    })
}
