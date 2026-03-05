use crate::data::fasta::fasta;
use crate::{
    args::{Args, SubCommand},
    errors::AppError,
};

pub fn dispatch(args: Args) -> Result<(), AppError> {
    match args.command {
        SubCommand::Fasta { file } => fasta::parse_fasta(file, args.global_opts.outfile)?,
        SubCommand::Fastq { file } => unimplemented!(""),
        #[cfg(feature = "bam")]
        SubCommand::Bam { file } => unimplemented!(""),
    }
    Ok(())
}
