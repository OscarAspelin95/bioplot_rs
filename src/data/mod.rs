pub mod fasta;
pub mod fastq;
mod utils;

#[cfg(feature = "bam")]
pub mod bam;

use crate::data::{fasta::fasta_dispatch, fastq::fastq_dispatch};
use crate::{
    args::{Args, SubCommand},
    errors::AppError,
};

pub fn dispatch(args: Args) -> Result<(), AppError> {
    match args.command {
        SubCommand::Fasta { files } => fasta_dispatch(files, args.global_opts.outfile)?,
        SubCommand::Fastq { files } => fastq_dispatch(files, args.global_opts.outfile)?,
        #[cfg(feature = "bam")]
        SubCommand::Bam { files } => unimplemented!("BAM support not available yet."),
    }
    Ok(())
}
