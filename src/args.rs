use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: SubCommand,

    #[clap(flatten)]
    pub global_opts: GlobalOpts,
}

#[derive(Debug, clap::Args)]
pub struct GlobalOpts {
    #[clap(short, long, global = true)]
    pub outfile: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Fasta {
        #[clap(short, long)]
        file: PathBuf,
    },

    Fastq {
        #[clap(short, long)]
        file: PathBuf,
    },

    #[cfg(feature = "bam")]
    Bam {
        #[clap(short, long)]
        file: PathBuf,
    },
}
