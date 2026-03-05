use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastqDetailSummary {
    pub read_name: String,
    pub len: usize,
    pub gc: f64,
    pub mean_error: f64,
    pub mean_phred: u8,
}
