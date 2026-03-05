use super::schema::FastqSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fastq_report.html")]
pub struct FastqReportTemplate<'a> {
    pub records: &'a [FastqSummary],
}
