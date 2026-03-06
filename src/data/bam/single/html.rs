use super::schema::BamDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "bam_single_report.html")]
pub struct BamReportTemplate<'a> {
    pub file_name: String,
    pub sha256: String,
    pub runtime: String,
    pub generated_at: u64,
    pub records: &'a [BamDetailSummary],
}

impl BamReportTemplate<'_> {
    pub fn total_contigs(&self) -> usize {
        self.records.len()
    }

    pub fn mean_coverage(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.mean_coverage).sum::<f64>() / self.records.len() as f64
    }

    pub fn max_coverage(&self) -> f64 {
        self.records
            .iter()
            .map(|r| r.mean_coverage)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn min_coverage(&self) -> f64 {
        self.records
            .iter()
            .map(|r| r.mean_coverage)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.contig_length).sum()
    }
}
