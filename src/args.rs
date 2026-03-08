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
    pub outfile: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Fasta {
        #[clap(short, long, num_args=1..)]
        files: Vec<PathBuf>,
    },

    Fastq {
        #[clap(short, long, num_args=1..)]
        files: Vec<PathBuf>,
    },

    Bam {
        #[clap(short, long, num_args=1..)]
        files: Vec<PathBuf>,
    },
}
