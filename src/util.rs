use std::time::SystemTime;

use chrono_humanize::{Accuracy, HumanTime, Tense};
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
