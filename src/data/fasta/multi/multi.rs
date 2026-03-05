use super::html::FastaReportTemplate;
use super::schema::FastaOverviewSummary;
use crate::{data::utils::multi_file_spinner, errors::AppError};
use askama::Template;
use bio_utils_rs::io::needletail_reader;
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn parse(fastas: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("multi_fasta.html"));

    let total = fastas.len() as u64;
    let spinner = multi_file_spinner(total);

    let fasta_summary: Vec<FastaOverviewSummary> = fastas
        .into_par_iter()
        .filter_map(|fasta| {
            let sample_name = fasta
                .file_name()
                .map(|name| name.display().to_string())
                .unwrap_or("unknown".to_string());

            let mut reader = match needletail_reader(Some(fasta)) {
                Ok(reader) => reader,
                Err(_) => return None,
            };

            let mut num_contigs: usize = 0;
            let mut num_bases: usize = 0;

            while let Some(record) = reader.next() {
                let record = match record {
                    Ok(record) => record,
                    Err(_) => continue,
                };

                num_bases += record.num_bases();
                num_contigs += 1;
            }
            spinner.inc(1);

            Some(FastaOverviewSummary {
                sample_name,
                num_contigs,
                num_bases,
            })
        })
        .collect();

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastaReportTemplate {
        records: &fasta_summary,
    }
    .write_into(&mut writer)?;
    spinner.finish_with_message("done");

    Ok(())
}
