use super::html::FastaReportTemplate;
use super::schema::FastaDetailSummary;
use crate::{data::utils::single_file_spinner, errors::AppError};
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{gc_content, nucleotide_counts, nucleotide_probabilities, shannon_entropy},
};
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn parse(fasta: PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("single_fasta.html"));

    let mut reader = needletail_reader(Some(fasta))?;
    let mut records: Vec<FastaDetailSummary> = vec![];

    let spinner = single_file_spinner();

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let contig_name = String::from_utf8_lossy(record.id()).to_string();
        let len = record.num_bases();
        let gc = gc_content(&record.seq());

        // nucleotide counts and entropy.
        let (canonical, num_softmask, num_hardmask) = nucleotide_counts(&record.seq());
        let probs = nucleotide_probabilities(&canonical);
        let entropy = shannon_entropy(&probs);

        records.push(FastaDetailSummary {
            contig_name,
            len,
            gc: gc,
            entropy,
            num_softmask,
            num_hardmask,
        });
    }

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastaReportTemplate { records: &records }.write_into(&mut writer)?;
    spinner.finish_with_message("done.");

    Ok(())
}
