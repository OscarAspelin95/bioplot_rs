use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastaOverviewSummary {
    pub sample_name: String,
    pub num_contigs: usize,
    pub num_bases: usize,
}

impl FastaOverviewSummary {
    pub fn mean_contig_len(&self) -> usize {
        self.num_bases / self.num_contigs
    }
}
