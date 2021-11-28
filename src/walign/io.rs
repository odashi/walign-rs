use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Result, Write};
use std::path::Path;

/// Trait to provide functionality to load things from reader.
pub trait Load: Sized {
    /// Loads things from reader.
    fn load(reader: &mut impl BufRead) -> Result<Self>;

    /// Loads things from file specified as a path.
    fn load_from_path(path: impl AsRef<Path>) -> Result<Self> {
        Self::load(&mut BufReader::new(File::open(path)?))
    }
}

/// Trait to provide functionality to save things into writer.
pub trait Save {
    /// Saves things into writer.
    fn save(&self, writer: &mut impl Write) -> Result<()>;

    /// Saves things into file specified as a path.
    fn save_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.save(&mut BufWriter::new(File::create(path)?))
    }
}
