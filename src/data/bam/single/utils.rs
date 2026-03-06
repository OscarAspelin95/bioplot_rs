use noodles::bam;
use noodles::bam::io::reader::Reader as BamReader;
use noodles::bgzf::io::reader::Reader as ZipReader;
use noodles::sam::alignment::record::Flags;
use std::fs::File;
use std::path::Path;

pub fn get_reference_index(
    reader: &mut BamReader<ZipReader<File>>,
) -> (Vec<String>, Vec<usize>, Vec<usize>) {
    let header = reader.read_header().expect("could not get header.");

    let num_contigs = header.reference_sequences().len();

    let mut contig_name_index = vec![String::new(); num_contigs];
    let mut contig_len_index = vec![0_usize; num_contigs];
    let contig_bases_index = vec![0_usize; num_contigs];

    for (id, (name, reference)) in header.reference_sequences().iter().enumerate() {
        let contig_len = reference.length().get();

        contig_name_index[id] = name.to_string();
        contig_len_index[id] = contig_len;
    }

    (contig_name_index, contig_len_index, contig_bases_index)
}

pub fn index_bam(bam: &Path) {
    bam::fs::index(bam).expect("failed to index BAM");
}

pub fn is_primary_alignment(flags: &Flags) -> bool {
    !(flags.is_unmapped() || flags.is_secondary() || flags.is_supplementary())
}
