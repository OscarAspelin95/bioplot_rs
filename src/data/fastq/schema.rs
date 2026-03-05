use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastqSummary {
    pub sample_name: String,
    pub num_reads: usize,
    pub num_bases: usize,
    pub mean_phred: u8,
}

impl FastqSummary {
    pub fn mean_read_len(&self) -> usize {
        self.num_bases / self.num_reads
    }
}
