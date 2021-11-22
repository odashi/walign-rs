use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;
use walign::corpus::Corpus;

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

    #[structopt(short, long, parse(from_os_str), help = "Output model file.")]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let in_file = File::open(&opt.input)?;
    let reader = BufReader::new(in_file);
    let corpus = Corpus::load(reader).unwrap();

    println!("{:?}", corpus);

    Ok(())
}
