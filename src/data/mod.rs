pub mod bam;
pub mod fasta;
pub mod fastq;
mod utils;

use crate::data::{bam::bam_dispatch, fasta::fasta_dispatch, fastq::fastq_dispatch};
use crate::{
    args::{Args, SubCommand},
    errors::AppError,
};

pub fn dispatch(args: Args) -> Result<(), AppError> {
    match args.command {
        SubCommand::Fasta { files } => fasta_dispatch(files, args.global_opts.outfile)?,
        SubCommand::Fastq { files } => fastq_dispatch(files, args.global_opts.outfile)?,
        SubCommand::Bam { files } => bam_dispatch(files, args.global_opts.outfile)?,
    }
    Ok(())
}
