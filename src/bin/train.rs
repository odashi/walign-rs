use std::path::PathBuf;
use structopt::StructOpt;
use walign::alignment::AlignmentGenerator;
use walign::io::{Load, Save};

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

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let corpus = walign::corpus::Corpus::load_from_path(opt.input)?;
    let model = walign::model::IbmModel1::train(&corpus, opt.iteration);

    macro_rules! save {
        ( $obj:expr, $ext:expr ) => {
            $obj.save_to_path(opt.output.with_extension($ext))?
        };
    }

    save!(corpus.source_vocab, "source.vocab");
    save!(corpus.target_vocab, "target.vocab");
    save!(model, "ibm1");
    save!(AlignmentGenerator::new(&corpus, &model), "viterbi");

    Ok(())
}
