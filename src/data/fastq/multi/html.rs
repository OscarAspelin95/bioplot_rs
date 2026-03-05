use super::schema::FastqOverviewSummary;
use askama::Template;

#[derive(Template)]
#[template(path = "fastq_multi_report.html")]
pub struct FastqReportTemplate<'a> {
    pub records: &'a [FastqOverviewSummary],
}
