use super::schema::FastaDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fasta_single_report.html")]
pub struct FastaReportTemplate<'a> {
    pub file_name: String,
    pub sha256: String,
    pub runtime: String,
    pub generated_at: u64,
    pub records: &'a [FastaDetailSummary],
}

impl FastaReportTemplate<'_> {
    pub fn total_contigs(&self) -> usize {
        self.records.len()
    }

    pub fn total_bases(&self) -> usize {
        self.records.iter().map(|r| r.len).sum()
    }

    pub fn mean_length(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }
        self.total_bases() / self.total_contigs()
    }

    pub fn mean_gc(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        self.records.iter().map(|r| r.gc).sum::<f64>() / self.records.len() as f64
    }

    pub fn largest_contig(&self) -> usize {
        self.records.iter().map(|r| r.len).max().unwrap_or(0)
    }

    pub fn smallest_contig(&self) -> usize {
        self.records.iter().map(|r| r.len).min().unwrap_or(0)
    }

    pub fn n50(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }
        let mut lengths: Vec<usize> = self.records.iter().map(|r| r.len).collect();
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        let half = self.total_bases() / 2;
        let mut cumsum = 0;
        for len in &lengths {
            cumsum += len;
            if cumsum >= half {
                return *len;
            }
        }
        0
    }

    pub fn l50(&self) -> usize {
        if self.records.is_empty() {
            return 0;
        }
        let mut lengths: Vec<usize> = self.records.iter().map(|r| r.len).collect();
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        let half = self.total_bases() / 2;
        let mut cumsum = 0;
        for (i, len) in lengths.iter().enumerate() {
            cumsum += len;
            if cumsum >= half {
                return i + 1;
            }
        }
        0
    }
}
