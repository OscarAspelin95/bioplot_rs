use super::schema::FastaOverviewSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fasta_multi_report.html")]
pub struct FastaReportTemplate<'a> {
    pub runtime: String,
    pub generated_at: u64,
    pub records: &'a [FastaOverviewSummary],
}

impl FastaReportTemplate<'_> {
    pub fn num_files(&self) -> usize {
        self.records.len()
    }

    pub fn total_contigs(&self) -> usize {
        self.records.iter().map(|r| r.num_contigs).sum()
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.num_bases).sum()
    }
}
