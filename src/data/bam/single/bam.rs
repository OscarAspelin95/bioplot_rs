use super::html::BamReportTemplate;
use super::schema::BamDetailSummary;
use super::utils::{get_reference_index, index_bam, is_primary_alignment};
use crate::{
    data::utils::{compute_sha256, format_duration, single_file_spinner, unix_timestamp},
    errors::AppError,
};
use askama::Template;
use noodles::bam;
use noodles::sam::alignment::record::cigar::op::Kind;
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

pub fn parse(bam_path: PathBuf, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let start = Instant::now();
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("single_bam.html"));

    let file_name = bam_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let sha256 = compute_sha256(&bam_path)?;

    let mut reader = bam::io::Reader::new(File::open(&bam_path).expect("failed to open BAM"));

    let spinner = single_file_spinner();
    index_bam(&bam_path);

    let (contig_name_index, contig_len_index, mut contig_bases_index) =
        get_reference_index(&mut reader);

    for record in reader.records() {
        let record = match record {
            Ok(record) => record,
            Err(_) => continue,
        };

        if !is_primary_alignment(&record.flags()) {
            continue;
        }

        let contig_id = match record.reference_sequence_id() {
            Some(Ok(contig_id)) => contig_id,
            _ => continue,
        };

        let valid_bases: usize = record
            .cigar()
            .iter()
            .filter_map(|c| c.ok())
            .filter(|op| {
                matches!(
                    op.kind(),
                    Kind::Match | Kind::SequenceMatch | Kind::SequenceMismatch
                )
            })
            .map(|op| op.len())
            .sum();

        contig_bases_index[contig_id] += valid_bases;
    }

    let mut records: Vec<BamDetailSummary> = vec![];

    for ((contig_name, contig_length), contig_bases) in contig_name_index
        .iter()
        .zip(contig_len_index.iter())
        .zip(contig_bases_index.iter())
    {
        let mean_coverage = *contig_bases as f64 / *contig_length as f64;

        records.push(BamDetailSummary {
            contig_name: contig_name.clone(),
            contig_length: *contig_length,
            mean_coverage,
        });
    }

    let mut writer = BufWriter::new(File::create(outfile)?);

    BamReportTemplate {
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
