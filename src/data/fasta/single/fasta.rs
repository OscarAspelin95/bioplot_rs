use super::html::FastaReportTemplate;
use super::schema::FastaDetailSummary;
use crate::{
    data::utils::{compute_sha256, format_duration, single_file_spinner, unix_timestamp},
    errors::AppError,
};
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{gc_content, nucleotide_counts, nucleotide_probabilities, shannon_entropy},
};
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

pub fn parse(fasta: PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let start = Instant::now();
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("single_fasta.html"));

    let file_name = fasta
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let sha256 = compute_sha256(&fasta)?;

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
            gc,
            entropy,
            num_softmask,
            num_hardmask,
        });
    }

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastaReportTemplate {
        file_name,
        sha256,
        runtime: format_duration(start),
        generated_at: unix_timestamp(),
        records: &records,
    }
    .write_into(&mut writer)?;
    spinner.finish_with_message("done.");

    Ok(())
}
