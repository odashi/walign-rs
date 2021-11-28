use std::fs::File;
use std::io::{BufWriter, Result, Write};
use std::path::Path;

/// Trait to provide functionality to save things into writer.
pub trait Save {
    /// Saves things into writer.
    fn save(&self, writer: &mut impl Write) -> Result<()>;

    /// Saves things into file specified as a path.
    fn save_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        self.save(&mut BufWriter::new(File::create(path)?))
    }
}
