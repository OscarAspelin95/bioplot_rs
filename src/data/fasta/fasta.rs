use super::html::FastaReportTemplate;
use super::schema::FastaSummary;
use crate::errors::AppError;
use askama::Template;
use bio_utils_rs::io::needletail_reader;
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn parse_fasta(fastas: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("fasta.html"));

    let fasta_summary: Vec<FastaSummary> = fastas
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

            Some(FastaSummary {
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

    Ok(())
}
