use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use walign::corpus::SentencePair;
use walign::model::Model;

#[derive(Debug, StructOpt)]
#[structopt(name = "walign trainer", about = "Trains word alignment model.")]
struct Opt {
    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Input corpus with fast-align format."
    )]
    input: PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Output prefix of model files."
    )]
    output: PathBuf,

    #[structopt(
        long,
        default_value = "10",
        help = "Number of training epochs."
    )]
    iteration: u32,
}

/// Generates alignments for each sentence pair and dump it to file.
fn save_viterbi_alignments(
    corpus: &[SentencePair],
    model: &impl Model,
    writer: &mut impl Write,
) -> std::io::Result<()> {
    for pair in corpus {
        writeln!(writer, "{}", model.make_viterbi_alignment(&pair))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let reader = BufReader::new(File::open(opt.input)?);
    let (source_vocab, target_vocab, corpus) = walign::corpus::load(reader)?;
    let model = walign::model::IbmModel1::train(
        &source_vocab,
        &target_vocab,
        &corpus,
        opt.iteration,
    );

    macro_rules! save {
        ( $obj:expr, $ext:expr ) => {
            $obj.save(&mut File::create(opt.output.with_extension($ext))?)?
        };
    }

    save!(source_vocab, "source.vocab");
    save!(target_vocab, "target.vocab");
    save!(model, "ibm1");
    save_viterbi_alignments(
        &corpus,
        &model,
        &mut File::create(opt.output.with_extension("viterbi"))?,
    )?;

    Ok(())
}
