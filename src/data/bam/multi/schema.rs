use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BamMultiDetailSummary {
    pub sample_name: String,
    pub contig_name: String,
    pub contig_length: usize,
    pub mean_coverage: f64,
}
