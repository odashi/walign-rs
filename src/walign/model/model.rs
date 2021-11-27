use crate::alignment::Alignment;
use crate::corpus::SentencePair;

/// Common interface of models.
pub trait Model {
    /// Saves a model to file.
    /// File format is implementation dependent.
    fn save(&self, writer: &mut impl std::io::Write) -> std::io::Result<()>;

    /// Generates Viterbi alignment for given sentence pair.
    fn make_viterbi_alignment(&self, pair: &SentencePair) -> Alignment;
}
