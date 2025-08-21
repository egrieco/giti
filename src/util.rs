use chrono_humanize::{Accuracy, HumanTime, Tense};
use color_eyre::Result;
use std::{fs, path::Path, time::SystemTime};
use yansi::{Color::*, Paint, Style};

pub(crate) fn format_display_time(time: SystemTime) -> String {
    // Convert timestamp to human-readable format
    let datetime: chrono::DateTime<chrono::Local> = time.into();

    // Calculate human-readable time difference
    let now = chrono::Local::now();
    let duration = now.signed_duration_since(datetime);

    let human_duration = HumanTime::from(duration).to_text_en(Accuracy::Rough, Tense::Past);

    // Color the output based on the duration
    let color: Style = if duration.num_days() < 7 {
        // Green for less than a week
        Green.into()
    } else if duration.num_days() < 30 {
        // Blue for less than a month
        Blue.into()
    } else if duration.num_days() < 365 {
        // Yellow for less than a year
        Yellow.into()
    } else {
        // Red for any other interval
        Red.into()
    };

    format!(
        "{} ({})",
        datetime.format("%Y-%m-%d %H:%M:%S"),
        human_duration.paint(color)
    )
}

pub(crate) fn calculate_directory_size(dir: &Path) -> Result<u64> {
    let mut total_size = 0;

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                total_size += calculate_directory_size(&path)?;
            } else {
                total_size += entry.metadata()?.len();
            }
        }
    }

    Ok(total_size)
}
