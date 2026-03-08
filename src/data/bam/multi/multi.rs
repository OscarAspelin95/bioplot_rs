use super::html::BamMultiReportTemplate;
use super::schema::BamMultiDetailSummary;
use crate::data::bam::single::utils::{get_reference_index, index_bam, is_primary_alignment};
use crate::{
    data::utils::{format_duration, multi_file_spinner, unix_timestamp},
    errors::AppError,
};
use askama::Template;
use noodles::bam;
use noodles::sam::alignment::record::cigar::op::Kind;
use rayon::prelude::*;
use std::{fs::File, io::BufWriter, path::PathBuf, time::Instant};

pub fn parse(bams: Vec<PathBuf>, outfile: Option<PathBuf>) -> Result<(), AppError> {
    let start = Instant::now();
    let outfile = outfile.unwrap_or_else(|| PathBuf::from("multi_bam.html"));

    let total = bams.len() as u64;
    let spinner = multi_file_spinner(total);

    let records: Vec<BamMultiDetailSummary> = bams
        .into_par_iter()
        .flat_map(|bam_path| {
            let sample_name = bam_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let file = match File::open(&bam_path) {
                Ok(f) => f,
                Err(_) => {
                    spinner.inc(1);
                    return vec![];
                }
            };

            index_bam(&bam_path);

            let mut reader = bam::io::Reader::new(file);
            let (contig_name_index, contig_len_index, mut contig_bases_index) =
                get_reference_index(&mut reader);

            for record in reader.records() {
                let record = match record {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                if !is_primary_alignment(&record.flags()) {
                    continue;
                }

                let contig_id = match record.reference_sequence_id() {
                    Some(Ok(id)) => id,
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

            let mut summaries: Vec<BamMultiDetailSummary> = Vec::new();
            for ((contig_name, contig_length), contig_bases) in contig_name_index
                .iter()
                .zip(contig_len_index.iter())
                .zip(contig_bases_index.iter())
            {
                let mean_coverage = *contig_bases as f64 / *contig_length as f64;
                summaries.push(BamMultiDetailSummary {
                    sample_name: sample_name.clone(),
                    contig_name: contig_name.clone(),
                    contig_length: *contig_length,
                    mean_coverage,
                });
            }

            spinner.inc(1);
            summaries
        })
        .collect();

    let mut writer = BufWriter::new(File::create(outfile)?);

    BamMultiReportTemplate {
        runtime: format_duration(start),
        generated_at: unix_timestamp(),
        records: &records,
    }
    .write_into(&mut writer)?;
    spinner.finish_with_message("done.");

    Ok(())
}
