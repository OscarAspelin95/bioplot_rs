use super::html::FastqReportTemplate;
use super::schema::FastqDetailSummary;
use crate::{data::utils::single_file_spinner, errors::AppError};
use askama::Template;
use bio_utils_rs::{
    io::needletail_reader,
    nucleotide::{PHRED_TO_ERROR, error_to_phred, gc_content},
};
use std::{fs::File, io::BufWriter, path::PathBuf};

pub fn parse(fastq: PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("single_fastq.html"));

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

    FastqReportTemplate { records: &records }.write_into(&mut writer)?;
    spinner.finish_with_message("done.");

    Ok(())
}
