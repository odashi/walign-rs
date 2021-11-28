use crate::alignment::Alignment;
use crate::corpus::SentencePair;

/// Common interface of models.
pub trait Model: crate::io::Save {
    /// Generates Viterbi alignment for given sentence pair.
    fn make_viterbi_alignment(&self, pair: &SentencePair) -> Alignment;
}
