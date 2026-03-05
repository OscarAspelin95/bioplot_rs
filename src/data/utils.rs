use indicatif::{ProgressBar, ProgressStyle};

pub fn multi_file_spinner(total: u64) -> ProgressBar {
    let spinner = ProgressBar::new(total);

    spinner.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files parsed ({elapsed})",
        )
        .expect("failed to initialize progress spinner.")
        .progress_chars("=>-"),
    );

    spinner
}

pub fn single_file_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();

    spinner.set_style(
        ProgressStyle::with_template("{spinner:.green} ({elapsed}) processing file")
            .expect("failed to initialize progres spinner."),
    );

    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner
}
