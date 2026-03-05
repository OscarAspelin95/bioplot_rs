use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastaDetailSummary {
    pub contig_name: String,
    pub len: usize,
    pub gc: f64,
    pub entropy: f32,
    pub num_softmask: usize,
    pub num_hardmask: usize,
}
