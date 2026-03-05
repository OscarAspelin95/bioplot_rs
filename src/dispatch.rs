use crate::data::{fasta::fasta, fastq::fastq};
use crate::{
    args::{Args, SubCommand},
    errors::AppError,
};

pub fn dispatch(args: Args) -> Result<(), AppError> {
    match args.command {
        SubCommand::Fasta { files } => fasta::parse_fasta(files, args.global_opts.outfile)?,
        SubCommand::Fastq { files } => fastq::parse_fastq(files, args.global_opts.outfile)?,
        #[cfg(feature = "bam")]
        SubCommand::Bam { file } => unimplemented!(""),
    }
    Ok(())
}
