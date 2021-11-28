use std::io::{Result, Write};

/// Trait to provide functionality to save things into writer.
pub trait Save {
    fn save(&self, writer: &mut impl Write) -> Result<()>;
}
