use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FastaRecord {
    pub name: String,
    pub num_bases: usize,
    pub gc: f64,
}
