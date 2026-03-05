use super::schema::FastaDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fasta_single_report.html")]
pub struct FastaReportTemplate<'a> {
    pub records: &'a [FastaDetailSummary],
}
