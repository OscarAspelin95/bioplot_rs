use super::schema::FastaRecord;
use askama::Template;

#[derive(Template)]
#[template(path = "fasta_report.html")]
pub struct FastaReportTemplate<'a> {
    pub records: &'a [FastaRecord],
}
