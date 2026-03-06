use super::html::FastqReportTemplate;
use super::schema::FastqOverviewSummary;
use crate::{
    data::utils::{format_duration, multi_file_spinner, unix_timestamp},
    errors::AppError,
};
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{PHRED_TO_ERROR, error_to_phred},
};
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

pub fn parse(fastqs: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let start = Instant::now();
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("multi_fastq.html"));

    let total = fastqs.len() as u64;
    let spinner = multi_file_spinner(total);

    let fastq_summary: Vec<FastqOverviewSummary> = fastqs
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

            let summary = Some(FastqOverviewSummary {
                sample_name,
                num_reads,
                num_bases,
                mean_phred,
            });
            spinner.inc(1);
            summary
        })
        .collect();

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastqReportTemplate {
        runtime: format_duration(start),
        generated_at: unix_timestamp(),
        records: &fastq_summary,
    }
    .write_into(&mut writer)?;

    spinner.finish_with_message("done");

    Ok(())
}
