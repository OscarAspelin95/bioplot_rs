use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use std::{
    io::Read,
    path::Path,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

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

pub fn compute_sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let result = hasher.finalize();
    Ok(result.iter().map(|b| format!("{:02x}", b)).collect())
}

pub fn format_duration(start: Instant) -> String {
    let secs = start.elapsed().as_secs_f64();
    if secs < 60.0 {
        format!("{:.1}s", secs)
    } else if secs < 3600.0 {
        format!("{}m {:.0}s", (secs / 60.0) as u64, secs % 60.0)
    } else {
        format!(
            "{}h {}m",
            (secs / 3600.0) as u64,
            ((secs % 3600.0) / 60.0) as u64
        )
    }
}

pub fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
