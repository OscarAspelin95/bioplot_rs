use super::schema::FastqDetailSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fastq_single_report.html")]
pub struct FastqReportTemplate<'a> {
    pub records: &'a [FastqDetailSummary],
}
