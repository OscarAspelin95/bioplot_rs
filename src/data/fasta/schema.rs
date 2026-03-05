use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastaSummary {
    pub sample_name: String,
    pub num_contigs: usize,
    pub num_bases: usize,
}

impl FastaSummary {
    pub fn mean_contig_len(&self) -> usize {
        self.num_bases / self.num_contigs
    }
}
