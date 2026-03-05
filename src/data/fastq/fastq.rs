use super::html::FastqReportTemplate;
use super::schema::FastqSummary;
use crate::errors::AppError;
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{PHRED_TO_ERROR, error_to_phred},
};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn parse_fastq(fastqs: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("fastq.html"));

    let total = fastqs.len() as u64;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files parsed ({elapsed})")
            .unwrap()
            .progress_chars("=>-"),
    );

    let fastq_summary: Vec<FastqSummary> = fastqs
        .into_par_iter()
        .filter_map(|fastq| {
            let sample_name = fastq
                .file_name()
                .map(|name| name.display().to_string())
                .unwrap_or("unknown".to_string());

            let mut reader = match needletail_reader(Some(fastq)) {
                Ok(reader) => reader,
                Err(_) => return None,
            };

            let mut num_reads: usize = 0;
            let mut num_bases: usize = 0;
            let mut error_sum: f64 = 0.0;

            while let Some(record) = reader.next() {
                let record = match record {
                    Ok(record) => record,
                    Err(_) => continue,
                };

                num_bases += record.num_bases();
                num_reads += 1;

                let qual = record
                    .qual()
                    .expect("Sample {sample_name} is missing quality");

                qual.iter()
                    .for_each(|phred| error_sum += PHRED_TO_ERROR[*phred as usize]);
            }

            let mean_error = error_sum / num_bases as f64;
            let mean_phred = error_to_phred(mean_error);

            let summary = Some(FastqSummary {
                sample_name,
                num_reads,
                num_bases,
                mean_phred,
            });
            pb.inc(1);
            summary
        })
        .collect();

    pb.finish_with_message("done");

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastqReportTemplate {
        records: &fastq_summary,
    }
    .write_into(&mut writer)?;

    Ok(())
}
