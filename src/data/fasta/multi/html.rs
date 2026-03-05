use super::schema::FastaOverviewSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fasta_multi_report.html")]
pub struct FastaReportTemplate<'a> {
    pub records: &'a [FastaOverviewSummary],
}
