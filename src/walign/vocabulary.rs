use crate::corpus::WordId;
use std::collections::HashMap;
use std::io::Write;

/// Vocabulary
#[derive(Debug)]
pub struct Vocabulary {
    /// Word to ID map.
    stoi: HashMap<String, WordId>,
}

impl Vocabulary {
    /// Creates a new Vocabulary.
    pub fn new() -> Self {
        Vocabulary {
            stoi: HashMap::new(),
        }
    }

    /// Obtains word ID.
    /// If the vocabulary don't have an entry for a given word,
    /// this function inserts a new entry.
    pub fn get_or_add_id(&mut self, word: &str) -> WordId {
        // NOTE(odashi): not using `try_insert()` to prevent unnecessary copy of `word`.
        match self.stoi.get(word) {
            Some(&id) => id,
            None => {
                let id = WordId::new(self.stoi.len() as u32);
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

impl crate::io::Save for Vocabulary {
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
    fn save(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writeln!(writer, "{}", self.len())?;

        let mut kv: Vec<_> = self.stoi.iter().collect();
        kv.sort_by(|a, b| (&a.1.id).cmp(&b.1.id));
        for word in kv.iter().map(|x| x.0) {
            writeln!(writer, "{}", word)?;
        }

        Ok(())
    }
}
