# bioplot_rs
CLI tool for visualizing bioinformatic data.

## Motivation
There is a need for high-performance CLI tools that can quickly summarize and visualize large amounts of bioinformatic data. However, existing tools usually either:
- Only support one file format (e.g., FASTQ)
- Cannot handle huge amounts of data efficiently.

`bioplot_rs` changes this by using Rust for heavy computations and industry standard frameworks for visualization.

## Supported data formats

|Format|Visualizations|
|--|--|
|FASTA	| Length vs GC scatter|
|FASTQ	| Length vs Phred scatter|
|BAM	| [to be implemented]|

## Tech Stack
- [needletail](https://docs.rs/needletail/latest/needletail/) for FASTA/FASTQ parsing.
- [noodles](https://docs.rs/noodles/latest/noodles/) for BAM parsing.
- [askama](https://docs.rs/askama/latest/askama/) for HTML generation.
- [headless_chrome](https://docs.rs/headless_chrome/latest/headless_chrome/) for PDF generation. 
- [DataTables.js](https://datatables.net/) for interactive tables.
- [Apache ECharts](https://echarts.apache.org/en/index.html) for plotting.

## AI disclaimer
I've used Claude for most of the HTML and CSS parts. With that said, the core Rust parts have been written by me.
