use super::html::FastqReportTemplate;
use super::schema::FastqDetailSummary;
use crate::{
    data::utils::{compute_sha256, format_duration, single_file_spinner, unix_timestamp},
    errors::AppError,
};
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{PHRED_TO_ERROR, error_to_phred, gc_content},
};
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

pub fn parse(fastq: PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let start = Instant::now();
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("single_fastq.html"));

    let file_name = fastq
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let sha256 = compute_sha256(&fastq)?;

    let mut reader = needletail_reader(Some(fastq))?;
    let mut records: Vec<FastqDetailSummary> = vec![];

    let spinner = single_file_spinner();

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let read_name = String::from_utf8_lossy(record.id()).to_string();

        let qual = record
            .qual()
            .expect("Sample {sample_name} is missing quality");

        let gc = gc_content(&record.seq());

        let error_sum = qual
            .iter()
            .map(|phred| PHRED_TO_ERROR[*phred as usize])
            .sum::<f64>();

        let mean_error = error_sum / record.num_bases() as f64;
        let mean_phred = error_to_phred(mean_error);

        records.push(FastqDetailSummary {
            read_name,
            len: record.num_bases(),
            gc,
            mean_error,
            mean_phred,
        })
    }

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastqReportTemplate {
        file_name,
        sha256,
        runtime: format_duration(start),
        generated_at: unix_timestamp(),
        low_qual_phred: 10,
        records: &records,
    }
    .write_into(&mut writer)?;
    spinner.finish_with_message("done.");

    Ok(())
}
