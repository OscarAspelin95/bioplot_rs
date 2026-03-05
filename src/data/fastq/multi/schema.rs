use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastqOverviewSummary {
    pub sample_name: String,
    pub num_reads: usize,
    pub num_bases: usize,
    pub mean_phred: u8,
}
