use super::schema::BamMultiDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "bam_multi_report.html")]
pub struct BamMultiReportTemplate<'a> {
    pub runtime: String,
    pub generated_at: u64,
    pub records: &'a [BamMultiDetailSummary],
}

impl BamMultiReportTemplate<'_> {
    pub fn num_files(&self) -> usize {
        let mut names: Vec<&str> = self.records.iter().map(|r| r.sample_name.as_str()).collect();
        names.sort_unstable();
        names.dedup();
        names.len()
    }

    pub fn total_contigs(&self) -> usize {
        self.records.len()
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.contig_length).sum()
    }

    pub fn mean_coverage(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.mean_coverage).sum::<f64>() / self.records.len() as f64
    }

    pub fn sample_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .records
            .iter()
            .map(|r| r.sample_name.clone())
            .collect();
        names.sort_unstable();
        names.dedup();
        names
    }
}
