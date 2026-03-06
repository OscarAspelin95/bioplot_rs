use super::schema::FastqDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fastq_single_report.html")]
pub struct FastqReportTemplate<'a> {
    pub file_name: String,
    pub sha256: String,
    pub runtime: String,
    pub generated_at: u64,
    pub low_qual_phred: usize,
    pub records: &'a [FastqDetailSummary],
}

impl<'a> FastqReportTemplate<'a> {
    pub fn total_reads(&self) -> usize {
        self.records.len()
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.len).sum()
    }

    pub fn mean_length(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }
        self.total_bases() / self.total_reads()
    }

    pub fn mean_gc(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.gc).sum::<f64>() / self.records.len() as f64
    }

    pub fn mean_phred(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records
            .iter()
            .map(|r| r.mean_phred as f64)
            .sum::<f64>()
            / self.records.len() as f64
    }

    pub fn low_qual_records(&'a self) -> impl Iterator<Item = &'a FastqDetailSummary> {
        self.records
            .iter()
            .filter(|record| record.mean_phred < self.low_qual_phred as u8)
            .take(100)
    }
}
