use crate::alignment::{Alignment, Edge, Position};
use crate::corpus::{Corpus, SentencePair};
use byteorder::{LittleEndian, WriteBytesExt};
use ndarray::prelude::*;

/// IBM Model 1.
#[derive(Debug)]
pub struct IbmModel1 {
    /// Translation probability from source word `f` to target word `e`.
    /// t_fe[(f, e)] = Pr(e|f)
    /// Shape: (|source_vocab|, |target_vocab|)
    pub t_fe: Array2<f64>,

    /// Translation probability from null word to target word `e`.
    /// t_e0[e] = Pr(e|NULL)
    /// Shape: (|target_vocab|,)
    pub t_0e: Array1<f64>,
}

impl IbmModel1 {
    /// Trains IBM Model 1.
    pub fn train(corpus: &Corpus, iteration: u32) -> Self {
        const EPS: f64 = 1e-30;
        let f_size = corpus.source_vocab.len();
        let e_size = corpus.target_vocab.len();

        eprintln!("Initializing model:");

        // Initializes probabilities with uniform PDF.
        let t_init = 1. / (e_size as f64 + 1.);
        let mut t_fe = Array2::<f64>::zeros((f_size, e_size)) + t_init;
        let mut t_0e = Array1::<f64>::zeros(e_size) + t_init;

        for epoch in 0..iteration {
            eprintln!("Epoch {}:", epoch + 1);

            // Corpus-wide probabilistic counts.
            // c_fe[(f, e)] = count(f, e)
            // c_0e[e]      = count(f=NULL, e)
            // c_f[f]       = count(f)         = sum_e count(f, e)
            // c_0          = count(f=NULL)    = sum_e count(f=NULL, e)
            let mut c_fe = Array2::<f64>::zeros((f_size, e_size));
            let mut c_0e = Array1::<f64>::zeros(e_size);
            let mut c_f = Array1::<f64>::zeros(f_size) + EPS;
            let mut c_0 = EPS;

            // Negative log-likelihood of the current model.
            let mut nll = 0f64;

            for pair in &corpus.pairs {
                let f_words = &pair.source.words;
                let e_words = &pair.target.words;

                // Sentence-wise robabilistic counts for each target word type.
                let mut c_e = Array1::<f64>::zeros(e_size) + EPS;
                // Likelihood of this sentence in terms of current model.
                let mut likelihood = 0f64;

                // Counts all alignment edges.
                for e in e_words.iter().map(|e| e.id as usize) {
                    // Source words.
                    for f in f_words.iter().map(|f| f.id as usize) {
                        let delta = t_fe[(f, e)];
                        c_e[e] += delta;
                        likelihood += delta;
                    }
                    // NULL word.
                    let delta = t_0e[e];
                    c_e[e] += delta;
                    likelihood += delta;
                }

                nll -= likelihood.log2()
                    - e_size as f64 * ((f_size + 1) as f64).log2();

                // Update corpus-wide probabilistic counts.
                for e in e_words.iter().map(|e| e.id as usize) {
                    // Source words.
                    for f in f_words.iter().map(|f| f.id as usize) {
                        let delta = t_fe[(f, e)] / c_e[e];
                        c_fe[(f, e)] += delta;
                        c_f[f] += delta;
                    }
                    // NULL word.
                    let delta = t_0e[e] / c_e[e];
                    c_0e[e] += delta;
                    c_0 += delta;
                }
            }

            eprintln!("nll = {}", nll);

            // Update model.
            for e in 0..e_size {
                for f in 0..f_size {
                    t_fe[(f, e)] = c_fe[(f, e)] / c_f[f];
                }
                t_0e[e] = c_0e[e] / c_0;
            }
        }

        Self { t_fe, t_0e }
    }
}

impl crate::io::Save for IbmModel1 {
    /// # File format
    ///
    /// - `f_size`: `u32`
    /// - `e_size`: `u32`
    /// - `t_fe`: `[f64; f_size * e_size]`
    /// - `t_0e`: `[f64; e_size]`
    ///
    /// All values are of little endian.
    /// `t_fe` and `t_0e` are stored in row-major ascending order.
    ///
    /// - `t_fe`: `[(0, 0)], [(0, 1)]..., [(0, el-1)], [(1, 0)], ...`
    /// - `t_0e`: `[0], [1], ...`
    fn save(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.t_fe.nrows() as u32)?;
        writer.write_u32::<LittleEndian>(self.t_fe.ncols() as u32)?;
        for &val in self.t_fe.iter() {
            writer.write_f64::<LittleEndian>(val)?;
        }
        for &val in self.t_0e.iter() {
            writer.write_f64::<LittleEndian>(val)?;
        }
        Ok(())
    }
}

impl crate::model::Model for IbmModel1 {
    fn make_viterbi_alignment(&self, pair: &SentencePair) -> Alignment {
        let mut edges = vec![];
        let f_words = &pair.source.words;
        let e_words = &pair.target.words;

        for (i, e) in e_words.iter().map(|e| e.id as usize).enumerate() {
            let mut best_f = 0u32;
            let mut best_t = -1f64;

            for (j, f) in f_words.iter().map(|f| f.id as usize).enumerate() {
                let t = self.t_fe[(f, e)];
                if t > best_t {
                    best_f = j as u32;
                    best_t = t;
                }
            }

            if best_t > self.t_0e[e] {
                edges.push(Edge::new(
                    Position::new(best_f),
                    Position::new(i as u32),
                ));
            }
        }

        Alignment::new(edges)
    }
}
