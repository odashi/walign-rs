use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let reader = BufReader::new(File::open(opt.input)?);
    let (source_vocab, target_vocab, corpus) = walign::corpus::load(reader)?;
    let model = walign::ibm_model_1::Model::train(
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

    for pair in corpus {
        println!("{}", model.make_viterbi_alignment(&pair));
    }

    Ok(())
}
