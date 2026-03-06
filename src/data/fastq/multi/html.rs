use super::schema::FastqOverviewSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fastq_multi_report.html")]
pub struct FastqReportTemplate<'a> {
    pub runtime: String,
    pub generated_at: u64,
    pub records: &'a [FastqOverviewSummary],
}

impl FastqReportTemplate<'_> {
    pub fn num_files(&self) -> usize {
        self.records.len()
    }

    pub fn total_reads(&self) -> usize {
        self.records.iter().map(|r| r.num_reads).sum()
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.num_bases).sum()
    }
}
