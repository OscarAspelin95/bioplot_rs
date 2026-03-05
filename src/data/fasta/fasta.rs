use super::html::FastaReportTemplate;
use super::schema::FastaRecord;
use crate::errors::AppError;
use askama::Template;
use bio_utils_rs::{io::needletail_reader, nucleotide::gc_content};
use std::{fs::File, io::BufWriter, path::PathBuf};

/// We'll start with just collecting stats per contig
pub fn parse_fasta(fasta: PathBuf, outfile: PathBuf) -> Result<(), AppError> {
    let mut fasta_records: Vec<FastaRecord> = vec![];

    let mut reader = needletail_reader(Some(fasta))?;

    while let Some(record) = reader.next() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        let name = String::from_utf8_lossy(record.id()).to_string();
        let num_bases = record.num_bases();
        let gc = gc_content(&record.seq());

        fasta_records.push(FastaRecord {
            name,
            num_bases,
            gc,
        });
    }

    let mut writer = BufWriter::new(File::create(outfile)?);

    FastaReportTemplate {
        records: &fasta_records,
    }
    .write_into(&mut writer)?;

    Ok(())
}
